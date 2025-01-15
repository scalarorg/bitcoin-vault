use dotenv::from_path;
use serde::Deserialize;
use std::env;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Env {
    #[validate(length(min = 10))]
    pub btc_node_address: String,

    #[validate(length(min = 3))]
    pub btc_node_user: String,

    #[validate(length(min = 3))]
    pub btc_node_password: String,

    #[validate(length(equal = 52))]
    pub bond_holder_private_key: String,

    #[validate(length(min = 20))]
    pub bond_holder_address: String,

    #[validate(length(min = 1))]
    pub bond_holder_wallet: String,

    #[validate(length(min = 52))]
    pub protocol_private_key: String,

    #[validate(length(equal = 5))]
    pub covenant_private_keys: Vec<String>,

    #[validate(length(equal = 16))]
    pub destination_chain: String,

    #[validate(length(equal = 40))]
    pub destination_token_address: String,

    #[validate(length(equal = 40))]
    pub destination_recipient_address: String,

    #[validate(range(min = 0))]
    pub covenant_quorum: u8,

    #[validate(length(min = 3))]
    pub network: String,

    #[validate(length(min = 6))]
    pub tag: String,

    #[validate(range(min = 0))]
    pub version: u8,

    pub service_tag: String,
}

impl Env {
    pub fn new(path: Option<&str>) -> Result<Self, ValidationError> {
        // Load environment variables from .env file
        let current_dir = env::current_dir().unwrap();
        if let Some(path) = path {
            from_path(current_dir.join(path)).ok();
        } else {
            from_path(current_dir.join(".env")).ok();
        }

        let default_env = Env::default();

        let env = Env {
            btc_node_address: env::var("BTC_NODE_ADDRESS").unwrap_or(default_env.btc_node_address),
            btc_node_user: env::var("BTC_NODE_USER").unwrap_or(default_env.btc_node_user),
            btc_node_password: env::var("BTC_NODE_PASSWORD")
                .unwrap_or(default_env.btc_node_password),
            bond_holder_private_key: env::var("BOND_HOLDER_PRIVATE_KEY").unwrap(),
            bond_holder_address: env::var("BOND_HOLDER_ADDRESS").unwrap(),
            bond_holder_wallet: env::var("BOND_HOLDER_WALLET")
                .unwrap_or(default_env.bond_holder_wallet),
            protocol_private_key: env::var("PROTOCOL_PRIVATE_KEY").unwrap(),
            covenant_private_keys: env::var("COVENANT_PRIVKEYS")
                .unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            destination_chain: env::var("DESTINATION_CHAIN")
                .unwrap_or(default_env.destination_chain),

            destination_token_address: env::var("DESTINATION_TOKEN_ADDRESS")
                .unwrap_or(default_env.destination_token_address),
            destination_recipient_address: env::var("DESTINATION_RECIPIENT_ADDRESS")
                .unwrap_or(default_env.destination_recipient_address),
            covenant_quorum: env::var("COVENANT_QUORUM")
                .map(|v| v.parse().unwrap_or(default_env.covenant_quorum))
                .unwrap_or(default_env.covenant_quorum),
            network: env::var("NETWORK").unwrap_or(default_env.network),
            tag: env::var("TAG").unwrap_or(default_env.tag),
            version: env::var("VERSION")
                .map(|v| v.parse().unwrap_or(default_env.version))
                .unwrap_or(default_env.version),
            service_tag: env::var("SERVICE_TAG").unwrap_or(default_env.service_tag),
        };

        if let Err(err) = env.validate() {
            panic!("Validation error: {:?}", err);
        }

        Ok(env)
    }
}

impl Default for Env {
    fn default() -> Self {
        Env {
            btc_node_address: "localhost:18332".to_string(),
            btc_node_user: "user".to_string(),
            btc_node_password: "password".to_string(),
            bond_holder_private_key: "".to_string(),
            bond_holder_address: "".to_string(),
            bond_holder_wallet: "legacy".to_string(),
            protocol_private_key: "".to_string(),
            covenant_private_keys: vec![],
            destination_chain: "0100000000AA36A7".to_string(),
            destination_token_address: "".to_string(),
            destination_recipient_address: "".to_string(),
            covenant_quorum: 3,
            network: "regtest".to_string(),
            tag: "".to_string(),
            version: 0,
            service_tag: "".to_string(),
        }
    }
}
