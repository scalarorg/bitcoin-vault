use bitcoin::{NetworkKind, Psbt, XOnlyPublicKey};

use super::{
    BuildCovenantsProtocolSpendParams, BuildStakingOutputParams, BuildUserProtocolSpendParams,
    CoreError, StakingOutput,
};

pub trait Staking {
    type Error;

    fn build(&self, params: &BuildStakingOutputParams) -> Result<StakingOutput, Self::Error>;
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

    // fn build_covenants_user_spend(
    //     &self,
    //     params: &BuildCovenantsUserSpendParams,
    // ) -> Result<Psbt, Self::Error>;
}

pub trait Signing {
    type PsbtHex;

    fn sign_psbt_by_single_key(
        psbt: &mut Psbt,
        privkey: &[u8],
        network_kind: NetworkKind,
        finalize: bool,
    ) -> Result<Self::PsbtHex, CoreError>;
}

pub trait BuildUserProtocolBranch {
    fn build(
        user_pub_key: &XOnlyPublicKey,
        protocol_pub_key: &XOnlyPublicKey,
    ) -> Result<Self, CoreError>
    where
        Self: Sized;
}

pub trait BuildCovenantProtocolBranch {
    fn build(
        covenant_pub_keys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        protocol_pub_key: &XOnlyPublicKey,
    ) -> Result<Self, CoreError>
    where
        Self: Sized;
}

pub trait BuildCovenantUserBranch {
    fn build(
        covenant_pub_keys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        user_pub_key: &XOnlyPublicKey,
    ) -> Result<Self, CoreError>
    where
        Self: Sized;
}

pub trait BuildOnlyCovenantsBranch {
    fn build(covenant_pub_keys: &[XOnlyPublicKey], covenant_quorum: u8) -> Result<Self, CoreError>
    where
        Self: Sized;
}
