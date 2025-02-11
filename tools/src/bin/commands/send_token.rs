use alloy::{
    network::EthereumWallet, primitives::Address, providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use bitcoin_vault::hex_to_vec;
use clap::Parser;

use crate::{executors::SendTokenExecutor, TvlMaker};

use super::TvlCommand;

#[derive(Parser, Debug)]
pub struct SendTokenCommand {
    /// Amount of token to send
    #[arg(short = 'a', long)]
    pub amount: u64,

    /// Wallet private key
    #[arg(short = 'k', long)]
    pub private_key: String,

    /// Token address
    #[arg(short = 't', long)]
    pub token_address: String,

    /// Gateway address
    #[arg(short = 'g', long)]
    pub gateway_address: String,

    /// Destination chain
    #[arg(short = 'c', long)]
    pub destination_chain: String,

    /// Destination recipient address
    #[arg(short = 'r', long)]
    pub destination_recipient_address: String,

    /// Destination token address
    #[arg(short = 'u', long)]
    pub rpc_url: String,
}

impl TvlCommand for SendTokenCommand {
    fn name(&self) -> String {
        "send_token".to_string()
    }

    #[tokio::main]
    async fn execute(&self, _tvl_maker: Option<&TvlMaker>) -> anyhow::Result<()> {
        let executor = setup(
            &self.rpc_url,
            &self.private_key,
            &self.token_address,
            &self.gateway_address,
        )?;
        let result = executor.send_token(self.amount).await;
        match result {
            Ok(_) => println!("Token sent successfully"),
            Err(e) => println!("Error sending token: {}", e),
        }
        Ok(())
    }
}

fn setup(
    rpc_url: &str,
    private_key: &str,
    token_address: &str,
    gateway_address: &str,
) -> anyhow::Result<SendTokenExecutor> {
    let token_address: Address = token_address.parse()?;
    let gateway_address: Address = gateway_address.parse()?;

    Ok(SendTokenExecutor::new(
        private_key,
        rpc_url,
        token_address,
        gateway_address,
    ))
}
