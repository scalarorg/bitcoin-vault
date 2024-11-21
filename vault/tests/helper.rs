use std::str::FromStr;

use bitcoin::hex::DisplayHex;
use bitcoin::Amount;
use bitcoin::{
    address::NetworkChecked, key::Secp256k1, secp256k1::All, Address, NetworkKind, PrivateKey,
    PublicKey,
};

use bitcoincore_rpc::json::{
    GetTransactionResult, ListUnspentQueryOptions, ListUnspentResultEntry,
};

use bitcoincore_rpc::{Auth, Client, RpcApi};

pub fn hex_to_vec(hex: &str) -> Vec<u8> {
    let hex_str = hex.replace("0x", "").replace(" ", "");
    let mut vec = Vec::new();
    for i in (0..hex_str.len()).step_by(2) {
        vec.push(u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap());
    }
    vec
}

pub fn create_rpc(
    btc_node_address: &str,
    btc_node_user: &str,
    btc_node_password: &str,
    wallet_name: &str,
) -> Client {
    let url = format!("{}/wallet/{}", btc_node_address, wallet_name);
    let auth = Auth::UserPass(btc_node_user.to_string(), btc_node_password.to_string());
    Client::new(&url, auth).unwrap()
}

pub fn get_adress(network: &str, address: &str) -> Address<NetworkChecked> {
    let network = get_network_from_str(network);
    Address::from_str(address)
        .unwrap()
        .require_network(network)
        .unwrap()
}

pub fn get_network_from_str(network: &str) -> bitcoin::Network {
    match network {
        "testnet4" => bitcoin::Network::Testnet4,
        "regtest" => bitcoin::Network::Regtest,
        _ => panic!("Invalid network"),
    }
}

pub fn get_network_id_from_str(network: &str) -> NetworkKind {
    match network {
        "mainnet" => NetworkKind::Main,
        _ => NetworkKind::Test,
    }
}

pub fn key_from_wif(wif: &str, secp: &Secp256k1<All>) -> (PrivateKey, PublicKey) {
    let privkey = PrivateKey::from_wif(wif).unwrap();
    let pubkey = privkey.public_key(secp);
    (privkey, pubkey)
}

pub fn get_approvable_utxos(
    rpc: &Client,
    user_address: &Address<NetworkChecked>,
    btc_amount: u64,
) -> ListUnspentResultEntry {
    let utxos = rpc
        .list_unspent(
            Some(0),
            None,
            Some(&[&user_address]),
            Some(true),
            Some(ListUnspentQueryOptions {
                minimum_amount: Some(Amount::from_sat(btc_amount)),
                maximum_amount: None,
                maximum_count: None,
                minimum_sum_amount: None,
            }),
        )
        .unwrap();

    utxos[0].clone()
}

pub fn get_fee(n_outputs: u64) -> u64 {
    (148 + (34 * n_outputs) + 20) * get_fee_rate()
}

pub fn get_fee_rate() -> u64 {
    1
}

pub fn log_tx_result(result: &GetTransactionResult) {
    println!("\n=== Transaction Info ===");
    println!("TxID: {}", result.info.txid);
    println!("Confirmations: {}", result.info.confirmations);
    println!("Block Hash: {:?}", result.info.blockhash);
    println!("Block Index: {:?}", result.info.blockindex);
    println!("Block Time: {:?}", result.info.blocktime);
    println!("Block Height: {:?}", result.info.blockheight);
    println!("Time: {}", result.info.time);
    println!("Time Received: {}", result.info.timereceived);
    println!("BIP125 Replaceable: {:?}", result.info.bip125_replaceable);
    println!("Wallet Conflicts: {:?}\n", result.info.wallet_conflicts);

    println!("=== Transaction Details ===");
    for detail in &result.details {
        println!("Address: {:?}", detail.address);
        println!("Category: {:?}", detail.category);
        println!("Amount: {:?} BTC", detail.amount);
        println!("Label: {:?}", detail.label);
        println!("Vout: {}", detail.vout);
        println!("Fee: {:?}", detail.fee);
        println!("Abandoned: {:?}\n", detail.abandoned);
    }

    println!("=== Transaction Hex ===");
    let hex = result.hex.to_lower_hex_string();
    println!("{}\n", hex);
}
