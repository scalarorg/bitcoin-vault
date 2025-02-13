use alloy::primitives::Address;
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{
    commands::types::{CommandResult, CommandStatus},
    db::CommandHistory,
    executors::SendTokenExecutor,
    TvlMaker,
};

use super::TvlCommand;

#[derive(Parser, Debug, Serialize, Deserialize)]
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

    /// Token symbol
    #[arg(short = 's', long)]
    pub token_symbol: String,

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
    async fn execute(&self, tvl_maker: &TvlMaker) -> anyhow::Result<()> {
        let executor = setup(
            &self.rpc_url,
            &self.private_key,
            &self.token_address,
            &self.token_symbol,
            &self.gateway_address,
        )?;

        let result = executor
            .send_token(
                self.destination_chain.clone(),
                self.destination_recipient_address.clone(),
                self.amount,
            )
            .await;

        let command_result = match result {
            Ok(Some(tx_hash)) => {
                println!("Token sent successfully: {}", tx_hash);
                CommandResult {
                    txid: Some(tx_hash),
                    status: CommandStatus::Success,
                    error: None,
                }
            }
            Ok(None) => CommandResult {
                txid: None,
                status: CommandStatus::Error,
                error: None,
            },
            Err(e) => CommandResult {
                txid: None,
                status: CommandStatus::Error,
                error: Some(e.to_string()),
            },
        };

        println!("Result: {:?}", command_result);

        let command_history_params = serde_json::to_string(&self)?;

        let command_history = CommandHistory::new(
            self.name(),
            Some(self.suite_env_json(tvl_maker.suite.env())),
            Some(command_history_params),
            Some(serde_json::to_string(&command_result)?),
        );

        let id = tvl_maker
            .db_querier
            .save(&command_history)
            .map_err(|e| anyhow::anyhow!("Failed to create command history: {:?}", e))?;

        println!("Command history created: {:?}", id);

        Ok(())
    }
}

fn setup(
    rpc_url: &str,
    private_key: &str,
    token_address: &str,
    token_symbol: &str,
    gateway_address: &str,
) -> anyhow::Result<SendTokenExecutor> {
    let token_address: Address = token_address.parse()?;
    let gateway_address: Address = gateway_address.parse()?;

    Ok(SendTokenExecutor::new(
        private_key,
        rpc_url,
        token_address,
        token_symbol,
        gateway_address,
    ))
}
