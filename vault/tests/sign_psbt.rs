use crate::{BUILD_USER_PROTOCOL_SPEND_PARAMS, MANAGER, TEST_PSBT_HEX};
use bitcoin::{NetworkKind, PrivateKey};
use bitcoin_vault::{Signing, Unstaking};

const PRIVKEY_BYTES: [u8; 32] = [
    125, 221, 108, 89, 233, 54, 137, 38, 39, 96, 249, 37, 139, 178, 5, 233, 45, 53, 61, 29, 106,
    151, 199, 217, 217, 134, 194, 71, 252, 255, 206, 30,
];

const WIF: &str = "cRoNDamQJSyB6jeopAXrSW7F3WKr9kqzUU8qenF3cugyAVJZZP9Z";

#[test]
fn test_sign_psbt() {
    let mut psbt = MANAGER
        .build_user_protocol_spend(&BUILD_USER_PROTOCOL_SPEND_PARAMS)
        .unwrap();
    let psbt_hex = psbt.serialize_hex();
    assert_eq!(TEST_PSBT_HEX, psbt_hex);

    let privkey = PrivateKey::from_slice(&PRIVKEY_BYTES, NetworkKind::Test).unwrap();

    assert_eq!(WIF, privkey.to_wif());

    MANAGER
        .sign_psbt_by_single_key(&mut psbt, &PRIVKEY_BYTES, NetworkKind::Test, None)
        .unwrap();
}
