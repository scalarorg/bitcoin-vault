use bitcoin::{
    key::Secp256k1,
    opcodes::all::{OP_CHECKSIG, OP_CHECKSIGADD, OP_CHECKSIGVERIFY, OP_GREATERTHANOREQUAL},
    script,
    secp256k1::All,
    taproot::{TaprootBuilder, TaprootSpendInfo},
    ScriptBuf, XOnlyPublicKey,
};

use lazy_static::lazy_static;

use super::CoreError;

lazy_static! {
    pub static ref NUMS_BIP_341: XOnlyPublicKey = XOnlyPublicKey::from_slice(&[
        0x50, 0x92, 0x9b, 0x74, 0xc1, 0xa0, 0x49, 0x54, 0xb7, 0x8b, 0x4b, 0x60, 0x35, 0xe9, 0x7a,
        0x5e, 0x07, 0x8a, 0x5a, 0x0f, 0x28, 0xec, 0x96, 0xd5, 0x47, 0xbf, 0xee, 0x9a, 0xce, 0x80,
        0x3a, 0xc0,
    ])
    .unwrap();
}

pub struct TaprootManager;

impl TaprootManager {
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
    /// * `Result<ScriptBuf, CoreError>` - The resulting Taproot script or an error
    ///
    pub fn build_taproot_tree(
        secp: &Secp256k1<All>,
        user_pub_key: &XOnlyPublicKey,
        protocol_pub_key: &XOnlyPublicKey,
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        have_only_covenants: bool,
    ) -> Result<TaprootSpendInfo, CoreError> {
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
            .map_err(|_| CoreError::TaprootFinalizationFailed)?;

        Ok(taproot_spend_info)
    }

    pub fn user_protocol_banch(
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

    pub fn covenants_protocol_branch(
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        protocol_pub_key: &XOnlyPublicKey,
    ) -> Result<ScriptBuf, CoreError> {
        Self::create_covenant_branch(covenant_pubkeys, covenant_quorum, Some(protocol_pub_key))
    }

    pub fn covenants_user_branch(
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        user_pub_key: &XOnlyPublicKey,
    ) -> Result<ScriptBuf, CoreError> {
        Self::create_covenant_branch(covenant_pubkeys, covenant_quorum, Some(user_pub_key))
    }

    pub fn only_covenants_branch(
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
    ) -> Result<ScriptBuf, CoreError> {
        Self::create_covenant_branch(covenant_pubkeys, covenant_quorum, None)
    }

    fn create_covenant_branch(
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        initial_key: Option<&XOnlyPublicKey>,
    ) -> Result<ScriptBuf, CoreError> {
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
                return Err(CoreError::DuplicateCovenantKeys);
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
