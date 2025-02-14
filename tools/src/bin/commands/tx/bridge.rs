use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use vault::{get_adress, get_approvable_utxos, TaprootTreeType};

use crate::{executors::BridgeExecutor, TvlMaker};

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

        let command_name = match &self.command {
            BridgeSubCommands::Upc(_) => self.name().to_owned() + "_upc",
            BridgeSubCommands::CustodianOnly(_) => self.name().to_owned() + "_custodian",
        };

        let utxo = get_approvable_utxos(
            &tvl_maker.suite.rpc,
            &get_adress(
                tvl_maker.suite.env().network.as_str(),
                &params.wallet_address,
            ),
            params.amount,
        )
        .map_err(|e| anyhow::anyhow!("Failed to get approvable utxos: {:?}", e))?;

        // Execute staking operation
        let command_history = BridgeExecutor::execute_bridge(
            &tvl_maker,
            command_name.as_str(),
            params,
            tree_type,
            utxo,
        )
        .map_err(|e| anyhow::anyhow!("Failed to execute bridge: {:?}", e))?;

        let id = tvl_maker
            .db_querier
            .save(&command_history)
            .map_err(|e| anyhow::anyhow!("Failed to create command history: {:?}", e))?;

        println!("Command history id: {}", id);

        Ok(())
    }
}
