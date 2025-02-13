use alloy::primitives::Address;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::{
    commands::{CommandResult, CommandStatus},
    db::{CommandHistory, DbOperations},
    executors::RedeemExecutor,
    TvlMaker,
};

use super::TvlCommand;

#[derive(Parser, Debug)]
pub struct RedeemCommands {
    #[command(subcommand)]
    command: RedeemSubCommands,
}

#[derive(Subcommand, Debug)]
enum RedeemSubCommands {
    /// Run redeem bitcoin traction based on user-protocol-custodial model
    Upc(RedeemCmdParams),
    /// Run redeem bitcoin traction based on custodial model
    CustodianOnly(RedeemCmdParams),
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct RedeemCmdParams {
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

    /// BTC locking script
    #[arg(short = 'l', long)]
    pub locking_script: String,

    /// Destination token address
    #[arg(short = 'u', long)]
    pub rpc_url: String,
}

impl TvlCommand for RedeemCommands {
    fn name(&self) -> String {
        "redeem".to_string()
    }

    #[tokio::main]
    async fn execute(&self, tvl_maker: &TvlMaker) -> anyhow::Result<()> {
        let (params, command_name) = match &self.command {
            RedeemSubCommands::Upc(params) => {
                // TODO: UPC implementation will be added in the future
                println!("UPC redeem implementation pending");
                return Ok(());
            }
            RedeemSubCommands::CustodianOnly(params) => (params, "custodian_only"),
        };

        let executor = setup(
            &params.rpc_url,
            &params.private_key,
            &params.token_address,
            &params.token_symbol,
            &params.gateway_address,
        )?;

        // Execute staking operation
        let result = executor
            .redeem_token(
                params.destination_chain.clone(),
                params.amount,
                params.locking_script.clone(),
            )
            .await;

        let command_result = match result {
            Ok(Some(tx_hash)) => {
                println!("Token redeemed successfully: {}", tx_hash);
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

        let command_history_params = serde_json::to_string(&params)?;

        let command_history = CommandHistory::new(
            self.name().to_owned() + "_" + &command_name,
            Some(self.suite_env_json(tvl_maker.suite.env())),
            Some(command_history_params),
            Some(serde_json::to_string(&command_result)?),
        );

        let id =
            <dyn DbOperations<CommandHistory>>::create(&tvl_maker.db_querier, &command_history)
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
) -> anyhow::Result<RedeemExecutor> {
    let token_address: Address = token_address.parse()?;
    let gateway_address: Address = gateway_address.parse()?;

    Ok(RedeemExecutor::new(
        private_key,
        rpc_url,
        token_address,
        token_symbol,
        gateway_address,
    ))
}
