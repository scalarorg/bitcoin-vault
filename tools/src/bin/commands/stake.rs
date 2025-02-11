use bitcoin_vault::TaprootTreeType;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::{
    commands::{CommandResult, CommandStatus},
    db::{CommandHistory, DbOperations},
    executors::StakingExecutor,
    TvlMaker,
};

use super::TvlCommand;

#[derive(Parser, Debug)]
pub struct StakeCommands {
    #[command(subcommand)]
    command: StakeSubCommands,
}

#[derive(Subcommand, Debug)]
enum StakeSubCommands {
    /// Run staking bitcoin traction based on user-protocol-custodial model
    Upc(StakingCmdParams),
    /// Run staking bitcoin traction based on custodial model
    CustodianOnly(StakingCmdParams),
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct StakingCmdParams {
    /// Amount of BTC to stake
    #[arg(short, long)]
    pub amount: u64,

    /// Wallet address
    #[arg(short, long)]
    pub address: String,

    /// Wallet private key
    #[arg(short, long)]
    pub private_key: String,
}

impl TvlCommand for StakeCommands {
    fn name(&self) -> String {
        "stake".to_string()
    }

    fn execute(&self, tvl_maker: &TvlMaker) -> anyhow::Result<()> {
        // Extract params and tree type from command variant
        let (params, tree_type) = match &self.command {
            StakeSubCommands::Upc(params) => (params, TaprootTreeType::UPCBranch),
            StakeSubCommands::CustodianOnly(params) => (params, TaprootTreeType::CustodianOnly),
        };

        // Serialize params once for command history
        let command_history_params = serde_json::to_string(params)?;

        // Execute staking operation
        let result = StakingExecutor::execute_staking(
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
                StakeSubCommands::Upc(_) => self.name().to_owned() + "_upc",
                StakeSubCommands::CustodianOnly(_) => self.name().to_owned() + "_custodian",
            },
            Some(self.suite_env_json(&tvl_maker.suite.env())),
            Some(command_history_params),
            Some(serde_json::to_string(&result)?),
        );

        let id =
            <dyn DbOperations<CommandHistory>>::create(&tvl_maker.db_querier, &command_history)
                .map_err(|e| anyhow::anyhow!("Failed to create command history: {:?}", e))?;

        // Ensure txid exists before unwrapping
        match result.txid {
            Some(txid) => {
                println!("Command history id: {}", id);
                println!("Staking transaction sent with txid: {}", txid);
                Ok(())
            }
            None => anyhow::bail!("Failed to get transaction ID from result"),
        }
    }
}
