use alloy::{dyn_abi::DynSolValue, primitives::Address};
use vault::{hex_to_vec, UnstakingTaprootTreeType};

use crate::executors::{calculate_contract_call_with_token_payload, ContractCallWithTokenPayload};

use super::EvmExecutor;

pub struct RedeemExecutor {
    evm_executor: EvmExecutor,
}

impl RedeemExecutor {
    pub fn new(
        private_key: &str,
        rpc_url: &str,
        token_address: Address,
        token_symbol: &str,
        gateway_address: Address,
    ) -> Self {
        let evm_executor = EvmExecutor::new(
            private_key,
            rpc_url,
            token_address,
            token_symbol,
            gateway_address,
        );

        Self { evm_executor }
    }

    pub async fn redeem_token(
        &self,
        destination_chain: String,
        amount: u64,
        locking_script: String,
    ) -> Result<Option<String>, String> {
        let amount = self.evm_executor.validate_token_amount(amount).await?;

        let approve_tx_hash = self.evm_executor.handle_token_approval(amount).await?;

        println!("approve tx_hash: {:?}", approve_tx_hash);

        println!("token_symbol: {:?}", self.evm_executor.token_symbol);

        let payload = calculate_contract_call_with_token_payload(ContractCallWithTokenPayload {
            payload_type: UnstakingTaprootTreeType::CustodianOnly,
            fee_options: 0,
            rbf: true,
            recipient_chain_identifier: Some(hex_to_vec(&locking_script)),
            psbt: None,
        })?;

        let tx_hash = self
            .evm_executor
            .gateway_contract
            .function(
                "callContractWithToken",
                &[
                    DynSolValue::from(destination_chain),
                    DynSolValue::from("0x0000000000000000000000000000000000000000".to_string()),
                    DynSolValue::from(payload),
                    DynSolValue::from(self.evm_executor.token_symbol.clone()),
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
