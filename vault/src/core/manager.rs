use std::collections::BTreeMap;

use bitcoin::{bip32::{DerivationPath, Fingerprint}, key::Secp256k1, psbt::Input, secp256k1::All, taproot::{LeafVersion, TaprootSpendInfo}, Amount, PublicKey, ScriptBuf, TapLeafHash, TxOut, XOnlyPublicKey};
use lazy_static::lazy_static;

use super::PreviousStakingUTXO;

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

#[derive(Debug)]
pub struct XOnlyKeys {
    pub user: XOnlyPublicKey,
    pub protocol: XOnlyPublicKey,
    pub custodians: Vec<XOnlyPublicKey>,
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

    fn prepare_psbt_inputs(
        &self,
        inputs: &[PreviousStakingUTXO],
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
        inputs: &[PreviousStakingUTXO],
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

    fn convert_to_x_only_keys(&self, pub_keys: &[PublicKey]) -> Vec<XOnlyPublicKey> {
        pub_keys
            .iter()
            .map(|pk| XOnlyPublicKey::from(*pk))
            .collect()
    }

    fn add_inputs_to_builder(
        &self,
        tx_builder: &mut UnstakingTransactionBuilder,
        inputs: &[PreviousStakingUTXO],
    ) {
        for input in inputs {
            tx_builder.add_input(input.outpoint);
        }
    }

    fn add_indexed_output_to_builder(
        &self,
        tx_builder: &mut UnstakingTransactionBuilder,
        flags: UnstakingTaprootTreeType,
        session_sequence: u64,
        custodian_group_uid: [u8; HASH_SIZE],
    ) -> Result<(), CoreError> {
        let indexed_output =
            self.create_indexed_output(flags, session_sequence, &custodian_group_uid)?;

        tx_builder.add_output(indexed_output.amount_in_sats, indexed_output.locking_script);

        Ok(())
    }

    fn create_indexed_output(
        &self,
        flags: UnstakingTaprootTreeType,
        session_sequence: u64,
        custodian_group_uid: &[u8; HASH_SIZE],
    ) -> Result<UnstakingOutput, CoreError> {
        let unstaking_script = DataScript::new_unstaking(&UnstakingDataScriptParams {
            tag: self.tag(),
            version: self.version(),
            network_id: self.network_id(),
            service_tag: self.service_tag(),
            flags,
            session_sequence,
            custodian_group_uid,
        })?;
        Ok(UnstakingOutput {
            amount_in_sats: Amount::ZERO,
            locking_script: unstaking_script.into_script(),
        })
    }

    fn add_outputs_to_builder(
        &self,
        tx_builder: &mut UnstakingTransactionBuilder,
        outputs: &[UnstakingOutput],
    ) {
        for output in outputs {
            tx_builder.add_output(output.amount_in_sats, output.locking_script.clone());
        }
    }

    fn calculate_change(&self, total_input_value: Amount, total_output_value: Amount) -> Amount {
        total_input_value - total_output_value
    }

    fn add_change_output_placeholder(
        &self,
        tx_builder: &mut UnstakingTransactionBuilder,
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
    pub fn build_unstaking_transaction(
        &self,
        total_input_value: Amount,
        total_output_value: Amount,
        inputs: &[PreviousStakingUTXO],
        unstaking_outputs: &[UnstakingOutput],
        tree_type: UnstakingTaprootTreeType,
        script: &ScriptBuf,
        rbf: bool,
        fee_rate: u64,
        custodian_quorum: u8,
        session_sequence: u64,
        custodian_group_uid: [u8; HASH_SIZE],
    ) -> Result<Transaction, CoreError> {
        let mut tx_builder = UnstakingTransactionBuilder::new(rbf);

        self.add_inputs_to_builder(&mut tx_builder, inputs);

        // output[0]: indexed output (op_return)
        // output[1->n-2]: unstaking outputs
        // output[n-1]: change output

        self.add_indexed_output_to_builder(
            &mut tx_builder,
            tree_type,
            session_sequence,
            custodian_group_uid,
        )?;

        self.add_outputs_to_builder(&mut tx_builder, unstaking_outputs);

        let change = self.calculate_change(total_input_value, total_output_value);
        if change > Amount::ZERO {
            self.add_change_output_placeholder(&mut tx_builder, script);
        }

        let mut unsigned_tx = tx_builder.build();

        let fee = self.calculate_unstaking_fee(UnstakingFeeParams {
            n_inputs: unsigned_tx.input.len() as u64,
            n_outputs: unsigned_tx.output.len() as u64,
            fee_rate,
            quorum: custodian_quorum,
        });

        self.distribute_fee(&mut unsigned_tx, total_output_value, fee)?;

        if change > Amount::ZERO {
            self.replace_change_output(&mut unsigned_tx, change, script);
        }

        Ok(unsigned_tx)
    }
}
