use crate::{BUILD_USER_PROTOCOL_SPEND_PARAMS, MANAGER, TEST_PSBT_HEX};
use bitcoin_vault::Unstaking;

#[test]
fn test_build_user_protocol_spend() {
    let psbt = MANAGER
        .build_user_protocol_spend(&BUILD_USER_PROTOCOL_SPEND_PARAMS)
        .unwrap();
    let psbt_hex = psbt.serialize_hex();
    assert_eq!(TEST_PSBT_HEX, psbt_hex);

    println!("psbt_hex: {:?}", psbt_hex);
}
