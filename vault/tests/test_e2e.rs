use bitcoin::{secp256k1::All, Psbt};
use bitcoin_vault::{SignByKeyMap, Signing, TaprootTreeType, UnstakingType, VaultManager};

use crate::{log_tx_result, suite::*};

// cargo test --package bitcoin-vault --test mod -- test_e2e::test_staking --exact --show-output
#[test]
fn test_staking() {
    let suite = TestSuite::new();
    let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::ManyBranchNoCovenants, None);
    println!("tx_id: {:?}", staking_tx.compute_txid());
}

// Note: if you want to test on testnet4, you need to set the network to testnet4 in the .env file, ssh <testnet4> -L 48332:127.0.0.1:48332

// cargo test --package bitcoin-vault --test mod -- test_e2e::test_user_protocol_unstaking --exact --show-output
#[test]
fn test_user_protocol_unstaking() {
    // prepare staking tx
    let suite = TestSuite::new();
    let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::ManyBranchNoCovenants, None);

    // prepare unstaking tx
    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::UserProtocol, None);

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

// cargo test --package bitcoin-vault --test mod -- test_e2e::test_covenants_user_unstaking --exact --show-output
#[test]
fn test_covenants_user_unstaking() {
    let suite = TestSuite::new();

    let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::ManyBranchNoCovenants, None);

    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::CovenantsUser, None);

    // Sign with user key first
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &suite.user_privkey().to_bytes(),
        suite.network_id(),
        false,
    )
    .unwrap();

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

    // // Finalize the PSBT
    <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

    // println!("\n\n === unstaked_psbt====\n\n{:?}", unstaked_psbt);

    // // Extract and send
    let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();
    log_tx_result(&result);
}

// cargo test --package bitcoin-vault --test mod -- test_e2e::test_covenants_protocol_unstaking --exact --show-output
#[test]
fn test_covenants_protocol_unstaking() {
    let suite = TestSuite::new();
    let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::ManyBranchNoCovenants, None);
    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::CovenantsProtocol, None);

    // Sign with user key first
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &suite.protocol_privkey().to_bytes(),
        suite.network_id(),
        false,
    )
    .unwrap();

    let signing_privkeys = suite.get_random_covenant_privkeys();

    println!("signing_privkeys: {:?}", signing_privkeys.len());

    // Sign with each covenant key in order
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

// cargo test --package bitcoin-vault --test mod -- test_e2e::test_only_covenants_unstaking --exact --show-output
// #[test]
// fn test_only_covenants_unstaking() {
//     // prepare staking tx
//     let suite = TestSuite::new();
//     let staking_tx =
//         suite.prepare_staking_tx(1000, TaprootTreeType::ManyBranchWithCovenants, Some(true));

//     // prepare unstaking tx
//     let mut unstaked_psbt =
//         suite.build_unstaking_tx(&staking_tx, UnstakingType::OnlyCovenants, Some(true));

//     let signing_privkeys = suite.get_random_covenant_privkeys();

//     println!("signing_privkeys: {:?}", signing_privkeys.len());

//     // Sign with each covenant key in order
//     for privkey_bytes in signing_privkeys {
//         <VaultManager as Signing>::sign_psbt_by_single_key(
//             &mut unstaked_psbt,
//             privkey_bytes.as_slice(),
//             suite.network_id(),
//             false,
//         )
//         .unwrap();
//     }

//     // Finalize the PSBT
//     <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

//     //  send unstaking tx
//     let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();

//     log_tx_result(&result);
// }
