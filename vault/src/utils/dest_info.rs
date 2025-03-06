use dotenv::from_path;
use macros::EnvLoad;
use serde::Deserialize;
use std::{env, fmt::Debug};
use validator::{Validate, ValidationError};

use crate::{DestinationChain, DestinationRecipientAddress, DestinationTokenAddress};

use super::hex_to_vec;

#[derive(Clone, Deserialize, Validate, EnvLoad)]
pub struct DestinationInfoEnv {
    #[validate(length(equal = 16))]
    #[env_var(key = "DESTINATION_CHAIN")]
    pub destination_chain: String,

    #[validate(length(equal = 40))]
    #[env_var(key = "DESTINATION_TOKEN_ADDRESS")]
    pub destination_token_address: String,

    #[validate(length(equal = 40))]
    #[env_var(key = "DESTINATION_RECIPIENT_ADDRESS")]
    pub destination_recipient_address: String,
}

impl Default for DestinationInfoEnv {
    fn default() -> Self {
        DestinationInfoEnv {
            destination_chain: "0100000000AA36A7".to_string(),
            destination_token_address: "".to_string(),
            destination_recipient_address: "".to_string(),
        }
    }
}

impl Debug for DestinationInfoEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "======== DESTINATION INFO ========")?;
        writeln!(f, "{:<40}{}", "DESTINATION CHAIN:", self.destination_chain)?;
        writeln!(
            f,
            "{:<40}{}",
            "DESTINATION TOKEN ADDRESS:", self.destination_token_address
        )?;
        writeln!(
            f,
            "{:<40}{}",
            "DESTINATION RECIPIENT ADDRESS:", self.destination_recipient_address
        )?;
        writeln!(f, "======== DESTINATION INFO ========")?;
        Ok(())
    }
}

impl DestinationInfoEnv {
    pub fn new(path: Option<&str>) -> Result<Self, ValidationError> {
        let current_dir = env::current_dir().unwrap();
        if let Some(path) = path {
            from_path(current_dir.join(path)).ok();
        } else {
            from_path(current_dir.join(".env")).ok();
        }

        let destination_info = DestinationInfoEnv::load_from_env();

        if let Err(err) = destination_info.validate() {
            panic!("Validation error: {:?}", err);
        }

        Ok(destination_info)
    }
}

#[derive(Clone, Debug)]
pub struct DestinationInfo {
    pub destination_chain: DestinationChain,
    pub destination_token_address: DestinationTokenAddress,
    pub destination_recipient_address: DestinationRecipientAddress,
}

impl DestinationInfo {
    pub fn new(env: DestinationInfoEnv) -> Self {
        Self {
            destination_chain: hex_to_vec(&env.destination_chain).try_into().unwrap(),
            destination_token_address: hex_to_vec(&env.destination_token_address)
                .try_into()
                .unwrap(),
            destination_recipient_address: hex_to_vec(&env.destination_recipient_address)
                .try_into()
                .unwrap(),
        }
    }
}
