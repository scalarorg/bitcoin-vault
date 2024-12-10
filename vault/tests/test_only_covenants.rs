#[cfg(test)]
mod common;

#[cfg(test)]
mod test_only_covenants {
    use bitcoin::{secp256k1::All, Amount, Psbt};
    use bitcoin_vault::{SignByKeyMap, Signing, TaprootTreeType, VaultManager};
    use bitcoincore_rpc::jsonrpc::base64;

    use crate::common::helper::log_tx_result;
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
                "unstaked_psbt: {:?}",
                unstaked_psbt.inputs[0].tap_script_sigs
            );
            println!(
                "unstaked_psbt: {:?}",
                unstaked_psbt.inputs[1].tap_script_sigs
            );
        }

        // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

        //  send unstaking tx
        let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();

        log_tx_result(&result);
    }
}
