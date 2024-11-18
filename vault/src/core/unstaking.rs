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
    pub input_utxo: PreviousStakingUTXO,
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
    pub input_utxo: PreviousStakingUTXO,
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
        // TODO: validate more params
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

        let (branch, keys): (&ScriptBuf, Vec<XOnlyPublicKey>) = match unstaking_type {
            UnstakingType::UserProtocol => {
                let branch = &tree.user_protocol_branch;
                let keys = vec![x_only_keys.user, x_only_keys.protocol];
                (branch, keys)
            }
            UnstakingType::CovenantsProtocol => {
                let branch = &tree.covenants_protocol_branch;
                let mut keys = vec![x_only_keys.protocol];
                keys.extend_from_slice(&x_only_keys.covenants);
                (branch, keys)
            }
            UnstakingType::CovenantsUser => {
                let branch = &tree.covenants_user_branch;
                let mut keys = vec![x_only_keys.user];
                keys.extend_from_slice(&x_only_keys.covenants);
                (branch, keys)
            }
            UnstakingType::OnlyCovenants => {
                (&tree.only_covenants_branch.unwrap(), x_only_keys.covenants)
            }
        };

        let mut psbt = self.prepare_psbt(
            &params.input_utxo.outpoint,
            &params.unstaking_output,
            params.rbf,
        )?;

        let input = self.prepare_psbt_input(&params.input_utxo, &tree.root, branch, &keys);

        psbt.inputs = vec![input];

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

        let mut psbt = self.prepare_psbt(
            &params.input_utxo.outpoint,
            &params.unstaking_output,
            params.rbf,
        )?;

        let input = self.prepare_psbt_input(
            &params.input_utxo,
            &tree.root,
            &tree.only_covenants_branch.unwrap(),
            &covenants_x_only,
        );

        psbt.inputs = vec![input];

        Ok(psbt)
    }
}

impl VaultManager {
    fn prepare_psbt(
        &self,
        outpoint: &OutPoint,
        output: &TxOut,
        rbf: bool,
    ) -> Result<Psbt, CoreError> {
        let unsigned_tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: *outpoint,
                script_sig: ScriptBuf::default(),
                sequence: match rbf {
                    true => Sequence::ENABLE_RBF_NO_LOCKTIME,
                    false => Sequence::MAX,
                },
                witness: Witness::default(),
            }],
            output: vec![output.clone()],
        };

        Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)
    }

    fn prepare_psbt_input(
        &self,
        input_utxo: &PreviousStakingUTXO,
        tree: &TaprootSpendInfo,
        branch: &ScriptBuf,
        keys: &[XOnlyPublicKey],
    ) -> Input {
        let tap_key_origins = self.create_tap_key_origins(branch, keys);

        let tap_scripts = self.create_tap_scripts(tree, branch);

        // 5. Create psbt input
        self.create_psbt_input(input_utxo, tree, &tap_scripts, &tap_key_origins)
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

    fn create_psbt_input(
        &self,
        input_utxo: &PreviousStakingUTXO,
        tree: &bitcoin::taproot::TaprootSpendInfo,
        tap_scripts: &BTreeMap<bitcoin::taproot::ControlBlock, (ScriptBuf, LeafVersion)>,
        tap_key_origins: &BTreeMap<
            XOnlyPublicKey,
            (Vec<TapLeafHash>, (Fingerprint, DerivationPath)),
        >,
    ) -> Input {
        Input {
            // Add the UTXO being spent
            witness_utxo: Some(TxOut {
                value: input_utxo.amount_in_sats,
                script_pubkey: input_utxo.script_pubkey.clone(),
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
    }
}
