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

        let staking_tx = suite.prepare_staking_tx(10000, TaprootTreeType::UPCBranch);

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
}
