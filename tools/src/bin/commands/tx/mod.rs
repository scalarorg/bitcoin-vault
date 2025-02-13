use clap::{Parser, Subcommand};

use crate::{TvlCommand, TvlMaker};

mod bridge;
mod redeem;
mod send_token;

pub use bridge::*;
pub use redeem::*;
pub use send_token::*;

#[derive(Parser, Debug)]
pub struct TxCommands {
    /// Test environment to use (regtest, testnet4 or custom to the env file)
    #[arg(short, long, default_value = ".env")]
    pub test_env: String,
    /// Service tag
    #[arg(short = 'x', long)]
    pub service_tag: String,

    #[command(subcommand)]
    command: SubTxCommands,
}

#[derive(Subcommand, Debug)]
enum SubTxCommands {
    /// Staking related commands
    Bridge(BridgeCommands),
    /// Send token related commands
    SendToken(SendTokenCommand),
    /// Redeem related commands
    Redeem(RedeemCommands),
    // /// Monitoring and status commands
    // Monitor(MonitorCommands),
}

impl TvlCommand for TxCommands {
    fn name(&self) -> String {
        "tx".to_string()
    }

    fn execute(&self, tvl_maker: &TvlMaker) -> anyhow::Result<()> {
        match &self.command {
            SubTxCommands::Bridge(bridge_cmd) => bridge_cmd.execute(&tvl_maker),
            SubTxCommands::SendToken(send_token_cmd) => send_token_cmd.execute(&tvl_maker),
            SubTxCommands::Redeem(redeem_cmd) => redeem_cmd.execute(&tvl_maker),
        }
    }
}
