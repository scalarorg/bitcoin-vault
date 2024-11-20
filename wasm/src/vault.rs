use std::convert::{TryFrom, TryInto};

use crate::errors::VaultABIError;
use crate::{decoder::Decoder, encoder::Encoder};
use bitcoin::{Amount, NetworkKind, OutPoint, TxOut};
use bitcoin_vault::{
    BuildStakingParams, BuildStakingWithOnlyCovenantsParams, BuildUnstakingParams,
    BuildUnstakingWithOnlyCovenantsParams, DestinationChain, DestinationContractAddress,
    DestinationRecipientAddress, PreviousStakingUTXO, Signing, Staking, Unstaking, UnstakingType,
    VaultManager,
};
use wasm_bindgen::prelude::*;
impl From<VaultABIError> for JsValue {
    fn from(err: VaultABIError) -> Self {
        JsValue::from(err.to_string())
    }
}

#[wasm_bindgen]
pub struct UnstakingOutput {
    script_pubkey: Vec<u8>,
    amount: u64,
}

#[wasm_bindgen]
impl UnstakingOutput {
    #[wasm_bindgen(constructor)]
    pub fn new(script_pubkey: Vec<u8>, amount: u64) -> Self {
        Self {
            script_pubkey,
            amount,
        }
    }
}

impl TryFrom<UnstakingOutput> for TxOut {
    type Error = VaultABIError;

    fn try_from(output: UnstakingOutput) -> Result<Self, Self::Error> {
        Ok(TxOut {
            value: Amount::from_sat(output.amount),
            script_pubkey: Decoder::decode_script_pubkey(&output.script_pubkey),
        })
    }
}

#[wasm_bindgen]
pub struct UnstakingInput {
    script_pubkey: Vec<u8>,
    txid: Vec<u8>,
    vout: u32,
    amount: u64,
}

#[wasm_bindgen]
impl UnstakingInput {
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

impl TryFrom<UnstakingInput> for PreviousStakingUTXO {
    type Error = VaultABIError;

