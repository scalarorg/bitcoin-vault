use std::collections::BTreeMap;

use super::{
    CoreError, ReversedPreviousStakingUTXO, TaprootManager, Unstaking, VaultManager, UTXO,
};

use super::PreviousStakingUTXO;
use bitcoin::psbt::{Input, PsbtSighashType};
use bitcoin::taproot::LeafVersion;
use bitcoin::{
    absolute, transaction, Psbt, PublicKey, ScriptBuf, Sequence, TapSighashType, Transaction, TxIn,
    TxOut, Witness, XOnlyPublicKey,
};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct BuildUserProtocolSpendParams {
    pub input_utxo: PreviousStakingUTXO,
    pub unstaking_output: TxOut,
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub covenant_pubkeys: Vec<PublicKey>,
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

impl Unstaking for VaultManager {
    type Error = CoreError;

    fn build_user_protocol_spend(
        &self,
        params: &BuildUserProtocolSpendParams,
    ) -> Result<Psbt, Self::Error> {
        let x_only_user_pub_key = XOnlyPublicKey::from(params.user_pub_key);
        let x_only_protocol_pub_key = XOnlyPublicKey::from(params.protocol_pub_key);
        let covenant_pubkeys_x_only: Vec<XOnlyPublicKey> = params
            .covenant_pubkeys
            .iter()
            .map(|pk| XOnlyPublicKey::from(*pk))
            .collect();

        let script =
            TaprootManager::user_protocol_banch(&x_only_user_pub_key, &x_only_protocol_pub_key);

        let tree = TaprootManager::build_taproot_tree(
            self.secp(),
            &x_only_user_pub_key,
            &x_only_protocol_pub_key,
            &covenant_pubkeys_x_only,
            params.covenant_quorum,
            params.have_only_covenants,
        )?;

        // TODO: refactor this stuff, check ReversedPreviousStakingUTXO
        let reversed_input_utxo = ReversedPreviousStakingUTXO::from(params.input_utxo.clone());

        // Create the unsigned transaction
        let unsigned_tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: reversed_input_utxo.outpoint,
                script_sig: ScriptBuf::default(),
                sequence: match params.rbf {
                    true => Sequence::ENABLE_RBF_NO_LOCKTIME,
                    false => Sequence::MAX,
                },
                witness: Witness::default(),
            }],
            output: vec![params.unstaking_output.clone()],
        };

        // 2. Create base PSBT from unsigned transaction
        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

        // 3. Add key origin information for both user and protocol keys
        let input = Input {
            // Add the UTXO being spent
            witness_utxo: Some(TxOut {
                value: reversed_input_utxo.amount_in_sats,
                script_pubkey: reversed_input_utxo.script_pubkey.clone(),
            }),

            // Add Taproot-specific data
            tap_internal_key: Some(tree.internal_key()),
            tap_merkle_root: tree.merkle_root(),

            // Add the script we're using to spend
            tap_scripts: {
                let mut map = BTreeMap::new();
                map.insert(
                    tree.control_block(&(script.clone(), LeafVersion::TapScript))
                        .unwrap(),
                    (script.clone(), LeafVersion::TapScript),
                );
                map
            },

            tap_key_origins: {
                let mut tap_key_origins = BTreeMap::new();

                // Add user key origin
                tap_key_origins.insert(
                    x_only_user_pub_key,
                    (
                        vec![script.tapscript_leaf_hash()],
                        ([0u8; 4].into(), vec![].into()), // ? Check this
                    ), // Use [0u8; 4] for fingerprint
                );

                // Add protocol key origin
                tap_key_origins.insert(
                    x_only_protocol_pub_key,
                    (
                        vec![script.tapscript_leaf_hash()],
                        ([0u8; 4].into(), vec![].into()), // Use [0u8; 4] for fingerprint
                    ),
                );

                tap_key_origins
            },

            // Set default sighash type for Taproot
            sighash_type: Some(PsbtSighashType::from(TapSighashType::Default)),

            ..Default::default()
        };

        // 5. Set the input in the PSBT
        psbt.inputs = vec![input];

        Ok(psbt)
    }

    // fn build_covenants_protocol_spend(
    //     &self,
    //     params: &BuildCovenantsProtocolSpendParams,
    // ) -> Result<Psbt, CoreError> {
    //     let covenant_pubkeys_x_only: Vec<XOnlyPublicKey> = params
    //         .covenant_pubkeys
    //         .iter()
    //         .map(|pk| XOnlyPublicKey::from(*pk))
    //         .collect();

    //     let protocol_pub_key_x_only = XOnlyPublicKey::from(*params.protocol_pub_key);

    //     let (script, control_block) = Self::get_covenants_protocol_control_block(
    //         &self.secp,
    //         &protocol_pub_key_x_only,
    //         &covenant_pubkeys_x_only,
    //         params.covenant_quorum,
    //     )?;

    //     self.create_covenant_spend_psbt(
    //         params.input_utxo,
    //         params.unstaking_output,
    //         params.protocol_pub_key,
    //         params.covenant_pubkeys,
    //         control_block,
    //         script,
    //     )
    // }

    // fn build_covenants_user_spend(
    //     &self,
    //     params: &BuildCovenantsUserSpendParams,
    // ) -> Result<Psbt, CoreError> {
    //     let covenant_pubkeys_x_only: Vec<XOnlyPublicKey> = params
    //         .covenant_pubkeys
    //         .iter()
    //         .map(|pk| XOnlyPublicKey::from(*pk))
    //         .collect();

    //     let user_pub_key_x_only = XOnlyPublicKey::from(*params.user_pub_key);

    //     let (script, control_block) = Self::get_covenants_user_control_block(
    //         &self.secp,
    //         &user_pub_key_x_only,
    //         &covenant_pubkeys_x_only,
    //         params.covenant_quorum,
    //     )?;

    //     self.create_covenant_spend_psbt(
    //         params.input_utxo,
    //         params.unstaking_output,
    //         params.user_pub_key,
    //         params.covenant_pubkeys,
    //         control_block,
    //         script,
    //     )
    // }

    // fn create_covenant_spend_psbt(
    //     &self,
    //     input_utxo: &UTXO,
    //     staking_output: &TxOut,
    //     main_pubkey: &PublicKey,
    //     covenant_pubkeys: &[PublicKey],
    //     control_block: ControlBlock,
    //     script: ScriptBuf,
    // ) -> Result<Psbt, CoreError> {
    //     let mut builder = TaprootBuilder::new();
    //     builder = builder.add_leaf(2, script.clone())?;
    //     let spend_info = builder
    //         .finalize(&self.secp, *NUMS_BIP_341)
    //         .map_err(|_| CoreError::TaprootFinalizationFailed)?;

    //     // Create the unsigned transaction
    //     let unsigned_tx = Transaction {
    //         version: transaction::Version::TWO,
    //         lock_time: absolute::LockTime::ZERO,
    //         input: vec![TxIn {
    //             previous_output: input_utxo.outpoint,
    //             script_sig: ScriptBuf::default(),
    //             sequence: Sequence::MAX,
    //             witness: Witness::default(),
    //         }],
    //         output: vec![staking_output.clone()],
    //     };

    //     let mut psbt =
    //         Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

    //     // Create PSBT input with Taproot-specific fields
    //     let mut input = Input {
    //         witness_utxo: Some(TxOut {
    //             value: input_utxo.amount_in_sats,
    //             script_pubkey: staking_output.script_pubkey.clone(),
    //         }),
    //         tap_internal_key: Some(control_block.internal_key),
    //         tap_merkle_root: spend_info.merkle_root(),
    //         tap_scripts: {
    //             let mut map = BTreeMap::new();
    //             map.insert(
    //                 control_block.clone(),
    //                 (script.clone(), LeafVersion::TapScript),
    //             );
    //             map
    //         },
    //         sighash_type: Some(PsbtSighashType::from(TapSighashType::All)),
    //         ..Default::default()
    //     };

    //     // Add key origin information for all keys
    //     let script_hash = script.tapscript_leaf_hash();
    //     let mut tap_key_origins = BTreeMap::new();

    //     // Add main key (user or protocol)
    //     tap_key_origins.insert(
    //         XOnlyPublicKey::from(*main_pubkey),
    //         (
    //             vec![script_hash], // Use TapLeafHash directly instead of converting to bytes
    //             ([0u8; 4].into(), vec![].into()),
    //         ),
    //     );

    //     // Add covenant keys
    //     for pubkey in covenant_pubkeys {
    //         tap_key_origins.insert(
    //             XOnlyPublicKey::from(*pubkey),
    //             (
    //                 vec![script_hash], // Use TapLeafHash directly instead of converting to bytes
    //                 ([0u8; 4].into(), vec![].into()),
    //             ),
    //         );
    //     }

    //     input.tap_key_origins = tap_key_origins;
    //     psbt.inputs = vec![input];

    //     Ok(psbt)
    // }

    // /// Gets control block for Covenants + Protocol spending path
    // fn get_covenants_protocol_control_block(
    //     secp: &Secp256k1<All>,
    //     protocol_pub_key: &XOnlyPublicKey,
    //     covenant_pubkeys: &[XOnlyPublicKey],
    //     covenant_quorum: u8,
    // ) -> Result<(ScriptBuf, ControlBlock), CoreError> {
    //     let mut builder = TaprootBuilder::new();

    //     // Create the script
    //     let script =
    //         Self::covenants_protocol_branch(covenant_pubkeys, covenant_quorum, protocol_pub_key)?;

    //     // Always at depth 2 in the tree
    //     builder = builder.add_leaf(2, script.clone())?;

    //     let spend_info = builder
    //         .finalize(secp, *NUMS_BIP_341)
    //         .map_err(|_| CoreError::TaprootFinalizationFailed)?;

    //     let control_block = spend_info
    //         .control_block(&(script.clone(), LeafVersion::TapScript))
    //         .ok_or(CoreError::ControlBlockNotFound)?;

    //     Ok((script, control_block))
    // }

    // /// Gets control block for Covenants + User spending path
    // fn get_covenants_user_control_block(
    //     secp: &Secp256k1<All>,
    //     user_pub_key: &XOnlyPublicKey,
    //     covenant_pubkeys: &[XOnlyPublicKey],
    //     covenant_quorum: u8,
    // ) -> Result<(ScriptBuf, ControlBlock), CoreError> {
    //     let mut builder = TaprootBuilder::new();

    //     // Create the script
    //     let script = Self::covenants_user_branch(covenant_pubkeys, covenant_quorum, user_pub_key)?;

    //     // Always at depth 2 in the tree
    //     builder = builder.add_leaf(2, script.clone())?;

    //     let spend_info = builder
    //         .finalize(secp, *NUMS_BIP_341)
    //         .map_err(|_| CoreError::TaprootFinalizationFailed)?;

    //     let control_block = spend_info
    //         .control_block(&(script.clone(), LeafVersion::TapScript))
    //         .ok_or(CoreError::ControlBlockNotFound)?;

    //     Ok((script, control_block))
    // }

    // Gets control block for Only Covenants spending path
    // fn get_only_covenants_control_block(
    //     secp: &Secp256k1<All>,
    //     covenant_pubkeys: &[XOnlyPublicKey],
    //     covenant_quorum: u8,
    // ) -> Result<(ScriptBuf, ControlBlock), CoreError> {
    //     let mut builder = TaprootBuilder::new();

    //     // Create the script
    //     let script = Self::only_covenants_branch(covenant_pubkeys, covenant_quorum)?;

    //     // Always at depth 2 in the tree
    //     builder = builder.add_leaf(2, script.clone())?;

    //     let spend_info = builder
    //         .finalize(secp, *NUMS_BIP_341)
    //         .map_err(|_| CoreError::TaprootFinalizationFailed)?;

    //     let control_block = spend_info
    //         .control_block(&(script.clone(), LeafVersion::TapScript))
    //         .ok_or(CoreError::ControlBlockNotFound)?;

    //     Ok((script, control_block))
    // }
}
