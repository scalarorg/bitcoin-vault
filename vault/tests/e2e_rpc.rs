use std::collections::BTreeMap;

use bitcoin::bip32::DerivationPath;
use bitcoin::psbt::Input;
use bitcoin::{
    absolute, transaction, NetworkKind, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
};
use bitcoin::{OutPoint, Psbt};
use bitcoin_vault::{Signing, Staking, UnstakingType, VaultManager};
use bitcoincore_rpc::json::{FinalizePsbtResult, WalletProcessPsbtResult};

use bitcoincore_rpc::Client;
use bitcoincore_rpc::{jsonrpc::base64, RpcApi};

use crate::{suite::TestSuite, MANAGER, SUITE};

pub trait RpcTestSuite {
    fn prepare_staking_tx(&self, have_only_covenants: Option<bool>) -> Transaction;

    fn process_psbt(&self, psbt: &Psbt) -> WalletProcessPsbtResult;

    fn process_and_broadcast_psbt(&self, psbt: &Psbt) -> Transaction;
}

impl<'a> RpcTestSuite for TestSuite<'a> {
    fn prepare_staking_tx(&self, have_only_covenants: Option<bool>) -> Transaction {
        let mut params = self.get_staking_params();
        params.have_only_covenants = have_only_covenants.unwrap_or(params.have_only_covenants);

        let utxo = self.get_approvable_utxos(self.get_staking_amount());

        let outputs = <VaultManager as Staking>::build(&MANAGER, &params)
            .unwrap()
            .into_tx_outs();

        let fee = self.get_fee(outputs.len() as u64);

        let change =
            utxo.amount.to_sat() - outputs.iter().map(|o| o.value.to_sat()).sum::<u64>() - fee;

        let mut unsigned_tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: OutPoint::new(utxo.txid, utxo.vout),
                script_sig: ScriptBuf::default(),
                sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::new(),
            }],
            output: outputs,
        };

        if change > 0 {
            unsigned_tx.output.push(TxOut {
                value: bitcoin::Amount::from_sat(change),
                script_pubkey: self.get_user_address().script_pubkey(),
            });
        }

        let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();

        psbt.inputs[0] = Input {
            witness_utxo: Some(TxOut {
                value: utxo.amount,
                script_pubkey: utxo.script_pub_key.clone(),
            }),
            tap_key_origins: {
                let mut map = BTreeMap::new();

                map.insert(
                    self.get_user_pubkey().inner.x_only_public_key().0,
                    (
                        vec![utxo.script_pub_key.tapscript_leaf_hash()],
                        ([0u8; 4].into(), DerivationPath::default()),
                    ),
                );
                map
            },
            ..Default::default()
        };

        <TestSuite as RpcTestSuite>::process_and_broadcast_psbt(&self, &psbt)
    }

    fn process_psbt(&self, psbt: &Psbt) -> WalletProcessPsbtResult {
        let rpc: &Client = self.get_rpc();

        let mut psbt_bytes = Vec::new();
        psbt.serialize_to_writer(&mut psbt_bytes).unwrap();

        let psbt_base64 = base64::encode(psbt_bytes);

        rpc.wallet_process_psbt(&psbt_base64, Some(true), None, None)
            .unwrap()
    }

    fn process_and_broadcast_psbt(&self, psbt: &Psbt) -> Transaction {
        let rpc: &Client = self.get_rpc();

        let result = <TestSuite as RpcTestSuite>::process_psbt(&self, psbt);

        let finalized_psbt: FinalizePsbtResult =
            rpc.finalize_psbt(&result.psbt, Some(true)).unwrap();

        let hex = finalized_psbt.hex.unwrap();

        let tx = bitcoin::consensus::deserialize(&hex).unwrap();

        rpc.send_raw_transaction(&hex).unwrap();
        tx
    }
}

//cargo test --package bitcoin-vault --test mod -- e2e_rpc::test_staking --exact --show-output

