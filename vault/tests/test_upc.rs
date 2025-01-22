#[cfg(test)]
mod common;

#[cfg(test)]
use crate::common::helper::log_tx_result;
#[cfg(test)]
use crate::common::TestSuite;

#[cfg(test)]
mod test_upc {

    use bitcoin::{secp256k1::All, Psbt};
    use bitcoin_vault::{SignByKeyMap, Signing, TaprootTreeType, UnstakingType, VaultManager};

    use super::*;

    #[test]
    fn test_staking() {
        let suite = TestSuite::new();
        let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::UPCBranch);
        println!("tx_id: {:?}", staking_tx.compute_txid());
    }

    #[test]
    fn test_user_protocol() {
        let suite = TestSuite::new();
        let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::UPCBranch);

        // prepare unstaking tx
        let mut unstaked_psbt =
            suite.build_upc_unstaking_tx(&staking_tx, UnstakingType::UserProtocol);

        // sign unstaking psbt
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            &suite.user_privkey().to_bytes(),
            suite.network_id(),
            false,
        )
        .unwrap();

        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            &suite.protocol_privkey().to_bytes(),
            suite.network_id(),
            true,
        )
        .unwrap();

        //  send unstaking tx
        let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();

        log_tx_result(&result);
    }

    #[test]
    fn test_custodian_user() {
        let suite = TestSuite::new();

        let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::UPCBranch);

        let mut unstaked_psbt =
            suite.build_upc_unstaking_tx(&staking_tx, UnstakingType::CustodianUser);

        // Sign with user key first
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            &suite.user_privkey().to_bytes(),
            suite.network_id(),
            false,
        )
        .unwrap();

        let signing_privkeys = suite.pick_random_custodian_privkeys();

        for privkey in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey.as_slice(),
                suite.network_id(),
                false,
            )
            .unwrap();
        }

        // // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

        // println!("\n\n === unstaked_psbt====\n\n{:?}", unstaked_psbt);

        // // Extract and send
        let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();
        log_tx_result(&result);
    }

    // cargo test --package bitcoin-vault --test test_upc -- test_upc::test_custodian_protocol --exact --show-output
    #[test]
    fn test_custodian_protocol() {
        let suite = TestSuite::new();
        let staking_tx = suite.prepare_staking_tx(10000, TaprootTreeType::UPCBranch);
        let mut unstaked_psbt =
            suite.build_upc_unstaking_tx(&staking_tx, UnstakingType::CustodianProtocol);

        // Sign with user key first
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            &suite.protocol_privkey().to_bytes(),
            suite.network_id(),
            false,
        )
        .unwrap();

        let signing_privkeys = suite.pick_random_custodian_privkeys();

        println!("signing_privkeys: {:?}", signing_privkeys.len());

        // Sign with each custodian key in order
        for privkey_bytes in signing_privkeys {
            <VaultManager as Signing>::sign_psbt_by_single_key(
                &mut unstaked_psbt,
                privkey_bytes.as_slice(),
                suite.network_id(),
                false,
            )
            .unwrap();
        }

        // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

        // Extract and send
        let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();
        log_tx_result(&result);
    }

    #[test]
    fn test_parallel_custodian_user() {
        use std::sync::mpsc;
        use std::thread;

        let suite = TestSuite::new();

        let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::UPCBranch);

        let mut original_psbt =
            suite.build_upc_unstaking_tx(&staking_tx, UnstakingType::CustodianUser);

        // Sign with user key first
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut original_psbt,
            &suite.user_privkey().to_bytes(),
            suite.network_id(),
            false,
        )
        .unwrap();
        // Get signing keys
        let signing_privkeys = suite.pick_random_custodian_privkeys();

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
            let network_id = suite.network_id();

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
        let result = suite.send_psbt_by_rpc(final_psbt).unwrap();
        log_tx_result(&result);
        println!("🚀 ==== DONE ==== 🚀");
    }
}
