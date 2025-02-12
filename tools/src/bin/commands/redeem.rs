use bitcoin_vault::TaprootTreeType;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::{
    commands::{CommandResult, CommandStatus},
    db::{CommandHistory, DbOperations},
    executors::BridgeExecutor,
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
    // Upc(RedeemCmdParams),
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

    /// Destination recipient address
    #[arg(short = 'r', long)]
    pub destination_recipient_address: String,

    /// Destination token address
    #[arg(short = 'u', long)]
    pub rpc_url: String,
}

impl TvlCommand for RedeemCommands {
    fn name(&self) -> String {
        "redeem".to_string()
    }

    fn execute(&self, tvl_maker: &TvlMaker) -> anyhow::Result<()> {
        // Extract params and tree type from command variant
        // let (params, tree_type) = match &self.command {
        //     // RedeemSubCommands::Upc(params) => (params, TaprootTreeType::UPCBranch),
        //     RedeemSubCommands::CustodianOnly(params) => (params, TaprootTreeType::CustodianOnly),
        // };

        // // Serialize params once for command history
        // let command_history_params = serde_json::to_string(params)?;

        // // Execute staking operation
        // let result = BridgeExecutor::execute_bridge(
        //     &tvl_maker.suite,
        //     tvl_maker.suite.env().network.to_string(),
        //     params,
        //     tree_type,
        // )
        // .map(|tx| {
        //     CommandResult::new(
        //         Some(tx.compute_txid().to_string()),
        //         CommandStatus::Success,
        //         None,
        //     )
        // })
        // .unwrap_or_else(|e| CommandResult::new(None, CommandStatus::Error, Some(e)));

        // // Create and store command history
        // let command_history = CommandHistory::new(
        //     match &self.command {
        //         // RedeemSubCommands::Upc(_) => self.name().to_owned() + "_upc",
        //         RedeemSubCommands::CustodianOnly(_) => self.name().to_owned() + "_custodian",
        //     },
        //     Some(self.suite_env_json(tvl_maker.suite.env())),
        //     Some(command_history_params),
        //     Some(serde_json::to_string(&result)?),
        // );

        // let id =
        //     <dyn DbOperations<CommandHistory>>::create(&tvl_maker.db_querier, &command_history)
        //         .map_err(|e| anyhow::anyhow!("Failed to create command history: {:?}", e))?;

        // // Ensure txid exists before unwrapping
        // match result.txid {
        //     Some(txid) => {
        //         println!("Command history id: {}", id);
        //         println!("Bridge transaction sent with txid: {}", txid);
        //         Ok(())
        //     }
        //     None => anyhow::bail!("Failed to get transaction ID from result"),
        // }
        Ok(())
    }
}