#[test]
fn test_user_protocol_unstaking() {
    let mut suite = SUITE.lock().unwrap();
    // prepare staking tx
    // let staking_tx = <TestSuite as RpcTestSuite>::prepare_staking_tx(&suite, None);

    // let staking_tx_id = staking_tx.compute_txid();

    // println!("staking_tx_id: {:?}", staking_tx_id);

    // // prepare unstaking tx
    // let unstaked_psbt = suite.build_unstaking_tx(&staking_tx, UnstakingType::UserProtocol, None);

    // let psbt_base64 = base64::encode(unstaked_psbt.serialize());

    let psbt_base64 = "cHNidP8BAF4CAAAAAT8EA91qLeiVbI33ijacKDo7uxdm6q6YE0Neyw0opvSxAAAAAAD9////AbUGAAAAAAAAIlEglQM9SLYCkXTtO6ITkHVsVukMQe7u9BwXLIHR0JoWfNoAAAAAAAEBK3cHAAAAAAAAIlEggOH6H6omUalwy43ov6fxoYSp2tDBBRWvDmZV+Z0solMBAwQAAAAAQhXAUJKbdMGgSVS3i0tgNel6XgeKWg8o7JbVR7/ums6AOsAuGldaBNe1a9khid2JrCWcr3vCP0UDWvq5+oHkXEVEO0UgKuMeqHCa7agZS6Pi9+fpXmgOi2UTXImDwKKY0XvFNQqtIBOHqrITA3grF+dgxnBDJVnfOWjlLLgswtj5vkOiJ9XcrMAhFhOHqrITA3grF+dgxnBDJVnfOWjlLLgswtj5vkOiJ9XcJQGLISCYocn5X632m6v+c4w0iXIV6RcH8f26mfpUdNk7HwAAAAAhFirjHqhwmu2oGUuj4vfn6V5oDotlE1yJg8CimNF7xTUKJQGLISCYocn5X632m6v+c4w0iXIV6RcH8f26mfpUdNk7HwAAAAABFyBQkpt0waBJVLeLS2A16XpeB4paDyjsltVHv+6azoA6wAEYIJTgMFNK2/kyTfNDehDM41MtJSmjWc4f4UVIIZKtYmxtAAA=";

    let psbt_base64 = base64::decode(psbt_base64).unwrap();

    let mut unstaked_psbt = Psbt::deserialize(&psbt_base64).unwrap();

    let psbt_base64 = "cHNidP8BAF4CAAAAAT8EA91qLeiVbI33ijacKDo7uxdm6q6YE0Neyw0opvSxAAAAAAD9////AbUGAAAAAAAAIlEglQM9SLYCkXTtO6ITkHVsVukMQe7u9BwXLIHR0JoWfNoAAAAAAAEBK3cHAAAAAAAAIlEggOH6H6omUalwy43ov6fxoYSp2tDBBRWvDmZV+Z0solMBAwQAAAAAQRQq4x6ocJrtqBlLo+L35+leaA6LZRNciYPAopjRe8U1CoshIJihyflfrfabq/5zjDSJchXpFwfx/bqZ+lR02TsfQPDZCdj+hAEoYJtvSiTPcqnFhW9ApxAoK1EwbJrJsA+hwh4bhjv7qtj7ANYMj/bongSW6lC9mj1sje62Ir4OhcJCFcBQkpt0waBJVLeLS2A16XpeB4paDyjsltVHv+6azoA6wC4aV1oE17Vr2SGJ3YmsJZyve8I/RQNa+rn6geRcRUQ7RSAq4x6ocJrtqBlLo+L35+leaA6LZRNciYPAopjRe8U1Cq0gE4eqshMDeCsX52DGcEMlWd85aOUsuCzC2Pm+Q6In1dyswCEWE4eqshMDeCsX52DGcEMlWd85aOUsuCzC2Pm+Q6In1dwlAYshIJihyflfrfabq/5zjDSJchXpFwfx/bqZ+lR02TsfAAAAACEWKuMeqHCa7agZS6Pi9+fpXmgOi2UTXImDwKKY0XvFNQolAYshIJihyflfrfabq/5zjDSJchXpFwfx/bqZ+lR02TsfAAAAAAEXIFCSm3TBoElUt4tLYDXpel4HiloPKOyW1Ue/7prOgDrAARgglOAwU0rb+TJN80N6EMzjUy0lKaNZzh/hRUghkq1ibG0AAA==";

    let psbt_base64 = base64::decode(psbt_base64).unwrap();

    let mut unstaked_psbt = Psbt::deserialize(&psbt_base64).unwrap();

    println!("unstaked_psbt: {:?}", unstaked_psbt.serialize_hex());

    suite.set_rpc("protocol");

    let result = <TestSuite as RpcTestSuite>::process_psbt(&suite, &unstaked_psbt);

    println!("result: {:?}", result);
}
