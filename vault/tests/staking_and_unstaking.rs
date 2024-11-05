use std::collections::BTreeMap;
use std::str::FromStr;
use std::time::Duration;

use bitcoin::bip32::DerivationPath;
use bitcoin::psbt::Input;
use bitcoin::OutPoint;
use bitcoin::{
    absolute, address::NetworkChecked, key::Secp256k1, secp256k1::All, transaction, Address,
    NetworkKind, PrivateKey, Psbt, PublicKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut,
    Witness,
};
use bitcoin_vault::{BuildStakingParams, Signing, Staking, VaultManager};

use std::thread::sleep;

use crate::{get_env, hex_to_vec, MANAGER};

use lazy_static::lazy_static;

use bitcoincore_rpc::{Auth, Client, RpcApi};

lazy_static! {
    static ref SECP: Secp256k1<All> = Secp256k1::new();
}

#[test]
fn test_staking() {
    let env = get_env();

    // Create RPC client with wallet name in URL
    let wallet_url = format!("{}/wallet/{}", env.btc_node_address, "legacy");
    let rpc = Client::new(
        &wallet_url,
        Auth::UserPass(env.btc_node_user.clone(), env.btc_node_password.clone()),
    )
    .expect("Failed to create RPC client");

    let (user_privkey, user_pubkey) = key_from_wif(&env.user_private_key);
    let (_, protocol_pubkey) = key_from_wif(&env.protocol_private_key);

    let user_address: Address<NetworkChecked> = Address::from_str(&env.user_address)
        .unwrap()
        .require_network(bitcoin::Network::Regtest)
        .unwrap();

    let utxos = // Get UTXOs for the address
    // Parameters: minconf, maxconf, [addresses]
    rpc.list_unspent(
        Some(0),               // minconf - include unconfirmed
        None,                  // maxconf - no maximum
        Some(&[&user_address]), // addresses to filter
        None,                  // include_unsafe
        None,                  // query_options
    )
    .expect("Failed to get UTXOs");

    let destination_chain_id = env.destination_chain_id.to_le_bytes();
    let destination_contract_address = hex_to_vec!(env.destination_contract_address);
    let destination_recipient_address = hex_to_vec!(env.destination_recipient_address);

    let params = BuildStakingParams {
        user_pub_key: user_pubkey,
        protocol_pub_key: protocol_pubkey,
        covenant_pub_keys: env
            .covenant_private_keys
            .iter()
            .map(|s| key_from_wif(s).1)
            .collect(),
        covenant_quorum: env.covenant_quorum,
        staking_amount: env.staking_amount,
        have_only_covenants: env.have_only_covenants,
        destination_chain_id: destination_chain_id,
        destination_contract_address: destination_contract_address.try_into().unwrap(),
        destination_recipient_address: destination_recipient_address.try_into().unwrap(),
    };

    let outputs = <VaultManager as Staking>::build(&MANAGER, &params)
        .unwrap()
        .into_tx_outs();

    let fee_rate = 1;

    // get input from utxos > env.staking_amount
    let seleted_utxo = utxos
        .iter()
        .find(|u| u.amount >= bitcoin::Amount::from_sat(env.staking_amount))
        .unwrap();

    // Calculate approximate transaction size (in virtual bytes)
    // P2PKH input: ~148 vbytes
    // P2PKH output: ~34 vbytes
    // Other transaction overhead: ~10 vbytes
    let estimated_tx_size = 148 + (34 * outputs.len() as u64) + 10;

    // Calculate fee in satoshis
    let fee = fee_rate * estimated_tx_size;

    let amount = outputs.iter().map(|o| o.value.to_sat()).sum::<u64>();

    let change = seleted_utxo.amount.to_sat() - amount - fee;

    let unsigned_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::new(seleted_utxo.txid, seleted_utxo.vout),
            script_sig: ScriptBuf::default(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }],
        output: outputs,
    };

    let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();

    psbt.inputs[0] = Input {
        witness_utxo: Some(TxOut {
            value: seleted_utxo.amount,
            script_pubkey: seleted_utxo.script_pub_key.clone(),
        }),
        tap_key_origins: {
            let mut map = BTreeMap::new();

            map.insert(
                user_pubkey.inner.x_only_public_key().0,
                (
                    vec![seleted_utxo.script_pub_key.tapscript_leaf_hash()],
                    ([0u8; 4].into(), DerivationPath::default()),
                ),
            );
            map
        },
        ..Default::default()
    };

    if change > 0 {
        psbt.unsigned_tx.output.push(TxOut {
            value: bitcoin::Amount::from_sat(change),
            script_pubkey: user_address.script_pubkey(),
        });
    }

    let user_privkey_bytes = user_privkey.to_bytes();

    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut psbt,
        &user_privkey_bytes,
        NetworkKind::Test,
        true,
    )
    .map_err(|e| panic!("Error signing psbt: {:?}", e))
    .unwrap();

    let finalized_tx = psbt.extract_tx().unwrap();

    let txid = rpc.send_raw_transaction(&finalized_tx).unwrap();

    let retry = 3;
    println!("waiting for tx to be confirmed");
    for _ in 0..retry {
        // get tx
        let tx = rpc.get_transaction(&txid, None).unwrap();
        println!("tx: {:?}", tx);
        // check if tx is confirmed
        sleep(Duration::from_secs(5));
    }



    // unstake
    

}

fn key_from_wif(wif: &str) -> (PrivateKey, PublicKey) {
    let privkey = PrivateKey::from_wif(wif).unwrap();
    let pubkey = privkey.public_key(&SECP);
    (privkey, pubkey)
}
