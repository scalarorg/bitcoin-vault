use std::convert::{TryFrom, TryInto};

use crate::errors::VaultABIError;
use crate::{decoder::Decoder, encoder::Encoder};
use bitcoin::{Amount, NetworkKind, OutPoint, PublicKey, XOnlyPublicKey};
use bitcoin_vault::{
    BuildStakingParams, BuildStakingWithOnlyCovenantsParams, BuildUnstakingParams,
    BuildUnstakingWithOnlyCovenantsParams, DestinationChain, DestinationContractAddress,
    DestinationRecipientAddress, LockingScriptWithOnlyCovenantsParams, PreviousStakingUTXO,
    Signing, Staking, Unstaking, UnstakingOutput as VaultUnstakingOutput, UnstakingType,
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

impl TryFrom<UnstakingOutput> for VaultUnstakingOutput {
    type Error = VaultABIError;

    fn try_from(output: UnstakingOutput) -> Result<Self, Self::Error> {
        Ok(VaultUnstakingOutput {
            amount_in_sats: Amount::from_sat(output.amount),
            locking_script: Decoder::decode_script_pubkey(&output.script_pubkey),
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

impl VaultWasm {
    fn parse_pubkeys(
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        covenant_pubkeys: &[u8],
    ) -> Result<(PublicKey, PublicKey, Vec<PublicKey>), VaultABIError> {
        Ok((
            Decoder::decode_33bytes_pubkey(staker_pubkey)?,
            Decoder::decode_33bytes_pubkey(protocol_pubkey)?,
            Decoder::decode_33bytes_pubkey_list(covenant_pubkeys)?,
        ))
    }

    fn convert_error<T, E: std::fmt::Debug>(result: Result<T, E>) -> Result<T, JsValue> {
        result.map_err(|e| JsValue::from(format!("{:?}", e)))
    }

    fn parse_destination_params(
        &self,
        destination_chain: &[u8],
        destination_smartcontract_address: &[u8],
        destination_recipient_address: &[u8],
    ) -> Result<
        (
            DestinationChain,
            DestinationContractAddress,
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
        let (destination_chain, destination_contract_address, destination_recipient_address) = self
            .parse_destination_params(
                destination_chain,
                destination_smartcontract_address,
                destination_recipient_address,
            )?;

        let (user_pub_key, protocol_pub_key, covenant_pub_keys) =
            Self::parse_pubkeys(staker_pubkey, protocol_pubkey, custodial_pubkeys)?;

        let params = BuildStakingParams {
            staking_amount,
            user_pub_key,
            protocol_pub_key,
            covenant_pub_keys,
            covenant_quorum,
            have_only_covenants,
            destination_chain,
            destination_contract_address,
            destination_recipient_address,
        };

        Self::handle_serialize_result(
            <VaultManager as Staking>::build(&self.manager, &params),
            |output| Encoder::serialize_tx_outs(&output.into_tx_outs()),
        )
    }

    #[wasm_bindgen]
    pub fn build_user_protocol_spend(
        &self,
        input: UnstakingInput,
        locking_script: &[u8],
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        fee_rate: u64,
        rbf: bool,
    ) -> Result<Vec<u8>, JsValue> {
        self.build_unstaking(
            input,
            locking_script,
            staker_pubkey,
            protocol_pubkey,
            covenant_pubkeys,
            covenant_quorum,
            have_only_covenants,
            fee_rate,
            rbf,
            UnstakingType::UserProtocol,
        )
    }

    #[wasm_bindgen]
    pub fn build_covenants_protocol_spend(
        &self,
        input: UnstakingInput,
        locking_script: &[u8],
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        fee_rate: u64,
        rbf: bool,
    ) -> Result<Vec<u8>, JsValue> {
        // ### Description
        // ### Reversed txid is used to match the byte order of the txid in the previous staking UTXO.
        // ### References: https://learnmeabitcoin.com/technical/general/byte-order/#natural-byte-order
        // input_txid.reverse();

        self.build_unstaking(
            input,
            locking_script,
            staker_pubkey,
            protocol_pubkey,
            covenant_pubkeys,
            covenant_quorum,
            have_only_covenants,
            fee_rate,
            rbf,
            UnstakingType::CovenantsProtocol,
        )
    }

    #[wasm_bindgen]
    pub fn build_covenants_user_spend(
        &self,
        input: UnstakingInput,
        locking_script: &[u8],
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        fee_rate: u64,
        rbf: bool,
    ) -> Result<Vec<u8>, JsValue> {
        self.build_unstaking(
            input,
            locking_script,
            staker_pubkey,
            protocol_pubkey,
            covenant_pubkeys,
            covenant_quorum,
            have_only_covenants,
            fee_rate,
            rbf,
            UnstakingType::CovenantsUser,
        )
    }

    fn build_unstaking(
        &self,
        input: UnstakingInput,
        locking_script: &[u8],
        staker_pubkey: &[u8],
        protocol_pubkey: &[u8],
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        have_only_covenants: bool,
        fee_rate: u64,
        rbf: bool,
        unstaking_type: UnstakingType,
    ) -> Result<Vec<u8>, JsValue> {
        // ### Description
        // ### Reversed txid is used to match the byte order of the txid in the previous staking UTXO.
        // ### References: https://learnmeabitcoin.com/technical/general/byte-order/#natural-byte-order
        let (user_pub_key, protocol_pub_key, covenant_pub_keys) =
            Self::parse_pubkeys(staker_pubkey, protocol_pubkey, covenant_pubkeys)?;

        let params = BuildUnstakingParams {
            input: input.try_into()?,
            locking_script: Decoder::decode_script_pubkey(locking_script),
            user_pub_key,
            protocol_pub_key,
            covenant_pub_keys,
            covenant_quorum,
            have_only_covenants,
            rbf,
            fee_rate,
        };

        Self::handle_serialize_result(
            <VaultManager as Unstaking>::build(&self.manager, &params, unstaking_type),
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
        let (destination_chain, destination_contract_address, destination_recipient_address) = self
            .parse_destination_params(
                destination_chain,
                destination_smartcontract_address,
                destination_recipient_address,
            )?;

        let params = BuildStakingWithOnlyCovenantsParams {
            staking_amount,
            covenant_pub_keys: Decoder::decode_33bytes_pubkey_list(custodial_pubkeys)?,
            covenant_quorum,
            destination_chain,
            destination_contract_address,
            destination_recipient_address,
        };

        Self::handle_serialize_result(
            <VaultManager as Staking>::build_with_only_covenants(&self.manager, &params),
            |output| Encoder::serialize_tx_outs(&output.into_tx_outs()),
        )
    }

    #[wasm_bindgen]
    pub fn build_unstaking_with_only_covenants(
        &self,
        inputs: Vec<UnstakingInput>,
        output: UnstakingOutput,
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
        fee_rate: u64,
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
            fee_rate,
            rbf,
        };

        Self::handle_serialize_result(
            <VaultManager as Unstaking>::build_with_only_covenants(&self.manager, &params),
            |psbt| psbt.serialize(),
        )
    }

    #[wasm_bindgen]
    pub fn only_covenants_locking_script(
        &self,
        covenant_pubkeys: &[u8],
        covenant_quorum: u8,
    ) -> Result<Vec<u8>, JsValue> {
        let covenant_pub_keys = Decoder::decode_33bytes_pubkey_list(covenant_pubkeys)?;
        let covenant_x_only_pubkeys: Vec<XOnlyPublicKey> = covenant_pub_keys
            .iter()
            .map(|p| XOnlyPublicKey::from(*p))
            .collect::<Vec<_>>();
        let script = self
            .manager
            .only_covenants_locking_script(&LockingScriptWithOnlyCovenantsParams {
                covenant_pub_keys: &covenant_x_only_pubkeys,
                covenant_quorum,
            })
            .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        Ok(script.into_script().to_bytes())
    }
}
