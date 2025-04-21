use std::collections::BTreeMap;

use bitcoin::{
    bip32::{DerivationPath, Fingerprint},
    key::Secp256k1,
    opcodes::all::OP_RETURN,
    psbt::{Input, PsbtSighashType},
    script,
    secp256k1::All,
    taproot::{LeafVersion, TaprootSpendInfo},
    Amount, PublicKey, ScriptBuf, TapLeafHash, TapSighashType, Transaction, TxOut, XOnlyPublicKey,
};
use lazy_static::lazy_static;

use super::{
    CoreError, DataScript, PreviousOutpoint, TransactionBuilder, UnlockingFeeParams,
    UnlockingTaprootTreeType, HASH_SIZE, UNLOCKING_EMBEDDED_DATA_SCRIPT_SIZE,
};

lazy_static! {
    static ref SECP: Secp256k1<All> = Secp256k1::new();
}

pub fn get_global_secp() -> &'static Secp256k1<All> {
    &SECP
}

#[derive(Debug)]
pub struct VaultManager {
    tag: Vec<u8>,
    service_tag: Vec<u8>,
    version: u8,
    network_id: u8,
}

pub struct UnlockingParams<'a> {
    pub total_input_value: Amount,
    pub total_output_value: Amount,
    pub inputs: &'a [PreviousOutpoint],
    pub outputs: &'a [TxOut],
    pub tree_type: UnlockingTaprootTreeType,
    pub script: &'a ScriptBuf,
    pub rbf: bool,
    pub fee_rate: u64,
    pub custodian_quorum: u8,
    pub session_sequence: u64,
    pub custodian_group_uid: [u8; HASH_SIZE],
}

impl VaultManager {
    pub fn new(tag: Vec<u8>, service_tag: Vec<u8>, version: u8, network_id: u8) -> Self {
        Self {
            tag,
            service_tag,
            version,
            network_id,
        }
    }

    pub fn tag(&self) -> &Vec<u8> {
        &self.tag
    }

