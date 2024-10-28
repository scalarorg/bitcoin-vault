use std::collections::BTreeMap;

use bitcoin::{
    absolute,
    consensus::Decodable,
    hashes::{sha256d::Hash as Sha256dHash, Hash},
    key::Secp256k1,
    opcodes::all::{
        OP_CHECKSIG, OP_CHECKSIGADD, OP_CHECKSIGVERIFY, OP_GREATERTHANOREQUAL, OP_RETURN,
    },
    psbt::{Input, PsbtSighashType},
    script,
    secp256k1::All,
    taproot::{ControlBlock, LeafVersion, TaprootBuilder},
    transaction, Amount, Psbt, PublicKey, ScriptBuf, Sequence, TapSighashType, Transaction, TxIn,
    TxOut, Witness, XOnlyPublicKey,
};

use lazy_static::lazy_static;

use super::{
    BuildCovenantsProtocolSpendParams, BuildCovenantsUserSpendParams, BuildStakingOutputParams,
    BuildUserProtocolSpendParams, DestinationAddress, DestinationChainId, EmbeddedData,
    StakingError, EMBEDDED_DATA_SCRIPT_SIZE, TAG_HASH_SIZE, UTXO,
};

lazy_static! {
    pub static ref NUMS_BIP_341: XOnlyPublicKey = XOnlyPublicKey::from_slice(&[
        0x50, 0x92, 0x9b, 0x74, 0xc1, 0xa0, 0x49, 0x54, 0xb7, 0x8b, 0x4b, 0x60, 0x35, 0xe9, 0x7a,
        0x5e, 0x07, 0x8a, 0x5a, 0x0f, 0x28, 0xec, 0x96, 0xd5, 0x47, 0xbf, 0xee, 0x9a, 0xce, 0x80,
        0x3a, 0xc0,
    ])
    .unwrap();
}

pub trait Staking {
    type Error;

    fn build_staking_outputs(
        &self,
        params: &BuildStakingOutputParams,
    ) -> Result<Vec<TxOut>, Self::Error>;
}

pub trait Unstaking {
    type Error;

    fn build_user_protocol_spend(
        &self,
        params: &BuildUserProtocolSpendParams,
    ) -> Result<Psbt, Self::Error>;

    fn build_covenants_protocol_spend(
        &self,
        params: &BuildCovenantsProtocolSpendParams,
    ) -> Result<Psbt, Self::Error>;

    fn build_covenants_user_spend(
        &self,
        params: &BuildCovenantsUserSpendParams,
    ) -> Result<Psbt, Self::Error>;
}

pub trait Parsing {
    fn parse_embedded_data(tx_hex: Vec<u8>) -> Result<EmbeddedData, StakingError>;
}

pub struct StakingManager {
    secp: Secp256k1<All>,
    tag: Vec<u8>,
    version: u8,
}

impl Staking for StakingManager {
    type Error = StakingError;

    /// This function is used to build the staking outputs
    ///
    /// ### Arguments
    /// * `params` - The parameters for building the staking outputs
    ///
    /// ### Returns
    /// * `Result<Vec<TxOut>, Self::Error>` - The staking outputs or an error
    ///
    fn build_staking_outputs(
        &self,
        params: &BuildStakingOutputParams,
    ) -> Result<Vec<TxOut>, Self::Error> {
        // TODO: 0.validate params by use validator create
        let user_pub_key_x_only = params.user_pub_key.inner.x_only_public_key().0;
        let protocol_pub_key_x_only = params.protocol_pub_key.inner.x_only_public_key().0;
        let covenant_pubkeys_x_only: Vec<XOnlyPublicKey> = params
            .covenant_pubkeys
            .iter()
            .map(|pk| pk.inner.x_only_public_key().0)
            .collect();

        let lock_script = Self::create_locking_script(
            &self.secp,
            &user_pub_key_x_only,
            &protocol_pub_key_x_only,
            &covenant_pubkeys_x_only,
            params.covenant_quorum,
            params.have_only_covenants,
        )?;

        let embedded_data_script = Self::create_embedded_data_script(
            &self.tag,
            self.version,
            &params.destination_chain_id,
            &params.destination_contract_address,
            &params.destination_recipient_address,
        )?;

        Ok(vec![
            TxOut {
                value: Amount::from_sat(params.staking_amount),
                script_pubkey: lock_script,
            },
            TxOut {
                value: Amount::from_sat(0),
                script_pubkey: embedded_data_script,
            },
        ])
    }
}

