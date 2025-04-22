use std::convert::{TryFrom, TryInto};

use crate::errors::VaultABIError;
use crate::{decoder::Decoder, encoder::Encoder};
use bitcoin::{Amount, NetworkKind, OutPoint, PublicKey, TxOut};
use vault::{
    CustodianOnly, CustodianOnlyLockingParams, DestinationChain, DestinationRecipientAddress,
    DestinationTokenAddress, PreviousOutpoint, Signing, UPCLockingParams, UPCUnlockingParams,
    UPCUnlockingType, VaultManager, UPC,
};

use wasm_bindgen::prelude::*;

impl From<VaultABIError> for JsValue {
    fn from(err: VaultABIError) -> Self {
        JsValue::from(err.to_string())
    }
}

#[wasm_bindgen]
pub enum UnlockingTypeWasm {
    UserProtocol,
    CustodianProtocol,
    CustodianUser,
}

impl TryFrom<UnlockingTypeWasm> for UPCUnlockingType {
    type Error = VaultABIError;
    fn try_from(value: UnlockingTypeWasm) -> Result<Self, Self::Error> {
        match value {
            UnlockingTypeWasm::UserProtocol => Ok(UPCUnlockingType::UserProtocol),
            UnlockingTypeWasm::CustodianProtocol => Ok(UPCUnlockingType::CustodianProtocol),
            UnlockingTypeWasm::CustodianUser => Ok(UPCUnlockingType::CustodianUser),
        }
    }
}

#[wasm_bindgen]
pub struct TxOutWasm {
    script_pubkey: Vec<u8>,
    amount: u64,
}

#[wasm_bindgen]
impl TxOutWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(script_pubkey: Vec<u8>, amount: u64) -> Self {
        Self {
            script_pubkey,
            amount,
        }
    }
}

impl TryFrom<TxOutWasm> for TxOut {
    type Error = VaultABIError;

    fn try_from(output: TxOutWasm) -> Result<Self, Self::Error> {
        Ok(TxOut {
            value: Amount::from_sat(output.amount),
            script_pubkey: Decoder::decode_script_pubkey(&output.script_pubkey),
        })
    }
}

#[wasm_bindgen]
pub struct PreviousOutpointWasm {
    script_pubkey: Vec<u8>,
    txid: Vec<u8>,
    vout: u32,
    amount: u64,
}

#[wasm_bindgen]
impl PreviousOutpointWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(script_pubkey: Vec<u8>, txid: Vec<u8>, vout: u32, amount: u64) -> Self {
        Self {
            script_pubkey,
            txid,
            vout,
            amount,
        }
    }
}

impl TryFrom<PreviousOutpointWasm> for PreviousOutpoint {
    type Error = VaultABIError;

    fn try_from(input: PreviousOutpointWasm) -> Result<Self, Self::Error> {
        // ### Description
        // ### Reversed txid is used to match the byte order of the txid in the previous staking UTXO.
        // ### References: https://learnmeabitcoin.com/technical/general/byte-order/#natural-byte-order
        let mut txid_bytes = input.txid.clone();
        txid_bytes.reverse();

        let txid = Decoder::decode_txid(&txid_bytes)?;
        let script_pubkey = Decoder::decode_script_pubkey(&input.script_pubkey);

        Ok(PreviousOutpoint {
            script_pubkey,
            outpoint: OutPoint {
                txid,
                vout: input.vout,
            },
            amount_in_sats: Amount::from_sat(input.amount),
        })
    }
}

#[wasm_bindgen]
pub struct UpcLockingParamsWasm {
    staking_amount: u64,
    staker_pubkey: Vec<u8>,
    protocol_pubkey: Vec<u8>,
    custodial_pubkeys: Vec<u8>,
    custodian_quorum: u8,
    destination_chain: Vec<u8>,
    destination_token_address: Vec<u8>,
    destination_recipient_address: Vec<u8>,
}

