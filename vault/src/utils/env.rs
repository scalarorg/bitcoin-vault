use dotenv::from_path;
use macros::EnvLoad;
use serde::Deserialize;
use std::{env, fmt::Debug};
use validator::{Validate, ValidationError};

#[derive(Clone, Deserialize, Validate, EnvLoad)]
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

    #[validate(length(equal = 52))]
    #[env_var(key = "BOND_HOLDER_PRIVATE_KEY")]
    pub bond_holder_private_key: String,

    #[validate(length(min = 20))]
    #[env_var(key = "BOND_HOLDER_ADDRESS")]
    pub bond_holder_address: String,

    #[validate(length(min = 1))]
    #[env_var(key = "BOND_HOLDER_WALLET")]
    pub bond_holder_wallet: String,

    #[validate(length(min = 52))]
    #[env_var(key = "PROTOCOL_PRIVATE_KEY")]
    pub protocol_private_key: String,

    #[validate(length(equal = 5))]
    #[env_var(key = "CUSTODIAN_PRIVKEYS")]
    pub custodian_private_keys: Vec<String>,

    #[validate(length(equal = 16))]
    #[env_var(key = "DESTINATION_CHAIN")]
    pub destination_chain: String,

    #[validate(length(equal = 40))]
    #[env_var(key = "DESTINATION_TOKEN_ADDRESS")]
    pub destination_token_address: String,

    #[validate(length(equal = 40))]
    #[env_var(key = "DESTINATION_RECIPIENT_ADDRESS")]
    pub destination_recipient_address: String,

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

    #[validate(length(min = 3))]
    #[env_var(key = "SERVICE_TAG")]
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

        let env = Env::load_from_env();

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
            custodian_private_keys: vec![],
            destination_chain: "0100000000AA36A7".to_string(),
            destination_token_address: "".to_string(),
            destination_recipient_address: "".to_string(),
            custodian_quorum: 3,
            network: "regtest".to_string(),
            tag: "".to_string(),
            version: 0,
            service_tag: "".to_string(),
        }
    }
}

impl Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "======== ENVIRONMENT VARIABLES ========\n")?;
        write!(f, "{:<40}{}\n", "BTC_NODE_ADDRESS:", self.btc_node_address)?;
        write!(f, "{:<40}{}\n", "BTC_NODE_USER:", self.btc_node_user)?;
        write!(
            f,
            "{:<40}{}\n",
            "BTC_NODE_PASSWORD:", self.btc_node_password
        )?;
        write!(f, "{:<40}{}\n", "NETWORK:", self.network)?;
        write!(f, "{:<40}{}\n", "TAG:", self.tag)?;
        write!(f, "{:<40}{}\n", "VERSION:", self.version)?;
        write!(f, "{:<40}{}\n", "SERVICE_TAG:", self.service_tag)?;
        write!(
            f,
            "{:<40}{}\n",
            "BOND_HOLDER_PRIVATE_KEY:", self.bond_holder_private_key
        )?;
        write!(
            f,
            "{:<40}{}\n",
            "BOND_HOLDER_ADDRESS:", self.bond_holder_address
        )?;
        write!(
            f,
            "{:<40}{}\n",
            "BOND_HOLDER_WALLET:", self.bond_holder_wallet
        )?;
        write!(
            f,
            "{:<40}{}\n",
            "PROTOCOL_PRIVATE_KEY:", self.protocol_private_key
        )?;
        write!(f, "{:<40}", "CUSTODIAN_PRIVKEYS:")?;
        for (i, key) in self.custodian_private_keys.iter().enumerate() {
            if i == 0 {
                write!(f, "{}\n", key)?;
            } else {
                write!(f, "{:<40}{}\n", "", key)?;
            }
        }
        write!(
            f,
            "{:<40}{}\n",
            "DESTINATION_CHAIN:", self.destination_chain
        )?;
        write!(
            f,
            "{:<40}{}\n",
            "DESTINATION_TOKEN_ADDRESS:", self.destination_token_address
        )?;
        write!(
            f,
            "{:<40}{}\n",
            "DESTINATION_RECIPIENT_ADDRESS:", self.destination_recipient_address
        )?;
        write!(f, "{:<40}{}\n", "CUSTODIAN_QUORUM:", self.custodian_quorum)?;

        write!(f, "======== ENVIRONMENT VARIABLES ========\n")?;

        Ok(())
    }
}

#[test]
fn test_macro() {
    let env = Env::new(Some(".env.test.testnet4")).unwrap();
    println!("{:?}", env);
}
