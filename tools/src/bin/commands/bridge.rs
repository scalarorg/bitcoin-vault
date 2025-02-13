use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use vault::TaprootTreeType;

use crate::{
    commands::{CommandResult, CommandStatus},
    db::{CommandHistory, DbOperations},
    executors::BridgeExecutor,
    TvlMaker,
};

use super::TvlCommand;

#[derive(Parser, Debug)]
pub struct BridgeCommands {
    #[command(subcommand)]
    command: BridgeSubCommands,
}

#[derive(Subcommand, Debug)]
enum BridgeSubCommands {
    /// Run staking bitcoin traction based on user-protocol-custodial model
    Upc(BridgeCmdParams),
    /// Run staking bitcoin traction based on custodial model
    CustodianOnly(BridgeCmdParams),
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct BridgeCmdParams {
    /// Amount of BTC to stake
    #[arg(short = 'a', long)]
    pub amount: u64,

    /// Wallet address
    #[arg(short = 'w', long)]
    pub wallet_address: String,

    /// Wallet private key
    #[arg(short = 'k', long)]
    pub private_key: String,

    /// Destination chain
    #[arg(short = 'c', long)]
    pub destination_chain: String,

    /// Destination token address
    #[arg(short = 't', long)]
    pub destination_token_address: String,

    /// Destination recipient address
    #[arg(short = 'r', long)]
    pub destination_recipient_address: String,
}

impl TvlCommand for BridgeCommands {
    fn name(&self) -> String {
        "bridge".to_string()
    }

    fn execute(&self, tvl_maker: &TvlMaker) -> anyhow::Result<()> {
        // Extract params and tree type from command variant
        let (params, tree_type) = match &self.command {
            BridgeSubCommands::Upc(params) => (params, TaprootTreeType::UPCBranch),
            BridgeSubCommands::CustodianOnly(params) => (params, TaprootTreeType::CustodianOnly),
        };

        // Serialize params once for command history
        let command_history_params = serde_json::to_string(params)?;

        // Execute staking operation
        let result = BridgeExecutor::execute_bridge(
            &tvl_maker.suite,
            tvl_maker.suite.env().network.to_string(),
            params,
            tree_type,
        )
        .map(|tx| {
            CommandResult::new(
                Some(tx.compute_txid().to_string()),
                CommandStatus::Success,
                None,
            )
        })
        .unwrap_or_else(|e| CommandResult::new(None, CommandStatus::Error, Some(e)));

        // Create and store command history
        let command_history = CommandHistory::new(
            match &self.command {
                BridgeSubCommands::Upc(_) => self.name().to_owned() + "_upc",
                BridgeSubCommands::CustodianOnly(_) => self.name().to_owned() + "_custodian",
            },
            Some(self.suite_env_json(tvl_maker.suite.env())),
            Some(command_history_params),
            Some(serde_json::to_string(&result)?),
        );

        let id = <dyn DbOperations<CommandHistory>>::create(tvl_maker.db_querier, &command_history)
            .map_err(|e| anyhow::anyhow!("Failed to create command history: {:?}", e))?;

        // Ensure txid exists before unwrapping
        match result.txid {
            Some(txid) => {
                println!("Command history id: {}", id);
                println!("Bridge transaction sent with txid: {}", txid);
                Ok(())
            }
            None => anyhow::bail!("Failed to get transaction ID from result"),
        }
    }
}
