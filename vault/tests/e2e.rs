use bitcoin::{secp256k1::All, NetworkKind, Psbt};
use bitcoin_vault::{SignByKeyMap, Signing, UnstakingType, VaultManager};

use crate::SUITE;

// cargo test --package bitcoin-vault --test mod -- e2e::test_staking --exact --show-output
#[test]
fn test_staking() {
    let suite = SUITE.lock().unwrap();
    let staking_tx = suite.prepare_staking_tx(None);
    println!("tx_id: {:?}", staking_tx.compute_txid());
}

// Note: if you want to test on testnet4, you need to set the network to testnet4 in the .env file, ssh <testnet4> -L 48332:127.0.0.1:48332

// cargo test --package bitcoin-vault --test mod -- e2e::test_user_protocol_unstaking --exact --show-output
#[test]
fn test_user_protocol_unstaking() {
    // prepare staking tx
    let suite = SUITE.lock().unwrap();
    let staking_tx = suite.prepare_staking_tx(None);

    // prepare unstaking tx
    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::UserProtocol, None);

    // sign unstaking psbt
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &suite.get_user_privkey_bytes(),
        NetworkKind::Test,
        false,
    )
    .unwrap();

    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &suite.get_protocol_privkey_bytes(),
        NetworkKind::Test,
        true,
    )
    .unwrap();

    //  send unstaking tx
    let result = suite.send_psbt(unstaked_psbt).unwrap();

    println!("unstaked tx result: {:?}", result);
}

// cargo test --package bitcoin-vault --test mod -- e2e::test_user_protocol_unstaking_with_flag --exact --show-output
#[test]
fn test_user_protocol_unstaking_with_flag() {
    // prepare staking tx
    let suite = SUITE.lock().unwrap();
    let staking_tx = suite.prepare_staking_tx(Some(true));

    // prepare unstaking tx
    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::UserProtocol, Some(true));

    // sign unstaking psbt
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &suite.get_user_privkey_bytes(),
        NetworkKind::Test,
        false,
    )
    .unwrap();

    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &suite.get_protocol_privkey_bytes(),
        NetworkKind::Test,
        true,
    )
    .unwrap();

    //  send unstaking tx
    let result = suite.send_psbt(unstaked_psbt).unwrap();

    println!("unstaked tx result: {:?}", result);
}

// cargo test --package bitcoin-vault --test mod -- e2e::test_covenants_user_unstaking --exact --show-output
#[test]
fn test_covenants_user_unstaking() {
    let suite = SUITE.lock().unwrap();
    let staking_tx = suite.prepare_staking_tx(None);
    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::CovenantsUser, None);

    // Sign with user key first
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &suite.get_user_privkey_bytes(),
        NetworkKind::Test,
        false,
    )
    .unwrap();

    // Sign with each covenant key in order
    for privkey_bytes in suite.get_covenant_privkeys() {
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            privkey_bytes.as_slice(),
            NetworkKind::Test,
            false,
        )
        .unwrap();
    }

    // Finalize the PSBT
    <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

    // Extract and send
    let result = suite.send_psbt(unstaked_psbt);
    println!("unstaked tx result: {:?}", result);
}

// cargo test --package bitcoin-vault --test mod -- e2e::test_covenants_user_unstaking_with_flag --exact --show-output
#[test]
fn test_covenants_user_unstaking_with_flag() {
    let suite = SUITE.lock().unwrap();
    let staking_tx = suite.prepare_staking_tx(Some(true));
    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::CovenantsUser, Some(true));

    // Sign with user key first
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &suite.get_user_privkey_bytes(),
        NetworkKind::Test,
        false,
    )
    .unwrap();

    // Sign with each covenant key in order
    for privkey_bytes in suite.get_covenant_privkeys() {
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            privkey_bytes.as_slice(),
            NetworkKind::Test,
            false,
        )
        .unwrap();
    }

    // Finalize the PSBT
    <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

    // Extract and send
    let result = suite.send_psbt(unstaked_psbt);
    println!("unstaked tx result: {:?}", result);
}

// cargo test --package bitcoin-vault --test mod -- e2e::test_covenants_protocol_unstaking --exact --show-output
#[test]
fn test_covenants_protocol_unstaking() {
    let suite = SUITE.lock().unwrap();
    let staking_tx = suite.prepare_staking_tx(None);
    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::CovenantsProtocol, None);

    // Sign with user key first
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &suite.get_protocol_privkey_bytes(),
        NetworkKind::Test,
        false,
    )
    .unwrap();

    // Sign with each covenant key in order
    for privkey_bytes in suite.get_covenant_privkeys() {
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            privkey_bytes.as_slice(),
            NetworkKind::Test,
            false,
        )
        .unwrap();
    }

    // Finalize the PSBT
    <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

    // Extract and send
    let result = suite.send_psbt(unstaked_psbt);
    println!(
        "protocol pubkey: {:?}",
        suite
            .get_protocol_pubkey()
            .inner
            .x_only_public_key()
            .0
            .to_string()
    );
    println!("unstaked tx result: {:?}", result);
}

// cargo test --package bitcoin-vault --test mod -- e2e::test_covenants_protocol_unstaking_with_flag --exact --show-output
#[test]
fn test_covenants_protocol_unstaking_with_flag() {
    let suite = SUITE.lock().unwrap();
    let staking_tx = suite.prepare_staking_tx(Some(true));
    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::CovenantsProtocol, Some(true));

    // Sign with user key first
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &suite.get_protocol_privkey_bytes(),
        NetworkKind::Test,
        false,
    )
    .unwrap();

    // Sign with each covenant key in order
    for privkey_bytes in suite.get_covenant_privkeys() {
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            privkey_bytes.as_slice(),
            NetworkKind::Test,
            false,
        )
        .unwrap();
    }

    // Finalize the PSBT
    <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

    // Extract and send
    let result = suite.send_psbt(unstaked_psbt);
    println!(
        "protocol pubkey: {:?}",
        suite
            .get_protocol_pubkey()
            .inner
            .x_only_public_key()
            .0
            .to_string()
    );
    println!("unstaked tx result: {:?}", result);
}

// cargo test --package bitcoin-vault --test mod -- e2e::test_only_covenants_unstaking --exact --show-output
#[test]
fn test_only_covenants_unstaking() {
    // prepare staking tx
    let suite = SUITE.lock().unwrap();
    let staking_tx = suite.prepare_staking_tx(Some(true));

    // prepare unstaking tx
    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::OnlyCovenants, Some(true));

    // Sign with each covenant key in order
    for privkey_bytes in suite.get_covenant_privkeys() {
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            privkey_bytes.as_slice(),
            NetworkKind::Test,
            false,
        )
        .unwrap();
    }

    // Finalize the PSBT
    <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

    //  send unstaking tx
    let result = suite.send_psbt(unstaked_psbt).unwrap();

    println!("unstaked tx result: {:?}", result);
}