impl StakingManager {
    pub fn new(tag: Vec<u8>, version: u8) -> Self {
        let secp = Secp256k1::new();
        Self { secp, tag, version }
    }
    /// Creates a Taproot locking script with multiple spending conditions.
    ///
    /// This function constructs a Taproot script tree with different spending paths:
    /// - Covenants + Protocol
    /// - Covenants + User
    /// - User + Protocol
    /// - Only Covenants (optional)
    ///
    /// The resulting tree structure depends on the `have_only_covenants` parameter:
    ///
    /// When `have_only_covenants` is `false`:
    /// ```text
    ///        Root
    ///       /    \
    ///      /      \
    ///     /        \
    ///    /          \
    ///   1            2
    ///   |           / \
    ///   |          /   \
    ///   |         /     \
    ///   |        3       4
    ///   |        |       |
    /// U + P    C + P   C + U
    /// ```
    ///
    /// When `have_only_covenants` is `true`:
    /// ```text
    ///         Root
    ///        /    \
    ///       /      \
    ///      /        \
    ///     2          2
    ///    / \        / \
    ///   /   \      /   \
    ///  3     4    5     6
    ///  |     |    |     |
    /// C+P   C+U  U+P  Only C
    /// ```
    ///
    /// ### Arguments
    /// * `secp` - The secp256k1 context
    /// * `user_pub_key` - The user's public key
    /// * `protocol_pub_key` - The protocol's public key
    /// * `covenant_pubkeys` - A slice of covenant public keys
    /// * `covenant_quorum` - The number of covenant signatures required
    /// * `have_only_covenants` - Whether to include an "Only Covenants" spending path
    ///
    /// ### Returns
    /// * `Result<ScriptBuf, StakingError>` - The resulting Taproot script or an error
    ///
    fn create_locking_script(
        secp: &Secp256k1<All>,
        user_pub_key: &XOnlyPublicKey,
        protocol_pub_key: &XOnlyPublicKey,
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        have_only_covenants: bool,
    ) -> Result<ScriptBuf, StakingError> {
        let mut builder = TaprootBuilder::new();

        let user_protocol_branch = Self::user_protocol_banch(user_pub_key, protocol_pub_key);
        let covenants_protocol_branch =
            Self::covenants_protocol_branch(covenant_pubkeys, covenant_quorum, protocol_pub_key)?;
        let covenants_user_branch =
            Self::covenants_user_branch(covenant_pubkeys, covenant_quorum, user_pub_key)?;

        builder = builder.add_leaf(2, covenants_protocol_branch)?;
        builder = builder.add_leaf(2, covenants_user_branch)?;

        if have_only_covenants {
            builder = builder.add_leaf(2, user_protocol_branch)?;
            let only_covenants_branch =
                Self::only_covenants_branch(covenant_pubkeys, covenant_quorum)?;
            builder = builder.add_leaf(2, only_covenants_branch)?;
        } else {
            builder = builder.add_leaf(1, user_protocol_branch)?;
        }

        let taproot_spend_info = builder
            .finalize(secp, *NUMS_BIP_341)
            .map_err(|_| StakingError::TaprootFinalizationFailed)?;

        Ok(ScriptBuf::new_p2tr(
            secp,
            taproot_spend_info.internal_key(),
            taproot_spend_info.merkle_root(),
        ))
    }

    /// Creates an embedded data script for the staking transaction.
    ///
    /// This script is used to embed additional data in the staking transaction.
    ///
    /// # Arguments
    /// * `tag` - The tag for the embedded data: 4 bytes
    /// * `version` - The version of the embedded data: 1 byte
    /// * `destination_chain_id` - The destination chain ID: 8 bytes
    /// * `destination_contract_address` - The destination address: 20 bytes
    /// * `destination_recipient_address` - The destination recipient address: 20 bytes
    ///
    /// # The script is constructed as follows:
    /// ```text
    /// OP_RETURN <embedded_data_script_size> <hash> <version> <destination_chain_id> <destination_contract_address> <destination_recipient_address>
    /// ```
    ///
    /// # Returns
    /// * `Result<ScriptBuf, StakingError>` - The resulting embedded data script or an error
    ///
    fn create_embedded_data_script(
        tag: &Vec<u8>,
        version: u8,
        destination_chain_id: &DestinationChainId,
        destination_contract_address: &DestinationAddress,
        destination_recipient_address: &DestinationAddress,
    ) -> Result<ScriptBuf, StakingError> {
        let tag_bytes = tag.as_slice();

        let hash: [u8; TAG_HASH_SIZE] = if tag.len() <= TAG_HASH_SIZE {
            tag_bytes[0..TAG_HASH_SIZE]
                .try_into()
                .map_err(|_| StakingError::InvalidTag)?
        } else {
            Sha256dHash::hash(tag_bytes)[0..TAG_HASH_SIZE]
                .try_into()
                .map_err(|_| StakingError::InvalidTag)?
        };

        let embedded_data_script = script::Builder::new()
            .push_opcode(OP_RETURN)
            .push_int(EMBEDDED_DATA_SCRIPT_SIZE as i64)
            .push_slice(hash)
            .push_slice(version.to_le_bytes())
            .push_slice(destination_chain_id)
            .push_slice(destination_contract_address)
            .push_slice(destination_recipient_address)
            .into_script();

        Ok(embedded_data_script)
    }

