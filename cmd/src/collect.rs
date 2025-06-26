use std::str::FromStr;

use bitcoin::consensus::serialize;
use bitcoin::hashes::Hash;
use bitcoin::hex::DisplayHex;
use bitcoin::Network;
use bitcoin::{
    transaction::Version, Address, Amount, PrivateKey, ScriptBuf, Transaction, TxIn, TxOut, Witness,
};
use bitcoin::{OutPoint, Psbt};
use clap::Parser;
use electrum_client::{Client, ElectrumApi};
use rust_mempool::{AddressType, BitcoinWallet, MempoolClient};
use serde::{Deserialize, Serialize};
use std::process;
use std::sync::Arc;
use vault::core::*;
use vault::utils::*;

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct CollectCmd {
    /// Mnemonics
    #[arg(short, long)]
    mnemonic: String,

    /// Account count
    #[arg(short, long)]
    count: u32,

    /// Amount
    #[arg(long)]
    amount: Option<u64>,
}

impl CollectCmd {
    const NETWORK: Network = Network::Testnet4;

    pub async fn execute(&self) -> anyhow::Result<()> {
        // 1. Prepare accounts
        let wallet =
            BitcoinWallet::new(&self.mnemonic, Self::NETWORK, AddressType::P2WPKH).unwrap();
        let secp = bitcoin::secp256k1::Secp256k1::new();

        let accounts = (0..self.count + 1)
            .map(|i| wallet.get_account(&secp, i))
            .collect::<Result<Vec<(PrivateKey, Address)>, _>>()?;

        let client = MempoolClient::new(Self::NETWORK);

        let first_account = &accounts[0];
        let others_accounts = &accounts[1..];

        // 2. Get utxos
        let address_strings: Vec<String> = others_accounts
            .iter()
            .map(|(_, addr)| addr.to_string())
            .collect();

        let address_refs: Vec<&str> = address_strings.iter().map(|s| s.as_str()).collect();

        let batch_adddresses_utxo = client
            .get_batch_of_addresses_utxo(&address_refs)
            .await
            .unwrap();

        // 3. Build transaction inputs

        let mut inputs_info: Vec<(TxIn, (PrivateKey, Address), u64)> = vec![];
        let mut total_value = 0;

        let base_tx_in = TxIn {
            previous_output: bitcoin::OutPoint::null(),
            script_sig: ScriptBuf::new(),
            sequence: bitcoin::Sequence::MAX,
            witness: Witness::new(),
        };

        for account in others_accounts {
            let utxos = batch_adddresses_utxo.get(&account.1.to_string());

            if utxos.is_none() {
                continue;
            }

            let utxos = utxos.unwrap();

            for utxo in utxos {
                let outpoint = bitcoin::OutPoint {
                    txid: bitcoin::Txid::from_str(&utxo.txid).unwrap(),
                    vout: utxo.vout,
                };
                let mut input = base_tx_in.clone();
                input.previous_output = outpoint;
                inputs_info.push((input, account.clone(), utxo.value));
                total_value += utxo.value;
            }
        }

        let total_fee = calculate_fee(inputs_info.len() as u32, 1);
        let output_value = total_value - total_fee;

        // 4. Build transaction outputs
        let output = TxOut {
            value: Amount::from_sat(output_value),
            script_pubkey: first_account.1.script_pubkey(),
        };

        // 5. Build transaction
        let mut tx = Transaction {
            version: Version::TWO,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: inputs_info
                .clone()
                .into_iter()
                .map(|(input, _, _)| input)
                .collect(),
            output: vec![output],
        };

        // 6. Sign transaction and add witness to each input
        for (index, (_, (privkey, address), amount)) in inputs_info.iter().enumerate() {
            let publickey = privkey.public_key(&secp);
            // Create sighash
            let sighash_type = bitcoin::sighash::EcdsaSighashType::All;
            let sighash = bitcoin::sighash::SighashCache::new(&tx).p2wpkh_signature_hash(
                index,
                &address.script_pubkey(),
                Amount::from_sat(*amount),
                sighash_type,
            )?;

            // Sign the hash
            let signature = secp.sign_ecdsa(
                &bitcoin::secp256k1::Message::from_digest_slice(
                    &sighash.to_raw_hash().to_byte_array(),
                )?,
                &privkey.inner,
            );

            let mut sig_with_sighash = signature.serialize_der().to_vec();
            sig_with_sighash.push(sighash_type as u8);

            // add into witness
            let mut witness = Witness::new();
            witness.push(sig_with_sighash);
            witness.push(publickey.inner.serialize());

            tx.input[index].witness = witness;
        }

        let tx_hex = serialize(&tx);

        // 7. Broadcast transaction
        let result = client
            .broadcast_transaction(tx_hex.to_lower_hex_string().as_str())
            .await;

        match result {
            Ok(result) => {
                println!("Transaction sent successfully!");
                println!("Transaction ID: {}", result);
                Ok(())
            }
            Err(e) => {
                println!("Transaction failed to send: {:?}", e);
                Err(anyhow::anyhow!("Transaction failed to send: {:?}", e))
            }
        }
    }
}

const INPUT_SIZE: u32 = 68;
const OUTPUT_SIZE: u32 = 31;
const OVERHEAD: u32 = 11;
/// Only support for p2wpkh now
/// Not use for others
fn calculate_fee(num_inputs: u32, num_outputs: u32) -> u64 {
    (INPUT_SIZE * num_inputs + OUTPUT_SIZE * num_outputs + OVERHEAD) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_fee() {
        let num_inputs = 1;
        let num_outputs = 2;
        let fee = calculate_fee(num_inputs, num_outputs);
        assert_eq!(fee, 141);
    }
}

