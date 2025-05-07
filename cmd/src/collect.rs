use std::str::FromStr;

use bitcoin::Network;
use bitcoin::consensus::serialize;
use bitcoin::hashes::Hash;
use bitcoin::hex::DisplayHex;
use bitcoin::{
    Address, Amount, PrivateKey, ScriptBuf, Transaction, TxIn, TxOut, Witness, transaction::Version,
};
use clap::Parser;
use rust_mempool::{AddressType, BitcoinWallet, MempoolClient};
use serde::{Deserialize, Serialize};

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
