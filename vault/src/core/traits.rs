use bitcoin::{NetworkKind, Psbt, TxOut};

use super::{BuildStakingOutputParams, BuildUserProtocolSpendParams, CoreError};

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

    // fn build_covenants_protocol_spend(
    //     &self,
    //     params: &BuildCovenantsProtocolSpendParams,
    // ) -> Result<Psbt, Self::Error>;

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
