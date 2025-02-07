use std::collections::BTreeMap;

use crate::core::fee::FeeParams;

use super::{
    manager, CoreError, CustodianOnlyUnstakingParams, DataScript, TaprootTree,
    UPCTaprootTreeParams, UPCUnstakingParams, Unstaking, UnstakingDataScriptParams,
    UnstakingOutput, VaultManager, XOnlyKeys,
};

use super::PreviousStakingUTXO;
use bitcoin::bip32::{DerivationPath, Fingerprint};
use bitcoin::psbt::{Input, PsbtSighashType};
use bitcoin::taproot::{LeafVersion, TaprootSpendInfo};
use bitcoin::{
    absolute, transaction, Amount, OutPoint, Psbt, PublicKey, ScriptBuf, Sequence, TapLeafHash,
    TapSighashType, Transaction, TxIn, TxOut, Witness, XOnlyPublicKey,
};

#[derive(Debug, PartialEq)]
pub enum UnstakingType {
    UserProtocol,
    CustodianProtocol,
    CustodianUser,
}

impl Unstaking for VaultManager {
    type Error = CoreError;

    fn build_upc(
        &self,
        params: &UPCUnstakingParams,
        unstaking_type: UnstakingType,
    ) -> Result<Psbt, Self::Error> {
        let x_only_keys = manager::VaultManager::convert_upc_to_x_only_keys(
            &params.user_pub_key,
            &params.protocol_pub_key,
            &params.custodian_pub_keys,
        );

        let tree = TaprootTree::new_upc(
            self.secp(),
            &UPCTaprootTreeParams {
                user_pub_key: x_only_keys.user,
                protocol_pub_key: x_only_keys.protocol,
                custodian_pub_keys: x_only_keys.custodians.clone(),
                custodian_quorum: params.custodian_quorum,
            },
        )?;

        let (branch, keys) =
            UnstakingKeys::get_branch_and_keys_for_type(&x_only_keys, unstaking_type, &tree);

        let mut tx_builder = UnstakingTransactionBuilder::new(params.rbf);

        tx_builder.add_input(params.input.outpoint);

        // The order of the outputs:
        // [0] indexed output
        // [1] unstaking output

        let indexed_output = self.create_indexed_output()?;

        tx_builder.add_output(indexed_output.amount_in_sats, indexed_output.locking_script);

        // the user will receive the amount they staked in the previous staking tx
        let total_output_value = params.input.amount_in_sats;

        tx_builder.add_output(total_output_value, params.locking_script.clone());

        let mut unsigned_tx = tx_builder.build();

        let fee = self.calculate_transaction_fee(FeeParams {
            n_inputs: unsigned_tx.input.len() as u64,
            n_outputs: unsigned_tx.output.len() as u64,
            fee_rate: params.fee_rate,
        });

        self.distribute_fee(&mut unsigned_tx, total_output_value, fee)?;

        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

        psbt.inputs = self.prepare_psbt_inputs(&[params.input.clone()], &tree.root, branch, &keys);

        Ok(psbt)
    }

    fn build_custodian_only(
        &self,
        params: &CustodianOnlyUnstakingParams,
    ) -> Result<Psbt, Self::Error> {
        let x_only_pub_keys = self.convert_to_x_only_keys(&params.custodian_pub_keys);

        let tree = TaprootTree::new_custodian_only(
            self.secp(),
            &x_only_pub_keys,
            params.custodian_quorum,
        )?;

        let custodian_only_script = tree.clone().into_script(self.secp());

        let mut tx_builder = UnstakingTransactionBuilder::new(params.rbf);

        self.add_inputs_to_builder(&mut tx_builder, &params.inputs);

        // The order of the outputs:
        // [0] indexed output
        // [1 - n-2] unstaking outputs
        // [n-1] change output

        let indexed_output = self.create_indexed_output()?;

        tx_builder.add_output(indexed_output.amount_in_sats, indexed_output.locking_script);

        let total_output_value =
            self.add_outputs_to_builder(&mut tx_builder, &params.unstaking_outputs);

        let change = self.calculate_change(&params.inputs, total_output_value);

        if change > Amount::ZERO {
            self.add_change_output_placeholder(&mut tx_builder, &custodian_only_script);
        }

        let mut unsigned_tx = tx_builder.build();

        let fee = self.calculate_transaction_fee(FeeParams {
            n_inputs: unsigned_tx.input.len() as u64,
            n_outputs: unsigned_tx.output.len() as u64,
            fee_rate: params.fee_rate,
        });

        self.distribute_fee(&mut unsigned_tx, total_output_value, fee)?;

        let (branch, keys) = (
            tree.only_custodian_branch.as_ref().unwrap(),
            x_only_pub_keys,
        );

        if change > Amount::ZERO {
            self.replace_change_output(&mut unsigned_tx, change, &custodian_only_script);
        }

        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

        psbt.inputs = self.prepare_psbt_inputs(&params.inputs, &tree.root, branch, &keys);

        Ok(psbt)
    }
}

