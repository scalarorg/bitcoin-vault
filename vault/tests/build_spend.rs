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

// #[test]
// fn test_build_covenants_protocol_spend() {
//     let params = BuildCovenantsProtocolSpendParams {
//         input_utxo: &UTXO {
//             amount_in_sats: 10000,
//             script_pubkey: ScriptBuf::new(),
//             outpoint: OutPoint::null(),
//         },
//         unstaking_output: &TxOut {
//             value: 9000,
//             script_pubkey: ScriptBuf::new(),
//         },
//         protocol_pub_key: &PROTOCOL_PUBKEY,
//         covenant_pubkeys: &COVENANT_PUBKEYS,
//         covenant_quorum: 2,
//     };

//     let psbt = MANAGER
//         .build_covenants_protocol_spend(&params)
//         .unwrap();

//     // Verify PSBT structure
//     assert_eq!(psbt.inputs.len(), 1);
//     assert!(psbt.inputs[0].tap_scripts.len() > 0);
//     assert!(psbt.inputs[0].tap_key_origins.len() > 0);
// }