    fn user_protocol_banch(
        user_pub_key: &XOnlyPublicKey,
        service_pub_key: &XOnlyPublicKey,
    ) -> ScriptBuf {
        script::Builder::new()
            .push_x_only_key(user_pub_key)
            .push_opcode(OP_CHECKSIGVERIFY)
            .push_x_only_key(service_pub_key)
            .push_opcode(OP_CHECKSIG)
            .into_script()
    }

    fn covenants_protocol_branch(
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        protocol_pub_key: &XOnlyPublicKey,
    ) -> Result<ScriptBuf, StakingError> {
        Self::create_covenant_branch(covenant_pubkeys, covenant_quorum, Some(protocol_pub_key))
    }

    fn covenants_user_branch(
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        user_pub_key: &XOnlyPublicKey,
    ) -> Result<ScriptBuf, StakingError> {
        Self::create_covenant_branch(covenant_pubkeys, covenant_quorum, Some(user_pub_key))
    }

    fn only_covenants_branch(
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
    ) -> Result<ScriptBuf, StakingError> {
        Self::create_covenant_branch(covenant_pubkeys, covenant_quorum, None)
    }

    fn create_covenant_branch(
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        initial_key: Option<&XOnlyPublicKey>,
    ) -> Result<ScriptBuf, StakingError> {
        let mut builder = script::Builder::new();

        // Initial key check
        if let Some(initial_key) = initial_key {
            builder = builder
                .push_x_only_key(initial_key)
                .push_opcode(OP_CHECKSIGVERIFY);
        }

        // Sort covenant public keys
        let mut sorted_pks = covenant_pubkeys.to_owned();
        sorted_pks.sort();

        // Check for duplicates
        for i in 0..sorted_pks.len() - 1 {
            if sorted_pks[i] == sorted_pks[i + 1] {
                return Err(StakingError::DuplicateCovenantKeys);
            }
        }

        // Add covenant keys to the script
        builder = builder.push_x_only_key(&sorted_pks[0]);
        builder = builder.push_opcode(OP_CHECKSIG);

        for pk in sorted_pks.iter().skip(1) {
            builder = builder.push_x_only_key(pk);
            builder = builder.push_opcode(OP_CHECKSIGADD);
        }

        // Add quorum check
        builder = builder
            .push_int(covenant_quorum as i64)
            .push_opcode(OP_GREATERTHANOREQUAL);

        Ok(builder.into_script())
    }
}

impl Unstaking for StakingManager {
    type Error = StakingError;

    fn build_user_protocol_spend(
        &self,
        params: &BuildUserProtocolSpendParams,
    ) -> Result<Psbt, Self::Error> {
        let (script, control_block) = Self::get_user_protocol_control_block(
            &self.secp,
            &params.user_pub_key.inner.x_only_public_key().0,
            &params.protocol_pub_key.inner.x_only_public_key().0,
            params.have_only_covenants,
        )?;

        self.create_user_protocol_psbt(
            params.input_utxo,
            params.staking_output,
            params.user_pub_key,
            params.protocol_pub_key,
            control_block,
            script,
        )
    }

