use bitcoin::{
    key::{Secp256k1, UntweakedPublicKey},
    opcodes::all::{OP_CHECKSIG, OP_CHECKSIGADD, OP_CHECKSIGVERIFY, OP_GREATERTHANOREQUAL},
    script,
    secp256k1::All,
    taproot::{TaprootBuilder, TaprootSpendInfo},
    ScriptBuf, TapNodeHash, XOnlyPublicKey,
};

use lazy_static::lazy_static;

use super::{
    BuildCovenantProtocolBranch, BuildCovenantUserBranch, BuildOnlyCovenantsBranch,
    BuildUserProtocolBranch, CoreError,
};

lazy_static! {
    pub static ref NUMS_BIP_341: XOnlyPublicKey = XOnlyPublicKey::from_slice(&[
        0x50, 0x92, 0x9b, 0x74, 0xc1, 0xa0, 0x49, 0x54, 0xb7, 0x8b, 0x4b, 0x60, 0x35, 0xe9, 0x7a,
        0x5e, 0x07, 0x8a, 0x5a, 0x0f, 0x28, 0xec, 0x96, 0xd5, 0x47, 0xbf, 0xee, 0x9a, 0xce, 0x80,
        0x3a, 0xc0,
    ])
    .unwrap();
}

type UserProtocolBranch = ScriptBuf;

impl BuildUserProtocolBranch for UserProtocolBranch {
    fn build(
        user_pub_key: &XOnlyPublicKey,
        protocol_pub_key: &XOnlyPublicKey,
    ) -> Result<Self, CoreError> {
        Ok(script::Builder::new()
            .push_x_only_key(user_pub_key)
            .push_opcode(OP_CHECKSIGVERIFY)
            .push_x_only_key(protocol_pub_key)
            .push_opcode(OP_CHECKSIG)
            .into_script())
    }
}

type CovenantProtocolBranch = ScriptBuf;

impl BuildCovenantProtocolBranch for CovenantProtocolBranch {
    fn build(
        covenant_pub_keys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        protocol_pub_key: &XOnlyPublicKey,
    ) -> Result<Self, CoreError> {
        let covenant_script = CovenantScriptBuilder::build(
            covenant_pub_keys,
            covenant_quorum,
            Some(protocol_pub_key),
        )?;
        Ok(covenant_script)
    }
}

type CovenantUserBranch = ScriptBuf;

impl BuildCovenantUserBranch for CovenantUserBranch {
    fn build(
        covenant_pub_keys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        user_pub_key: &XOnlyPublicKey,
    ) -> Result<Self, CoreError> {
        let covenant_script =
            CovenantScriptBuilder::build(covenant_pub_keys, covenant_quorum, Some(user_pub_key))?;
        Ok(covenant_script)
    }
}

type OnlyCovenantsBranch = ScriptBuf;

impl BuildOnlyCovenantsBranch for OnlyCovenantsBranch {
    fn build(covenant_pub_keys: &[XOnlyPublicKey], covenant_quorum: u8) -> Result<Self, CoreError> {
        let covenant_script =
            CovenantScriptBuilder::build(covenant_pub_keys, covenant_quorum, None)?;
        Ok(covenant_script)
    }
}

pub struct CovenantScriptBuilder;

type CovenantScript = ScriptBuf;

