use std::str::FromStr;

use bitcoin::hex::DisplayHex;
use bitcoin::{
    address::NetworkChecked, key::Secp256k1, secp256k1::All, Address, NetworkKind, PrivateKey,
    PublicKey,
};
use bitcoin::{AddressType, Amount};

use bitcoincore_rpc::json::{GetRawTransactionResult, ListUnspentQueryOptions};

use bitcoincore_rpc::{Auth, Client, RpcApi};

use super::NeededUtxo;

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
) -> Result<Vec<NeededUtxo>, String> {
    let utxos = rpc
        .list_unspent(
            Some(0),
            None,
            Some(&[user_address]),
            Some(true),
            Some(ListUnspentQueryOptions {
                minimum_amount: Some(Amount::from_sat(btc_amount)),
                maximum_amount: None,
                maximum_count: None,
                minimum_sum_amount: None,
            }),
        )
        .unwrap();

    if utxos.is_empty() {
        return Err("No utxos found".to_string());
    }

    Ok(utxos
        .iter()
        .map(|utxo| NeededUtxo {
            txid: utxo.txid,
            vout: utxo.vout,
            amount: utxo.amount,
        })
        .collect())
}

pub fn get_approvable_utxo(
    rpc: &Client,
    user_address: &Address<NetworkChecked>,
    btc_amount: u64,
) -> Result<NeededUtxo, String> {
    let utxos = get_approvable_utxos(rpc, user_address, btc_amount)?;
    Ok(NeededUtxo {
        txid: utxos[0].txid,
        vout: utxos[0].vout,
        amount: utxos[0].amount,
    })
}

pub fn get_fee_rate() -> u64 {
    1
}

pub fn get_basic_fee(i: u64, o: u64, rate: u64, address: AddressType) -> u64 {
    const BUFFER: u64 = 100;
    let size = match address {
        AddressType::P2tr => (58 + BUFFER) * i + 43 * o,
        AddressType::P2wpkh => (68 + BUFFER) * i + 31 * o,
        AddressType::P2pkh => (140 + BUFFER) * i + 34 * o,
        _ => (93 + BUFFER) * i + 32 * o,
    };
    (size + 10) * rate
}

pub fn log_tx_result(result: &GetRawTransactionResult) {
    println!("=== Transaction Info ===");

    println!("TxID: {}", result.txid);
    println!("Confirmations: {}", result.confirmations.unwrap_or(0));
    println!("Block Hash: {:?}", result.blockhash);
    println!("Version: {}", result.version);
    println!("Locktime: {}", result.locktime);

    println!("=== Transaction Hex ===");
    let hex = result.hex.to_lower_hex_string();
    println!("{}\n", hex);
}
