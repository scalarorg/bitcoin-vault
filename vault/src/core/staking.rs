use bitcoin::{Amount, PublicKey, TxOut};
use validator::Validate;

use super::{
    manager, CoreError, DataScript, DataScriptParams, LockingScript, LockingScriptParams, Staking,
    VaultManager, ADDRESS_SIZE, CHAIN_ID_SIZE,
};

pub type DestinationAddress = [u8; ADDRESS_SIZE];

pub type DestinationChainId = [u8; CHAIN_ID_SIZE];

#[derive(Debug, Validate)]
pub struct BuildStakingParams {
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub covenant_pub_keys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub staking_amount: u64,
    pub have_only_covenants: bool,
    pub destination_chain_id: DestinationChainId,
    pub destination_contract_address: DestinationAddress,
    pub destination_recipient_address: DestinationAddress,
}

#[derive(Debug)]
pub struct StakingOutput {
    staking_amount: u64,
    locking_script: LockingScript,
    embedded_data_script: DataScript,
}

impl StakingOutput {
    pub fn new(
        staking_amount: u64,
        locking_script: LockingScript,
        embedded_data_script: DataScript,
    ) -> Self {
        Self {
            staking_amount,
            locking_script,
            embedded_data_script,
        }
    }

    pub fn into_tx_outs(self) -> Vec<TxOut> {
        vec![
            TxOut {
                value: Amount::from_sat(self.staking_amount),
                script_pubkey: self.locking_script.into_script(),
            },
            TxOut {
                value: Amount::from_sat(0),
                script_pubkey: self.embedded_data_script.into_script(),
            },
        ]
    }
}

impl Staking for VaultManager {
    type Error = CoreError;

    fn build(&self, params: &BuildStakingParams) -> Result<StakingOutput, Self::Error> {
        // TODO: 0.validate params by use validator create
        let x_only_keys = manager::VaultManager::convert_to_x_only_keys(
            &params.user_pub_key,
            &params.protocol_pub_key,
            &params.covenant_pub_keys,
        );

        let locking_script = LockingScript::new(
            self.secp(),
            &LockingScriptParams {
                user_pub_key: &x_only_keys.user,
                protocol_pub_key: &x_only_keys.protocol,
                covenant_pub_keys: &x_only_keys.covenants,
                covenant_quorum: params.covenant_quorum,
                have_only_covenants: params.have_only_covenants,
            },
        )?;

        let embedded_data_script = DataScript::new(&DataScriptParams {
            tag: self.tag(),
            version: self.version(),
            destination_chain_id: &params.destination_chain_id,
            destination_contract_address: &params.destination_contract_address,
            destination_recipient_address: &params.destination_recipient_address,
        })?;

        Ok(StakingOutput::new(
            params.staking_amount,
            locking_script,
            embedded_data_script,
        ))
    }
}