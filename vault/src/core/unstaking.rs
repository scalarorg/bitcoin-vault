use std::collections::BTreeMap;

use super::{
    manager, CoreError, ReversedPreviousStakingUTXO, TaprootTree, Unstaking, VaultManager,
    XOnlyKeys, UTXO,
};

use super::PreviousStakingUTXO;
use bitcoin::bip32::{DerivationPath, Fingerprint};
use bitcoin::psbt::{Input, PsbtSighashType};
use bitcoin::taproot::{LeafVersion, TaprootSpendInfo};
use bitcoin::{
    absolute, transaction, OutPoint, Psbt, PublicKey, ScriptBuf, Sequence, TapLeafHash,
    TapSighashType, Transaction, TxIn, TxOut, Witness, XOnlyPublicKey,
};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct BuildUserProtocolSpendParams {
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
pub struct BuildCovenantsProtocolSpendParams<'a> {
    pub input_utxo: &'a UTXO,
    pub unstaking_output: &'a TxOut,
    pub protocol_pub_key: &'a PublicKey,
    pub covenant_pubkeys: &'a [PublicKey],
    pub covenant_quorum: u8,
}

#[derive(Debug, Validate)]
pub struct BuildCovenantsUserSpendParams<'a> {
    pub input_utxo: &'a UTXO,
    pub unstaking_output: &'a TxOut,
    pub user_pub_key: &'a PublicKey,
    pub covenant_pubkeys: &'a [PublicKey],
    pub covenant_quorum: u8,
}

#[derive(Debug)]
pub struct BuildUserProtocolSpendOutput {
    pub psbt: Psbt,
}

impl BuildUserProtocolSpendOutput {
    pub fn new(psbt: Psbt) -> Self {
        Self { psbt }
    }

    pub fn into_psbt(self) -> Psbt {
        self.psbt
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
                previous_output: outpoint.clone(),
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
        reversed_input_utxo: &ReversedPreviousStakingUTXO,
        tree: &TaprootSpendInfo,
        branch: &ScriptBuf,
        x_only_keys: &XOnlyKeys,
    ) -> Input {
        let tap_key_origins =
            self.create_tap_key_origins(&branch, &[x_only_keys.user, x_only_keys.protocol]);

        let tap_scripts = self.create_tap_scripts(tree, branch);

        // 5. Create psbt input
        self.create_psbt_input(&reversed_input_utxo, tree, &tap_scripts, &tap_key_origins)
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
        reversed_input_utxo: &ReversedPreviousStakingUTXO,
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
                value: reversed_input_utxo.amount_in_sats,
                script_pubkey: reversed_input_utxo.script_pubkey.clone(),
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

impl Unstaking for VaultManager {
    type Error = CoreError;

    fn build_user_protocol_spend(
        &self,
        params: &BuildUserProtocolSpendParams,
    ) -> Result<Psbt, Self::Error> {
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

        // 3. Create reversed input utxo, unsigned transaction and psbt
        // TODO: refactor this stuff, check ReversedPreviousStakingUTXO
        let reversed_input_utxo = ReversedPreviousStakingUTXO::from(params.input_utxo.clone());

        let mut psbt = self.prepare_psbt(
            &reversed_input_utxo.outpoint,
            &params.unstaking_output,
            params.rbf,
        )?;

        let input = self.prepare_psbt_input(
            &reversed_input_utxo,
            &tree.root,
            &tree.user_protocol_branch,
            &x_only_keys,
        );

        psbt.inputs = vec![input];

        Ok(psbt)
    }

    fn build_covenants_protocol_spend(
        &self,
        params: &BuildCovenantsProtocolSpendParams,
    ) -> Result<Psbt, Self::Error> {
        todo!()
    }

    // fn build_covenants_protocol_spend(
    //     &self,
    //     params: &BuildCovenantsProtocolSpendParams,
    // ) -> Result<Psbt, Self::Error> {
    //     // 1. Convert protocol key to x-only format
    //     let x_only_protocol_pub_key = XOnlyPublicKey::from(*params.protocol_pub_key);

    //     // 2. Convert covenant keys to x-only format
    //     let covenant_pubkeys_x_only: Vec<XOnlyPublicKey> = params
    //         .covenant_pubkeys
    //         .iter()
    //         .map(|pk| XOnlyPublicKey::from(*pk))
    //         .collect();

    //     // 3. Build script and prepare taproot tree
    //     let script = TaprootManager::covenants_protocol_branch(
    //         &covenant_pubkeys_x_only,
    //         params.covenant_quorum,
    //         &x_only_protocol_pub_key,
    //     )?;

    //     // 4. Create unsigned transaction
    //     let unsigned_tx = self.create_unsigned_tx(
    //         params.input_utxo.outpoint,
    //         params.unstaking_output.clone(),
    //         false, // RBF is not needed for covenant spends
    //     );

    //     let mut psbt = Psbt::from_unsigned_tx(unsigned_tx)
    //         .map_err(|_| CoreError::FailedToCreatePSBT)?;

    //     // 5. Create taproot tree
    //     let tree = TaprootManager::build_taproot_tree(
    //         self.secp(),
    //         &NUMS_BIP_341, // Use NUMS key as placeholder for user key
    //         &x_only_protocol_pub_key,
    //         &covenant_pubkeys_x_only,
    //         params.covenant_quorum,
    //         false, // No need for only_covenants path
    //     )?;

    //     // 6. Create taproot key origins and scripts
    //     let mut tap_key_origins = self.create_tap_key_origins(&script, &[x_only_protocol_pub_key]);
    //     // Add covenant keys to origins
    //     for key in covenant_pubkeys_x_only.iter() {
    //         tap_key_origins.insert(
    //             *key,
    //             (
    //                 vec![script.tapscript_leaf_hash()],
    //                 ([0u8; 4].into(), DerivationPath::default()),
    //             ),
    //         );
    //     }

    //     let tap_scripts = self.create_tap_scripts(&tree, &script);

    //     // 7. Create and set PSBT input
    //     let input = Input {
    //         witness_utxo: Some(TxOut {
    //             value: params.input_utxo.amount_in_sats,
    //             script_pubkey: params.input_utxo.script_pubkey.clone(),
    //         }),
    //         tap_internal_key: Some(tree.internal_key()),
    //         tap_merkle_root: tree.merkle_root(),
    //         tap_scripts: tap_scripts.clone(),
    //         tap_key_origins: tap_key_origins.clone(),
    //         sighash_type: Some(PsbtSighashType::from(TapSighashType::Default)),
    //         ..Default::default()
    //     };

    //     psbt.inputs = vec![input];

    //     Ok(psbt)
    // }
}
