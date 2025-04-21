use bitcoin::{Amount, PublicKey, TxOut};
use validator::Validate;

use super::{
    CoreError, DestinationChain, DestinationRecipientAddress, DestinationTokenAddress,
    PreviousOutpoint, UnlockingType, HASH_SIZE,
};

// TODO: Add validate for params
#[derive(Debug, Validate)]
pub struct UPCLockingParams {
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub staking_amount: u64,
    pub destination_chain: DestinationChain,
    pub destination_token_address: DestinationTokenAddress,
    pub destination_recipient_address: DestinationRecipientAddress,
}

/// Because the unlocking tx is formed from a previous staking tx, 1 - 1 mapping is used.
/// So we just need one input and one output.
#[derive(Debug, Validate)]
pub struct UPCUnlockingParams {
    pub inputs: Vec<PreviousOutpoint>,
    pub output: TxOut,
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub rbf: bool,
    pub fee_rate: u64,
    pub typ: UnlockingType,
}

impl UPCUnlockingParams {
    pub fn validate(&self) -> Result<(Amount, Amount), CoreError> {
        if self.inputs.is_empty() {
            return Err(CoreError::InvalidParams(
                "UPCUnlockingParams must have at least one input".to_string(),
            ));
        }

        if self.output.value == Amount::ZERO {
            return Err(CoreError::InvalidParams(
                "Unlocking output amount must be greater than 0".to_string(),
            ));
        }

        let total_input_value: Amount = self.inputs.iter().map(|input| input.amount_in_sats).sum();

        // Note: because of the fee will be deducted from the total output value, so we not need to satify the equation
        if total_input_value < self.output.value {
            return Err(CoreError::InvalidParams(format!(
                "Total input value must be greater than unlocking output value: {} <= {}",
                total_input_value, self.output.value
            )));
        }

        Ok((total_input_value, self.output.value))
    }
}

#[derive(Debug, Validate)]
pub struct CustodianOnlyLockingParams {
    pub staking_amount: u64,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub destination_chain: DestinationChain,
    pub destination_token_address: DestinationTokenAddress,
    pub destination_recipient_address: DestinationRecipientAddress,
}

#[derive(Debug, Validate)]
pub struct CustodianOnlyUnlockingParams {
    pub inputs: Vec<PreviousOutpoint>,
    pub outputs: Vec<TxOut>,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub rbf: bool,
    pub fee_rate: u64,
    pub session_sequence: u64,
    pub custodian_group_uid: [u8; HASH_SIZE],
}

impl CustodianOnlyUnlockingParams {
    pub fn validate(&self) -> Result<(Amount, Amount), CoreError> {
        if self.inputs.is_empty() {
            return Err(CoreError::InvalidParams(
                "CustodianOnlyUnlockingParams must have at least one input".to_string(),
            ));
        }

        if self.outputs.is_empty() {
            return Err(CoreError::InvalidParams(
                "CustodianOnlyUnlockingParams must have at least one unlocking output".to_string(),
            ));
        }

        let total_input_value: Amount = self.inputs.iter().map(|input| input.amount_in_sats).sum();

        let total_output_value: Amount = self.outputs.iter().map(|output| output.value).sum();

        // Note: because of the fee will be deducted from the total output value, so we not need to satify the equation
        if total_input_value < total_output_value {
            return Err(CoreError::InvalidParams(format!(
                "Total input value must be greater than total output value: {} <= {}",
                total_input_value, total_output_value
            )));
        }

        Ok((total_input_value, total_output_value))
    }
}

#[derive(Debug, Validate)]
pub struct TimeLockedExitParams {
    pub staking_amount: u64,
}
