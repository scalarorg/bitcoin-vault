// use bitcoin::key::{Keypair, Secp256k1};
// use bitcoin::secp256k1::All;
// use bitcoin::{NetworkKind, PrivateKey, Psbt, PublicKey, TxOut};
// use bitcoin_vault::{BuildCovenantsProtocolSpendParams, Signing, Unstaking, VaultManager, UTXO};

// // Test covenant keys
// const COVENANT_1_PRIVKEY: [u8; 32] = [
//     1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
// ];

// const COVENANT_2_PRIVKEY: [u8; 32] = [
//     2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
//     2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2
// ];

// #[test]
// fn test_build_and_sign_covenants_protocol_spend() {
//     let secp = Secp256k1::new();
//     let manager = VaultManager::new(NetworkKind::Test);

//     // Create test keys
//     let protocol_keypair = Keypair::from_seckey_slice(&secp, &PROTOCOL_PRIVKEY_BYTES).unwrap();
//     let covenant1_keypair = Keypair::from_seckey_slice(&secp, &COVENANT_1_PRIVKEY).unwrap();
//     let covenant2_keypair = Keypair::from_seckey_slice(&secp, &COVENANT_2_PRIVKEY).unwrap();

//     let protocol_pubkey = PublicKey::from(protocol_keypair.public_key());
//     let covenant_pubkeys = vec![
//         PublicKey::from(covenant1_keypair.public_key()),
//         PublicKey::from(covenant2_keypair.public_key()),
//     ];

//     // Create test parameters
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
//         protocol_pub_key: &protocol_pubkey,
//         covenant_pubkeys: &covenant_pubkeys,
//         covenant_quorum: 2,
//     };

//     // Build PSBT
//     let mut psbt = manager.build_covenants_protocol_spend(&params).unwrap();

//     // Sign with covenant keys
//     let psbt_hex = VaultManager::sign_psbt_by_single_key(
//         &mut psbt,
//         &COVENANT_1_PRIVKEY,
//         NetworkKind::Test,
//         false,
//     )
//     .unwrap();

//     let psbt_hex = VaultManager::sign_psbt_by_single_key(
//         &mut psbt,
//         &COVENANT_2_PRIVKEY,
//         NetworkKind::Test,
//         false,
//     )
//     .unwrap();

//     // Finally sign with protocol key
//     let final_hex = VaultManager::sign_psbt_by_single_key(
//         &mut psbt,
//         &PROTOCOL_PRIVKEY_BYTES,
//         NetworkKind::Test,
//         true,
//     )
//     .unwrap();

//     // Verify the transaction is properly finalized
//     assert!(psbt.inputs[0].final_script_witness.is_some());
// }
