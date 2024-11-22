use bitcoin::hex::DisplayHex;
use bitcoin_vault::LockingScriptWithOnlyCovenantsParams;

use crate::TestSuite;

// cargo test --package bitcoin-vault --test mod -- test_scripts::test_only_covenants_locking_script --exact --show-output
#[test]
fn test_only_covenants_locking_script() {
    let suite = TestSuite::new();
    let script = suite
        .manager
        .only_covenants_locking_script(&LockingScriptWithOnlyCovenantsParams {
            covenant_pub_keys: &suite.covenant_x_only_pubkeys(),
            covenant_quorum: 3,
        })
        .unwrap();

    println!(
        "script: {:?}",
        script.into_script().to_bytes().to_lower_hex_string()
    );
}
