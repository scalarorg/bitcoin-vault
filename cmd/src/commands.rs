use clap::Subcommand;

use crate::CollectCmd;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Collect utxos from multiple addresses
    Collect(CollectCmd),
}

impl Commands {
    pub async fn execute(&self) -> anyhow::Result<()> {
        match self {
            Commands::Collect(cmd) => cmd.execute().await,
        }
    }
}
