use clap::{Parser, Subcommand};

use super::TvlCommand;

#[derive(Parser)]
struct MonitorCommands {
    #[command(subcommand)]
    command: MonitorSubCommands,
}

#[derive(Subcommand)]
enum MonitorSubCommands {
    /// Show current TVL statistics
    Stats,
    /// Monitor staking transactions
    Transactions,
    /// Show process status
    Status,
}

impl TvlCommand for MonitorCommands {
    fn execute(&self) {
        match &self.command {
            MonitorSubCommands::Stats => {
                println!("Showing TVL statistics");
            }
            MonitorSubCommands::Transactions => {
                println!("Monitoring staking transactions");
            }
            MonitorSubCommands::Status => {
                println!("Showing process status");
            }
        }
    }
}