impl VaultManager {
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

    fn create_indexed_output(&self) -> Result<UnstakingOutput, CoreError> {
        let unstaking_script = DataScript::new_unstaking(&UnstakingDataScriptParams {
            tag: self.tag(),
            version: self.version(),
            network_id: self.network_id(),
            service_tag: self.service_tag(),
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
    ) -> Amount {
        let mut total_output_value = Amount::ZERO;
        for output in outputs {
            tx_builder.add_output(output.amount_in_sats, output.locking_script.clone());
            total_output_value += output.amount_in_sats;
        }
        total_output_value
    }

    fn calculate_change(
        &self,
        inputs: &[PreviousStakingUTXO],
        total_output_value: Amount,
    ) -> Amount {
        inputs
            .iter()
            .map(|input| input.amount_in_sats)
            .sum::<Amount>()
            - total_output_value
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
}

struct UnstakingTransactionBuilder {
    version: transaction::Version,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    rbf: bool,
}

impl UnstakingTransactionBuilder {
    fn new(rbf: bool) -> Self {
        Self {
            version: transaction::Version::TWO,
            inputs: Vec::new(),
            outputs: Vec::new(),
            rbf,
        }
    }

    fn add_input(&mut self, outpoint: OutPoint) {
        self.inputs.push(TxIn {
            previous_output: outpoint,
            script_sig: ScriptBuf::default(),
            sequence: match self.rbf {
                true => Sequence::ENABLE_RBF_NO_LOCKTIME,
                false => Sequence::MAX,
            },
            witness: Witness::default(),
        });
    }

    fn add_output(&mut self, value: Amount, script_pubkey: ScriptBuf) {
        self.outputs.push(TxOut {
            value,
            script_pubkey,
        });
    }

    fn build(self) -> Transaction {
        Transaction {
            version: self.version,
            lock_time: absolute::LockTime::ZERO,
            input: self.inputs,
            output: self.outputs,
        }
    }
}

struct UnstakingKeys;

impl UnstakingKeys {
    fn get_branch_and_keys_for_type<'a>(
        x_only_keys: &XOnlyKeys,
        unstaking_type: UnstakingType,
        tree: &'a TaprootTree,
    ) -> (&'a ScriptBuf, Vec<XOnlyPublicKey>) {
        match unstaking_type {
            UnstakingType::UserProtocol => (
                tree.user_protocol_branch.as_ref().unwrap(),
                vec![x_only_keys.user, x_only_keys.protocol],
            ),
            UnstakingType::CustodianProtocol => {
                let mut keys = vec![x_only_keys.protocol];
                keys.extend_from_slice(&x_only_keys.custodians);
                (&tree.protocol_custodian_branch.as_ref().unwrap(), keys)
            }
            UnstakingType::CustodianUser => {
                let mut keys = vec![x_only_keys.user];
                keys.extend_from_slice(&x_only_keys.custodians);
                (&tree.user_custodian_branch.as_ref().unwrap(), keys)
            }
        }
    }
}

// Helper trait for Amount calculations
pub trait AmountExt {
    fn checked_sub_fee(&self, fee: Amount) -> Result<Amount, CoreError>;
}

impl AmountExt for Amount {
    fn checked_sub_fee(&self, fee: Amount) -> Result<Amount, CoreError> {
        self.checked_sub(fee).ok_or(CoreError::InsufficientFunds)
    }
}
