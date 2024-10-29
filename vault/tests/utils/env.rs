use dotenv::from_path;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct Env {
    #[validate(length(min = 10))]
    pub btc_node_address: String,

    #[validate(length(min = 3))]
    pub btc_node_user: String,

    #[validate(length(min = 3))]
    pub btc_node_password: String,

    #[validate(length(equal = 52))]
    pub user_private_key: String,

    #[validate(length(min = 52))]
    pub protocol_private_key: String,

    #[validate(length(equal = 5))]
    pub covenant_private_keys: Vec<String>,

    #[validate(length(equal = 64))]
    pub utxo_tx_id: String,

    #[validate(range(min = 0))]
    pub utxo_amount: u64,

    #[validate(range(min = 0))]
    pub utxo_vout: u32,

    #[validate(length(min = 44))]
    pub script_pubkey: String,
}

lazy_static! {
    static ref ENV: Env = Env::new(None).unwrap();
    static ref ENV_TEST: Env = Env::new(Some(".env.test")).unwrap();
}

pub fn get_env() -> &'static Env {
    if cfg!(test) {
        &ENV_TEST
    } else {
        &ENV
    }
}

impl Env {
    fn new(path: Option<&str>) -> Result<Self, ValidationError> {
        // Load environment variables from .env file
        let current_dir = env::current_dir().unwrap();
        if let Some(path) = path {
            from_path(current_dir.join(path)).ok();
        } else {
            from_path(current_dir.join(".env")).ok();
        }

        let env = Env {
            btc_node_address: env::var("BTC_NODE_ADDRESS").unwrap(),
            btc_node_user: env::var("BTC_NODE_USER").unwrap(),
            btc_node_password: env::var("BTC_NODE_PASSWORD").unwrap(),
            user_private_key: env::var("USER_PRIVATE_KEY").unwrap(),
            protocol_private_key: env::var("PROTOCOL_PRIVATE_KEY").unwrap(),
            covenant_private_keys: env::var("COVENANT_PRIVATE_KEYS")
                .unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            utxo_tx_id: env::var("UTXO_TX_ID").unwrap(),
            utxo_amount: env::var("UTXO_AMOUNT").unwrap().parse().unwrap(),
            utxo_vout: env::var("UTXO_VOUT").unwrap().parse().unwrap(),
            script_pubkey: env::var("SCRIPT_PUBKEY").unwrap(),
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
            user_private_key: "".to_string(),
            protocol_private_key: "".to_string(),
            covenant_private_keys: vec![],
            utxo_tx_id: "".to_string(),
            utxo_amount: 0,
            utxo_vout: 0,
            script_pubkey: "".to_string(),
        }
    }
}