    pub fn service_tag(&self) -> &Vec<u8> {
        &self.service_tag
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn network_id(&self) -> u8 {
        self.network_id
    }

    pub fn prepare_psbt_inputs(
        &self,
        inputs: &[PreviousOutpoint],
        tree: &TaprootSpendInfo,
        branch: &ScriptBuf,
        keys: &[XOnlyPublicKey],
    ) -> Vec<Input> {
        let tap_key_origins = self.create_tap_key_origins(branch, keys);

        let tap_scripts = self.create_tap_scripts(tree, branch);

        self.create_psbt_inputs(inputs, tree, &tap_scripts, &tap_key_origins)
    }

    fn create_tap_key_origins(
        &self,
        script: &ScriptBuf,
        keys: &[XOnlyPublicKey],
    ) -> BTreeMap<XOnlyPublicKey, (Vec<TapLeafHash>, (Fingerprint, DerivationPath))> {
        let mut tap_key_origins = BTreeMap::new();

        for &key in keys {
            tap_key_origins.insert(
                key,
                (
                    vec![script.tapscript_leaf_hash()],
                    ([0u8; 4].into(), DerivationPath::default()),
                ),
            );
        }

        tap_key_origins
    }

    fn create_tap_scripts(
        &self,
        tree: &bitcoin::taproot::TaprootSpendInfo,
        script: &ScriptBuf,
    ) -> BTreeMap<bitcoin::taproot::ControlBlock, (ScriptBuf, LeafVersion)> {
        let mut map = BTreeMap::new();
        map.insert(
            tree.control_block(&(script.clone(), LeafVersion::TapScript))
                .unwrap(),
            (script.clone(), LeafVersion::TapScript),
        );
        map
    }

    fn create_psbt_inputs(
        &self,
        inputs: &[PreviousOutpoint],
        tree: &bitcoin::taproot::TaprootSpendInfo,
        tap_scripts: &BTreeMap<bitcoin::taproot::ControlBlock, (ScriptBuf, LeafVersion)>,
        tap_key_origins: &BTreeMap<
            XOnlyPublicKey,
            (Vec<TapLeafHash>, (Fingerprint, DerivationPath)),
        >,
    ) -> Vec<Input> {
        inputs
            .iter()
            .map(|input| {
                Input {
                    // Add the UTXO being spent
                    witness_utxo: Some(TxOut {
                        value: input.amount_in_sats,
                        script_pubkey: input.script_pubkey.clone(),
                    }),

                    // Add Taproot-specific data
                    tap_internal_key: Some(tree.internal_key()),
                    tap_merkle_root: tree.merkle_root(),

                    // Add the script we're using to spend
                    tap_scripts: tap_scripts.clone(),

                    tap_key_origins: tap_key_origins.clone(),

                    // Set default sighash type for Taproot
                    sighash_type: Some(PsbtSighashType::from(TapSighashType::Default)),

                    ..Default::default()
                }
            })
            .collect()
    }

    pub fn convert_to_x_only_keys(pub_keys: &[PublicKey]) -> Vec<XOnlyPublicKey> {
        pub_keys
            .iter()
            .map(|pk| XOnlyPublicKey::from(*pk))
            .collect()
    }

    fn add_inputs_to_builder(
        &self,
        tx_builder: &mut TransactionBuilder,
        inputs: &[PreviousOutpoint],
    ) {
        for input in inputs {
            tx_builder.add_input(input.outpoint);
        }
    }

    fn add_indexed_output_to_builder(
        &self,
        tx_builder: &mut TransactionBuilder,
        flags: UnlockingTaprootTreeType,
        session_sequence: u64,
        custodian_group_uid: [u8; HASH_SIZE],
    ) -> Result<(), CoreError> {
        let script =
            self.create_indexed_unlocking_script(flags, session_sequence, &custodian_group_uid)?;

        tx_builder.add_output(Amount::ZERO, script.into_script());

        Ok(())
    }

    fn create_indexed_unlocking_script(
        &self,
        flags: UnlockingTaprootTreeType,
        session_sequence: u64,
        custodian_group_uid: &[u8; HASH_SIZE],
    ) -> Result<DataScript, CoreError> {
        let tag_hash = DataScript::compute_tag_hash(self.tag.as_slice())?;
        let service_tag_hash = DataScript::compute_service_tag_hash(self.service_tag.as_slice())?;

        let mut data = Vec::<u8>::with_capacity(UNLOCKING_EMBEDDED_DATA_SCRIPT_SIZE);
        data.extend_from_slice(&tag_hash);
        data.push(self.version);
        data.push(self.network_id);
        data.push(flags as u8);
        data.extend_from_slice(&service_tag_hash);
        data.extend_from_slice(&session_sequence.to_be_bytes());
        data.extend_from_slice(custodian_group_uid);
        let data_slice: &[u8; UNLOCKING_EMBEDDED_DATA_SCRIPT_SIZE] = data
            .as_slice()
            .try_into()
            .map_err(|_| CoreError::CannotConvertOpReturnDataToSlice)?;

        let script = script::Builder::new()
            .push_opcode(OP_RETURN)
            .push_slice(data_slice)
            .into_script();

        Ok(DataScript(script))
    }

    fn calculate_change(&self, total_input_value: Amount, total_output_value: Amount) -> Amount {
        total_input_value - total_output_value
    }

    fn add_change_output_placeholder(
        &self,
        tx_builder: &mut TransactionBuilder,
        script: &ScriptBuf,
    ) {
        tx_builder.add_output(Amount::ZERO, script.clone());
    }

    fn replace_change_output(
        &self,
        unsigned_tx: &mut Transaction,
        change: Amount,
        script: &ScriptBuf,
    ) {
        unsigned_tx.output.pop();
        unsigned_tx.output.push(TxOut {
            value: change,
            script_pubkey: script.clone(),
        });
    }

    // New helper method to extract common transaction building logic
    pub fn build_unlocking_transaction(
        &self,
        params: &UnlockingParams,
    ) -> Result<Transaction, CoreError> {
        let mut tx_builder = TransactionBuilder::new(params.rbf);

        self.add_inputs_to_builder(&mut tx_builder, params.inputs);

        // output[0]: indexed output (op_return)
        // output[1->n-2]: unlocking outputs
        // output[n-1]: change output

        self.add_indexed_output_to_builder(
            &mut tx_builder,
            params.tree_type,
            params.session_sequence,
            params.custodian_group_uid,
        )?;

        tx_builder.add_outputs(params.outputs);

        let change = self.calculate_change(params.total_input_value, params.total_output_value);
        if change > Amount::ZERO {
            self.add_change_output_placeholder(&mut tx_builder, params.script);
        }

        let mut unsigned_tx = tx_builder.build();

        let fee = self.calculate_unlocking_fee(UnlockingFeeParams {
            n_inputs: unsigned_tx.input.len() as u64,
            n_outputs: unsigned_tx.output.len() as u64,
            fee_rate: params.fee_rate,
            quorum: params.custodian_quorum,
        });

        self.distribute_fee(&mut unsigned_tx, params.total_output_value, fee)?;

        if change > Amount::ZERO {
            self.replace_change_output(&mut unsigned_tx, change, params.script);
        }

        Ok(unsigned_tx)
    }
}
