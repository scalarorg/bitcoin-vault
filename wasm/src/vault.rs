use crate::decoder::Decoder;
use crate::errors::VaultABIError;
use bitcoin_vault::Vault;
use wasm_bindgen::prelude::*;

impl From<VaultABIError> for JsValue {
    fn from(err: VaultABIError) -> Self {
        JsValue::from(err.to_string())
    }
}
#[wasm_bindgen]
pub struct VaultWasm {
    tag: Vec<u8>,
    version: u8,
    value: Vault,
}
#[wasm_bindgen]
impl VaultWasm {
    #[wasm_bindgen]
    pub fn new(tag: &[u8], version: u8) -> Self {
        VaultWasm {
            tag: tag.to_vec(),
            version,
            value: Vault::new(),
        }
    }
    #[wasm_bindgen]
    pub fn create_unsigned_vault_psbt(
        &self,
        staker_address: &[u8],
        //33 bytes pubkey
        staker_pubkey: &[u8],
        //33 bytes pubkey
        protocol_pubkey: &[u8],
        //encoded 33 bytes pubkey list, length of each pubkey is 32 bytes
        custodial_pubkeys: &[u8],
        quorum: u8,
        dst_chain_id: u64,
        dst_user_address: &[u8],
        dst_smart_contract_address: &[u8],
    ) -> Result<Vec<u8>, JsValue> {
        let staker_address = Decoder::decode_address(staker_address)?;
        // let xonly_staker_pubkey = Decoder::decode_xonly_pubkey(staker_pubkey)?;
        let staker_pubkey = Decoder::decode_33bytes_pubkey(staker_pubkey)?;
        let protocol_pubkey = Decoder::decode_33bytes_pubkey(protocol_pubkey)?;
        let custodial_pubkeys = Decoder::decode_33bytes_pubkey_list(custodial_pubkeys)?;
        // Ok(staker_address.to_string().as_bytes().to_vec())
        // Ok(xonly_staker_pubkey.serialize().to_vec())
        Ok(staker_pubkey.to_bytes().to_vec())
    }
    #[wasm_bindgen]
    pub fn create_unstaking_vault_psbt(
        &self,
        staker_address: &[u8],
        receiver_address: &[u8],
        tx_hex: &[u8],
        custodial_pubkeys: &[u8],
        quorum: u8,
    ) -> Result<Vec<u8>, JsValue> {
        let staker_address = Decoder::decode_address(staker_address)?;
        let receiver_address = Decoder::decode_address(receiver_address)?;
        let custodial_pubkeys = Decoder::decode_33bytes_pubkey_list(custodial_pubkeys)?;
        Ok(vec![])
    }
}
