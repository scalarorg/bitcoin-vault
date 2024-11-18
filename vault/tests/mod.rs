use std::sync::LazyLock;
use std::sync::Mutex;

pub const TEST_CUSTODIAL_QUORUM: u8 = 1;
pub const TEST_HAVE_ONLY_COVENANTS: bool = false;
pub const TEST_RBF: bool = true;
pub const TEST_UTXO_AMOUNT: u64 = 10_000;
pub const TEST_FEE_AMOUNT: u64 = 1000;
pub const TEST_UTXO_VOUT: u32 = 0;
pub const TEST_VERSION: u8 = 0;
pub const TEST_TAG: [u8; 6] = *b"SCALAR";
pub const TEST_SERVICE_TAG: [u8; 5] = *b"light";

pub static MOCK_ENV: LazyLock<Env> = LazyLock::new(|| {
    let mut default = Env::default();

    default.user_private_key = "cRoNDamQJSyB6jeopAXrSW7F3WKr9kqzUU8qenF3cugyAVJZZP9Z".to_string();
    default.protocol_private_key =
        "cRuyaW8qfcs883Dy8E6iW34V3Q2n8XkGMaqtyMSBDQm55MXh28oM".to_string();
    default.covenant_private_keys = vec![
        "cU5tEWuenDn5mnaqKj6BG62CqM8fxaK75m17uaJnp7xaguyTEE1U".to_string(),
        "cSeWRDV4Zh85PR1FK52XauRCJFexhhS7Q6Hqu47iFurT6hAwTEvd".to_string(),
        "cU2JyH1TnXFZnxizaZkMtv9JnyifYXe2fidGdaXVi8KgiAJGdkQS".to_string(),
        "cVeRiZx5L7KrTi64fCqvwmPUT5L6kPpzY6giVeqKUAmbLQ65KUEm".to_string(),
        "cN1jnejt9RxjRJEj7H6Z4rHj3RRESG9k58mmbRq9axEDpE3FWfeY".to_string(),
    ];
    default.utxo_tx_id =
        "6037ee1d2622a98e7861a61f1b7d12061e649ba8afc9224c588f11c94683a6e0".to_string();
    default.utxo_amount = TEST_UTXO_AMOUNT;
    default.utxo_vout = TEST_UTXO_VOUT;
    default.script_pubkey =
        "512054bfa5690019d09073d75d1094d6eb9a551a5d61b0fcfc1fd474da6bfea88627".to_string();
    default
});

lazy_static! {
    pub static ref MANAGER: VaultManager = VaultManager::new(
        TEST_TAG.to_vec(),
        TEST_SERVICE_TAG.to_vec(),
        TEST_VERSION,
        bitcoin::NetworkKind::Test as u8
    );
    pub static ref SUITE: Mutex<TestSuite<'static>> = Mutex::new(TestSuite::new());
}

pub fn hex_to_vec(hex: &str) -> Vec<u8> {
    let hex_str = hex.replace("0x", "").replace(" ", "");
    let mut vec = Vec::new();
    for i in (0..hex_str.len()).step_by(2) {
        vec.push(u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap());
    }
    vec
}

#[cfg(test)]
mod suite;

use bitcoin_vault::VaultManager;
use env::Env;
use lazy_static::lazy_static;
use suite::TestSuite;

#[cfg(test)]
mod env;

#[cfg(test)]
mod sign_psbt;

#[cfg(test)]
mod e2e;

#[cfg(test)]
mod utxos;

#[cfg(test)]
mod e2e_rpc;

#[cfg(test)]
mod only_covenants;
