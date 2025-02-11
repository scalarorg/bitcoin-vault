#[cfg(test)]
mod test_custodians {
    use bitcoin::key::Secp256k1;
    use bitcoin::{secp256k1::All, Amount, Psbt};
    use bitcoin_vault::helper::{get_adress, key_from_wif, log_tx_result};
    use bitcoin_vault::{
        AccountEnv, DestinationInfo, DestinationInfoEnv, SignByKeyMap, Signing, SuiteAccount,
        TaprootTreeType, TestSuite, UnstakingOutput, VaultManager,
    };
    use bitcoincore_rpc::jsonrpc::base64;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_SUITE: TestSuite = TestSuite::new();
        static ref TEST_ACCOUNT: SuiteAccount =
            SuiteAccount::new(AccountEnv::new(Some(TEST_SUITE.env_path())).unwrap());
        static ref TEST_DESTINATION_INFO: DestinationInfo =
            DestinationInfo::new(DestinationInfoEnv::new(Some(TEST_SUITE.env_path())).unwrap());
    }

    #[test]
    fn test_staking() {
        let staking_tx = TEST_SUITE.prepare_staking_tx(
            1000,
            TaprootTreeType::CustodianOnly,
            TEST_ACCOUNT.clone(),
            TEST_DESTINATION_INFO.clone(),
        );
        println!("tx_id: {:?}", staking_tx.unwrap().compute_txid());
    }

    #[test]
    fn test_basic_flow() {
        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                10000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
            )
            .unwrap();

        let mut unstaked_psbt = TEST_SUITE.build_batch_custodian_only_unstaking_tx(
            &[staking_tx],
            vec![UnstakingOutput {
                amount_in_sats: Amount::from_sat(8000),
                locking_script: TEST_ACCOUNT.address().script_pubkey(),
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
        let result = TEST_SUITE.send_psbt_by_rpc(unstaked_psbt).unwrap();

        log_tx_result(&result);
    }

    #[test]
    fn test_partial_unstaking() {
        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                100000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
            )
            .unwrap();

        let mut unstaked_psbt = TEST_SUITE.build_batch_custodian_only_unstaking_tx(
            &[staking_tx],
            vec![UnstakingOutput {
                amount_in_sats: Amount::from_sat(8000),
                locking_script: TEST_ACCOUNT.address().script_pubkey(),
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
        let result = TEST_SUITE.send_psbt_by_rpc(unstaked_psbt).unwrap();

        log_tx_result(&result);
    }

    #[test]
    fn test_partial_unstaking_multiple_utxos() {
        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                100000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
            )
            .unwrap();

        let staking_tx2 = TEST_SUITE
            .prepare_staking_tx(
                100000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
            )
            .unwrap();

        let mut unstaked_psbt = TEST_SUITE.build_batch_custodian_only_unstaking_tx(
            &[staking_tx, staking_tx2],
            vec![UnstakingOutput {
                amount_in_sats: Amount::from_sat(7000),
                locking_script: TEST_ACCOUNT.address().script_pubkey(),
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
        let result = TEST_SUITE.send_psbt_by_rpc(unstaked_psbt).unwrap();

        log_tx_result(&result);
    }

    #[test]
    fn test_parallel_signing_multiple_utxos() {
        use std::sync::mpsc;
        use std::thread;

        // Create multiple staking transactions (inputs)
        let staking_txs: Vec<_> = (0..2)
            .map(|_| {
                TEST_SUITE
                    .prepare_staking_tx(
                        100000,
                        TaprootTreeType::CustodianOnly,
                        TEST_ACCOUNT.clone(),
                        TEST_DESTINATION_INFO.clone(),
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
                UnstakingOutput {
                    amount_in_sats: Amount::from_sat(7000),
                    locking_script: TEST_ACCOUNT.address().script_pubkey(),
                },
                UnstakingOutput {
                    amount_in_sats: Amount::from_sat(4000),
                    locking_script: another_address.script_pubkey(),
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
        let result = TEST_SUITE.send_psbt_by_rpc(final_psbt).unwrap();
        log_tx_result(&result);
        println!("🚀 ==== DONE ==== 🚀");
    }

    #[test]
    fn test_sign_wrong_pubkey() {
        let secp = Secp256k1::new();

        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                100000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
            )
            .unwrap();

        let staking_tx2 = TEST_SUITE
            .prepare_staking_tx(
                100000,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
            )
            .unwrap();

        let mut unstaked_psbt = TEST_SUITE.build_batch_custodian_only_unstaking_tx(
            &[staking_tx, staking_tx2],
            vec![UnstakingOutput {
                amount_in_sats: Amount::from_sat(7000),
                locking_script: TEST_ACCOUNT.address().script_pubkey(),
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
}
