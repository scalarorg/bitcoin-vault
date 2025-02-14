use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::db::{Config, Querier};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct ConfigCommand {
    /// Name for the config unique
    #[arg(short, long)]
    pub name: String,

    /// Bitcoin node address
    #[arg(short, long)]
    pub btc_node_address: String,

    /// Bitcoin node user
    #[arg(short, long)]
    pub btc_node_user: String,

    /// Bitcoin node password
    #[arg(short, long)]
    pub btc_node_password: String,

    /// Bitcoin node wallet
    #[arg(short, long)]
    pub btc_node_wallet: String,

    /// Protocol private key
    #[arg(short, long)]
    pub protocol_private_key: String,

    /// Custodian private keys
    #[arg(short, long)]
    pub custodian_private_keys: Vec<String>,

    /// Custodian quorum
    #[arg(short, long)]
    pub custodian_quorum: u32,

    /// Network, regtest, testnet4, testnet3, mainnet, signet
    #[arg(short, long)]
    pub network: String,

    /// Tag
    #[arg(short, long)]
    pub tag: String,

    /// Version
    #[arg(short, long)]
    pub version: u32,

    /// Mnemonic
    #[arg(short, long)]
    pub mnemonic: String,

    #[arg(short, long)]
    pub destination_chain: String,

    /// Destination token address
    #[arg(short, long)]
    pub destination_token_address: String,

    /// Destination recipient address
    #[arg(short, long)]
    pub destination_recipient_address: String,

    /// Mempool url
    #[arg(short, long)]
    pub mempool_url: String,
}

impl ConfigCommand {
    pub fn execute(&self, db_querier: &Querier) -> anyhow::Result<()> {
        let config = Config::new(
            self.name.clone(),
            self.btc_node_address.clone(),
            self.btc_node_user.clone(),
            self.btc_node_password.clone(),
            self.btc_node_wallet.clone(),
            self.protocol_private_key.clone(),
            self.custodian_private_keys.clone(),
            self.custodian_quorum,
            self.network.clone(),
            self.tag.clone(),
            self.version,
            self.mnemonic.clone(),
            self.destination_chain.clone(),
            self.destination_token_address.clone(),
            self.destination_recipient_address.clone(),
            self.mempool_url.clone(),
        );

        let id = db_querier
            .save(&config)
            .map_err(|e| anyhow::anyhow!("Failed to create config: {:?}", e))?;

        println!("Config created: {:?}", id);
        Ok(())
    }
}
