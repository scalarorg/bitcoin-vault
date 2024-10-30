use crate::{BUILD_USER_PROTOCOL_SPEND_PARAMS, MANAGER, MOCK_ENV, TEST_PSBT_HEX};
use bitcoin::key::{Keypair, Secp256k1};
use bitcoin::secp256k1::All;
use bitcoin::{NetworkKind, PrivateKey};
use bitcoin_vault::{Signing, Unstaking};
use lazy_static::lazy_static;

const PRIVKEY_BYTES: [u8; 32] = [
    125, 221, 108, 89, 233, 54, 137, 38, 39, 96, 249, 37, 139, 178, 5, 233, 45, 53, 61, 29, 106,
    151, 199, 217, 217, 134, 194, 71, 252, 255, 206, 30,
];

const WIF: &str = "cRoNDamQJSyB6jeopAXrSW7F3WKr9kqzUU8qenF3cugyAVJZZP9Z";

lazy_static! {
    static ref SECP256K1: Secp256k1<All> = Secp256k1::new();
    static ref USER_KEY_PAIR: Keypair =
        Keypair::from_seckey_slice(&SECP256K1, &PRIVKEY_BYTES).unwrap();
}

#[test]
fn test_sign_psbt() {
    let mut psbt = MANAGER
        .build_user_protocol_spend(&BUILD_USER_PROTOCOL_SPEND_PARAMS)
        .unwrap();
    let psbt_hex = psbt.serialize_hex();
    assert_eq!(TEST_PSBT_HEX, psbt_hex);

    let privkey = PrivateKey::from_slice(&PRIVKEY_BYTES, NetworkKind::Test).unwrap();

    assert_eq!(WIF, privkey.to_wif());

    let _ = MANAGER
        .sign_psbt_by_single_key(&mut psbt, &PRIVKEY_BYTES, NetworkKind::Test, None)
        .unwrap();

    let tap_script_sigs = psbt.inputs[0].tap_script_sigs.clone();

    println!("tap_script_sigs: {:?}", tap_script_sigs);

    assert_eq!(tap_script_sigs.len(), 1);
    let ((pubkey, leaf_hash), signature) = tap_script_sigs.first_key_value().unwrap();

    assert_eq!(
        pubkey.to_string(),
        "f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f"
    );

    assert_eq!(
        leaf_hash.to_string(),
        "9e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a"
    );

    // Note: This assertion is not stable when enabling "rand-std" feature.
    assert_eq!(
        signature.signature.to_string(),
        "b21c79a3f1196e8d8d309eff56b4ca2f39cb2957c0a540f66aed88d1ca33bdcaea2434cc02c71c30bb2ceaa629dcdf2fd2b6a5efef019cd07bde292edeb2230d"
    );

    println!("User signed psbt successfully");
}
