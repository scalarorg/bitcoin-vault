use std::collections::BTreeMap;

use super::{manager, CoreError, TaprootTree, Unstaking, VaultManager};

use super::PreviousStakingUTXO;
use bitcoin::bip32::{DerivationPath, Fingerprint};
use bitcoin::psbt::{Input, PsbtSighashType};
use bitcoin::taproot::{LeafVersion, TaprootSpendInfo};
use bitcoin::{
    absolute, transaction, OutPoint, Psbt, PublicKey, ScriptBuf, Sequence, TapLeafHash,
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

#[derive(Debug, Validate)]
pub struct BuildUnstakingParams {
    pub inputs: Vec<PreviousStakingUTXO>,
    pub unstaking_output: TxOut,
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub covenant_pub_keys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub have_only_covenants: bool,
    pub rbf: bool,
}

#[derive(Debug, Validate)]
pub struct BuildUnstakingWithOnlyCovenantsParams {
    pub inputs: Vec<PreviousStakingUTXO>,
    pub unstaking_output: TxOut,
    pub covenant_pub_keys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub rbf: bool,
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

        // Extract common PSBT building logic into a helper method
        self.build_unstaking_psbt(
            &params.inputs,
            &params.unstaking_output,
            params.rbf,
            |secp| {
                let tree = TaprootTree::new(
                    secp,
                    &x_only_keys.user,
                    &x_only_keys.protocol,
                    &x_only_keys.covenants,
                    params.covenant_quorum,
                    params.have_only_covenants,
                )?;

                let cloned_tree = tree.clone();

                let (branch, keys) = match unstaking_type {
                    UnstakingType::UserProtocol => (
                        &cloned_tree.user_protocol_branch,
                        vec![x_only_keys.user, x_only_keys.protocol],
                    ),
                    UnstakingType::CovenantsProtocol => {
                        let mut keys = vec![x_only_keys.protocol];
                        keys.extend_from_slice(&x_only_keys.covenants);
                        (&cloned_tree.covenants_protocol_branch, keys)
                    }
                    UnstakingType::CovenantsUser => {
                        let mut keys = vec![x_only_keys.user];
                        keys.extend_from_slice(&x_only_keys.covenants);
                        (&cloned_tree.covenants_user_branch, keys)
                    }
                    UnstakingType::OnlyCovenants => (
                        cloned_tree.only_covenants_branch.as_ref().unwrap(),
                        x_only_keys.covenants,
                    ),
                };
                Ok((tree, branch.clone(), keys))
            },
        )
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

        // Use the same helper method
        self.build_unstaking_psbt(
            &params.inputs,
            &params.unstaking_output,
            params.rbf,
            |secp| {
                let tree = TaprootTree::new_with_only_covenants(
                    secp,
                    &covenants_x_only,
                    params.covenant_quorum,
                )?;

                Ok((
                    tree.clone(),
                    tree.only_covenants_branch.as_ref().unwrap().clone(),
                    covenants_x_only.clone(),
                ))
            },
        )
    }
}

impl VaultManager {
    fn build_unstaking_psbt<F>(
        &self,
        inputs: &[PreviousStakingUTXO],
        output: &TxOut,
        rbf: bool,
        build_tree: F,
    ) -> Result<Psbt, CoreError>
    where
        F: FnOnce(
            &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>,
        ) -> Result<(TaprootTree, ScriptBuf, Vec<XOnlyPublicKey>), CoreError>,
    {
        let (tree, branch, keys) = build_tree(self.secp())?;

        let mut psbt = self.prepare_psbt(
            &inputs
                .iter()
                .map(|input| input.outpoint)
                .collect::<Vec<OutPoint>>(),
            output,
            rbf,
        )?;

        psbt.inputs = self.prepare_psbt_inputs(inputs, &tree.root, &branch, &keys);

        Ok(psbt)
    }

    fn prepare_psbt(
        &self,
        outpoints: &[OutPoint],
        output: &TxOut,
        rbf: bool,
    ) -> Result<Psbt, CoreError> {
        let unsigned_tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: outpoints
                .iter()
                .map(|outpoint| TxIn {
                    previous_output: *outpoint,
                    script_sig: ScriptBuf::default(),
                    sequence: match rbf {
                        true => Sequence::ENABLE_RBF_NO_LOCKTIME,
                        false => Sequence::MAX,
                    },
                    witness: Witness::default(),
                })
                .collect(),
            output: vec![output.clone()],
        };

        Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)
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

        // 5. Create psbt input
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
