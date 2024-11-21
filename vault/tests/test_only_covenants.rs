use bitcoin::{secp256k1::All, Psbt};
use bitcoin_vault::{SignByKeyMap, Signing, TaprootTreeType, VaultManager};

use crate::{log_tx_result, TestSuite};

// cargo test --package bitcoin-vault --test mod -- test_only_covenants::test_e2e --exact --show-output
#[test]
fn test_e2e() {
    let suite = TestSuite::new();
    let staking_tx = suite.prepare_staking_tx(1000, TaprootTreeType::OneBranchOnlyCovenants, None);

    let mut unstaked_psbt = suite.build_only_covenants_unstaking_tx(&staking_tx);

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
