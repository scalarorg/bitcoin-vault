use std::convert::TryInto;

use crate::errors::VaultABIError;
use crate::{decoder::Decoder, encoder::Encoder};
use bitcoin::{Amount, NetworkKind, OutPoint, TxOut, Txid};
use bitcoin_vault::{
    BuildStakingOutputParams, BuildUserProtocolSpendParams, DestinationAddress,
    PreviousStakingUTXO, Signing, Staking, StakingManager, Unstaking,
};
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
    pub fn build_user_protocol_spend(
        &self,
        input_script_pubkey: &[u8],
        input_txid: &[u8],
        input_vout: u32,
        input_amount: u64,
        output_script_pubkey: &[u8],
        output_amount: u64,
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        rbf: bool,
    ) -> Result<Vec<u8>, JsValue> {
        let txid: Txid = Decoder::decode_txid(input_txid)?;

        let user_pub_key = Decoder::decode_33bytes_pubkey(staker_pubkey)?;
        let protocol_pub_key = Decoder::decode_33bytes_pubkey(protocol_pubkey)?;
        let covenant_pubkeys = Decoder::decode_33bytes_pubkey_list(covenant_pubkeys)?;

        let params = BuildUserProtocolSpendParams {
            input_utxo: PreviousStakingUTXO {
                script_pubkey: Decoder::decode_script_pubkey(input_script_pubkey),
                outpoint: OutPoint {
                    txid,
                    vout: input_vout,
                },
                amount_in_sats: Amount::from_sat(input_amount),
            },
            unstaking_output: TxOut {
                value: Amount::from_sat(output_amount),
                script_pubkey: Decoder::decode_script_pubkey(output_script_pubkey),
            },
            user_pub_key,
            protocol_pub_key,
            covenant_pubkeys,
            covenant_quorum,
            have_only_covenants,
            rbf,
        };

        match self.staking.build_user_protocol_spend(&params) {
            Ok(psbt) => Ok(psbt.serialize()),
            Err(_) => Ok(vec![]),
        }
    }

    #[wasm_bindgen]
    pub fn sign_psbt_by_single_key(
        &self,
        psbt: &[u8],
        privkey: &[u8], //32 bytes
        is_testnet: bool,
        finalize: bool,
    ) -> Result<Vec<u8>, JsValue> {
        let mut psbt = Decoder::decode_psbt(psbt)?;
        let network_kind = if is_testnet {
            NetworkKind::Test
        } else {
            NetworkKind::Main
        };
        let signed_psbt =
            StakingManager::sign_psbt_by_single_key(&mut psbt, privkey, network_kind, finalize)
                .map_err(|e| VaultABIError::DecodingError(format!("{}", e)))?;
        Ok(signed_psbt)
    }
}