#[derive(Parser, Debug)]
pub struct CollectUtxosCmd {
    /// Environment name (e.g., PEPE)
    #[arg(long, default_value = "PEPE")]
    env: String,

    /// Electrum host
    #[arg(long, default_value = "127.0.0.1")]
    electrum_host: String,

    /// Electrum port
    #[arg(long, default_value_t = 60001)]
    electrum_port: u16,

    /// Batch size limit
    #[arg(long, default_value_t = 500)]
    limit: usize,
}

impl CollectUtxosCmd {
    pub async fn execute(&self) -> anyhow::Result<()> {
        const VOUT: u32 = 1;
        let client = Arc::new(
            Client::new(format!("tcp://{}:{}", self.electrum_host, self.electrum_port).as_str())
                .unwrap(),
        );
        let mempool_client = Arc::new(MempoolClient::new(Network::Testnet4));
        let test_suite = TestSuite::new_with_loaded_env(&self.env);
        let test_account = SuiteAccount::new(AccountEnv::new(test_suite.env_path()).unwrap());
        let custodian_quorum = test_suite.env().custodian_quorum;
        let script = match <VaultManager as CustodianOnly>::locking_script(
            &test_suite.custodian_pubkeys(),
            custodian_quorum,
        ) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to build locking script: {}", e);
                process::exit(1);
            }
        };
        let network = get_network_from_str(&test_suite.env().network);
        let address = match Address::from_script(&script.clone().into_script(), network) {
            Ok(addr) => addr,
            Err(e) => {
                eprintln!("Failed to create address from script: {}", e);
                process::exit(1);
            }
        };
        println!("address: {}", address);
        println!("collect address: {}", test_account.address());
        println!("Connected to Electrum");
        let mut utxos = match client.script_list_unspent(&script.into_script()) {
            Ok(u) => u,
            Err(e) => {
                eprintln!("Failed to fetch UTXOs: {}", e);
                process::exit(1);
            }
        };
        utxos.reverse();
        let utxos = utxos
            .into_iter()
            .map(|utxo| NeededUtxo {
                txid: utxo.tx_hash,
                vout: utxo.tx_pos as u32,
                amount: Amount::from_sat(utxo.value),
            })
            .collect::<Vec<_>>();
        let len = utxos.len();
        let utxos = if len > self.limit {
            utxos.into_iter().take(len - self.limit).collect::<Vec<_>>()
        } else {
            utxos
        };
        println!("number of utxos: {}", utxos.len());
        let batch_futures = utxos
            .chunks(self.limit)
            .enumerate()
            .map(|(i, utxos_chunk)| {
                let address = address.clone();
                let account_address = test_account.address().clone();
                let manager = test_suite.manager().clone();
                let custodian_pubkeys = test_suite.custodian_pubkeys().clone();
                let custodian_quorum = test_suite.env().custodian_quorum;
                let network_id = test_suite.network_id();
                let signing_privkeys = test_suite.custodian_privkeys().clone();
                let utxos_chunk: Vec<_> = utxos_chunk.to_vec();
                let mempool_client = mempool_client.clone();
                tokio::spawn(async move {
                    let total: u64 = utxos_chunk.iter().map(|utxo| utxo.amount.to_sat()).sum();
                    println!("Batch {}: Processing {} utxos", i + 1, utxos_chunk.len());
                    let mut unstaked_psbt =
                        match <VaultManager as CustodianOnly>::build_unlocking_psbt(
                            &manager,
                            &CustodianOnlyUnlockingParams {
                                inputs: utxos_chunk
                                    .iter()
                                    .map(|u| PreviousOutpoint {
                                        outpoint: OutPoint::new(u.txid, VOUT),
                                        amount_in_sats: u.amount,
                                        script_pubkey: address.script_pubkey(),
                                    })
                                    .collect(),
                                outputs: vec![TxOut {
                                    value: Amount::from_sat(total),
                                    script_pubkey: account_address.script_pubkey(),
                                }],
                                custodian_pubkeys: custodian_pubkeys.clone(),
                                custodian_quorum,
                                fee_rate: 2,
                                rbf: false,
                                session_sequence: 0,
                                custodian_group_uid: [0u8; HASH_SIZE],
                            },
                        ) {
                            Ok(psbt) => psbt,
                            Err(e) => {
                                eprintln!("Failed to build PSBT for batch {}: {}", i + 1, e);
                                return;
                            }
                        };
                    for privkey in signing_privkeys {
                        let _ = <VaultManager as Signing>::sign_psbt_by_single_key(
                            &mut unstaked_psbt,
                            privkey.as_slice(),
                            network_id,
                            false,
                        )
                        .unwrap();
                    }
                    <Psbt as SignByKeyMap<bitcoin::secp256k1::All>>::finalize(&mut unstaked_psbt);
                    let finalized_tx = match unstaked_psbt.extract_tx() {
                        Ok(tx) => tx,
                        Err(e) => {
                            eprintln!("Failed to extract tx for batch {}: {}", i + 1, e);
                            return;
                        }
                    };
                    let tx_hex = bitcoin::consensus::serialize(&finalized_tx);
                    let result = mempool_client
                        .broadcast_transaction(tx_hex.to_lower_hex_string().as_str())
                        .await;
                    match result {
                        Ok(tx_id) => {
                            println!("Batch {} tx_id: {:?}", i + 1, tx_id);
                        }
                        Err(e) => {
                            eprintln!("Broadcast error for batch {}: {:?}", i + 1, e);
                        }
                    }
                })
            });
        futures::future::join_all(batch_futures).await;
        Ok(())
    }
}
