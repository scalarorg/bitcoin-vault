#[cfg(test)]
mod test_custodians {
    use std::str::FromStr;

    use bitcoin::key::Secp256k1;
    use bitcoin::{secp256k1::All, Amount, Psbt};
    use bitcoin::{Address, OutPoint, TxOut, Txid};
    use bitcoincore_rpc::jsonrpc::base64;
    use bitcoincore_rpc::RawTx;
    use rust_mempool::MempoolClient;
    use vault::helper::{get_adress, key_from_wif, log_tx_result};
    use vault::{
        get_approvable_utxos, get_network_from_str, AccountEnv, CustodianOnly,
        CustodianOnlyUnlockingParams, DestinationInfo, DestinationInfoEnv, NeededUtxo,
        PreviousOutpoint, SignByKeyMap, Signing, SuiteAccount, TaprootTreeType, TestSuite,
        VaultManager, HASH_SIZE,
    };

    use futures::future::join_all;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_SUITE: TestSuite = TestSuite::new_with_loaded_env("PEPE");
        static ref TEST_ACCOUNT: SuiteAccount =
            SuiteAccount::new(AccountEnv::new(TEST_SUITE.env_path()).unwrap());
        static ref TEST_DESTINATION_INFO: DestinationInfo =
            DestinationInfo::new(DestinationInfoEnv::new(TEST_SUITE.env_path()).unwrap());
    }

    #[test]
    fn test_staking() {
        let utxos = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 2000).unwrap();
        let staking_tx = TEST_SUITE.prepare_staking_tx(
            2000,
            TaprootTreeType::CustodianOnly,
            TEST_ACCOUNT.clone(),
            TEST_DESTINATION_INFO.clone(),
            utxos,
        );
        println!("tx_id: {:?}", staking_tx.unwrap().compute_txid());
    }

    #[test]
    fn test_basic_flow() {
        let utxo = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 10000).unwrap();
        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                10000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxo,
            )
            .unwrap();

        let mut unstaked_psbt = TEST_SUITE.build_batch_custodian_only_unstaking_tx(
            &[staking_tx],
            vec![TxOut {
                value: Amount::from_sat(8000),
                script_pubkey: TEST_ACCOUNT.address().script_pubkey(),
            }],
        );

        let psbt_base64 = base64::encode(unstaked_psbt.serialize());
        let psbt_hex = hex::encode(unstaked_psbt.serialize());

        println!("=== UNSTAKED PSBT ===");
        println!("psbt_base64: {}", psbt_base64);
        println!("psbt_hex: {}", psbt_hex);

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

        // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

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
    fn test_partial_unstaking() {
        let utxos = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 10000).unwrap();
        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                10000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos,
            )
            .unwrap();

        let mut unstaked_psbt = TEST_SUITE.build_batch_custodian_only_unstaking_tx(
            &[staking_tx],
            vec![TxOut {
                value: Amount::from_sat(8000),
                script_pubkey: TEST_ACCOUNT.address().script_pubkey(),
            }],
        );

        let psbt_base64 = base64::encode(unstaked_psbt.serialize());
        println!("psbt_base64: {}", psbt_base64);

        let psbt_hex = hex::encode(unstaked_psbt.serialize());
        println!("psbt_hex: {}", psbt_hex);

        let signing_privkeys = TEST_SUITE.pick_random_custodian_privkeys();

        println!("signing_privkeys: {:?}", signing_privkeys);

        for privkey in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey.as_slice(),
                TEST_SUITE.network_id(),
                false,
            )
            .unwrap();

            println!(
                "unstaked_psbt: {:?}",
                unstaked_psbt.inputs[0].tap_script_sigs
            );
        }

        // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

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
    fn test_partial_unstaking_multiple_utxos() {
        let utxos = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 10000).unwrap();

        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                10000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos,
            )
            .unwrap();

        let utxos2 = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 3000).unwrap();

        let staking_tx2 = TEST_SUITE
            .prepare_staking_tx(
                3000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos2,
            )
            .unwrap();

        let mut unstaked_psbt = TEST_SUITE.build_batch_custodian_only_unstaking_tx(
            &[staking_tx, staking_tx2],
            vec![TxOut {
                value: Amount::from_sat(7000),
                script_pubkey: TEST_ACCOUNT.address().script_pubkey(),
            }],
        );

        let psbt_base64 = base64::encode(unstaked_psbt.serialize());
        println!("psbt_base64: {}", psbt_base64);

        let psbt_hex = hex::encode(unstaked_psbt.serialize());
        println!("psbt_hex: {}", psbt_hex);

        let signing_privkeys = TEST_SUITE.pick_random_custodian_privkeys();

        println!("signing_privkeys: {:?}", signing_privkeys);

        for privkey in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey.as_slice(),
                TEST_SUITE.network_id(),
                false,
            )
            .unwrap();

            println!(
                "unstaked_psbt[0]: {:?}\n",
                unstaked_psbt.inputs[0].tap_script_sigs
            );
            println!(
                "unstaked_psbt[1]: {:?} \n",
                unstaked_psbt.inputs[1].tap_script_sigs
            );
        }

        // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

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
    fn test_parallel_signing_multiple_utxos() {
        use std::sync::mpsc;
        use std::thread;

        // Create multiple staking transactions (inputs)
        let staking_txs: Vec<_> = (0..2)
            .map(|_| {
                let utxos =
                    get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 100000).unwrap();
                TEST_SUITE
                    .prepare_staking_tx(
                        100000,
                        TaprootTreeType::CustodianOnly,
                        TEST_ACCOUNT.clone(),
                        TEST_DESTINATION_INFO.clone(),
                        utxos,
                    )
                    .unwrap()
            })
            .collect();

        let another_address = match TEST_SUITE.env().network.as_str() {
            "testnet4" => "tb1p5hpkty3ykt92qx6m0rastprnreqx6dqexagg8mgp3hgz53p9lk3qd2c4f2",
            "regtest" => "bcrt1qwu0w6haezr25hmgqm5una9f8vdjk9fk363d59c",
            _ => panic!("Unknown network"),
        };

        let another_address = get_adress(&TEST_SUITE.env().network, another_address);

        println!("script_pubkey: {:?}", another_address.script_pubkey());

        // Create the original unsigned PSBT
        let original_psbt: Psbt = TEST_SUITE.build_batch_custodian_only_unstaking_tx(
            &staking_txs,
            vec![
                TxOut {
                    value: Amount::from_sat(7000),
                    script_pubkey: TEST_ACCOUNT.address().script_pubkey(),
                },
                TxOut {
                    value: Amount::from_sat(4000),
                    script_pubkey: another_address.script_pubkey(),
                },
            ],
        );

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

        println!(
            "\nfinal_psbt[0]: {:?}",
            final_psbt.inputs[0].tap_script_sigs
        );
        println!(
            "\nfinal_psbt[1]: {:?}",
            final_psbt.inputs[1].tap_script_sigs
        );

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

    #[test]
    fn test_sign_wrong_pubkey() {
        let utxos = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 1000).unwrap();

        let secp = Secp256k1::new();

        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                1000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos,
            )
            .unwrap();

        let utxos2 = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 1000).unwrap();

        let staking_tx2 = TEST_SUITE
            .prepare_staking_tx(
                1000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos2,
            )
            .unwrap();

        let mut unstaked_psbt = TEST_SUITE.build_batch_custodian_only_unstaking_tx(
            &[staking_tx, staking_tx2],
            vec![TxOut {
                value: Amount::from_sat(2000),
                script_pubkey: TEST_ACCOUNT.address().script_pubkey(),
            }],
        );

        let psbt_base64 = base64::encode(unstaked_psbt.serialize());
        println!("psbt_base64: {}", psbt_base64);

        let psbt_hex = hex::encode(unstaked_psbt.serialize());
        println!("psbt_hex: {}", psbt_hex);

        let mut signing_privkeys = TEST_SUITE.pick_random_custodian_privkeys();

        let wif = "cNGbmJbymnzaFUPZ8XSLvQQxHEEcTkh1ojBMMpvg5vFX5V1afcmR";

        let wrong_key = key_from_wif(wif, &secp);

        println!("before signing_privkeys: {:?}", signing_privkeys);

        signing_privkeys[0] = wrong_key.0.to_bytes();

        println!("after signing_privkeys: {:?}", signing_privkeys);

        for privkey in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey.as_slice(),
                TEST_SUITE.network_id(),
                false,
            )
            .unwrap();

            println!(
                "unstaked_psbt[0]: {:?}\n",
                unstaked_psbt.inputs[0].tap_script_sigs
            );
            println!(
                "unstaked_psbt[1]: {:?} \n",
                unstaked_psbt.inputs[1].tap_script_sigs
            );
        }

        println!(
            "length of tap_script_sigs[0]: {:?}",
            unstaked_psbt.inputs[0].tap_script_sigs.len()
        );
        println!(
            "length of tap_script_sigs[1]: {:?}",
            unstaked_psbt.inputs[1].tap_script_sigs.len()
        );
        // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

        let psbt_base64 = base64::encode(unstaked_psbt.serialize());
        println!("psbt_base64: {}", psbt_base64);

        let psbt_hex = hex::encode(unstaked_psbt.serialize());
        println!("psbt_hex: {}", psbt_hex);

        //  send unstaking tx
        // let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();

        // log_tx_result(&result);
    }

    #[test]
    fn test_collect_all_available_utxos() {
        use electrum_client::{Client, ElectrumApi};

        let _: Result<(), Box<dyn std::error::Error>> = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                const VOUT: u32 = 1;
                const LIMIT: usize = 500;

                let script = <VaultManager as CustodianOnly>::locking_script(
                    &TEST_SUITE.custodian_pubkeys(),
                    TEST_SUITE.env().custodian_quorum,
                )
                .unwrap();

                let network = get_network_from_str(&TEST_SUITE.env().network);
                let address = Address::from_script(&script.clone().into_script(), network).unwrap();

                println!("address: {:?}", address);

                let client = Client::new("tcp://127.0.0.1:60001").unwrap();

                let utxos = client.script_list_unspent(&script.into_script()).unwrap();

                let mut utxos = utxos
                    .iter()
                    .filter(|utxo| utxo.height > 85000)
                    .collect::<Vec<_>>();

                // sort ascending by block height
                utxos.reverse();

                let utxos = utxos
                    .into_iter()
                    .map(|utxo| NeededUtxo {
                        txid: utxo.tx_hash,
                        vout: utxo.tx_pos as u32,
                        amount: Amount::from_sat(utxo.value),
                    })
                    .collect::<Vec<_>>();

                let batch_futures = utxos.chunks(LIMIT).enumerate().map(|(i, utxos_chunk)| {
                    // Clone what you need inside the async block
                    let address = address.clone();
                    let test_account_address = TEST_ACCOUNT.address().clone();
                    let manager = TEST_SUITE.manager();
                    let custodian_pubkeys = TEST_SUITE.custodian_pubkeys();
                    let custodian_quorum = TEST_SUITE.env().custodian_quorum;
                    let network_id = TEST_SUITE.network_id();
                    let signing_privkeys = TEST_SUITE.pick_random_custodian_privkeys();
                    let client = Client::new("tcp://127.0.0.1:60001").unwrap();

                    let utxos_chunk: Vec<_> = utxos_chunk.to_vec();

                    tokio::spawn(async move {
                        let total: u64 = utxos_chunk.iter().map(|utxo| utxo.amount.to_sat()).sum();

                        println!("Batch {}: Processing {} utxos", i + 1, utxos_chunk.len());

                        let mut unstaked_psbt =
                            match <VaultManager as CustodianOnly>::build_unlocking_psbt(
                                &manager,
                                &CustodianOnlyUnlockingParams {
                                    inputs: utxos_chunk
                                        .iter()
                                        .map(|u| PreviousOutpoint {
                                            outpoint: OutPoint::new(u.txid, VOUT),
                                            amount_in_sats: u.amount,
                                            script_pubkey: address.script_pubkey(),
                                        })
                                        .collect(),
                                    outputs: vec![TxOut {
                                        value: Amount::from_sat(total),
                                        script_pubkey: test_account_address.script_pubkey(),
                                    }],
                                    custodian_pubkeys,
                                    custodian_quorum,
                                    fee_rate: 2,
                                    rbf: false,
                                    session_sequence: 0,
                                    custodian_group_uid: [0u8; HASH_SIZE],
                                },
                            ) {
                                Ok(psbt) => psbt,
                                Err(e) => {
                                    println!("Failed to build PSBT for batch {}: {}", i + 1, e);
                                    return;
                                }
                            };

                        for privkey in signing_privkeys {
                            let _ = <VaultManager as Signing>::sign_psbt_by_single_key(
                                &mut unstaked_psbt,
                                privkey.as_slice(),
                                network_id,
                                false,
                            )
                            .unwrap();
                        }

                        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

                        let finalized_tx = match unstaked_psbt.extract_tx() {
                            Ok(tx) => tx,
                            Err(e) => {
                                println!("Failed to extract tx for batch {}: {}", i + 1, e);
                                return;
                            }
                        };

                        match client.transaction_broadcast(&finalized_tx) {
                            Ok(tx_id) => {
                                println!("Batch {} tx_id: {:?}", i + 1, tx_id);
                            }
                            Err(e) => {
                                println!("Broadcast error for batch {}: {:?}", i + 1, e);
                            }
                        }
                    })
                });

                join_all(batch_futures).await;

                Ok(())
            });
    }
}
