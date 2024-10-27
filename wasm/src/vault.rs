use std::convert::TryInto;

use crate::errors::VaultABIError;
use crate::{decoder::Decoder, encoder::Encoder};
use bitcoin::{Amount, TxOut};
use bitcoin_vault::{BuildStakingOutputParams, DestinationAddress, Staking, StakingManager};
use wasm_bindgen::prelude::*;
use web_sys::console;
impl From<VaultABIError> for JsValue {
    fn from(err: VaultABIError) -> Self {
        JsValue::from(err.to_string())
    }
}
#[wasm_bindgen]
pub struct VaultWasm {
    tag: Vec<u8>,
    version: u8,
    staking: StakingManager,
}
#[wasm_bindgen]
impl VaultWasm {
    #[wasm_bindgen]
    pub fn new(tag: &[u8], version: u8) -> Self {
        VaultWasm {
            tag: tag.to_vec(),
            version,
            staking: StakingManager::new(tag.to_vec(), version),
        }
    }
    #[wasm_bindgen]
    pub fn build_staking_output(
        &self,
        staking_amount: u64,
        //33 bytes pubkey
        staker_pubkey: &[u8],
        //33 bytes pubkey
        protocol_pubkey: &[u8],
        //encoded 33 bytes pubkey list, length of each pubkey is 32 bytes
        custodial_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        destination_chain_id: u64,
        destination_smartcontract_address: &[u8],
        destination_recipient_address: &[u8],
    ) -> Result<Vec<u8>, JsValue> {
        // console::debug_1(&"Wasm#build_staking_output".into());
        // let destination_smartcontract_address_str =
        //     String::from_utf8(destination_smartcontract_address.to_vec())
        //         .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        // console::debug_1(
        //     &format!(
        //         "input destination_smartcontract_address length: {:?}",
        //         destination_smartcontract_address.len()
        //     )
        //     .into(),
        // );
        // console::debug_1(&destination_smartcontract_address_str.into());
        let destination_contract_address: DestinationAddress = destination_smartcontract_address
            .try_into()
            .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        // console::debug_1(&"Wasm#parsed destination_contract_address".into());
        let destination_recipient_address: DestinationAddress = destination_recipient_address
            .try_into()
            .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        let params = BuildStakingOutputParams {
            staking_amount,
            user_pub_key: Decoder::decode_33bytes_pubkey(staker_pubkey)?,
            protocol_pub_key: Decoder::decode_33bytes_pubkey(protocol_pubkey)?,
            covenant_pubkeys: Decoder::decode_33bytes_pubkey_list(custodial_pubkeys)?,
            covenant_quorum,
            have_only_covenants,
            destination_chain_id: destination_chain_id.to_be_bytes(),
            destination_contract_address,
            destination_recipient_address,
        };

        match self.staking.build_staking_outputs(&params) {
            Ok(tx_outs) => Ok(Encoder::serialize_tx_outs(&tx_outs)),
            Err(_) => Ok(vec![]),
        }
    }
    #[wasm_bindgen]
    pub fn build_unstaking_output(
        &self,
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        custodial_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        tx_hex: &[u8],
    ) -> Result<Vec<u8>, JsValue> {
        Ok(vec![])
    }
    #[wasm_bindgen]
    pub fn create_unsigned_vault_psbt(
        &self,
        staker_script_pubkey: &[u8],
        //33 bytes pubkey
        staker_pubkey: &[u8],
        //33 bytes pubkey
        protocol_pubkey: &[u8],
        //encoded 33 bytes pubkey list, length of each pubkey is 32 bytes
        custodial_pubkeys: &[u8],
        quorum: u8,
        utxos: &[u8],
        dst_chain_id: u64,
        dst_user_address: &[u8],
        dst_smart_contract_address: &[u8],
    ) -> Result<Vec<u8>, JsValue> {
        let staker_script_buf = Decoder::decode_script_pubkey(staker_script_pubkey);
        // let xonly_staker_pubkey = Decoder::decode_xonly_pubkey(staker_pubkey)?;
        let staker_pubkey = Decoder::decode_33bytes_pubkey(staker_pubkey)?;
        let protocol_pubkey = Decoder::decode_33bytes_pubkey(protocol_pubkey)?;
        let custodial_pubkeys = Decoder::decode_33bytes_pubkey_list(custodial_pubkeys)?;
        // decode utxos
        let utxos = Decoder::decode_utxo_list(utxos)?;
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