    fn build_covenants_protocol_spend(
        &self,
        params: &BuildCovenantsProtocolSpendParams,
    ) -> Result<Psbt, StakingError> {
        let covenant_pubkeys_x_only: Vec<XOnlyPublicKey> = params
            .covenant_pubkeys
            .iter()
            .map(|pk| pk.inner.x_only_public_key().0)
            .collect();

        let (script, control_block) = Self::get_covenants_protocol_control_block(
            &self.secp,
            &params.protocol_pub_key.inner.x_only_public_key().0,
            &covenant_pubkeys_x_only,
            params.covenant_quorum,
        )?;

        self.create_covenant_spend_psbt(
            params.input_utxo,
            params.staking_output,
            params.protocol_pub_key,
            params.covenant_pubkeys,
            control_block,
            script,
        )
    }

    fn build_covenants_user_spend(
        &self,
        params: &BuildCovenantsUserSpendParams,
    ) -> Result<Psbt, StakingError> {
        let covenant_pubkeys_x_only: Vec<XOnlyPublicKey> = params
            .covenant_pubkeys
            .iter()
            .map(|pk| pk.inner.x_only_public_key().0)
            .collect();

        let (script, control_block) = Self::get_covenants_user_control_block(
            &self.secp,
            &params.protocol_pub_key.inner.x_only_public_key().0,
            &covenant_pubkeys_x_only,
            params.covenant_quorum,
        )?;

        self.create_covenant_spend_psbt(
            params.input_utxo,
            params.staking_output,
            params.protocol_pub_key,
            params.covenant_pubkeys,
            control_block,
            script,
        )
    }
}

impl Parsing for StakingManager {
    /// Parses embedded data from a transaction's OP_RETURN output
    ///
    /// # Arguments
    /// * `tx_hex` - The transaction hex string
    ///
    /// # Returns
    /// * `Result<EmbeddedData, StakingError>` - The parsed embedded data or an error
    ///
    fn parse_embedded_data(tx_hex: Vec<u8>) -> Result<EmbeddedData, StakingError> {
        // 1. Decode the transaction hex

        let tx: Transaction = Decodable::consensus_decode(&mut tx_hex.as_slice())
            .map_err(|_| StakingError::InvalidTransactionHex)?;

        // 2. Find the OP_RETURN output
        let op_return_output = tx
            .output
            .iter()
            .find(|output| output.script_pubkey.is_op_return() && !output.script_pubkey.is_empty())
            .ok_or(StakingError::NoEmbeddedData)?;

        // 3. Get the script instructions
        let mut instructions = op_return_output.script_pubkey.instructions();

        // Skip OP_RETURN
        instructions.next();

        // Skip embedded data size
        instructions.next();

        // 4. Parse each field
        let hash = instructions
            .next()
            .ok_or(StakingError::InvalidEmbeddedData)?
            .map_err(|_| StakingError::InvalidEmbeddedData)?
            .push_bytes()
            .ok_or(StakingError::InvalidEmbeddedData)?
            .to_owned()
            .as_bytes()
            .to_vec();
        let version = instructions
            .next()
            .ok_or(StakingError::InvalidEmbeddedData)?
            .map_err(|_| StakingError::InvalidEmbeddedData)?
            .to_owned()
            .push_bytes()
            .ok_or(StakingError::InvalidEmbeddedData)?[0];

        let destination_chain_id = instructions
            .next()
            .ok_or(StakingError::InvalidEmbeddedData)?
            .map_err(|_| StakingError::InvalidEmbeddedData)?
            .push_bytes()
            .ok_or(StakingError::InvalidEmbeddedData)?
            .to_owned()
            .as_bytes()
            .try_into()
            .map_err(|_| StakingError::InvalidEmbeddedData)?;

        let destination_contract = instructions
            .next()
            .ok_or(StakingError::InvalidEmbeddedData)?
            .map_err(|_| StakingError::InvalidEmbeddedData)?
            .push_bytes()
            .ok_or(StakingError::InvalidEmbeddedData)?
            .to_owned()
            .as_bytes()
            .try_into()
            .map_err(|_| StakingError::InvalidEmbeddedData)?;

        let destination_recipient = instructions
            .next()
            .ok_or(StakingError::InvalidEmbeddedData)?
            .map_err(|_| StakingError::InvalidEmbeddedData)?
            .push_bytes()
            .ok_or(StakingError::InvalidEmbeddedData)?
            .to_owned()
            .as_bytes()
            .try_into()
            .map_err(|_| StakingError::InvalidEmbeddedData)?;

        Ok(EmbeddedData {
            tag: hash,
            version,
            destination_chain_id,
            destination_contract,
            destination_recipient,
        })
    }
}

