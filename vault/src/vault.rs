use bitcoin::{Address, Network};

use crate::errors::VaultError;

pub struct Vault;
impl Vault {
    pub fn new() -> Self {
        Self
    }
}
impl Vault {
    pub fn create_unsigned_vault_psbt(
        &self,
        staker_address_ref: &[u8],
        staker_pubkey_ref: &[u8],
        protocol_pubkey_ref: &[u8],
        custodial_pubkeys_ref: &[u8],
        // Number of custodial pubkeys
        key_len: u8,
        quorum: u8,
        tag: &[u8],
        version: u8,
        dst_chain_id: u64,
        dst_user_address: &[u8],
        dst_smart_contract_address: &[u8],
    ) -> Result<Vec<u8>, VaultError> {
        Ok(vec![])
    }
    pub fn create_unstaking_vault_psbt(
        &self,
        staker_address: &[u8],
        receiver_address: &[u8],
        tx_hex: &[u8],
        tx_len: u32,
        custodial_pubkeys: &[u8],
        // Number of custodial pubkeys
        key_len: u8,
        quorum: u8,
    ) -> Result<Vec<u8>, VaultError> {
        let res = vec![0u8; 1000];

        Ok(res)
    }
}
