use std::collections::BTreeMap;

use super::{manager, CoreError, TaprootTree, Unstaking, UnstakingOutput, VaultManager, XOnlyKeys};

use super::PreviousStakingUTXO;
use bitcoin::bip32::{DerivationPath, Fingerprint};
use bitcoin::psbt::{Input, PsbtSighashType};
use bitcoin::taproot::{LeafVersion, TaprootSpendInfo};
use bitcoin::{
    absolute, transaction, Amount, OutPoint, Psbt, PublicKey, ScriptBuf, Sequence, TapLeafHash,
    TapSighashType, Transaction, TxIn, TxOut, Witness, XOnlyPublicKey,
};
use validator::Validate;

#[derive(Debug, PartialEq)]
pub enum UnstakingType {
    UserProtocol,
    CovenantsProtocol,
    CovenantsUser,
    OnlyCovenants,
}

/// Because the unstaking tx is formed from a previous staking tx, 1 - 1 mapping is used.
/// So we just need one input and one output.
#[derive(Debug, Validate)]
pub struct BuildUnstakingParams {
    pub input: PreviousStakingUTXO,
    pub locking_script: ScriptBuf,
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub covenant_pub_keys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub have_only_covenants: bool,
    pub rbf: bool,
    pub fee_rate: u64,
}

#[derive(Debug, Validate)]
pub struct BuildUnstakingWithOnlyCovenantsParams {
    pub inputs: Vec<PreviousStakingUTXO>,
    pub unstaking_output: UnstakingOutput,
    pub covenant_pub_keys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub rbf: bool,
    pub fee_rate: u64,
}

impl Unstaking for VaultManager {
    type Error = CoreError;

    fn build(
        &self,
        params: &BuildUnstakingParams,
        unstaking_type: UnstakingType,
    ) -> Result<Psbt, Self::Error> {
        // Validate params
        if unstaking_type == UnstakingType::OnlyCovenants && !params.have_only_covenants {
            return Err(CoreError::InvalidUnstakingType);
        }

        let x_only_keys = manager::VaultManager::convert_to_x_only_keys(
            &params.user_pub_key,
            &params.protocol_pub_key,
            &params.covenant_pub_keys,
        );

        let tree = TaprootTree::new(
            self.secp(),
            &x_only_keys.user,
            &x_only_keys.protocol,
            &x_only_keys.covenants,
            params.covenant_quorum,
            params.have_only_covenants,
        )?;

        let (branch, keys) =
            UnstakingKeys::get_branch_and_keys_for_type(&x_only_keys, unstaking_type, &tree);

        let mut tx_builder = UnstakingTransactionBuilder::new(params.rbf);
        tx_builder.add_input(params.input.outpoint);
        tx_builder.add_output(Amount::ZERO, params.locking_script.clone());

        let mut unsigned_tx = tx_builder.build();
        let fee = self.calculate_transaction_fee(
            unsigned_tx.output.len() as u64,
            unsigned_tx.input.len() as u64,
            params.fee_rate,
        );

        unsigned_tx.output[0].value = params.input.amount_in_sats - fee;

        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

        psbt.inputs = self.prepare_psbt_inputs(&[params.input.clone()], &tree.root, branch, &keys);

        Ok(psbt)
    }

    fn build_with_only_covenants(
        &self,
        params: &BuildUnstakingWithOnlyCovenantsParams,
    ) -> Result<Psbt, Self::Error> {
        let covenants_x_only: Vec<XOnlyPublicKey> = params
            .covenant_pub_keys
            .iter()
            .map(|pk| XOnlyPublicKey::from(*pk))
            .collect();

        let tree = TaprootTree::new_with_only_covenants(
            self.secp(),
            &covenants_x_only,
            params.covenant_quorum,
        )?;

        let only_covenants_script = tree.clone().into_script(self.secp());

        let mut tx_builder = UnstakingTransactionBuilder::new(params.rbf);

        // Add all inputs
        for input in &params.inputs {
            tx_builder.add_input(input.outpoint);
        }

        // Add main output
        tx_builder.add_output(Amount::ZERO, params.unstaking_output.locking_script.clone());

        // Calculate and add change if needed
        let sum_of_inputs = params
            .inputs
            .iter()
            .map(|input| input.amount_in_sats)
            .sum::<Amount>();

        let change = sum_of_inputs - params.unstaking_output.amount_in_sats;
        if change > Amount::ZERO {
            tx_builder.add_output(change, only_covenants_script);
        }

        let mut unsigned_tx = tx_builder.build();

        // Calculate and apply fee
        let fee = self.calculate_transaction_fee(
            unsigned_tx.input.len() as u64,
            unsigned_tx.output.len() as u64,
            params.fee_rate,
        );

        unsigned_tx.output[0].value = params.unstaking_output.amount_in_sats - fee;

        if unsigned_tx.output[0].value < Amount::ZERO {
            return Err(CoreError::InsufficientFunds);
        }

        let (branch, keys) = (
            tree.only_covenants_branch.as_ref().unwrap(),
            covenants_x_only,
        );

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
                &tree.user_protocol_branch,
                vec![x_only_keys.user, x_only_keys.protocol],
            ),
            UnstakingType::CovenantsProtocol => {
                let mut keys = vec![x_only_keys.protocol];
                keys.extend_from_slice(&x_only_keys.covenants);
                (&tree.covenants_protocol_branch, keys)
            }
            UnstakingType::CovenantsUser => {
                let mut keys = vec![x_only_keys.user];
                keys.extend_from_slice(&x_only_keys.covenants);
                (&tree.covenants_user_branch, keys)
            }
            UnstakingType::OnlyCovenants => (
                tree.only_covenants_branch.as_ref().unwrap(),
                x_only_keys.covenants.clone(),
            ),
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
