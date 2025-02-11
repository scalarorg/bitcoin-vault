use clap::{Parser, Subcommand};

use super::TvlCommand;

#[derive(Parser)]
struct UnstakeCommands {
    #[command(subcommand)]
    command: UnstakeSubCommands,
}

#[derive(Subcommand)]
enum UnstakeSubCommands {
    /// Unstake UPC transactions
    Upc(UnstakingParams),
    /// Unstake Custodian-only transactions
    CustodianOnly(UnstakingParams),
}

#[derive(Parser)]
struct UnstakingParams {
    /// TXID of the staking transaction to unstake
    #[arg(long)]
    txid: Option<String>,
    /// Amount to unstake (if different from staked amount)
    #[arg(long)]
    amount: Option<u64>,
    /// Destination address for unstaked funds
    #[arg(long)]
    address: Option<String>,
}

impl TvlCommand for UnstakeCommands {
    fn execute(&self) {
        match &self.command {
            UnstakeSubCommands::Upc(params) => {
                println!("Unstaking UPC transaction with TXID {:?}", params.txid);
            }
            UnstakeSubCommands::CustodianOnly(params) => {
                println!(
                    "Unstaking Custodian-only transaction with TXID {:?}",
                    params.txid
                );
            }
        }
    }
}
