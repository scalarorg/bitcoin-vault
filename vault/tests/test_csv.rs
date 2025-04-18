#[cfg(test)]
mod test_csv {
    use std::collections::BTreeMap;

    use bitcoin::{
        bip32::DerivationPath,
        hex::DisplayHex,
        opcodes::all::{OP_CHECKSIG, OP_CSV, OP_DROP, OP_ELSE, OP_ENDIF, OP_IF},
        psbt::Input,
        script::Builder,
        AddressType, Amount, OutPoint, Psbt, ScriptBuf, Sequence, TxOut, XOnlyPublicKey,
    };
    use vault::{
        get_approvable_utxo, log_tx_result, AccountEnv, BuildCustodianOnlyBranch, DestinationInfo,
        DestinationInfoEnv, PreviousStakingUTXO, Signing, SuiteAccount, TestSuite, UnstakingOutput,
        UnstakingTransactionBuilder, VaultManager,
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
        let lock_time: u16 = 0;
        let mut tx_builder = UnstakingTransactionBuilder::new(false);

        let utxo = get_approvable_utxo(&TEST_SUITE.rpc, &TEST_ACCOUNT.address(), 2000).unwrap();

        let script_pubkey = TEST_ACCOUNT.address().script_pubkey();
        // add inputs to builder

        // let custodians_x_only: Vec<XOnlyPublicKey> = TEST_SUITE
        //     .custodian_pubkeys()
        //     .iter()
        //     .map(|pk| XOnlyPublicKey::from(*pk))
        //     .collect();

        // let only_custodian_branch =
        //     <ScriptBuf as BuildCustodianOnlyBranch>::build(&custodians_x_only, 3).unwrap();
        // let only_custodian_branch_slice = only_custodian_branch.as_bytes().iter().as_slice();

        let x = TEST_ACCOUNT.address().script_pubkey();

        println!("x: {:?}", x.as_bytes().to_lower_hex_string());

        let data_slice: &[u8; 34] = x.as_bytes().try_into().unwrap();

        let script = Builder::new()
            .push_opcode(OP_IF)
            .push_int(lock_time as i64)
            .push_opcode(OP_CSV)
            .push_opcode(OP_DROP)
            .push_key(&TEST_ACCOUNT.public_key())
            .push_opcode(OP_CHECKSIG)
            .push_opcode(OP_ELSE)
            .push_slice([0u8; 30])
            .push_opcode(OP_ENDIF)
            .into_script();

        println!("script: {:?}", script);
        println!("utxo: {:?}", utxo);

        let output = TxOut {
            value: utxo.amount - Amount::from_sat(1000),
            script_pubkey: script.clone(),
        };

        tx_builder.add_input(OutPoint {
            txid: utxo.txid,
            vout: utxo.vout,
        });
        tx_builder.add_raw_output(output.clone());

        let unsigned_tx = tx_builder.build();

        let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();

        psbt.inputs[0] = Input {
            witness_utxo: Some(TxOut {
                value: utxo.amount,
                script_pubkey,
            }),

            // TODO: fix this, taproot address: leaf hash, no key origin
            // TODO: fix this, segwit address: no leaf hash, key origin
            tap_internal_key: match TEST_ACCOUNT.address().address_type() {
                Some(AddressType::P2tr) => Some(XOnlyPublicKey::from(TEST_ACCOUNT.public_key())),
                _ => None,
            },
            tap_key_origins: {
                let mut map = BTreeMap::new();
                // Note: no need leaf hash when staking
                map.insert(
                    XOnlyPublicKey::from(TEST_ACCOUNT.public_key()),
                    (vec![], ([0u8; 4].into(), DerivationPath::default())),
                );
                map
            },
            ..Default::default()
        };

        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut psbt,
            &TEST_ACCOUNT.private_key().to_bytes(),
            TEST_SUITE.network_id(),
            true,
        )
        .unwrap();

        let result = TestSuite::send_psbt(&TEST_SUITE.rpc, psbt)
            .unwrap()
            .unwrap();

        println!("result: {:?}", result);

        println!("====== FAUCET BTC ======");
        log_tx_result(&result);

        // spend the previout output

        let mut tx_builder = UnstakingTransactionBuilder::new(false);

        tx_builder.add_input(OutPoint {
            txid: result.txid,
            vout: 0,
        });
        tx_builder.add_raw_output(TxOut {
            script_pubkey: TEST_ACCOUNT.address().script_pubkey(),
            value: output.value - Amount::from_sat(1000),
        });

        let unsigned_tx = tx_builder.build();

        let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();

        psbt.inputs[0] = Input {
            witness_utxo: Some(TxOut {
                value: output.value,
                script_pubkey: script.clone(),
            }),

            // TODO: fix this, taproot address: leaf hash, no key origin
            // TODO: fix this, segwit address: no leaf hash, key origin
            tap_internal_key: match TEST_ACCOUNT.address().address_type() {
                Some(AddressType::P2tr) => Some(XOnlyPublicKey::from(TEST_ACCOUNT.public_key())),
                _ => None,
            },
            tap_key_origins: {
                let mut map = BTreeMap::new();
                // Note: no need leaf hash when staking
                map.insert(
                    XOnlyPublicKey::from(TEST_ACCOUNT.public_key()),
                    (vec![], ([0u8; 4].into(), DerivationPath::default())),
                );
                map
            },
            ..Default::default()
        };

        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut psbt,
            &TEST_ACCOUNT.private_key().to_bytes(),
            TEST_SUITE.network_id(),
            true,
        )
        .unwrap();

        let result = TestSuite::send_psbt(&TEST_SUITE.rpc, psbt)
            .unwrap()
            .unwrap();

        log_tx_result(&result);
    }
}
