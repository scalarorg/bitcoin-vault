use crate::{
    DestinationChain, DestinationRecipientAddress, DestinationTokenAddress, TaprootTreeType,
    UnstakingTaprootTreeType, COVENANT_QUORUM_SIZE, DEST_CHAIN_SIZE, DEST_RECIPIENT_ADDRESS_SIZE,
    DEST_TOKEN_ADDRESS_SIZE, FLAGS_SIZE, NETWORK_ID_SIZE, SERVICE_TAG_HASH_SIZE, TAG_HASH_SIZE,
    VERSION_SIZE,
};
use bitcoin::{consensus::Encodable, Amount, ScriptBuf, Transaction, TxIn, TxOut, Txid};
use log::debug;
use serde::{Deserialize, Serialize};

use super::error::ParserError;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VaultLockTxOutput {
    pub amount: Amount,
    pub script_pubkey: ScriptBuf,
}

impl From<&TxOut> for VaultLockTxOutput {
    fn from(txo: &TxOut) -> Self {
        Self {
            amount: txo.value,
            script_pubkey: txo.script_pubkey.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum VaultReturnTxOutputType {
    Unstaking,
    Staking,
}

impl Default for VaultReturnTxOutputType {
    fn default() -> Self {
        Self::Unstaking
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct VaultReturnTxOutput {
    pub tag: [u8; TAG_HASH_SIZE],
    pub version: u8,
    pub network_id: u8,
    pub flags: u8,
    pub service_tag: [u8; SERVICE_TAG_HASH_SIZE],
    pub transaction_type: VaultReturnTxOutputType,
    pub covenant_quorum: u8,
    pub destination_chain: DestinationChain,
    pub destination_token_address: DestinationTokenAddress,
    pub destination_recipient_address: DestinationRecipientAddress,
}

fn read_bytes(bytes: &[u8], cursor: &mut usize, len: usize) -> Result<Vec<u8>, ParserError> {
    if bytes.len() < *cursor + len {
        return Err(ParserError::InvalidEmbeddedData);
    }
    let data = bytes[*cursor..*cursor + len].to_vec();
    *cursor += len;
    Ok(data)
}
impl TryFrom<&TxOut> for VaultReturnTxOutput {
    type Error = ParserError;
    fn try_from(txo: &TxOut) -> Result<Self, Self::Error> {
        let mut instructions = txo.script_pubkey.instructions();
        // Skip OP_RETURN
        instructions.next();

        let instruction = instructions
            .next()
            .transpose()
            .map_err(ParserError::from)?
            .ok_or(ParserError::InvalidInstruction)?;

        let push_bytes = instruction
            .push_bytes()
            .ok_or(ParserError::InvalidEmbeddedData)?;

        // Create a cursor to read through the push_bytes sequentially
        let bytes = push_bytes.as_bytes();
        let mut cursor = 0;
        // Read hash (tag)
        let tag = read_bytes(bytes, &mut cursor, TAG_HASH_SIZE)?;
        // Read version
        let version = read_bytes(bytes, &mut cursor, VERSION_SIZE)?[0];
        // Read network_id
        let network_id = read_bytes(bytes, &mut cursor, NETWORK_ID_SIZE)?[0];
        // Read flags
        let flags = read_bytes(bytes, &mut cursor, FLAGS_SIZE)?[0];

        match UnstakingTaprootTreeType::try_from(flags) {
            Ok(UnstakingTaprootTreeType::CovenantOnly) => {
                let service_tag = read_bytes(bytes, &mut cursor, SERVICE_TAG_HASH_SIZE)?;
                return Ok(VaultReturnTxOutput {
                    tag: tag.try_into().unwrap(),
                    version,
                    network_id,
                    flags,
                    service_tag: service_tag.try_into().unwrap(),
                    transaction_type: VaultReturnTxOutputType::Unstaking,
                    ..Default::default()
                });
            }
            Err(_) => {
                let tree_type =
                    TaprootTreeType::try_from(flags).map_err(|_| ParserError::InvalidScript)?;

                let service_tag = read_bytes(bytes, &mut cursor, SERVICE_TAG_HASH_SIZE)?;

                // Read covenant_quorum
                let covenant_quorum = read_bytes(bytes, &mut cursor, COVENANT_QUORUM_SIZE)?[0];

                // Read destination_chain_id
                let destination_chain = read_bytes(bytes, &mut cursor, DEST_CHAIN_SIZE)?
                    .try_into()
                    .map_err(|_| ParserError::InvalidScript)?;

                // Read destination_contract_address
                let destination_token_address =
                    read_bytes(bytes, &mut cursor, DEST_TOKEN_ADDRESS_SIZE)?
                        .try_into()
                        .map_err(|_| ParserError::InvalidScript)?;

                // Read destination_recipient_address
                let destination_recipient_address =
                    read_bytes(bytes, &mut cursor, DEST_RECIPIENT_ADDRESS_SIZE)?
                        .try_into()
                        .map_err(|_| ParserError::InvalidScript)?;
                //Check if no extra bytes left
                if cursor != bytes.len() {
                    return Err(ParserError::InvalidScript);
                }
                debug!(
                    "Found candiate for Scalar VaultTx with tree_type: {:?}",
                    tree_type
                );
                Ok(VaultReturnTxOutput {
                    tag: tag.try_into().unwrap(),
                    service_tag: service_tag.try_into().unwrap(),
                    version,
                    network_id,
                    flags,
                    transaction_type: VaultReturnTxOutputType::Staking,
                    covenant_quorum,
                    destination_chain,
                    destination_token_address,
                    destination_recipient_address,
                })
            }
        }
    }
}

impl VaultReturnTxOutput {
    pub fn try_from_script_pubkey(script_pubkey: &[u8]) -> Result<Self, ParserError> {
        Self::try_from(&TxOut {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::from_bytes(script_pubkey.to_vec()),
        })
    }
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VaultChangeTxOutput {
    pub amount: Amount,
    pub address: String,
}
impl From<&TxOut> for VaultChangeTxOutput {
    fn from(txo: &TxOut) -> Self {
        Self {
            amount: txo.value,
            address: txo.script_pubkey.to_hex_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VaultTransaction {
    // 32 bytes hex string txid
    pub txid: Txid,
    pub tx_content: String,
    pub inputs: Vec<TxIn>,
    pub lock_tx: VaultLockTxOutput,
    pub return_tx: VaultReturnTxOutput,
    pub change_tx: Option<VaultChangeTxOutput>,
}

impl TryFrom<&Transaction> for VaultTransaction {
    type Error = ParserError;
    fn try_from(tx: &Transaction) -> Result<Self, Self::Error> {
        //1. Validate the transaction if it's a staking transaction
        if tx.output.len() < 2 || tx.output.len() > 3 {
            return Err(ParserError::InvalidTransactionHex);
        }
        let txid = tx.compute_txid();
        //2. Parse the transaction locking outputs
        let lock_tx = VaultLockTxOutput::from(&tx.output[0]);
        let return_tx = VaultReturnTxOutput::try_from(&tx.output[1])?;
        let change_tx = if tx.output.len() == 3 {
            Some(VaultChangeTxOutput::from(&tx.output[2]))
        } else {
            None
        };
        let Some(_) = tx.input.first() else {
            return Err(ParserError::InvalidTransactionHex);
        };
        let mut tx_content = vec![];
        tx.consensus_encode(&mut tx_content).unwrap();
        Ok(VaultTransaction {
            txid,
            tx_content: hex::encode(tx_content),
            inputs: tx.input.clone(),
            // inputs: tx
            //     .input
            //     .iter()
            //     .map(|txin| VaultTxInput::from(txin))
            //     .collect(),
            lock_tx,
            return_tx,
            change_tx,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::VaultReturnTxOutput;
    use bitcoin::Transaction;
    #[test]
    fn test_vault_return_tx_output() {
        let tx_hex = "02000000000101d9f3c9143498dd3eef49a1f25c8bb73814aa7e95779f08d771074c217fc031850000000000fdffffff030f2700000000000022512080e1fa1faa2651a970cb8de8bfa7f1a184a9dad0c10515af0e6655f99d2ca2530000000000000000416a3f5343414c41520001806c6967687403a736aa00000000001f98c06d8734d5a9ff0b53e3294626e62e4d232c130c4810d57140e1e62967cbf742caeae91b6ece0dca052a0100000022512095033d48b6029174ed3ba21390756c56e90c41eeeef41c172c81d1d09a167cda0140288d98fe6078eb68c4a93fac65a1e8febaec2ebd60a55aa584afd493b8ec5660d0c0b42f2063811e22f6cd35efbb293dbdaab6eed9872dcee597aa535f0b1afa00000000";
        let tx_hex_with_only_covenants = "02000000000101bb520c6dc5043249b56157246cf381d0ab8abcf6fae82f2c18f757a9d02e616e0200000000fdffffff030f270000000000002251207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d500000000000000003c6a3a5343414c415200014003a736aa00000000001f98c06d8734d5a9ff0b53e3294626e62e4d232c130c4810d57140e1e62967cbf742caeae91b6ece1aa2052a0100000022512095033d48b6029174ed3ba21390756c56e90c41eeeef41c172c81d1d09a167cda014097f6cab5f3e5c2a8e65120dd9f1582d2c5fce543728381cbf014ceb64bc79fb2420673c60540ad0040f44d4e9872ed47c43155e7edd4f6e9555ee87c37d2c10d00000000";
        let tx_hexs = vec![tx_hex, tx_hex_with_only_covenants];

        for tx_hex in tx_hexs {
            // Decode hex string to bytes
            let tx_bytes = hex::decode(tx_hex).unwrap();

            // Parse bytes into Transaction
            let tx: Transaction = bitcoin::consensus::deserialize(&tx_bytes).unwrap();
            // Now you can test VaultReturnTxOutput conversion
            let vault_return = VaultReturnTxOutput::try_from(&tx.output[1]).unwrap();
            println!("vault_return: {:?}", vault_return);
        }
    }
}
