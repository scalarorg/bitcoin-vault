#[cfg(test)]
mod test_csv {

    use std::str::FromStr;

    use bitcoin::{secp256k1::All, Amount, OutPoint, Psbt, ScriptBuf, Txid};
    use bitcoincore_rpc::jsonrpc::base64;
    use vault::{
        get_approvable_utxos, log_tx_result, AccountEnv, DestinationInfo, DestinationInfoEnv,
        PreviousOutpoint, SignByKeyMap, Signing, SuiteAccount, TaprootTreeType, TestSuite,
        TimeGatedUnlockingType, VaultManager,
    };

    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_SUITE: TestSuite = TestSuite::new_with_loaded_env("PEPE");
        static ref TEST_ACCOUNT: SuiteAccount =
            SuiteAccount::new(AccountEnv::new(TEST_SUITE.env_path()).unwrap());
        static ref TEST_DESTINATION_INFO: DestinationInfo =
            DestinationInfo::new(DestinationInfoEnv::new(TEST_SUITE.env_path()).unwrap());
    }

    #[test]
    fn test_locking() {
        let amount = 10000;
        let sequence = 2;
        let utxos = get_approvable_utxos(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), amount).unwrap();

        let staking_tx = TEST_SUITE
            .prepare_staking_tx(
                amount,
                TaprootTreeType::CustodianOnly,
                TEST_ACCOUNT.clone(),
                TEST_DESTINATION_INFO.clone(),
                utxos,
            )
            .unwrap();

        let output =
            TEST_SUITE.build_time_gated_locking_output(TEST_ACCOUNT.public_key(), amount, sequence);

        let mut unstaked_psbt = TEST_SUITE
            .build_batch_custodian_only_unstaking_tx(&[staking_tx], output.into_tx_outs());

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
        let result = TEST_SUITE.send_psbt_by_rpc(unstaked_psbt).unwrap().unwrap();

        println!("\n====== TIME GATED TX RESULT ======\n");
        log_tx_result(&result);
    }

    #[test]
    fn test_unlocking() {
        // read tx from env
        let txid = std::env::var("TIME_GATED_TXID").unwrap();
        let txid = Txid::from_str(&txid).unwrap();
        let amount_in_sats = Amount::from_sat(9737);
        let sequence = 2;
        let script_pubkey = ScriptBuf::from_hex(
            "5120ca3cce151096b695132aa1a410bf9125d34fc050f0d8428edcdb280653a2cfae",
        )
        .unwrap();

        let input = PreviousOutpoint {
            amount_in_sats,
            outpoint: OutPoint { txid, vout: 1 },
            script_pubkey,
        };

        println!("Input: {:?}", input);

        let mut unstaked_psbt = TEST_SUITE.build_time_gated_unlocking_psbt(
            input,
            TEST_ACCOUNT.public_key(),
            TEST_ACCOUNT.address().script_pubkey(),
            sequence,
            TimeGatedUnlockingType::PartyTimeGated,
        );

        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            TEST_ACCOUNT.private_key().to_bytes().as_slice(),
            TEST_SUITE.network_id(),
            true,
        )
        .unwrap();

        println!("b64: {}", base64::encode(&unstaked_psbt.serialize()));
        let result = TEST_SUITE.send_psbt_by_rpc(unstaked_psbt).unwrap().unwrap();

        println!("\n====== UNLOCKING TIME GATED TX RESULT ======\n");
        log_tx_result(&result);
    }
}
