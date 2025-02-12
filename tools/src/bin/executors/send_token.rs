use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    network::EthereumWallet,
    primitives::{Address, Uint},
    providers::ProviderBuilder,
    signers::{
        k256::ecdsa::SigningKey,
        local::{LocalSigner, PrivateKeySigner},
    },
};

use bitcoin_vault::hex_to_vec;
use bitcoin_vault_tools::{IERC20_ABI, IGATEWAY_ABI};

use super::AlloyContract;

pub struct SendTokenExecutor {
    pub user_signer: LocalSigner<SigningKey>,
    pub token_contract: AlloyContract,
    pub gateway_contract: AlloyContract,
    pub token_symbol: String,
}

impl SendTokenExecutor {
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

    pub async fn send_token(
        &self,
        destination_chain: String,
        destination_recipient_address: String,
        amount: u64,
    ) -> Result<Option<String>, String> {
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

        let approve_tx_hash = self.handle_token_approval(amount).await?;

        println!("approve tx_hash: {:?}", approve_tx_hash);

        println!("token_symbol: {:?}", self.token_symbol);

        let tx_hash = self
            .gateway_contract
            .function(
                "sendToken",
                &[
                    DynSolValue::from(destination_chain),
                    DynSolValue::from(destination_recipient_address),
                    DynSolValue::from(self.token_symbol.clone()),
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