    fn try_from(input: UnstakingInput) -> Result<Self, Self::Error> {
        // ### Description
        // ### Reversed txid is used to match the byte order of the txid in the previous staking UTXO.
        // ### References: https://learnmeabitcoin.com/technical/general/byte-order/#natural-byte-order
        let mut txid_bytes = input.txid.clone();
        txid_bytes.reverse();

        let txid = Decoder::decode_txid(&txid_bytes)?;
        let script_pubkey = Decoder::decode_script_pubkey(&input.script_pubkey);

        Ok(PreviousStakingUTXO {
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
pub struct VaultWasm {
    manager: VaultManager,
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
        destination_chain: &[u8],
        destination_smartcontract_address: &[u8],
        destination_recipient_address: &[u8],
    ) -> Result<Vec<u8>, JsValue> {
        let destination_contract_address: DestinationContractAddress =
            destination_smartcontract_address
                .try_into()
                .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        // console::debug_1(&"Wasm#parsed destination_contract_address".into());
        let destination_recipient_address: DestinationRecipientAddress =
            destination_recipient_address
                .try_into()
                .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        let destination_chain: DestinationChain = destination_chain
            .try_into()
            .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        let params = BuildStakingParams {
            staking_amount,
            user_pub_key: Decoder::decode_33bytes_pubkey(staker_pubkey)?,
            protocol_pub_key: Decoder::decode_33bytes_pubkey(protocol_pubkey)?,
            covenant_pub_keys: Decoder::decode_33bytes_pubkey_list(custodial_pubkeys)?,
            covenant_quorum,
            have_only_covenants,
            destination_chain,
            destination_contract_address,
            destination_recipient_address,
        };

        match <VaultManager as Staking>::build(&self.manager, &params) {
            Ok(staking_output) => Ok(Encoder::serialize_tx_outs(&staking_output.into_tx_outs())),
            Err(_) => Ok(vec![]),
        }
    }

    #[wasm_bindgen]
    pub fn build_user_protocol_spend(
        &self,
        inputs: Vec<UnstakingInput>,
        output: UnstakingOutput,
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        rbf: bool,
    ) -> Result<Vec<u8>, JsValue> {
        self.build_unstaking(
            inputs,
            output,
            staker_pubkey,
            protocol_pubkey,
            covenant_pubkeys,
            covenant_quorum,
            have_only_covenants,
            rbf,
            UnstakingType::UserProtocol,
        )
    }

    #[wasm_bindgen]
    pub fn build_covenants_protocol_spend(
        &self,
        inputs: Vec<UnstakingInput>,
        output: UnstakingOutput,
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        rbf: bool,
    ) -> Result<Vec<u8>, JsValue> {
        // ### Description
        // ### Reversed txid is used to match the byte order of the txid in the previous staking UTXO.
        // ### References: https://learnmeabitcoin.com/technical/general/byte-order/#natural-byte-order
        // input_txid.reverse();

        self.build_unstaking(
            inputs,
            output,
            staker_pubkey,
            protocol_pubkey,
            covenant_pubkeys,
            covenant_quorum,
            have_only_covenants,
            rbf,
            UnstakingType::CovenantsProtocol,
        )
    }

    #[wasm_bindgen]
    pub fn build_covenants_user_spend(
        &self,
        inputs: Vec<UnstakingInput>,
        output: UnstakingOutput,
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        rbf: bool,
    ) -> Result<Vec<u8>, JsValue> {
        self.build_unstaking(
            inputs,
            output,
            staker_pubkey,
            protocol_pubkey,
            covenant_pubkeys,
            covenant_quorum,
            have_only_covenants,
            rbf,
            UnstakingType::CovenantsUser,
        )
    }

    // #[wasm_bindgen]
    // pub fn build_only_covenants_spend(
    //     &self,
    //     inputs: Vec<UnstakingInput>,
    //     output: UnstakingOutput,
    //     staker_pubkey: &[u8],
    //     protocol_pubkey: &[u8],
    //     covenant_pubkeys: &[u8],
    //     covenant_quorum: u8,
    //     have_only_covenants: bool,
    //     rbf: bool,
    // ) -> Result<Vec<u8>, JsValue> {
    //     self.build_unstaking(
    //         inputs,
    //         output,
    //         staker_pubkey,
    //         protocol_pubkey,
    //         covenant_pubkeys,
    //         covenant_quorum,
    //         have_only_covenants,
    //         rbf,
    //         UnstakingType::OnlyCovenants,
    //     )
    // }

    fn build_unstaking(
        &self,
        inputs: Vec<UnstakingInput>,
        output: UnstakingOutput,
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        rbf: bool,
        unstaking_type: UnstakingType,
    ) -> Result<Vec<u8>, JsValue> {
        // ### Description
        // ### Reversed txid is used to match the byte order of the txid in the previous staking UTXO.
        // ### References: https://learnmeabitcoin.com/technical/general/byte-order/#natural-byte-order
        let user_pub_key = Decoder::decode_33bytes_pubkey(staker_pubkey)?;
        let protocol_pub_key = Decoder::decode_33bytes_pubkey(protocol_pubkey)?;
        let covenant_pub_keys = Decoder::decode_33bytes_pubkey_list(covenant_pubkeys)?;

        let params = BuildUnstakingParams {
            inputs: inputs
                .into_iter()
                .map(|input| input.try_into())
                .collect::<Result<Vec<PreviousStakingUTXO>, VaultABIError>>()?,
            unstaking_output: output.try_into()?,
            user_pub_key,
            protocol_pub_key,
            covenant_pub_keys,
            covenant_quorum,
            have_only_covenants,
            rbf,
        };

        match <VaultManager as Unstaking>::build(&self.manager, &params, unstaking_type) {
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
            VaultManager::sign_psbt_by_single_key(&mut psbt, privkey, network_kind, finalize)
                .map_err(|e| VaultABIError::DecodingError(format!("{}", e)))?;
        Ok(signed_psbt)
    }
}

#[wasm_bindgen]
impl VaultWasm {
    #[wasm_bindgen]
    pub fn build_staking_output_with_only_covenants(
        &self,
        staking_amount: u64,
        //33 bytes pubkey
        custodial_pubkeys: &[u8],
        covenant_quorum: u8,
        destination_chain: &[u8],
        destination_smartcontract_address: &[u8],
        destination_recipient_address: &[u8],
    ) -> Result<Vec<u8>, JsValue> {
        let destination_contract_address: DestinationContractAddress =
            destination_smartcontract_address
                .try_into()
                .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        let destination_recipient_address: DestinationRecipientAddress =
            destination_recipient_address
                .try_into()
                .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        let destination_chain: DestinationChain = destination_chain
            .try_into()
            .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        let params = BuildStakingWithOnlyCovenantsParams {
            staking_amount,
            covenant_pub_keys: Decoder::decode_33bytes_pubkey_list(custodial_pubkeys)?,
            covenant_quorum,
            destination_chain,
            destination_contract_address,
            destination_recipient_address,
        };

        match <VaultManager as Staking>::build_with_only_covenants(&self.manager, &params) {
            Ok(staking_output) => Ok(Encoder::serialize_tx_outs(&staking_output.into_tx_outs())),
            Err(_) => Ok(vec![]),
        }
    }

    #[wasm_bindgen]
    pub fn build_unstaking_with_only_covenants(
        &self,
        inputs: Vec<UnstakingInput>,
        output: UnstakingOutput,
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        rbf: bool,
    ) -> Result<Vec<u8>, JsValue> {
        let covenant_pub_keys = Decoder::decode_33bytes_pubkey_list(covenant_pubkeys)?;

        let params = BuildUnstakingWithOnlyCovenantsParams {
            inputs: inputs
                .into_iter()
                .map(|input| input.try_into())
                .collect::<Result<Vec<PreviousStakingUTXO>, VaultABIError>>()?,
            unstaking_output: output.try_into()?,
            covenant_pub_keys,
            covenant_quorum,
            rbf,
        };

        match <VaultManager as Unstaking>::build_with_only_covenants(&self.manager, &params) {
            Ok(psbt) => Ok(psbt.serialize()),
            Err(_) => Ok(vec![]),
        }
    }
}
