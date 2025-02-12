use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::{abi::Token, DynSolValue, DynToken},
    network::EthereumWallet,
    primitives::{Address, Uint, U8},
    providers::ProviderBuilder,
    signers::{
        k256::ecdsa::SigningKey,
        local::{LocalSigner, PrivateKeySigner},
    },
    sol,
};

mod bridge;
mod redeem;
mod send_token;
mod types;

pub use bridge::*;
pub use redeem::*;
pub use send_token::*;
use tools::{IERC20_ABI, IGATEWAY_ABI};
pub use types::*;
use vault::{hex_to_vec, UnstakingTaprootTreeType};

pub struct EvmExecutor {
    pub user_signer: LocalSigner<SigningKey>,
    pub token_contract: AlloyContract,
    pub gateway_contract: AlloyContract,
    pub token_symbol: String,
}

impl EvmExecutor {
    pub fn new(
        private_key: &str,
        rpc_url: &str,
        token_address: Address,
        token_symbol: &str,
        gateway_address: Address,
    ) -> Self {
        let private_key = hex_to_vec(&private_key);
        let user_signer = PrivateKeySigner::from_slice(&private_key).unwrap();
        let wallet = EthereumWallet::new(user_signer.clone());

        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .on_http(rpc_url.parse().unwrap());

        let token_contract = ContractInstance::new(
            token_address,
            provider.clone(),
            Interface::new(IERC20_ABI.clone()),
        );
        let gateway_contract = ContractInstance::new(
            gateway_address,
            provider.clone(),
            Interface::new(IGATEWAY_ABI.clone()),
        );

        Self {
            user_signer,
            token_contract,
            gateway_contract,
            token_symbol: token_symbol.to_string(),
        }
    }

    pub async fn validate_token_amount(&self, amount: u64) -> Result<Uint<256, 4>, String> {
        let balance = self
            .token_contract
            .function(
                "balanceOf",
                &[DynSolValue::from(self.user_signer.address())],
            )
            .unwrap()
            .call()
            .await
            .map_err(|e| e.to_string())?;

        let balance = balance[0].as_uint().unwrap();

        let balance = balance.0;

        let amount = Uint::from(amount);

        if balance < amount {
            return Err("Insufficient balance".to_string());
        }

        Ok(amount)
    }

    pub async fn handle_token_approval(
        &self,
        amount: Uint<256, 4>,
    ) -> Result<Option<String>, String> {
        let allowance = self
            .token_contract
            .function(
                "allowance",
                &[
                    DynSolValue::from(self.user_signer.address()),
                    DynSolValue::from(*self.gateway_contract.address()),
                ],
            )
            .unwrap()
            .call()
            .await
            .unwrap();

        let allowance = allowance[0].as_uint().unwrap();

        let allowance = allowance.0;

        println!("allowance: {:?}", allowance);

        if allowance >= amount {
            return Ok(None);
        }

        let tx_hash = self
            .token_contract
            .function(
                "approve",
                &[
                    DynSolValue::from(*self.gateway_contract.address()),
                    DynSolValue::from(amount),
                ],
            )
            .unwrap()
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        Ok(Some(tx_hash.to_string()))
    }
}

// contractCallWithTokenCustodianOnly = abi.Arguments{{Type: uint8Type}, {Type: boolType}, {Type: bytesType}}

#[derive(Debug)]
pub struct ContractCallWithTokenPayload {
    payload_type: UnstakingTaprootTreeType,
    fee_options: u8,
    rbf: bool,
    recipient_chain_identifier: Option<Vec<u8>>,
    psbt: Option<Vec<u8>>,
}

pub fn calculate_contract_call_with_token_payload(
    payload_args: ContractCallWithTokenPayload,
) -> Result<Vec<u8>, String> {
    let encoded_payload = match payload_args.payload_type {
        UnstakingTaprootTreeType::CustodianOnly => encode_custodian_only_payload(
            payload_args.fee_options,
            payload_args.rbf,
            payload_args
                .recipient_chain_identifier
                .ok_or_else(|| "recipient chain identifier is required".to_string())?,
        )?,
        UnstakingTaprootTreeType::UPCBranch => {
            let psbt = payload_args
                .psbt
                .ok_or_else(|| "psbt is required".to_string())?;

            encode_upc_payload(psbt)?
        }
    };

    let final_payload = append_payload(&payload_args.payload_type, &encoded_payload);

    Ok(final_payload)
}

fn encode_custodian_only_payload(
    fee_options: u8,
    rbf: bool,
    recipient_chain_id: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let encoded_payload = DynSolValue::Tuple(vec![
        DynSolValue::Uint(Uint::from(fee_options), 8),
        DynSolValue::Bool(rbf),
        DynSolValue::Bytes(recipient_chain_id),
    ]);

    Ok(encoded_payload.abi_encode())
}

fn encode_upc_payload(psbt: Vec<u8>) -> Result<Vec<u8>, String> {
    let encoded_payload = DynSolValue::Bytes(psbt);

    Ok(encoded_payload.abi_encode())
}

fn append_payload(payload_type: &UnstakingTaprootTreeType, encoded_payload: &[u8]) -> Vec<u8> {
    let type_byte = match payload_type {
        UnstakingTaprootTreeType::CustodianOnly => 0u8,
        UnstakingTaprootTreeType::UPCBranch => 1u8,
    };

    let mut final_payload = vec![type_byte];
    final_payload.extend_from_slice(encoded_payload);
    final_payload
}

#[cfg(test)]
mod tests {
    use bitcoin::hex::DisplayHex;

    use super::*;

    #[test]
    fn test_encode_custodian_only_payload() {
        let locking_script = hex_to_vec("001450dceca158a9c872eb405d52293d351110572c9e");
        let payload = ContractCallWithTokenPayload {
            payload_type: UnstakingTaprootTreeType::CustodianOnly,
            fee_options: 0,
            rbf: true,
            recipient_chain_identifier: Some(locking_script),
            psbt: None,
        };

        let encoded_payload = calculate_contract_call_with_token_payload(payload).unwrap();
        println!(
            "encoded_payload: {:?}",
            encoded_payload.to_lower_hex_string()
        );
    }
}
