use clap::Subcommand;

use crate::{CollectCmd, CollectUtxosCmd};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Collect utxos from multiple addresses
    Collect(CollectCmd),
    /// Collect and batch UTXOs from a vault script
    CollectUtxos(CollectUtxosCmd),
}

impl Commands {
    pub async fn execute(&self) -> anyhow::Result<()> {
        match self {
            Commands::Collect(cmd) => cmd.execute().await,
            Commands::CollectUtxos(cmd) => cmd.execute().await,
        }
    }
}
