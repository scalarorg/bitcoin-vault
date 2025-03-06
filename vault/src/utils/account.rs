use bitcoin::{address::NetworkChecked, key::Secp256k1, Address, PrivateKey, PublicKey};
use dotenv::from_path;
use macros::EnvLoad;
use serde::Deserialize;
use std::{env, fmt::Debug};
use validator::{Validate, ValidationError};

use super::helper::{get_adress, key_from_wif};

#[derive(Clone, Deserialize, Validate, EnvLoad)]
pub struct AccountEnv {
    #[validate(length(equal = 52))]
    #[env_var(key = "BOND_HOLDER_PRIVATE_KEY")]
    pub private_key: String,

    #[validate(length(min = 20))]
    #[env_var(key = "BOND_HOLDER_ADDRESS")]
    pub address: String,

    #[validate(length(min = 1))]
    #[env_var(key = "NETWORK")]
    pub network: String,
}

impl Default for AccountEnv {
    fn default() -> Self {
        AccountEnv {
            private_key: "".to_string(),
            address: "".to_string(),
            network: "regtest".to_string(),
        }
    }
}

impl Debug for AccountEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "======== ACCOUNT ========")?;
        writeln!(f, "{:<40}{}", "PRIVATE KEY:", self.private_key)?;
        writeln!(f, "{:<40}{}", "ADDRESS:", self.address)?;
        writeln!(f, "======== ACCOUNT ========")?;
        Ok(())
    }
}

impl AccountEnv {
    pub fn new(path: Option<&str>) -> Result<Self, ValidationError> {
        let current_dir = env::current_dir().unwrap();
        if let Some(path) = path {
            from_path(current_dir.join(path)).ok();
        } else {
            from_path(current_dir.join(".env")).ok();
        }

        let account = AccountEnv::load_from_env();

        if let Err(err) = account.validate() {
            panic!("Validation error: {:?}", err);
        }

        Ok(account)
    }
}

#[derive(Clone, Debug)]
pub struct SuiteAccount {
    private_key: PrivateKey,
    public_key: PublicKey,
    address: Address<NetworkChecked>,
}

impl SuiteAccount {
    pub fn new(env: AccountEnv) -> Self {
        let secp = Secp256k1::new();

        let (private_key, public_key) = key_from_wif(&env.private_key, &secp);

        let address = get_adress(&env.network, &env.address);

        Self {
            private_key,
            public_key,
            address,
        }
    }

    pub fn private_key(&self) -> PrivateKey {
        self.private_key
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn address(&self) -> &Address<NetworkChecked> {
        &self.address
    }
}
