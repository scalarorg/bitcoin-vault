#[cfg(test)]
mod test_csv {

    use bitcoin::{
        key::Secp256k1,
        secp256k1::{schnorr::Signature, All, Message},
        sighash::SighashCache,
        taproot::{ControlBlock, LeafVersion},
        OutPoint, Psbt, ScriptBuf, TapLeafHash, XOnlyPublicKey,
    };
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
    fn test_simple() {
        let amount = 10000;
        let sequence = 0;
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

        let cloned_psbt = unstaked_psbt.clone();

        // Finalize the PSBT
        <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

        //  send unstaking tx
        let result = TEST_SUITE.send_psbt_by_rpc(unstaked_psbt).unwrap().unwrap();

        println!("\n====== TIME GATED TX RESULT ======\n");
        log_tx_result(&result);

        // ========== SPEND TIME GATED TX BY CUSTODIAN ONLY ===========
        let locked_vout = 1;

        let input = PreviousOutpoint {
            amount_in_sats: cloned_psbt.unsigned_tx.output[locked_vout].value,
            outpoint: OutPoint {
                txid: result.txid,
                vout: 1,
            },
            script_pubkey: cloned_psbt.unsigned_tx.output[locked_vout]
                .script_pubkey
                .clone(),
        };

        println!("Input: {:?}", input);

        let mut unstaked_psbt = TEST_SUITE.build_time_gated_unlocking_psbt(
            input,
            TEST_ACCOUNT.public_key(),
            TEST_ACCOUNT.address().script_pubkey(),
            sequence,
            TimeGatedUnlockingType::PartyTimeGated,
        );

        let signing_privkeys = TEST_SUITE.pick_random_custodian_privkeys();

        // for privkey in signing_privkeys {
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            TEST_ACCOUNT.private_key().to_bytes().as_slice(),
            TEST_SUITE.network_id(),
            true,
        )
        .unwrap();

        println!("b64: {}", base64::encode(&unstaked_psbt.serialize()));
        // println!("b64: {}", base64::encode(&unstaked_psbt.serialize()));
        // panic!("")
        // }

        // // Finalize the PSBT
        // <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

        //  send unstaking tx
        let result = TEST_SUITE.send_psbt_by_rpc(unstaked_psbt).unwrap().unwrap();

        // println!("\n====== UNLOCKING TIME GATED TX RESULT ======\n");
        log_tx_result(&result);
    }

    #[test]
    fn test_simple2() {
        // Parse the leaf script
        let leaf_script = ScriptBuf::from_hex(
            "00b275202ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350aad",
        )
        .unwrap();

        // Parse the control block
        let control_block_bytes = hex::decode("c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac05a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb").unwrap();
        let control_block = ControlBlock::decode(&control_block_bytes).unwrap();

        // Extract the internal key from the control block
        let internal_key = control_block.internal_key;

        // Extract the leaf version
        let leaf_version = control_block.leaf_version;

        println!("Internal Key: {:#?}", internal_key);
        println!("Leaf Version: {:#?}", leaf_version);

        // // Compute the tap leaf hash
        // Get taproot spending data
        let tap_leaf_hash = TapLeafHash::from_script(&leaf_script, LeafVersion::TapScript);
    }
}
