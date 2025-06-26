#[cfg(test)]
mod test_upc {
    use bitcoin::{
        hex::DisplayHex, secp256k1::All, Address, Amount, Network, OutPoint, PrivateKey, Psbt,
        PublicKey, ScriptBuf, TxOut, Txid,
    };
    use rust_mempool::MempoolClient;
    use vault::{
        get_approvable_utxos, get_fee_rate, get_global_secp, helper::log_tx_result, AccountEnv,
        DestinationInfo, DestinationInfoEnv, PreviousOutpoint, SignByKeyMap, Signing, SuiteAccount,
        TaprootTreeType, TestSuite, UPCUnlockingParams, UPCUnlockingType, VaultManager, UPC,
    };

    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_SUITE: TestSuite = TestSuite::new_with_loaded_env("upc");
        static ref TEST_ACCOUNT: SuiteAccount =
            SuiteAccount::new(AccountEnv::new(TEST_SUITE.env_path()).unwrap());
        static ref TEST_DESTINATION_INFO: DestinationInfo =
            DestinationInfo::new(DestinationInfoEnv::new(TEST_SUITE.env_path()).unwrap());
    }

    #[test]
    fn test_staking() {
        let utxos = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 2000).unwrap();
        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                2000,
                TaprootTreeType::UPCBranch,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos,
            )
            .unwrap();
        println!("tx_id: {:?}", staking_tx.compute_txid());
    }

    #[test]
    fn test_user_protocol() {
        let utxos = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 2000).unwrap();
        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                2000,
                TaprootTreeType::UPCBranch,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos,
            )
            .unwrap();

        // prepare unstaking tx
        let mut unstaked_psbt = TEST_SUITE.build_upc_unstaking_tx(
            &staking_tx,
            UPCUnlockingType::UserProtocol,
            TEST_ACCOUNT.clone(),
            1000,
        );

        // sign unstaking psbt
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            &TEST_SUITE.protocol_privkey().to_bytes(),
            TEST_SUITE.network_id(),
            false,
        )
        .unwrap();

        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            &TEST_SUITE.protocol_privkey().to_bytes(),
            TEST_SUITE.network_id(),
            true,
        )
        .unwrap();

        //  send unstaking tx
        match TEST_SUITE.send_psbt_by_rpc(unstaked_psbt) {
            Ok(Some(result)) => {
                log_tx_result(&result);
            }
            Ok(None) => {
                panic!("tx not found");
            }
            Err(e) => {
                panic!("tx not found with error: {}", e);
            }
        }
    }

    #[test]
    fn test_custodian_user() {
        let utxos = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 3000).unwrap();
        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                2000,
                TaprootTreeType::UPCBranch,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos,
            )
            .unwrap();

        let mut unstaked_psbt = TEST_SUITE.build_upc_unstaking_tx(
            &staking_tx,
            UPCUnlockingType::CustodianUser,
            TEST_ACCOUNT.clone(),
            2000,
        );

        // Sign with user key first
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            &TEST_ACCOUNT.private_key().to_bytes(),
            TEST_SUITE.network_id(),
            false,
        )
        .unwrap();

        let unstaked_psbt_hex = hex::encode(unstaked_psbt.serialize());
        println!("unstaked_psbt_hex: {}", unstaked_psbt_hex);

        let signing_privkeys = TEST_SUITE.pick_random_custodian_privkeys();

        for privkey in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey.as_slice(),
                TEST_SUITE.network_id(),
                false,
            )
            .unwrap();
        }

        // // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

        // // Extract and send
        match TEST_SUITE.send_psbt_by_rpc(unstaked_psbt) {
            Ok(Some(result)) => {
                log_tx_result(&result);
            }
            Ok(None) => {
                panic!("tx not found");
            }
            Err(e) => {
                panic!("tx not found with error: {}", e);
            }
        }
    }

    // cargo test --package bitcoin-vault --test test_upc -- test_upc::test_custodian_protocol --exact --show-output
    #[test]
    fn test_custodian_protocol() {
        let utxos = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 2000).unwrap();
        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                2000,
                TaprootTreeType::UPCBranch,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos,
            )
            .unwrap();

        let mut unstaked_psbt = TEST_SUITE.build_upc_unstaking_tx(
            &staking_tx,
            UPCUnlockingType::CustodianProtocol,
            TEST_ACCOUNT.clone(),
            1000,
        );

        // Sign with user key first
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            &TEST_SUITE.protocol_privkey().to_bytes(),
            TEST_SUITE.network_id(),
            false,
        )
        .unwrap();

        let signing_privkeys = TEST_SUITE.pick_random_custodian_privkeys();

        println!("signing_privkeys: {:?}", signing_privkeys.len());

        // Sign with each custodian key in order
        for privkey_bytes in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey_bytes.as_slice(),
                TEST_SUITE.network_id(),
                false,
            )
            .unwrap();
        }

        // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

        // Extract and send
        match TEST_SUITE.send_psbt_by_rpc(unstaked_psbt) {
            Ok(Some(result)) => {
                log_tx_result(&result);
            }
            Ok(None) => {
                panic!("tx not found");
            }
            Err(e) => {
                panic!("tx not found with error: {}", e);
            }
        }
    }

    #[test]
    fn test_parallel_custodian_user() {
        use std::sync::mpsc;
        use std::thread;

        let utxos = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 2000).unwrap();
        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                2000,
                TaprootTreeType::UPCBranch,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos,
            )
            .unwrap();

        let mut original_psbt = TEST_SUITE.build_upc_unstaking_tx(
            &staking_tx,
            UPCUnlockingType::CustodianUser,
            TEST_ACCOUNT.clone(),
            1000,
        );

        // Sign with user key first
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut original_psbt,
            &TEST_SUITE.protocol_privkey().to_bytes(),
            TEST_SUITE.network_id(),
            false,
        )
        .unwrap();
        // Get signing keys
        let signing_privkeys = TEST_SUITE.pick_random_custodian_privkeys();

        // Channel for collecting signatures
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        println!("\n🔐 ==== START SIGNING ==== 🔐");

        println!("psbt_hex: {:?}\n", hex::encode(original_psbt.serialize()));

        // Spawn a thread for each signing key
        for (_, privkey) in signing_privkeys.iter().enumerate() {
            let mut psbt_clone = original_psbt.clone();
            let privkey = privkey.clone();
            let tx = tx.clone();
            let network_id = TEST_SUITE.network_id();

            let handle = thread::spawn(move || {
                // Extract signatures for each input
                let input_tap_script_sigs =
                    <VaultManager as Signing>::sign_psbt_and_collect_tap_script_sigs(
                        &mut psbt_clone,
                        privkey.as_slice(),
                        network_id,
                    )
                    .unwrap();

                tx.send(input_tap_script_sigs).unwrap();
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        drop(tx);

        println!("\n🔐 ==== START AGGREGATING ==== 🔐");

        // Aggregate signatures into final PSBT
        let mut final_psbt: Psbt = original_psbt.clone();
        while let Ok(input_tap_script_sigs) = rx.recv() {
            println!("recv input_tap_script_sigs: {:?}", input_tap_script_sigs);
            <VaultManager as Signing>::aggregate_tap_script_sigs(
                &mut final_psbt,
                &input_tap_script_sigs,
            )
            .unwrap();
        }

        let psbt_bytes = final_psbt.serialize();
        let psbt_hex = hex::encode(psbt_bytes.clone());
        println!("\npsbt_hex: {}", psbt_hex);

        // Finalize and send
        <Psbt as SignByKeyMap<All>>::finalize(&mut final_psbt);
        match TEST_SUITE.send_psbt_by_rpc(final_psbt) {
            Ok(Some(result)) => {
                log_tx_result(&result);
            }
            Ok(None) => {
                panic!("tx not found");
            }
            Err(e) => {
                panic!("tx not found with error: {}", e);
            }
        }
        println!("🚀 ==== DONE ==== 🚀");
    }

    // #[tokio::test]
    // async fn test_unstake_with_staked_utxos() {
    //     use std::str::FromStr;

    //     let user_priv_key = std::env::var("USER_PRIVKEY").unwrap();
    //     let secp = get_global_secp();

    //     for priv_key in TEST_SUITE.env().custodian_private_keys.iter() {
    //         let priv_key = PrivateKey::from_wif(&priv_key).unwrap();
    //         // println!("priv_key: {:?}", priv_key.to_bytes().to_lower_hex_string());
    //         println!(
    //             "pub_key: {:?}",
    //             priv_key.public_key(secp).to_bytes().to_lower_hex_string()
    //         );
    //     }

    //     let user_privkey = PrivateKey::from_wif(&user_priv_key).unwrap();
    //     let user_pubkey = user_privkey.public_key(secp);
    //     println!(
    //         "user_pubkey: {:?}",
    //         user_pubkey.to_bytes().to_lower_hex_string()
    //     );

    //     println!(
    //         "user_address: {:?}",
    //         user_pubkey
    //             .p2wpkh_script_code()
    //             .unwrap()
    //             .to_bytes()
    //             .to_lower_hex_string()
    //     );

    //     println!(
    //         "protocol_pubkey: {:?}",
    //         TEST_SUITE
    //             .protocol_pubkey()
    //             .to_bytes()
    //             .to_lower_hex_string()
    //     );

    //     let protocol_pubkey = PublicKey::from_str(
    //         "03a9a3ec96a1051310a80ea9eaaed56cc68b5d7dbe3caa6f145014da88b897e9fa",
    //     )
    //     .unwrap();

    //     let script = <VaultManager as UPC>::locking_script(
    //         &user_pubkey,
    //         &protocol_pubkey,
    //         &TEST_SUITE.custodian_pubkeys(),
    //         TEST_SUITE.env().custodian_quorum,
    //     )
    //     .unwrap();

    //     let address =
    //         Address::from_script(&script.clone().into_script(), Network::Testnet4).unwrap();

    //     let mempool_client = MempoolClient::new(Network::Testnet4);

    //     println!("address: {:?}", address);

    //     let utxo = mempool_client
    //         .get_address_utxo(&address.to_string())
    //         .await
    //         .unwrap();

    //     // let utxo = &utxo[0];

    //     // println!("utxo: {:?}", utxo);

    //     use bitcoin::ScriptBuf;

    //     let user_script_pub_key =
    //         ScriptBuf::from_hex("00148b59bebf94c43703da1e70d0cd6041f006a18d2b").unwrap();

    //     println!(
    //         "user_script_pub_key: {:?}",
    //         user_script_pub_key.to_bytes().to_lower_hex_string()
    //     );

    //     let protocol_pubkey = PublicKey::from_str(
    //         "03a9a3ec96a1051310a80ea9eaaed56cc68b5d7dbe3caa6f145014da88b897e9fa",
    //     )
    //     .unwrap();

    //     let mut unstaked_psbt = <VaultManager as UPC>::build_unlocking_psbt(
    //         &TEST_SUITE.manager(),
    //         &UPCUnlockingParams {
    //             inputs: vec![PreviousOutpoint {
    //                 outpoint: OutPoint::new(
    //                     Txid::from_str(
    //                         "3e1f9009397aedfdad2456ac1295e746b166878f8f38e31c9a4c6d2050b8fa6d",
    //                     )
    //                     .unwrap(),
    //                     1 as u32,
    //                 ),
    //                 amount_in_sats: Amount::from_sat(20_000_000),
    //                 script_pubkey: script.clone().into_script(),
    //             }],
    //             output: TxOut {
    //                 value: Amount::from_sat(20_000_000),
    //                 script_pubkey: user_script_pub_key,
    //             },
    //             user_pubkey: user_pubkey,
    //             protocol_pubkey: protocol_pubkey,
    //             custodian_pubkeys: TEST_SUITE.custodian_pubkeys(),
    //             custodian_quorum: 3,
    //             fee_rate: get_fee_rate() * 5,
    //             rbf: true,
    //             typ: UPCUnlockingType::CustodianUser,
    //         },
    //     )
    //     .unwrap();

    //     // Sign with user key first
    //     <VaultManager as Signing>::sign_psbt_by_single_key(
    //         &mut unstaked_psbt,
    //         &user_privkey.to_bytes(),
    //         TEST_SUITE.network_id(),
    //         false,
    //     )
    //     .unwrap();

    //     let signing_privkeys = TEST_SUITE.pick_random_custodian_privkeys();

    //     println!("signing_privkeys: {:?}", signing_privkeys.len());

    //     // Sign with each custodian key in order
    //     for privkey_bytes in TEST_SUITE.custodian_privkeys() {
    //         <VaultManager as Signing>::sign_psbt_by_single_key(
    //             &mut unstaked_psbt,
    //             privkey_bytes.as_slice(),
    //             TEST_SUITE.network_id(),
    //             false,
    //         )
    //         .unwrap();
    //     }

    //     // Finalize the PSBT
    //     <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

    //     // Extract and send
    //     match TEST_SUITE.send_psbt_by_rpc(unstaked_psbt) {
    //         Ok(Some(result)) => {
    //             log_tx_result(&result);
    //         }
    //         Ok(None) => {
    //             panic!("tx not found");
    //         }
    //         Err(e) => {
    //             panic!("tx not found with error: {}", e);
    //         }
    //     }
    // }
}
