use crate::{hex_to_vec, Env};

use lazy_static::lazy_static;
use std::sync::LazyLock;

use bitcoin::hashes::Hash;
use bitcoin::key::Secp256k1;
use bitcoin::{Amount, CompressedPublicKey, NetworkKind, OutPoint, ScriptBuf, TxOut};
use bitcoin::{PrivateKey, PublicKey, Txid};
use bitcoin_vault::VaultManager;
use bitcoin_vault::{BuildUnstakingParams, PreviousStakingUTXO};

pub const TEST_CUSTODIAL_QUORUM: u8 = 1;
pub const TEST_HAVE_ONLY_COVENANTS: bool = false;
pub const TEST_RBF: bool = true;
pub const TEST_UTXO_AMOUNT: u64 = 10_000;
pub const TEST_FEE_AMOUNT: u64 = 1000;
pub const TEST_UTXO_VOUT: u32 = 0;
pub const TEST_VERSION: u8 = 0;
pub const TEST_TAG: [u8; 6] = [83, 67, 65, 76, 65, 82];
pub const TEST_SERVICE_TAG: [u8; 5] = [100, 97, 118, 105, 100];
pub const TEST_PSBT_HEX: &str = "70736274ff0100520200000001e0a68346c9118f584c22c9afa89b641e06127d1b1fa661788ea922261dee37600000000000fdffffff012823000000000000160014acd07b22adf2299c56909c9ca537fd2c58127ecc000000000001012b102700000000000022512054bfa5690019d09073d75d1094d6eb9a551a5d61b0fcfc1fd474da6bfea88627010304000000004215c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac007e94635a4727997d13497f6529f00a9ca291c2e6e10253eb995eecd130a9eeb4520f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256fad20992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbbacc02116992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbb25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a000000002116f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0011820867e83e93516ecde27680f5af69af0bd633f9918874b975c7e65c0b2419047ee0000";

pub const USER_SCRIPT_PUBKEY: &str = "0014acd07b22adf2299c56909c9ca537fd2c58127ecc";

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
    pub static ref BUILD_USER_PROTOCOL_SPEND_PARAMS: BuildUnstakingParams =
        load_build_user_protocol_spend_params();
    pub static ref MANAGER: VaultManager = VaultManager::new(
        TEST_TAG.to_vec(),
        TEST_SERVICE_TAG.to_vec(),
        TEST_VERSION,
        NetworkKind::Test as u8
    );
}

fn load_build_user_protocol_spend_params() -> BuildUnstakingParams {
    let secp = &Secp256k1::new();

    let user_privkey = PrivateKey::from_wif(&MOCK_ENV.user_private_key).unwrap();
    let user_pub_key = user_privkey.public_key(secp);

    let protocol_privkey = PrivateKey::from_wif(&MOCK_ENV.protocol_private_key).unwrap();
    let protocol_pub_key = protocol_privkey.public_key(secp);

    let covenant_pub_keys: Vec<PublicKey> = MOCK_ENV
        .covenant_private_keys
        .iter()
        .map(|k| PrivateKey::from_wif(k).unwrap().public_key(secp))
        .collect();

    println!("===== KEYS =====");
    println!("user_pub_key: {:?}", user_pub_key.to_string());
    println!("protocol_pub_key: {:?}", protocol_pub_key.to_string());
    for (i, covenant_pubkey) in covenant_pub_keys.iter().enumerate() {
        println!("covenant_pubkey {}: {:?}", i, covenant_pubkey.to_string());
    }

    println!("===== UTXOS =====");
    println!("utxo_tx_id: {:?}", MOCK_ENV.utxo_tx_id);
    println!("utxo_vout: {:?}", MOCK_ENV.utxo_vout);
    println!("utxo_amount: {:?}", MOCK_ENV.utxo_amount);
    println!("script_pubkey: {:?}", MOCK_ENV.script_pubkey);
    println!("===== ---- =====");

    let compressed_user_pub_key =
        CompressedPublicKey::from_private_key(secp, &user_privkey).unwrap();

    let wpkh = compressed_user_pub_key.wpubkey_hash();
    let p2wpkh_script = ScriptBuf::new_p2wpkh(&wpkh);

    assert_eq!(p2wpkh_script.to_hex_string(), USER_SCRIPT_PUBKEY);

    BuildUnstakingParams {
        input_utxo: PreviousStakingUTXO {
            script_pubkey: ScriptBuf::from_hex(&MOCK_ENV.script_pubkey).unwrap(),
            outpoint: OutPoint {
                txid: {
                    let tx_bytes = hex_to_vec!(MOCK_ENV.utxo_tx_id);
                    let txid = Txid::from_slice(&tx_bytes).unwrap();
                    txid
                },
                vout: MOCK_ENV.utxo_vout,
            },
            amount_in_sats: Amount::from_sat(TEST_UTXO_AMOUNT),
        },
        unstaking_output: TxOut {
            value: Amount::from_sat(TEST_UTXO_AMOUNT - TEST_FEE_AMOUNT), // 1000 sats for fees -> 9_000
            script_pubkey: p2wpkh_script,
        },
        user_pub_key,
        protocol_pub_key,
        covenant_pub_keys,
        covenant_quorum: TEST_CUSTODIAL_QUORUM,
        have_only_covenants: TEST_HAVE_ONLY_COVENANTS,
        rbf: TEST_RBF,
    }
}