impl CovenantScriptBuilder {
    pub fn build(
        covenant_pub_keys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        initial_key: Option<&XOnlyPublicKey>,
    ) -> Result<CovenantScript, CoreError> {
        let mut builder = script::Builder::new();

        // Initial key check
        if let Some(initial_key) = initial_key {
            builder = builder
                .push_x_only_key(initial_key)
                .push_opcode(OP_CHECKSIGVERIFY);
        }

        // Sort covenant public keys
        let mut sorted_pks = covenant_pub_keys.to_owned();
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

#[derive(Debug)]
pub struct TaprootTreeParams {
    pub user_pub_key: XOnlyPublicKey,
    pub protocol_pub_key: XOnlyPublicKey,
    pub covenant_pub_keys: Vec<XOnlyPublicKey>,
    pub covenant_quorum: u8,
}

#[derive(Debug, Clone)]
pub struct TaprootTree {
    pub root: TaprootSpendInfo,
    pub user_protocol_branch: UserProtocolBranch,
    pub covenants_protocol_branch: CovenantProtocolBranch,
    pub covenants_user_branch: CovenantUserBranch,
    pub only_covenants_branch: Option<OnlyCovenantsBranch>,
}

impl TaprootTree {
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
    pub fn new(
        secp: &Secp256k1<All>,
        user_pub_key: &XOnlyPublicKey,
        protocol_pub_key: &XOnlyPublicKey,
        covenant_pub_keys: &[XOnlyPublicKey],
        covenant_quorum: u8,
    ) -> Result<Self, CoreError> {
        let mut builder = TaprootBuilder::new();

        let user_protocol_branch =
            <ScriptBuf as BuildUserProtocolBranch>::build(user_pub_key, protocol_pub_key)?;
        let covenants_protocol_branch = <ScriptBuf as BuildCovenantProtocolBranch>::build(
            covenant_pub_keys,
            covenant_quorum,
            protocol_pub_key,
        )?;
        let covenants_user_branch = <ScriptBuf as BuildCovenantUserBranch>::build(
            covenant_pub_keys,
            covenant_quorum,
            user_pub_key,
        )?;

        builder = builder.add_leaf(2, covenants_protocol_branch.clone())?;
        builder = builder.add_leaf(2, covenants_user_branch.clone())?;

        // if have_only_covenants {
        //     builder = builder.add_leaf(2, user_protocol_branch.clone())?;
        //     let only_covenants_branch =
        //         <ScriptBuf as BuildOnlyCovenantsBranch>::build(covenant_pub_keys, covenant_quorum)?;
        //     builder = builder.add_leaf(2, only_covenants_branch.clone())?;

        //     let taproot_spend_info = builder
        //         .finalize(secp, *NUMS_BIP_341)
        //         .map_err(|_| CoreError::TaprootFinalizationFailed)?;

        //     return Ok(Self {
        //         root: taproot_spend_info,
        //         user_protocol_branch,
        //         covenants_protocol_branch,
        //         covenants_user_branch,
        //         only_covenants_branch: Some(only_covenants_branch),
        //     });
        // }

        builder = builder.add_leaf(1, user_protocol_branch.clone())?;

        let taproot_spend_info = builder
            .finalize(secp, *NUMS_BIP_341)
            .map_err(|_| CoreError::TaprootFinalizationFailed)?;

        Ok(Self {
            root: taproot_spend_info,
            user_protocol_branch,
            covenants_protocol_branch,
            covenants_user_branch,
            only_covenants_branch: None,
        })
    }

    /// Creates a Taproot locking script with only covenants spending path.
    ///
    /// ```text
    /// Root
    /// |
    /// 1
    /// |
    /// Only C: OP_CHECKSIG + OP_CHECKSIGADD + ... + OP_GREATERTHANOREQUAL
    /// ```
    pub fn new_with_only_covenants(
        secp: &Secp256k1<All>,
        covenant_pub_keys: &[XOnlyPublicKey],
        covenant_quorum: u8,
    ) -> Result<Self, CoreError> {
        let mut builder = TaprootBuilder::new();

        let only_covenants_branch =
            <ScriptBuf as BuildOnlyCovenantsBranch>::build(covenant_pub_keys, covenant_quorum)?;

        builder = builder.add_leaf(0, only_covenants_branch.clone())?;

        let taproot_spend_info = builder
            .finalize(secp, *NUMS_BIP_341)
            .map_err(|_| CoreError::TaprootFinalizationFailed)?;

        Ok(Self {
            root: taproot_spend_info,
            user_protocol_branch: ScriptBuf::default(),
            covenants_protocol_branch: ScriptBuf::default(),
            covenants_user_branch: ScriptBuf::default(),
            only_covenants_branch: Some(only_covenants_branch),
        })
    }

    pub fn internal_key(&self) -> UntweakedPublicKey {
        self.root.internal_key()
    }

    pub fn merkle_root(&self) -> Option<TapNodeHash> {
        self.root.merkle_root()
    }
}

impl TaprootTree {
    pub fn into_script(self, secp: &Secp256k1<All>) -> ScriptBuf {
        ScriptBuf::new_p2tr(secp, self.internal_key(), self.merkle_root())
    }
}
