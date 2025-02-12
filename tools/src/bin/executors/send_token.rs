use alloy::{dyn_abi::DynSolValue, primitives::Address};

use super::EvmExecutor;

pub struct SendTokenExecutor {
    evm_executor: EvmExecutor,
}

impl SendTokenExecutor {
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

    pub async fn send_token(
        &self,
        destination_chain: String,
        destination_recipient_address: String,
        amount: u64,
    ) -> Result<Option<String>, String> {
        let amount = self.evm_executor.validate_token_amount(amount).await?;

        let approve_tx_hash = self.evm_executor.handle_token_approval(amount).await?;

        println!("approve tx_hash: {:?}", approve_tx_hash);

        println!("token_symbol: {:?}", self.evm_executor.token_symbol);

        let tx_hash = self
            .evm_executor
            .gateway_contract
            .function(
                "sendToken",
                &[
                    DynSolValue::from(destination_chain),
                    DynSolValue::from(destination_recipient_address),
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
