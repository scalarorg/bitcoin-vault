use bitcoin::{Amount, TxOut, XOnlyPublicKey};

use super::{
    get_global_secp, manager, CoreError, CustodianOnlyDataParams, CustodianOnlyLockingScriptParams,
    CustodianOnlyStakingParams, DataScript, DataScriptParams, LockingScript, Staking,
    UPCLockingScriptParams, UPCStakingParams, VaultManager, DEST_CHAIN_SIZE,
    DEST_RECIPIENT_ADDRESS_SIZE, DEST_TOKEN_ADDRESS_SIZE,
};

/// Type alias for destination address
pub type DestinationTokenAddress = [u8; DEST_TOKEN_ADDRESS_SIZE];

/// Type alias for destination recipient address
pub type DestinationRecipientAddress = [u8; DEST_RECIPIENT_ADDRESS_SIZE];

/// Type alias for destination chain
pub type DestinationChain = [u8; DEST_CHAIN_SIZE];

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
                value: Amount::from_sat(0),
                script_pubkey: self.embedded_data_script.into_script(),
            },
            TxOut {
                value: Amount::from_sat(self.staking_amount),
                script_pubkey: self.locking_script.into_script(),
            },
        ]
    }
}

impl Staking for VaultManager {
    type Error = CoreError;

    fn build_upc(&self, params: &UPCStakingParams) -> Result<StakingOutput, Self::Error> {
        let secp = get_global_secp();
        // TODO: validate params
        let x_only_keys = manager::VaultManager::convert_upc_to_x_only_keys(
            &params.user_pub_key,
            &params.protocol_pub_key,
            &params.custodian_pub_keys,
        );

        let locking_script = LockingScript::new_upc(
            secp,
            &UPCLockingScriptParams {
                user_pub_key: &x_only_keys.user,
                protocol_pub_key: &x_only_keys.protocol,
                custodian_pub_keys: &x_only_keys.custodians,
                custodian_quorum: params.custodian_quorum,
            },
        )?;

        let embedded_data_script = DataScript::new_upc(&DataScriptParams {
            tag: self.tag(),
            service_tag: self.service_tag(),
            version: self.version(),
            network_id: self.network_id(),
            custodian_quorum: params.custodian_quorum,
            destination_chain_id: &params.destination_chain,
            destination_token_address: &params.destination_token_address,
            destination_recipient_address: &params.destination_recipient_address,
        })?;

        Ok(StakingOutput::new(
            params.staking_amount,
            locking_script,
            embedded_data_script,
        ))
    }

    fn build_custodian_only(
        &self,
        params: &CustodianOnlyStakingParams,
    ) -> Result<StakingOutput, Self::Error> {
        let secp = get_global_secp();
        // TODO: validate params
        let custodians_x_only: Vec<XOnlyPublicKey> = params
            .custodian_pub_keys
            .iter()
            .map(|pk| XOnlyPublicKey::from(*pk))
            .collect();

        let locking_script = LockingScript::new_custodian_only(
            secp,
            &CustodianOnlyLockingScriptParams {
                custodian_pub_keys: &custodians_x_only,
                custodian_quorum: params.custodian_quorum,
            },
        )?;

        let embedded_data_script = DataScript::new_custodian_only(&CustodianOnlyDataParams {
            tag: self.tag(),
            version: self.version(),
            network_id: self.network_id(),
            service_tag: self.service_tag(),
            custodian_quorum: params.custodian_quorum,
            destination_chain_id: &params.destination_chain,
            destination_token_address: &params.destination_token_address,
            destination_recipient_address: &params.destination_recipient_address,
        })?;

        Ok(StakingOutput::new(
            params.staking_amount,
            locking_script,
            embedded_data_script,
        ))
    }
}
