use bitcoin::{
    key::{Secp256k1, UntweakedPublicKey},
    secp256k1::All,
    taproot::{TaprootBuilder, TaprootSpendInfo},
    ScriptBuf, TapNodeHash, XOnlyPublicKey,
};

use super::{
    BuildCustodianAndPartyBranch, BuildCustodianOnlyBranch, BuildPartyWithSequenceVerification,
    BuildTwoPartyBranch, CoreError, CustodianAndPartyBranch, CustodianOnlyBranch,
    PartyWithSequenceVerification, TwoPartyBranch,
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref NUMS_BIP_341: XOnlyPublicKey = XOnlyPublicKey::from_slice(&[
        0x50, 0x92, 0x9b, 0x74, 0xc1, 0xa0, 0x49, 0x54, 0xb7, 0x8b, 0x4b, 0x60, 0x35, 0xe9, 0x7a,
        0x5e, 0x07, 0x8a, 0x5a, 0x0f, 0x28, 0xec, 0x96, 0xd5, 0x47, 0xbf, 0xee, 0x9a, 0xce, 0x80,
        0x3a, 0xc0,
    ])
    .unwrap();
}

#[derive(Debug, Clone)]
pub struct UPCTaprootTree {
    pub user_protocol_branch: TwoPartyBranch,
    pub custodian_user_branch: CustodianAndPartyBranch,
    pub custodian_protocol_branch: CustodianAndPartyBranch,
}

#[derive(Debug, Clone)]
pub struct CustodianOnlyTree {
    pub only_custodian_branch: CustodianOnlyBranch,
}

#[derive(Debug, Clone)]
pub struct TimeGatedTree {
    pub csv_party_branch: PartyWithSequenceVerification,
    pub only_custodian_branch: CustodianOnlyBranch,
}

#[derive(Debug, Clone)]
pub struct TaprootTree<T> {
    pub root: TaprootSpendInfo,
    pub raw: T,
}

impl TaprootTree<UPCTaprootTree> {
    /// Creates a Taproot locking script with multiple spending conditions.
    ///
    /// This function constructs a Taproot script tree with different spending paths:
    /// - User + Protocol
    /// - User + Custodian
    /// - Protocol + Custodian
    /// - Only Custodian (optional)
    ///
    /// The resulting tree structure:
    ///
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
    /// U + P    U + C    P + C
    /// ```
    ///
    /// ### Arguments
    /// * `secp` - The secp256k1 context
    /// * `party_pub_keys` - The party's public keys
    /// * `custodian_pub_keys` - The custodian's public keys
    /// * `custodian_quorum` - The number of custodian signatures required
    ///
    /// ### Returns
    /// * `Result<ScriptBuf, CoreError>` - The resulting Taproot script or an error
    ///
    pub fn new(
        secp: &Secp256k1<All>,
        user_pub_key: XOnlyPublicKey,
        protocol_pub_key: XOnlyPublicKey,
        custodian_pub_keys: Vec<XOnlyPublicKey>,
        custodian_quorum: u8,
    ) -> Result<Self, CoreError> {
        let mut builder = TaprootBuilder::new();

        let up_branch =
            <ScriptBuf as BuildTwoPartyBranch>::build(&user_pub_key, &protocol_pub_key)?;

        let uc_branch = <ScriptBuf as BuildCustodianAndPartyBranch>::build(
            &user_pub_key,
            &custodian_pub_keys,
            custodian_quorum,
        )?;

        let pc_branch = <ScriptBuf as BuildCustodianAndPartyBranch>::build(
            &protocol_pub_key,
            &custodian_pub_keys,
            custodian_quorum,
        )?;

        builder = builder.add_leaf(1, up_branch.clone())?;
        builder = builder.add_leaf(2, uc_branch.clone())?;
        builder = builder.add_leaf(2, pc_branch.clone())?;

        let taproot_spend_info = builder
            .finalize(secp, *NUMS_BIP_341)
            .map_err(|_| CoreError::TaprootFinalizationFailed)?;

        Ok(Self {
            root: taproot_spend_info,
            raw: UPCTaprootTree {
                user_protocol_branch: up_branch,
                custodian_user_branch: uc_branch,
                custodian_protocol_branch: pc_branch,
            },
        })
    }
}

impl TaprootTree<CustodianOnlyTree> {
    /// Creates a Taproot locking script with only custodian spending path.
    ///
    /// ```text
    ///     Root
    ///      |
    ///      1
    ///      |
    ///    Only Custodian: OP_CHECKSIG + OP_CHECKSIGADD +... + OP_GREATERTHANOREQUAL
    /// ```
    pub fn new(
        secp: &Secp256k1<All>,
        custodian_pub_keys: &[XOnlyPublicKey],
        custodian_quorum: u8,
    ) -> Result<Self, CoreError> {
        let mut builder = TaprootBuilder::new();

        let only_custodian_branch =
            <ScriptBuf as BuildCustodianOnlyBranch>::build(custodian_pub_keys, custodian_quorum)?;

        builder = builder.add_leaf(0, only_custodian_branch.clone())?;

        let taproot_spend_info = builder
            .finalize(secp, *NUMS_BIP_341)
            .map_err(|_| CoreError::TaprootFinalizationFailed)?;

        Ok(Self {
            root: taproot_spend_info,
            raw: CustodianOnlyTree {
                only_custodian_branch,
            },
        })
    }
}

impl TaprootTree<TimeGatedTree> {
    /// Creates a Taproot locking script with only custodian spending path and a sequence locked path.
    ///
    /// ```text
    ///        Root
    ///       /    \
    ///      /      \
    ///     /        \
    ///    /          \
    ///   1            1
    ///   |            |
    ///   |            |
    ///   |            |
    ///   |            |
    /// CustodianOnly SequenceLocked   
    /// ```
    pub fn new(
        secp: &Secp256k1<All>,
        party: &XOnlyPublicKey,
        custodian_pub_keys: &[XOnlyPublicKey],
        custodian_quorum: u8,
        sequence: i64,
    ) -> Result<Self, CoreError> {
        let mut builder = TaprootBuilder::new();

        let only_custodian_branch =
            <ScriptBuf as BuildCustodianOnlyBranch>::build(custodian_pub_keys, custodian_quorum)?;

        let csv_branch = <ScriptBuf as BuildPartyWithSequenceVerification>::build(party, sequence)?;

        builder = builder.add_leaf(1, csv_branch.clone())?;
        builder = builder.add_leaf(1, only_custodian_branch.clone())?;

        let taproot_spend_info = builder
            .finalize(secp, *NUMS_BIP_341)
            .map_err(|_| CoreError::TaprootFinalizationFailed)?;

        Ok(Self {
            root: taproot_spend_info,
            raw: TimeGatedTree {
                csv_party_branch: csv_branch,
                only_custodian_branch,
            },
        })
    }
}

impl<T> TaprootTree<T> {
    pub fn internal_key(&self) -> UntweakedPublicKey {
        self.root.internal_key()
    }

    pub fn merkle_root(&self) -> Option<TapNodeHash> {
        self.root.merkle_root()
    }
    pub fn into_script(self, secp: &Secp256k1<All>) -> ScriptBuf {
        ScriptBuf::new_p2tr(secp, self.internal_key(), self.merkle_root())
    }
}
