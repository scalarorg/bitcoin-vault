use bitcoin::{PublicKey, ScriptBuf};
use validator::Validate;

use super::{
    DestinationChain, DestinationRecipientAddress, DestinationTokenAddress, PreviousStakingUTXO,
    UnstakingOutput,
};

// TODO: Add validate for params

#[derive(Debug, Validate)]
pub struct UPCStakingParams {
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub staking_amount: u64,
    pub destination_chain: DestinationChain,
    pub destination_token_address: DestinationTokenAddress,
    pub destination_recipient_address: DestinationRecipientAddress,
}

#[derive(Debug, Validate)]
pub struct CustodianOnlyStakingParams {
    pub staking_amount: u64,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub destination_chain: DestinationChain,
    pub destination_token_address: DestinationTokenAddress,
    pub destination_recipient_address: DestinationRecipientAddress,
}

/// Because the unstaking tx is formed from a previous staking tx, 1 - 1 mapping is used.
/// So we just need one input and one output.
#[derive(Debug, Validate)]
pub struct UPCUnstakingParams {
    pub input: PreviousStakingUTXO,
    pub locking_script: ScriptBuf,
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub rbf: bool,
    pub fee_rate: u64,
}

#[derive(Debug, Validate)]
pub struct CustodianOnlyUnstakingParams {
    pub inputs: Vec<PreviousStakingUTXO>,
    pub unstaking_outputs: Vec<UnstakingOutput>,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub rbf: bool,
    pub fee_rate: u64,
}
