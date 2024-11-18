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

    #[validate(length(min = 20))]
    pub user_address: String,

    #[validate(length(min = 52))]
    pub protocol_private_key: String,

    #[validate(length(equal = 5))]
    pub covenant_private_keys: Vec<String>,

    #[validate(range(min = 0))]
    pub destination_chain_id: u64,

    #[validate(length(equal = 40))]
    pub destination_contract_address: String,

    #[validate(length(equal = 40))]
    pub destination_recipient_address: String,

    #[validate(range(min = 0))]
    pub covenant_quorum: u8,

    #[validate(range(min = 0))]
    pub staking_amount: u64,

    pub have_only_covenants: bool,

    #[validate(length(equal = 64))]
    pub utxo_tx_id: String,

    #[validate(range(min = 0))]
    pub utxo_amount: u64,

    #[validate(range(min = 0))]
    pub utxo_vout: u32,

    #[validate(length(min = 44))]
    pub script_pubkey: String,

    #[validate(length(min = 3))]
    pub network: String,
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
            btc_node_address: env::var("BTC_NODE_ADDRESS").unwrap_or("localhost:18332".to_string()),
            btc_node_user: env::var("BTC_NODE_USER").unwrap_or("user".to_string()),
            btc_node_password: env::var("BTC_NODE_PASSWORD").unwrap_or("password".to_string()),
            user_private_key: env::var("BOND_HOLDER_PRIVATE_KEY").unwrap(),
            user_address: env::var("BOND_HOLDER_ADDRESS").unwrap(),
            protocol_private_key: env::var("PROTOCOL_PRIVATE_KEY").unwrap(),
            covenant_private_keys: env::var("COVENANT_PRIVKEYS")
                .unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect(),

            destination_chain_id: env::var("DESTINATION_CHAIN_ID")
                .unwrap_or("11155111".to_string())
                .parse()
                .unwrap_or_default(),
            destination_contract_address: env::var("DESTINATION_CONTRACT_ADDRESS")
                .unwrap_or("1F98C06D8734D5A9FF0b53e3294626E62e4d232C".to_string()),

            destination_recipient_address: env::var("DESTINATION_RECIPIENT_ADDRESS")
                .unwrap_or("130C4810D57140e1E62967cBF742CaEaE91b6ecE".to_string()),

            covenant_quorum: env::var("COVENANT_QUORUM")
                .unwrap_or("3".to_string())
                .parse()
                .unwrap_or_default(),

            staking_amount: env::var("STAKING_AMOUNT")
                .unwrap_or("9999".to_string()) //777 in hex
                .parse()
                .unwrap_or_default(),

            have_only_covenants: env::var("HAVE_ONLY_COVENANTS")
                .unwrap_or("false".to_string())
                .parse()
                .unwrap_or_default(),

            utxo_tx_id: env::var("UTXO_TX_ID").unwrap_or(
                "df0404d9a9baea8b6e71bfb3566517f1765300f6603f158bc530c3323f6dbd34".to_string(),
            ),
            utxo_amount: env::var("UTXO_AMOUNT")
                .unwrap_or("10000100".to_string())
                .parse()
                .unwrap_or_default(),
            utxo_vout: env::var("UTXO_VOUT")
                .unwrap_or("0".to_string())
                .parse()
                .unwrap_or_default(),
            script_pubkey: env::var("SCRIPT_PUBKEY")
                .unwrap_or("00141302a4ea98285baefb2d290de541d069356d88e9".to_string()),
            network: env::var("NETWORK").unwrap_or("regtest".to_string()),
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
            user_address: "".to_string(),
            protocol_private_key: "".to_string(),
            covenant_private_keys: vec![],
            destination_chain_id: 11155111,
            destination_contract_address: "1F98C06D8734D5A9FF0b53e3294626E62e4d232C".to_string(),
            destination_recipient_address: "130C4810D57140e1E62967cBF742CaEaE91b6ecE".to_string(),
            covenant_quorum: 3,
            staking_amount: 10_000,
            have_only_covenants: false,
            utxo_tx_id: "df0404d9a9baea8b6e71bfb3566517f1765300f6603f158bc530c3323f6dbd34"
                .to_string(),
            utxo_amount: 10_000_100,
            utxo_vout: 0,
            script_pubkey: "00141302a4ea98285baefb2d290de541d069356d88e9".to_string(),
            network: "regtest".to_string(),
        }
    }
}
