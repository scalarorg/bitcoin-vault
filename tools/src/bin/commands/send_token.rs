use clap::Parser;

use crate::TvlMaker;

use super::TvlCommand;

#[derive(Parser, Debug)]
pub struct SendTokenCommand {
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

impl TvlCommand for SendTokenCommand {
    fn name(&self) -> String {
        "send_token".to_string()
    }

    fn execute(&self, tvl_maker: &TvlMaker) -> anyhow::Result<()> {
        Ok(())
    }
}
