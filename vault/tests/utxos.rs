use crate::SUITE;

//cargo test --package bitcoin-vault --test mod -- utxos::tets_list_utxos --exact --show-output
#[test]
fn tets_list_utxos() {
    let utxo = SUITE.get_approvable_utxos(1911);
    println!("utxo: {:?}", utxo);
}
