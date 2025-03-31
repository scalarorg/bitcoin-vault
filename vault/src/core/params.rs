use bitcoin::{Amount, PublicKey};
use validator::Validate;

use super::{
    CoreError, DestinationChain, DestinationRecipientAddress, DestinationTokenAddress,
    PreviousStakingUTXO, UnstakingOutput, HASH_SIZE,
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
    pub inputs: Vec<PreviousStakingUTXO>,
    pub unstaking_output: UnstakingOutput,
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub rbf: bool,
    pub fee_rate: u64,
}

impl UPCUnstakingParams {
    pub fn validate(&self) -> Result<(Amount, Amount), CoreError> {
        if self.inputs.is_empty() {
            return Err(CoreError::InvalidParams(
                "UPCUnstakingParams must have at least one input".to_string(),
            ));
        }

        if self.unstaking_output.amount_in_sats == Amount::ZERO {
            return Err(CoreError::InvalidParams(
                "Unstaking output amount must be greater than 0".to_string(),
            ));
        }

        let total_input_value: Amount = self.inputs.iter().map(|input| input.amount_in_sats).sum();

        // Note: because of the fee will be deducted from the total output value, so we not need to satify the equation
        if total_input_value < self.unstaking_output.amount_in_sats {
            return Err(CoreError::InvalidParams(format!(
                "Total input value must be greater than unstaking output value: {} <= {}",
                total_input_value, self.unstaking_output.amount_in_sats
            )));
        }

        Ok((total_input_value, self.unstaking_output.amount_in_sats))
    }
}

#[derive(Debug, Validate)]
pub struct CustodianOnlyUnstakingParams {
    pub inputs: Vec<PreviousStakingUTXO>,
    pub unstaking_outputs: Vec<UnstakingOutput>,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub rbf: bool,
    pub fee_rate: u64,
    pub session_sequence: u64,
    pub custodian_group_uid: [u8; HASH_SIZE],
}

impl CustodianOnlyUnstakingParams {
    pub fn validate(&self) -> Result<(Amount, Amount), CoreError> {
        if self.inputs.is_empty() {
            return Err(CoreError::InvalidParams(
                "CustodianOnlyUnstakingParams must have at least one input".to_string(),
            ));
        }

        if self.unstaking_outputs.is_empty() {
            return Err(CoreError::InvalidParams(
                "CustodianOnlyUnstakingParams must have at least one unstaking output".to_string(),
            ));
        }

        let total_input_value: Amount = self.inputs.iter().map(|input| input.amount_in_sats).sum();

        let total_output_value: Amount = self
            .unstaking_outputs
            .iter()
            .map(|output| output.amount_in_sats)
            .sum();

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
