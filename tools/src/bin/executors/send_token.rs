use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    network::{Ethereum, EthereumWallet, TransactionBuilder},
    primitives::{address, Address, Uint, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::request::TransactionRequest,
    signers::{
        k256::ecdsa::SigningKey,
        local::{LocalSigner, PrivateKeySigner},
    },
};

use bitcoin_vault::hex_to_vec;
use bitcoin_vault_tools::{IGateway::IGatewayInstance, IERC20::IERC20Instance};

use super::AlloyProvider;

pub struct SendTokenExecutor {
    pub user_signer: LocalSigner<SigningKey>,
    pub token_contract: IERC20Instance<(), AlloyProvider, Ethereum>,
    pub gateway_contract: IGatewayInstance<(), AlloyProvider, Ethereum>,
    pub provider: AlloyProvider,
}

impl SendTokenExecutor {
    pub fn new(
        private_key: &str,
        rpc_url: &str,
        token_address: Address,
        gateway_address: Address,
    ) -> Self {
        let private_key = hex_to_vec(&private_key);
        let user_signer = PrivateKeySigner::from_slice(&private_key).unwrap();
        let wallet = EthereumWallet::new(user_signer.clone());

        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .on_http(rpc_url.parse().unwrap());

        let token_contract = IERC20Instance::new(token_address, provider.clone());
        let gateway_contract = IGatewayInstance::new(gateway_address, provider.clone());

        Self {
            token_contract,
            gateway_contract,
            user_signer,
            provider,
        }
    }

    pub async fn send_token(&self, amount: u64) -> Result<(), String> {
        let balance = self
            .token_contract
            .balanceOf(self.user_signer.address())
            .call()
            .await
            .map_err(|e| e.to_string())?;

        let balance = balance._0;

        let amount = Uint::from(amount);

        if balance < amount {
            return Err("Insufficient balance".to_string());
        }

        // self.handle_token_approval(amount).await?;

        let path = std::env::current_dir().unwrap().join("abi/IERC20.json");

        let artifact = std::fs::read(path).expect("Failed to read artifact");
        let abi_value: serde_json::Value = serde_json::from_slice(&artifact).unwrap();

        let abi = serde_json::from_str(&abi_value.to_string()).unwrap();

        // Create a new `ContractInstance` of the `Counter` contract from the abi
        let contract = ContractInstance::new(
            *self.token_contract.address(),
            self.provider.clone(),
            Interface::new(abi),
        );

        let vitalik = address!("72d3Fa31e9FdD2f2Ce195Bdf9aBA8393a717fe01");
        let amount = U256::from(100);

        let tx_hash = contract
            .function(
                "approve",
                &[
                    DynSolValue::from(vitalik),
                    DynSolValue::from(U256::from(100)),
                ],
            )
            .unwrap()
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        println!("tx_hash: {:?}", tx_hash);

        Ok(())
    }

    pub async fn handle_token_approval(&self, amount: Uint<256, 4>) -> Result<(), String> {
        // Build an EIP-1559 type transaction to send 100 wei to Vitalik.
        let vitalik = address!("72d3Fa31e9FdD2f2Ce195Bdf9aBA8393a717fe01");

        let allowance = self
            .token_contract
            .allowance(self.user_signer.address(), vitalik)
            .call()
            .await
            .map_err(|e| e.to_string())?;

        println!("allowance: {:?}", allowance._0);

        // if allowance < amount {
        // Create approval transaction
        let approve_data = self
            .token_contract
            .approve(vitalik, amount)
            .call()
            .await
            .map_err(|e| e.to_string())?;

        let approve_data = approve_data._0;

        println!("approve_data: {:?}", approve_data);

        Ok(())
    }
}
