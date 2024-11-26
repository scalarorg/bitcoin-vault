use bitcoin::{Amount, PublicKey, TxOut, XOnlyPublicKey};
use validator::Validate;

use super::{
    manager, CoreError, DataScript, DataScriptParams, DataScriptParamsWithOnlyCovenants,
    LockingScript, LockingScriptParams, LockingScriptWithOnlyCovenantsParams, Staking,
    VaultManager, DEST_CHAIN_SIZE, DEST_CONTRACT_ADDRESS_SIZE, DEST_RECIPIENT_ADDRESS_SIZE,
};

/// Type alias for destination address
pub type DestinationContractAddress = [u8; DEST_CONTRACT_ADDRESS_SIZE];

/// Type alias for destination recipient address
pub type DestinationRecipientAddress = [u8; DEST_RECIPIENT_ADDRESS_SIZE];

/// Type alias for destination chain
pub type DestinationChain = [u8; DEST_CHAIN_SIZE];

// TODO: Add validate for params

#[derive(Debug, Validate)]
pub struct BuildStakingParams {
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub covenant_pub_keys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub staking_amount: u64,
    pub have_only_covenants: bool,
    pub destination_chain: DestinationChain,
    pub destination_contract_address: DestinationContractAddress,
    pub destination_recipient_address: DestinationRecipientAddress,
}

#[derive(Debug, Validate)]
pub struct BuildStakingWithOnlyCovenantsParams {
    pub staking_amount: u64,
    pub covenant_pub_keys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub destination_chain: DestinationChain,
    pub destination_contract_address: DestinationContractAddress,
    pub destination_recipient_address: DestinationRecipientAddress,
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
            service_tag: self.service_tag(),
            version: self.version(),
            network_id: self.network_id(),
            have_only_covenants: params.have_only_covenants,
            covenant_quorum: params.covenant_quorum,
            destination_chain_id: &params.destination_chain,
            destination_contract_address: &params.destination_contract_address,
            destination_recipient_address: &params.destination_recipient_address,
        })?;

        Ok(StakingOutput::new(
            params.staking_amount,
            locking_script,
            embedded_data_script,
        ))
    }

    fn build_with_only_covenants(
        &self,
        params: &BuildStakingWithOnlyCovenantsParams,
    ) -> Result<StakingOutput, Self::Error> {
        let covenants_x_only: Vec<XOnlyPublicKey> = params
            .covenant_pub_keys
            .iter()
            .map(|pk| XOnlyPublicKey::from(*pk))
            .collect();

        let locking_script = LockingScript::new_with_only_covenants(
            self.secp(),
            &LockingScriptWithOnlyCovenantsParams {
                covenant_pub_keys: &covenants_x_only,
                covenant_quorum: params.covenant_quorum,
            },
        )?;

        let embedded_data_script =
            DataScript::new_with_only_covenants(&DataScriptParamsWithOnlyCovenants {
                tag: self.tag(),
                version: self.version(),
                network_id: self.network_id(),
                covenant_quorum: params.covenant_quorum,
                destination_chain_id: &params.destination_chain,
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

impl VaultManager {
    pub fn only_covenants_locking_script(
        &self,
        params: &LockingScriptWithOnlyCovenantsParams,
    ) -> Result<LockingScript, CoreError> {
        LockingScript::new_with_only_covenants(self.secp(), params)
    }
}
