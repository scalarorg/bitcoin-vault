use bitcoin::{NetworkKind, Psbt, XOnlyPublicKey};

use super::{
    CoreError, CustodianOnlyStakingParams, CustodianOnlyUnstakingParams, SigningKeyMap,
    StakingOutput, TapScriptSigsMap, UPCStakingParams, UPCUnstakingParams, UnstakingType,
};

pub trait Staking {
    type Error;

    fn build_upc(&self, params: &UPCStakingParams) -> Result<StakingOutput, Self::Error>;

    fn build_custodian_only(
        &self,
        params: &CustodianOnlyStakingParams,
    ) -> Result<StakingOutput, Self::Error>;
}

pub trait Unstaking {
    type Error;

    fn build_upc(
        &self,
        params: &UPCUnstakingParams,
        unstaking_type: UnstakingType,
    ) -> Result<Psbt, Self::Error>;

    fn build_custodian_only(
        &self,
        params: &CustodianOnlyUnstakingParams,
    ) -> Result<Psbt, Self::Error>;
}

pub trait Signing {
    type PsbtHex;

    type TxHex;

    fn sign_psbt_by_single_key(
        psbt: &mut Psbt,
        privkey: &[u8],
        network_kind: NetworkKind,
        finalize: bool,
    ) -> Result<(Self::PsbtHex, SigningKeyMap), CoreError>;

    fn sign_psbt_and_collect_tap_script_sigs(
        psbt: &mut Psbt,
        privkey: &[u8],
        network_kind: NetworkKind,
    ) -> Result<TapScriptSigsMap, CoreError>;

    fn aggregate_tap_script_sigs(
        psbt: &mut Psbt,
        tap_script_sigs: &TapScriptSigsMap,
    ) -> Result<Self::PsbtHex, CoreError>;

    fn finalize_psbt_and_extract_tx(psbt: &mut Psbt) -> Result<Self::TxHex, CoreError>;
}

pub trait BuildTwoPartyBranch {
    fn build(x: &XOnlyPublicKey, y: &XOnlyPublicKey) -> Result<Self, CoreError>
    where
        Self: Sized;
}

pub trait BuildCustodianAndPartyBranch {
    fn build(
        x: &XOnlyPublicKey,
        custodian_pub_keys: &[XOnlyPublicKey],
        custodian_quorum: u8,
    ) -> Result<Self, CoreError>
    where
        Self: Sized;
}

pub trait BuildCustodianOnlyBranch {
    fn build(
        custodian_pub_keys: &[XOnlyPublicKey],
        custodian_quorum: u8,
    ) -> Result<Self, CoreError>
    where
        Self: Sized;
}