impl StakingManager {
    /// Creates PSBT for User + Protocol spending
    fn create_user_protocol_psbt(
        &self,
        input_utxo: &UTXO,
        staking_output: &TxOut,
        user_pubkey: &PublicKey,
        protocol_pubkey: &PublicKey,
        control_block: ControlBlock,
        script: ScriptBuf,
    ) -> Result<Psbt, StakingError> {
        let mut builder = TaprootBuilder::new();
        builder = builder.add_leaf(1, script.clone())?;
        let spend_info = builder
            .finalize(&self.secp, *NUMS_BIP_341)
            .map_err(|_| StakingError::TaprootFinalizationFailed)?;

        // 1. Create the unsigned transaction
        let unsigned_tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: input_utxo.outpoint,
                script_sig: ScriptBuf::default(),
                sequence: Sequence::MAX,
                witness: Witness::default(),
            }],
            output: vec![staking_output.clone()],
        };

        // 2. Create base PSBT from unsigned transaction
        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| StakingError::FailedToCreatePSBT)?;

        // 3. Create the PSBT input with Taproot-specific fields
        let mut input = Input {
            // Add the UTXO being spent
            witness_utxo: Some(TxOut {
                value: input_utxo.amount_in_sats,
                script_pubkey: staking_output.script_pubkey.clone(),
            }),

            // Add Taproot-specific data
            tap_internal_key: Some(control_block.internal_key),
            tap_merkle_root: spend_info.merkle_root(),

            // Add the script we're using to spend
            tap_scripts: {
                let mut map = BTreeMap::new();
                map.insert(
                    control_block.clone(),
                    (script.clone(), LeafVersion::TapScript),
                );
                map
            },

            // Set default sighash type for Taproot
            sighash_type: Some(PsbtSighashType::from(TapSighashType::All)),

            ..Default::default()
        };

        // 4. Add key origin information for both user and protocol keys
        let mut tap_key_origins = BTreeMap::new();

        // Add user key origin
        tap_key_origins.insert(
            user_pubkey.inner.x_only_public_key().0,
            (
                vec![script.tapscript_leaf_hash()],
                ([0u8; 4].into(), vec![].into()), // ? Check this
            ), // Use [0u8; 4] for fingerprint
        );

        // Add protocol key origin
        tap_key_origins.insert(
            protocol_pubkey.inner.x_only_public_key().0,
            (
                vec![script.tapscript_leaf_hash()],
                ([0u8; 4].into(), vec![].into()), // Use [0u8; 4] for fingerprint
            ),
        );

        input.tap_key_origins = tap_key_origins;

        // 5. Set the input in the PSBT
        psbt.inputs = vec![input];

        Ok(psbt)
    }

    /// Helper function to create PSBT for covenant spending paths
    fn create_covenant_spend_psbt(
        &self,
        input_utxo: &UTXO,
        staking_output: &TxOut,
        main_pubkey: &PublicKey,
        covenant_pubkeys: &[PublicKey],
        control_block: ControlBlock,
        script: ScriptBuf,
    ) -> Result<Psbt, StakingError> {
        let mut builder = TaprootBuilder::new();
        builder = builder.add_leaf(2, script.clone())?;
        let spend_info = builder
            .finalize(&self.secp, *NUMS_BIP_341)
            .map_err(|_| StakingError::TaprootFinalizationFailed)?;

        // Create the unsigned transaction
        let unsigned_tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: input_utxo.outpoint,
                script_sig: ScriptBuf::default(),
                sequence: Sequence::MAX,
                witness: Witness::default(),
            }],
            output: vec![staking_output.clone()],
        };

        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| StakingError::FailedToCreatePSBT)?;

        // Create PSBT input with Taproot-specific fields
        let mut input = Input {
            witness_utxo: Some(TxOut {
                value: input_utxo.amount_in_sats,
                script_pubkey: staking_output.script_pubkey.clone(),
            }),
            tap_internal_key: Some(control_block.internal_key),
            tap_merkle_root: spend_info.merkle_root(),
            tap_scripts: {
                let mut map = BTreeMap::new();
                map.insert(
                    control_block.clone(),
                    (script.clone(), LeafVersion::TapScript),
                );
                map
            },
            sighash_type: Some(PsbtSighashType::from(TapSighashType::All)),
            ..Default::default()
        };

        // Add key origin information for all keys
        let script_hash = script.tapscript_leaf_hash();
        let mut tap_key_origins = BTreeMap::new();

        // Add main key (user or protocol)
        tap_key_origins.insert(
            main_pubkey.inner.x_only_public_key().0,
            (
                vec![script_hash], // Use TapLeafHash directly instead of converting to bytes
                ([0u8; 4].into(), vec![].into()),
            ),
        );

        // Add covenant keys
        for pubkey in covenant_pubkeys {
            tap_key_origins.insert(
                pubkey.inner.x_only_public_key().0,
                (
                    vec![script_hash], // Use TapLeafHash directly instead of converting to bytes
                    ([0u8; 4].into(), vec![].into()),
                ),
            );
        }

        input.tap_key_origins = tap_key_origins;
        psbt.inputs = vec![input];

        Ok(psbt)
    }

    /// Gets control block for Covenants + Protocol spending path
    fn get_covenants_protocol_control_block(
        secp: &Secp256k1<All>,
        protocol_pub_key: &XOnlyPublicKey,
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
    ) -> Result<(ScriptBuf, ControlBlock), StakingError> {
        let mut builder = TaprootBuilder::new();

        // Create the script
        let script =
            Self::covenants_protocol_branch(covenant_pubkeys, covenant_quorum, protocol_pub_key)?;

        // Always at depth 2 in the tree
        builder = builder.add_leaf(2, script.clone())?;

        let spend_info = builder
            .finalize(secp, *NUMS_BIP_341)
            .map_err(|_| StakingError::TaprootFinalizationFailed)?;

        let control_block = spend_info
            .control_block(&(script.clone(), LeafVersion::TapScript))
            .ok_or(StakingError::ControlBlockNotFound)?;

        Ok((script, control_block))
    }

    fn get_user_protocol_control_block(
        secp: &Secp256k1<All>,
        user_pub_key: &XOnlyPublicKey,
        protocol_pub_key: &XOnlyPublicKey,
        have_only_covenants: bool,
    ) -> Result<(ScriptBuf, ControlBlock), StakingError> {
        let mut builder = TaprootBuilder::new();

        // Create the script
        let script = Self::user_protocol_banch(user_pub_key, protocol_pub_key);

        // Add to builder with correct depth based on tree structure
        if have_only_covenants {
            builder = builder.add_leaf(2, script.clone())?;
        } else {
            builder = builder.add_leaf(1, script.clone())?;
        }

        // Finalize and get control block
        let spend_info = builder
            .finalize(secp, *NUMS_BIP_341)
            .map_err(|_| StakingError::TaprootFinalizationFailed)?;

        let control_block = spend_info
            .control_block(&(script.clone(), LeafVersion::TapScript))
            .ok_or(StakingError::ControlBlockNotFound)?;

        Ok((script, control_block))
    }

    /// Gets control block for Covenants + User spending path
    fn get_covenants_user_control_block(
        secp: &Secp256k1<All>,
        user_pub_key: &XOnlyPublicKey,
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
    ) -> Result<(ScriptBuf, ControlBlock), StakingError> {
        let mut builder = TaprootBuilder::new();

        // Create the script
        let script = Self::covenants_user_branch(covenant_pubkeys, covenant_quorum, user_pub_key)?;

        // Always at depth 2 in the tree
        builder = builder.add_leaf(2, script.clone())?;

        let spend_info = builder
            .finalize(secp, *NUMS_BIP_341)
            .map_err(|_| StakingError::TaprootFinalizationFailed)?;

        let control_block = spend_info
            .control_block(&(script.clone(), LeafVersion::TapScript))
            .ok_or(StakingError::ControlBlockNotFound)?;

        Ok((script, control_block))
    }

    // Gets control block for Only Covenants spending path
    // fn get_only_covenants_control_block(
    //     secp: &Secp256k1<All>,
    //     covenant_pubkeys: &[XOnlyPublicKey],
    //     covenant_quorum: u8,
    // ) -> Result<(ScriptBuf, ControlBlock), StakingError> {
    //     let mut builder = TaprootBuilder::new();

    //     // Create the script
    //     let script = Self::only_covenants_branch(covenant_pubkeys, covenant_quorum)?;

    //     // Always at depth 2 in the tree
    //     builder = builder.add_leaf(2, script.clone())?;

    //     let spend_info = builder
    //         .finalize(secp, *NUMS_BIP_341)
    //         .map_err(|_| StakingError::TaprootFinalizationFailed)?;

    //     let control_block = spend_info
    //         .control_block(&(script.clone(), LeafVersion::TapScript))
    //         .ok_or(StakingError::ControlBlockNotFound)?;

    //     Ok((script, control_block))
    // }
}
