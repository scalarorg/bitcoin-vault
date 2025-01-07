#[cfg(test)]
mod common;

#[cfg(test)]
mod test_only_covenants {
    use bitcoin::hex::DisplayHex;
    use bitcoin::{secp256k1::All, Amount, Psbt};
    use bitcoin_vault::{SignByKeyMap, Signing, TaprootTreeType, VaultManager};
    use bitcoincore_rpc::jsonrpc::base64;

    use crate::common::helper::{key_from_wif, log_tx_result};
    use crate::common::TestSuite;

    // cargo test --package bitcoin-vault --test test_only_covenants -- test_only_covenants::test_staking --exact --show-output
    #[test]
    fn test_staking() {
        let suite = TestSuite::new();
        let staking_tx =
            suite.prepare_staking_tx(1000, TaprootTreeType::OneBranchOnlyCovenants, None);
        println!("tx_id: {:?}", staking_tx.compute_txid());
    }

    // cargo test --package bitcoin-vault --test test_only_covenants -- test_only_covenants::test_e2e --exact --show-output
    #[test]
    fn test_e2e() {
        let suite = TestSuite::new();
        let staking_tx =
            suite.prepare_staking_tx(10000, TaprootTreeType::OneBranchOnlyCovenants, None);

        let mut unstaked_psbt = suite.build_only_covenants_unstaking_tx(&[staking_tx], None);

        let psbt_base64 = base64::encode(unstaked_psbt.serialize());
        println!("psbt_base64: {}", psbt_base64);

        let psbt_hex = hex::encode(unstaked_psbt.serialize());
        println!("psbt_hex: {}", psbt_hex);

        let signing_privkeys = suite.get_random_covenant_privkeys();

        for privkey in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey.as_slice(),
                suite.network_id(),
                false,
            )
            .unwrap();
        }

        // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

        //  send unstaking tx
        let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();

        log_tx_result(&result);
    }

    // cargo test --package bitcoin-vault --test test_only_covenants -- test_only_covenants::test_partial_unstaking --exact --show-output
    #[test]
    fn test_partial_unstaking() {
        let suite = TestSuite::new();
        let staking_tx =
            suite.prepare_staking_tx(100000, TaprootTreeType::OneBranchOnlyCovenants, None);

        let mut unstaked_psbt =
            suite.build_only_covenants_unstaking_tx(&[staking_tx], Some(Amount::from_sat(8000)));

        let psbt_base64 = base64::encode(unstaked_psbt.serialize());
        println!("psbt_base64: {}", psbt_base64);

        let psbt_hex = hex::encode(unstaked_psbt.serialize());
        println!("psbt_hex: {}", psbt_hex);

        let signing_privkeys = suite.get_random_covenant_privkeys();

        println!("signing_privkeys: {:?}", signing_privkeys);

        for privkey in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey.as_slice(),
                suite.network_id(),
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
        let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();

        log_tx_result(&result);
    }

    // cargo test --package bitcoin-vault --test test_only_covenants -- test_only_covenants::test_partial_unstaking_multiple_utxos --exact --show-output
    #[test]
    fn test_partial_unstaking_multiple_utxos() {
        let suite = TestSuite::new();
        let staking_tx =
            suite.prepare_staking_tx(100000, TaprootTreeType::OneBranchOnlyCovenants, None);

        let staking_tx2 =
            suite.prepare_staking_tx(100000, TaprootTreeType::OneBranchOnlyCovenants, None);

        let mut unstaked_psbt = suite.build_only_covenants_unstaking_tx(
            &[staking_tx, staking_tx2],
            Some(Amount::from_sat(7000)),
        );

        let psbt_base64 = base64::encode(unstaked_psbt.serialize());
        println!("psbt_base64: {}", psbt_base64);

        let psbt_hex = hex::encode(unstaked_psbt.serialize());
        println!("psbt_hex: {}", psbt_hex);

        let signing_privkeys = suite.get_random_covenant_privkeys();

        println!("signing_privkeys: {:?}", signing_privkeys);

        for privkey in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey.as_slice(),
                suite.network_id(),
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
        let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();

        log_tx_result(&result);
    }

    // cargo test --package bitcoin-vault --test test_only_covenants -- test_only_covenants::test_parallel_signing_multiple_utxos --exact --show-output
    #[test]
    fn test_parallel_signing_multiple_utxos() {
        use std::sync::mpsc;
        use std::thread;

        let suite = TestSuite::new();

        // Create multiple staking transactions (inputs)
        let staking_txs: Vec<_> = (0..2)
            .map(|_| {
                suite.prepare_staking_tx(100000, TaprootTreeType::OneBranchOnlyCovenants, None)
            })
            .collect();

        // Create the original unsigned PSBT
        let original_psbt =
            suite.build_only_covenants_unstaking_tx(&staking_txs, Some(Amount::from_sat(7000)));

        // Get signing keys
        let signing_privkeys = suite.get_random_covenant_privkeys();

        // Channel for collecting signatures
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        println!("\n🔐 ==== START SIGNING ==== 🔐");

        println!("psbt_hex: {:?}", hex::encode(original_psbt.serialize()));

        let signing_privkeys = signing_privkeys[0].clone();
        let signing_privkeys = [signing_privkeys];

        // Spawn a thread for each signing key
        for (_, privkey) in signing_privkeys.iter().enumerate() {
            let mut psbt_clone = original_psbt.clone();
            let privkey = privkey.clone();
            let tx = tx.clone();
            let network_id = suite.network_id();

            let handle = thread::spawn(move || {
                println!("privkey: {:?}", privkey.to_lower_hex_string());
                // Extract signatures for each input
                let input_tap_script_sigs =
                    <VaultManager as Signing>::sign_psbt_and_collect_tap_script_sigs(
                        &mut psbt_clone,
                        privkey.as_slice(),
                        network_id,
                    )
                    .unwrap();

                for sig in input_tap_script_sigs.iter() {
                    let sed = sig.serialize().unwrap();
                    println!("key: {:?}", sed.key.to_lower_hex_string());
                    println!("leaf_hash: {:?}", sed.leaf_hash.to_lower_hex_string());
                    println!("signature: {:?}", sed.signature.to_lower_hex_string());
                }

                tx.send(input_tap_script_sigs).unwrap();
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        drop(tx);

        println!("🔐 ==== START AGGREGATING ==== 🔐");

        // Aggregate signatures into final PSBT
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
        println!("psbt_hex: {}", psbt_hex);
        println!("psbt_bytes: {:?}", psbt_bytes);

        // Finalize and send
        <Psbt as SignByKeyMap<All>>::finalize(&mut final_psbt);
        let result = suite.send_psbt_by_rpc(final_psbt).unwrap();
        log_tx_result(&result);
        println!("🚀 ==== DONE ==== 🚀");
    }

    // cargo test --package bitcoin-vault --test test_only_covenants -- test_only_covenants::test_sign_wrong_pubkey --exact --show-output
    #[test]
    fn test_sign_wrong_pubkey() {
        let suite = TestSuite::new();
        let staking_tx =
            suite.prepare_staking_tx(100000, TaprootTreeType::OneBranchOnlyCovenants, None);

        let staking_tx2 =
            suite.prepare_staking_tx(100000, TaprootTreeType::OneBranchOnlyCovenants, None);

        let mut unstaked_psbt = suite.build_only_covenants_unstaking_tx(
            &[staking_tx, staking_tx2],
            Some(Amount::from_sat(7000)),
        );

        let psbt_base64 = base64::encode(unstaked_psbt.serialize());
        println!("psbt_base64: {}", psbt_base64);

        let psbt_hex = hex::encode(unstaked_psbt.serialize());
        println!("psbt_hex: {}", psbt_hex);

        let mut signing_privkeys = suite.get_random_covenant_privkeys();

        let wif = "cNGbmJbymnzaFUPZ8XSLvQQxHEEcTkh1ojBMMpvg5vFX5V1afcmR";

        let wrong_key = key_from_wif(wif, suite.manager.secp());

        println!("before signing_privkeys: {:?}", signing_privkeys);

        signing_privkeys[0] = wrong_key.0.to_bytes();

        println!("after signing_privkeys: {:?}", signing_privkeys);

        for privkey in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey.as_slice(),
                suite.network_id(),
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
