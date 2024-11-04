use crate::{hex_to_vec, TEST_PSBT_HEX};
use bitcoin::hex::DisplayHex;
use bitcoin::key::{Keypair, Secp256k1};
use bitcoin::secp256k1::All;
use bitcoin::{NetworkKind, PrivateKey, Psbt};
use bitcoin_vault::{Signing, VaultManager};
use lazy_static::lazy_static;

const PRIVKEY_BYTES: [u8; 32] = [
    125, 221, 108, 89, 233, 54, 137, 38, 39, 96, 249, 37, 139, 178, 5, 233, 45, 53, 61, 29, 106,
    151, 199, 217, 217, 134, 194, 71, 252, 255, 206, 30,
];

const PROTOCOL_PRIVKEY_BYTES: [u8; 32] = [
    129, 67, 227, 254, 41, 141, 43, 241, 73, 116, 48, 183, 95, 65, 218, 75, 107, 92, 246, 14, 109,
    138, 166, 68, 5, 239, 211, 223, 74, 4, 141, 159,
];

const EXPECTED_TX_HEX: &str = "02000000000101e0a68346c9118f584c22c9afa89b641e06127d1b1fa661788ea922261dee37600000000000fdffffff012823000000000000160014acd07b22adf2299c56909c9ca537fd2c58127ecc04406b665c5660454029a0dd164b076159e1a53f4d891199246329b5fa9d738d2fe9035fb3ea8ea82416b18d6fae118740e8cfbda706dccbaecf14a6bc70a69bda0e40b21c79a3f1196e8d8d309eff56b4ca2f39cb2957c0a540f66aed88d1ca33bdcaea2434cc02c71c30bb2ceaa629dcdf2fd2b6a5efef019cd07bde292edeb2230d4420f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256fad20992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbbac41c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac007e94635a4727997d13497f6529f00a9ca291c2e6e10253eb995eecd130a9eeb00000000";

const WIF: &str = "cRoNDamQJSyB6jeopAXrSW7F3WKr9kqzUU8qenF3cugyAVJZZP9Z";

lazy_static! {
    static ref SECP256K1: Secp256k1<All> = Secp256k1::new();
    static ref USER_KEY_PAIR: Keypair =
        Keypair::from_seckey_slice(&SECP256K1, &PRIVKEY_BYTES).unwrap();
}

#[test]
fn test_sign_psbt() {
    // let mut psbt = MANAGER
    //     .build_user_protocol_spend(&BUILD_USER_PROTOCOL_SPEND_PARAMS)
    //     .unwrap();
    // let psbt_hex = psbt.serialize_hex();
    // assert_eq!(TEST_PSBT_HEX, psbt_hex);

    let psbt_slice = hex_to_vec!(TEST_PSBT_HEX);
    let mut psbt = Psbt::deserialize(&psbt_slice).unwrap();

    let privkey = PrivateKey::from_slice(&PRIVKEY_BYTES, NetworkKind::Test).unwrap();

    assert_eq!(WIF, privkey.to_wif());

    let psbt_hex =
        VaultManager::sign_psbt_by_single_key(&mut psbt, &PRIVKEY_BYTES, NetworkKind::Test, false)
            .unwrap();

    assert_eq!("70736274ff0100520200000001e0a68346c9118f584c22c9afa89b641e06127d1b1fa661788ea922261dee37600000000000fdffffff012823000000000000160014acd07b22adf2299c56909c9ca537fd2c58127ecc000000000001012b102700000000000022512054bfa5690019d09073d75d1094d6eb9a551a5d61b0fcfc1fd474da6bfea88627010304000000004114f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f9e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a40b21c79a3f1196e8d8d309eff56b4ca2f39cb2957c0a540f66aed88d1ca33bdcaea2434cc02c71c30bb2ceaa629dcdf2fd2b6a5efef019cd07bde292edeb2230d4215c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac007e94635a4727997d13497f6529f00a9ca291c2e6e10253eb995eecd130a9eeb4520f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256fad20992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbbacc02116992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbb25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a000000002116f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0011820867e83e93516ecde27680f5af69af0bd633f9918874b975c7e65c0b2419047ee0000", psbt_hex.to_lower_hex_string());

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

    println!(
        "Before protocol signing: {:?}",
        psbt.inputs[0].final_script_witness
    );

    let psbt_hex = VaultManager::sign_psbt_by_single_key(
        &mut psbt,
        &PROTOCOL_PRIVKEY_BYTES,
        NetworkKind::Test,
        true,
    )
    .unwrap();

    assert_eq!(EXPECTED_TX_HEX, psbt_hex.to_lower_hex_string());
}
