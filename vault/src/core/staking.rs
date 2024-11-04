use bitcoin::{Amount, PublicKey, TxOut, XOnlyPublicKey};
use validator::Validate;

use super::{CoreError, ScriptBuilder, Staking, VaultManager, ADDRESS_SIZE, CHAIN_ID_SIZE};

pub type DestinationAddress = [u8; ADDRESS_SIZE];

pub type DestinationChainId = [u8; CHAIN_ID_SIZE];

#[derive(Debug, Validate)]
pub struct BuildStakingOutputParams {
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub covenant_pubkeys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub staking_amount: u64,
    pub have_only_covenants: bool,
    pub destination_chain_id: DestinationChainId,
    pub destination_contract_address: DestinationAddress,
    pub destination_recipient_address: DestinationAddress,
}

impl Staking for VaultManager {
    type Error = CoreError;

    /// This function is used to build the staking outputs
    ///
    /// ### Arguments
    /// * `params` - The parameters for building the staking outputs
    ///
    /// ### Returns
    /// * `Result<Vec<TxOut>, Self::Error>` - The staking outputs or an error
    ///
    fn build_staking_outputs(
        &self,
        params: &BuildStakingOutputParams,
    ) -> Result<Vec<TxOut>, Self::Error> {
        // TODO: 0.validate params by use validator create
        let user_pub_key_x_only = XOnlyPublicKey::from(params.user_pub_key);
        let protocol_pub_key_x_only = XOnlyPublicKey::from(params.protocol_pub_key);
        let covenant_pubkeys_x_only: Vec<XOnlyPublicKey> = params
            .covenant_pubkeys
            .iter()
            .map(|pk| XOnlyPublicKey::from(*pk))
            .collect();

        let lock_script = ScriptBuilder::create_locking_script(
            self.secp(),
            &user_pub_key_x_only,
            &protocol_pub_key_x_only,
            &covenant_pubkeys_x_only,
            params.covenant_quorum,
            params.have_only_covenants,
        )?;

        let embedded_data_script = ScriptBuilder::create_embedded_data_script(
            self.tag(),
            self.version(),
            &params.destination_chain_id,
            &params.destination_contract_address,
            &params.destination_recipient_address,
        )?;

        Ok(vec![
            TxOut {
                value: Amount::from_sat(params.staking_amount),
                script_pubkey: lock_script,
            },
            TxOut {
                value: Amount::from_sat(0),
                script_pubkey: embedded_data_script,
            },
        ])
    }
}
