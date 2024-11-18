use bitcoin::{secp256k1::All, NetworkKind, Psbt};
use bitcoin_vault::{SignByKeyMap, Signing, VaultManager};

use crate::SUITE;

// cargo test --package bitcoin-vault --test mod -- only_covenants::test_e2e --exact --show-output
#[test]
fn test_e2e() {
    let suite = SUITE.lock().unwrap();
    let staking_tx = suite.prepare_only_covenants_staking_tx();
    println!("tx_id: {:?}", staking_tx.compute_txid());

    let mut unstaked_psbt = suite.build_only_covenants_unstaking_tx(&staking_tx);

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
