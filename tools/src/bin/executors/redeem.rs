use alloy::{
    dyn_abi::{abi, DynSolValue},
    primitives::Address,
};
use vault::TestSuite;

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
        suite: &TestSuite,
        destination_chain: String,
        amount: u64,
        locking_script: String,
    ) -> Result<Option<String>, String> {
        let amount = self.evm_executor.validate_token_amount(amount).await?;

        let approve_tx_hash = self.evm_executor.handle_token_approval(amount).await?;

        println!("approve tx_hash: {:?}", approve_tx_hash);

        println!("token_symbol: {:?}", self.evm_executor.token_symbol);

        //     { "name": "payload", "type": "bytes", "internalType": "bytes" },
        //   { "name": "symbol", "type": "string", "internalType": "string" },
        //   { "name": "amount", "type": "uint256", "internalType": "uint256" }

        //     lockingScript, _ := hex.DecodeString("001450dceca158a9c872eb405d52293d351110572c9e")
        // feeOptions := types.MinimumFee
        // rbf := true

        // let payload = abi::encode_params(
        //     "redeem",
        //     &[
        //         DynSolValue::from(destination_chain),
        //         DynSolValue::from(locking_script),
        //     ],
        // )?
        // .into();

        let tx_hash = self
            .evm_executor
            .gateway_contract
            .function(
                "callContractWithToken",
                &[
                    DynSolValue::from(destination_chain),
                    DynSolValue::from("0x0000000000000000000000000000000000000000".to_string()),
                    DynSolValue::from(vec![0x00]),
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