impl TryFrom<UpcLockingParamsWasm> for UPCLockingParams {
    type Error = JsValue;
    fn try_from(params: UpcLockingParamsWasm) -> Result<Self, Self::Error> {
        let (destination_chain, destination_token_address, destination_recipient_address) =
            VaultWasm::parse_destination_params(
                &params.destination_chain,
                &params.destination_token_address,
                &params.destination_recipient_address,
            )?;

        let (user_pubkey, protocol_pubkey, custodian_pubkeys) = VaultWasm::parse_pubkeys(
            &params.staker_pubkey,
            &params.protocol_pubkey,
            &params.custodial_pubkeys,
        )?;

        Ok(UPCLockingParams {
            locking_amount: params.staking_amount,
            user_pubkey,
            protocol_pubkey,
            custodian_pubkeys,
            custodian_quorum: params.custodian_quorum,
            destination_chain,
            destination_token_address,
            destination_recipient_address,
        })
    }
}

#[wasm_bindgen]
pub struct UpcUnlockingParamsWasm {
    inputs: Vec<PreviousOutpointWasm>,
    output: TxOutWasm,
    staker_pubkey: Vec<u8>,
    protocol_pubkey: Vec<u8>,
    custodian_pubkeys: Vec<u8>,
    custodian_quorum: u8,
    fee_rate: u64,
    rbf: bool,
    unlocking_type: UnlockingTypeWasm,
}

impl TryFrom<UpcUnlockingParamsWasm> for UPCUnlockingParams {
    type Error = JsValue;
    fn try_from(params: UpcUnlockingParamsWasm) -> Result<Self, Self::Error> {
        let (user_pubkey, protocol_pubkey, custodian_pubkeys) = VaultWasm::parse_pubkeys(
            &params.staker_pubkey,
            &params.protocol_pubkey,
            &params.custodian_pubkeys,
        )?;

        let inputs: Vec<PreviousOutpoint> = params
            .inputs
            .into_iter()
            .map(|input| input.try_into().unwrap())
            .collect();

        Ok(UPCUnlockingParams {
            inputs,
            output: params.output.try_into()?,
            user_pubkey,
            protocol_pubkey,
            custodian_pubkeys,
            custodian_quorum: params.custodian_quorum,
            rbf: params.rbf,
            fee_rate: params.fee_rate,
            typ: UPCUnlockingType::try_from(params.unlocking_type)?,
        })
    }
}

#[wasm_bindgen]
pub struct VaultWasm {
    manager: VaultManager,
}

impl VaultWasm {
    fn parse_pubkeys(
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        custodian_pubkeys: &[u8],
    ) -> Result<(PublicKey, PublicKey, Vec<PublicKey>), VaultABIError> {
        Ok((
            Decoder::decode_33bytes_pubkey(staker_pubkey)?,
            Decoder::decode_33bytes_pubkey(protocol_pubkey)?,
            Decoder::decode_33bytes_pubkey_list(custodian_pubkeys)?,
        ))
    }

    fn convert_error<T, E: std::fmt::Debug>(result: Result<T, E>) -> Result<T, JsValue> {
        result.map_err(|e| JsValue::from(format!("{:?}", e)))
    }

    fn parse_destination_params(
        destination_chain: &[u8],
        destination_smartcontract_address: &[u8],
        destination_recipient_address: &[u8],
    ) -> Result<
        (
            DestinationChain,
            DestinationTokenAddress,
            DestinationRecipientAddress,
        ),
        JsValue,
    > {
        Ok((
            Self::convert_error(destination_chain.try_into())?,
            Self::convert_error(destination_smartcontract_address.try_into())?,
            Self::convert_error(destination_recipient_address.try_into())?,
        ))
    }

    fn handle_serialize_result<T>(
        result: Result<T, impl std::fmt::Debug>,
        f: impl FnOnce(T) -> Vec<u8>,
    ) -> Result<Vec<u8>, JsValue> {
        match result {
            Ok(output) => Ok(f(output)),
            Err(_) => Ok(vec![]),
        }
    }
}

