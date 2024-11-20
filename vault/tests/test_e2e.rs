use bitcoin::{
    hex::DisplayHex,
    key::rand,
    secp256k1::{ecdsa::Signature, All},
    taproot, Psbt, Script, Txid, XOnlyPublicKey,
};
use bitcoin_vault::{SignByKeyMap, Signing, TaprootTreeType, UnstakingType, VaultManager};
use rand::seq::SliceRandom;

use bitcoin::hashes::{sha256, Hash};

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
    use std::str::FromStr;

    let suite = TestSuite::new();

    let tx_id =
        Txid::from_str("69728b40d7739eaf50568e459bfd360632551c4870aaba7e5c3d86613233d654").unwrap();

    let staking_tx = suite.get_tx_by_id(&tx_id).unwrap();

    // let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::ManyBranchNoCovenants, None);

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

    println!("\n=== User signed ===");
    println!(
        "user privkey: {:?}",
        suite.user_privkey().to_bytes().to_lower_hex_string()
    );
    println!(
        "x only pubkey: {:?}\n\n",
        suite
            .user_pubkey()
            .inner
            .x_only_public_key()
            .to_owned()
            .0
            .to_string()
    );

    let signing = [0, 1, 2];
    let unsigning = [3, 4];

    let privkeys = suite.covenant_privkeys().clone();
    let pubkeys = suite.covenant_pubkeys().clone();
    let key_pairs: Vec<_> = privkeys.iter().zip(pubkeys.iter()).collect();

    let signing = signing
        .iter()
        .map(|i| key_pairs[*i].clone())
        .collect::<Vec<_>>();

    let unsigning = unsigning
        .iter()
        .map(|i| key_pairs[*i].clone())
        .collect::<Vec<_>>();

    for (privkey, pubkey) in signing {
        let result = <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            privkey.as_slice(),
            suite.network_id(),
            false,
        );

        match result {
            Ok(_) => {
                println!("=== Signing with ===");
                println!("privkey: {:?}", privkey.to_lower_hex_string());
                println!(
                    "x only pubkey: {:?}\n\n",
                    pubkey.inner.x_only_public_key().to_owned().0.to_string()
                );
            }
            Err(e) => println!(
                "error signing with: {:?}, error: {:?}",
                privkey.to_lower_hex_string(),
                e
            ),
        }
    }

    // fa01ad2fe8ea728a371ae098eb53b359de0b1507a2d45bffd4c6d7e74a4942af
    println!("len: {:?}", unstaked_psbt.inputs[0].tap_script_sigs.len());
    for ((pubkey, leaf_hash), sig) in unstaked_psbt.inputs[0].tap_script_sigs.iter() {
        println!("\n\n");
        println!("x only pubkey: {:?}", pubkey.to_string());
        println!("leaf_hash: {:?}", leaf_hash);
        println!("sig: {:?}", sig);
    }

    let leaf_hash = unstaked_psbt.inputs[0]
        .tap_script_sigs
        .keys()
        .next()
        .unwrap()
        .1;

    let binding = unstaked_psbt.inputs[0].tap_script_sigs.clone();
    let invalid_sig = binding.values().next().unwrap();
    // let invalid_sig = taproot::Signature::from_slice(&[0u8; 64]).unwrap();

    for (privkey, pubkey) in unsigning {
        // Create an invalid but well-formed Schnorr signature
        // R = G (generator point), s = 1

        unstaked_psbt.inputs[0].tap_script_sigs.insert(
            (
                XOnlyPublicKey::from_slice(&pubkey.to_bytes()[1..]).unwrap(),
                leaf_hash,
            ),
            invalid_sig.clone(),
        );
    }

    println!("After: \n\n\n");

    for ((pubkey, leaf_hash), sig) in unstaked_psbt.inputs[0].tap_script_sigs.iter() {
        println!("\n\n");
        println!("pubkey: {:?}", pubkey.to_string());
        println!("leaf_hash: {:?}", leaf_hash);
        println!("sig: {:?}", sig);
    }

    // // Finalize the PSBT
    <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

    // println!("\n\n === unstaked_psbt====\n\n{:?}", unstaked_psbt);

    // // Extract and send
    let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();
    // log_tx_result(&result);
}

// cargo test --package bitcoin-vault --test mod -- test_e2e::test_covenants_user_unstaking --exact --show-output
#[test]
fn test_full_covenants_user_unstaking() {
    let suite = TestSuite::new();
    let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::ManyBranchNoCovenants, None);

    let suite = TestSuite::new();

    // use std::str::FromStr;
    // let tx_id =
    //     Txid::from_str("69728b40d7739eaf50568e459bfd360632551c4870aaba7e5c3d86613233d654").unwrap();

    // let staking_tx = suite.get_tx_by_id(&tx_id).unwrap();

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

    for privkey in suite.covenant_privkeys() {
        let result = <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            privkey.as_slice(),
            suite.network_id(),
            false,
        );

        match result {
            Ok(_) => {
                println!("=== Signing with ===");
                println!("privkey: {:?}", privkey.to_lower_hex_string());
            }
            Err(e) => println!(
                "error signing with: {:?}, error: {:?}",
                privkey.to_lower_hex_string(),
                e
            ),
        }
    }

    // // Finalize the PSBT
    <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

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

    // Sign with each covenant key in order
    for privkey_bytes in suite.covenant_privkeys() {
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
#[test]
fn test_only_covenants_unstaking() {
    // prepare staking tx
    let suite = TestSuite::new();
    let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::ManyBranchWithCovenants, None);

    // prepare unstaking tx
    let mut unstaked_psbt =
        suite.build_unstaking_tx(&staking_tx, UnstakingType::OnlyCovenants, Some(true));

    // Sign with each covenant key in order
    for privkey_bytes in suite.covenant_privkeys() {
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

    //  send unstaking tx
    let result = suite.send_psbt_by_rpc(unstaked_psbt).unwrap();

    log_tx_result(&result);
}
