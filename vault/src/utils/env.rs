use dotenv::from_path;
use macros::EnvLoad;
use serde::{Deserialize, Serialize};
use std::{env, fmt::Debug};
use validator::{Validate, ValidationError};

#[derive(Clone, Deserialize, Serialize, Validate, EnvLoad)]
pub struct Env {
    #[validate(length(min = 10))]
    #[env_var(key = "BTC_NODE_ADDRESS")]
    pub btc_node_address: String,

    #[validate(length(min = 3))]
    #[env_var(key = "BTC_NODE_USER")]
    pub btc_node_user: String,

    #[validate(length(min = 3))]
    #[env_var(key = "BTC_NODE_PASSWORD")]
    pub btc_node_password: String,

    #[validate(length(min = 1))]
    #[env_var(key = "BTC_NODE_WALLET")]
    pub btc_node_wallet: String,

    #[validate(length(min = 52))]
    #[env_var(key = "PROTOCOL_PRIVATE_KEY")]
    pub protocol_private_key: String,

    #[validate(length(min = 4))]
    #[env_var(key = "CUSTODIAN_PRIVKEYS")]
    pub custodian_private_keys: Vec<String>,

    #[validate(range(min = 1))]
    #[env_var(key = "CUSTODIAN_QUORUM")]
    pub custodian_quorum: u8,

    #[validate(length(min = 3))]
    #[env_var(key = "NETWORK")]
    pub network: String,

    #[validate(length(min = 6))]
    #[env_var(key = "TAG")]
    pub tag: String,

    #[validate(range(min = 0))]
    #[env_var(key = "VERSION")]
    pub version: u8,
}

impl Default for Env {
    fn default() -> Self {
        Env {
            btc_node_address: "localhost:18332".to_string(),
            btc_node_user: "user".to_string(),
            btc_node_password: "password".to_string(),
            btc_node_wallet: "staker".to_string(),
            protocol_private_key: "".to_string(),
            custodian_private_keys: vec![],
            custodian_quorum: 3,
            network: "regtest".to_string(),
            tag: "".to_string(),
            version: 0,
        }
    }
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

        let env = Env::load_from_env();

        println!("{:?}", env);

        if let Err(err) = env.validate() {
            panic!("Validation error: {:?}", err);
        }

        Ok(env)
    }
}

impl Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "======== ENVIRONMENT VARIABLES ========")?;
        writeln!(f, "{:<40}{}", "BTC_NODE_ADDRESS:", self.btc_node_address)?;
        writeln!(f, "{:<40}{}", "BTC_NODE_USER:", self.btc_node_user)?;
        writeln!(f, "{:<40}{}", "BTC_NODE_PASSWORD:", self.btc_node_password)?;
        writeln!(f, "{:<40}{}", "NETWORK:", self.network)?;
        writeln!(f, "{:<40}{}", "TAG:", self.tag)?;
        writeln!(f, "{:<40}{}", "VERSION:", self.version)?;
        writeln!(f, "{:<40}{}", "BTC_NODE_WALLET:", self.btc_node_wallet)?;
        writeln!(
            f,
            "{:<40}{}",
            "PROTOCOL_PRIVATE_KEY:", self.protocol_private_key
        )?;
        write!(f, "{:<40}", "CUSTODIAN_PRIVKEYS:")?;
        for (i, key) in self.custodian_private_keys.iter().enumerate() {
            if i == 0 {
                writeln!(f, "{}", key)?;
            } else {
                writeln!(f, "{:<40}{}", "", key)?;
            }
        }
        writeln!(f, "{:<40}{}", "CUSTODIAN_QUORUM:", self.custodian_quorum)?;

        writeln!(f, "======== ENVIRONMENT VARIABLES ========")?;

        Ok(())
    }
}

#[test]
fn test_macro() {
    let env = Env::new(Some(".env.test.testnet4")).unwrap();
    println!("{:?}", env);
}