#[wasm_bindgen]
impl VaultWasm {
    #[wasm_bindgen]
    pub fn new(tag: &[u8], service_tag: &[u8], version: u8, network_id: u8) -> Self {
        VaultWasm {
            manager: VaultManager::new(tag.to_vec(), service_tag.to_vec(), version, network_id),
        }
    }

    #[wasm_bindgen]
    pub fn build_upc_locking(&self, params: UpcLockingParamsWasm) -> Result<Vec<u8>, JsValue> {
        Self::handle_serialize_result(
            <VaultManager as UPC>::build_locking_output(&self.manager, &params.try_into()?),
            |output| Encoder::serialize_tx_outs(&output.into_tx_outs()),
        )
    }

    #[wasm_bindgen]
    pub fn build_upc_unlocking(&self, params: UpcUnlockingParamsWasm) -> Result<Vec<u8>, JsValue> {
        // ### Description
        // ### Reversed txid is used to match the byte order of the txid in the previous staking UTXO.
        // ### References: https://learnmeabitcoin.com/technical/general/byte-order/#natural-byte-order

        Self::handle_serialize_result(
            <VaultManager as UPC>::build_unlocking_psbt(&self.manager, &params.try_into()?),
            |psbt| psbt.serialize(),
        )
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
        let (signed_psbt, _) =
            VaultManager::sign_psbt_by_single_key(&mut psbt, privkey, network_kind, finalize)
                .map_err(|e| VaultABIError::DecodingError(format!("{}", e)))?;
        Ok(signed_psbt)
    }
}

#[wasm_bindgen]
impl VaultWasm {
    #[wasm_bindgen]
    pub fn build_custodian_only_locking(
        &self,
        amount: u64,
        //33 bytes pubkey
        custodial_pubkeys: &[u8],
        custodian_quorum: u8,
        destination_chain: &[u8],
        destination_token_address: &[u8],
        destination_recipient_address: &[u8],
    ) -> Result<Vec<u8>, JsValue> {
        let (destination_chain, destination_token_address, destination_recipient_address) =
            VaultWasm::parse_destination_params(
                destination_chain,
                destination_token_address,
                destination_recipient_address,
            )?;

        let params = CustodianOnlyLockingParams {
            locking_amount: amount,
            custodian_pubkeys: Decoder::decode_33bytes_pubkey_list(custodial_pubkeys)?,
            custodian_quorum,
            destination_chain,
            destination_token_address,
            destination_recipient_address,
        };

        Self::handle_serialize_result(
            <VaultManager as CustodianOnly>::build_locking_output(&self.manager, &params),
            |output| Encoder::serialize_tx_outs(&output.into_tx_outs()),
        )
    }

    #[wasm_bindgen]
    pub fn custodian_only_locking_script(
        &self,
        custodian_pubkeys: &[u8],
        custodian_quorum: u8,
    ) -> Result<Vec<u8>, JsValue> {
        let custodian_pubkeys = Decoder::decode_33bytes_pubkey_list(custodian_pubkeys)?;

        let script =
            <VaultManager as CustodianOnly>::locking_script(&custodian_pubkeys, custodian_quorum)
                .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        Ok(script.into_script().to_bytes())
    }

    #[wasm_bindgen]
    pub fn upc_locking_script(
        &self,
        user_pubkey: &[u8],
        protocol_pubkey: &[u8],
        custodian_pubkeys: &[u8],
        custodian_quorum: u8,
    ) -> Result<Vec<u8>, JsValue> {
        let user_pubkey = Decoder::decode_33bytes_pubkey(user_pubkey)?;
        let protocol_pubkey = Decoder::decode_33bytes_pubkey(protocol_pubkey)?;
        let custodian_pubkeys = Decoder::decode_33bytes_pubkey_list(custodian_pubkeys)?;
        let script = <VaultManager as UPC>::locking_script(
            &user_pubkey,
            &protocol_pubkey,
            &custodian_pubkeys,
            custodian_quorum,
        )
        .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        Ok(script.into_script().to_bytes())
    }
}
