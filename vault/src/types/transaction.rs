use bitcoin::{consensus::Encodable, Amount, ScriptBuf, Transaction, TxIn, TxOut, Txid};
use serde::{Deserialize, Serialize};

use crate::{
    DestinationAddress, DestinationChainId, ADDRESS_SIZE, CHAIN_ID_SIZE, COVENANT_QUORUM_SIZE,
    HAVE_ONLY_COVENANTS_SIZE, NETWORK_ID_SIZE, SERVICE_TAG_HASH_SIZE, TAG_HASH_SIZE, VERSION_SIZE,
};

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
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VaultReturnTxOutput {
    pub tag: Vec<u8>,
    pub service_tag: Vec<u8>,
    pub version: u8,
    pub network_id: u8,
    pub have_only_covenants: bool,
    pub covenant_quorum: u8,
    pub destination_chain_id: DestinationChainId,
    pub destination_contract_address: DestinationAddress,
    pub destination_recipient_address: DestinationAddress,
}

impl TryFrom<&TxOut> for VaultReturnTxOutput {
    type Error = ParserError;
    fn try_from(txo: &TxOut) -> Result<Self, Self::Error> {
        let mut instructions = txo.script_pubkey.instructions();
        // Skip OP_RETURN
        instructions.next();

        println!("instructions: {:?}", instructions);

        let instruction = instructions
            .next()
            .transpose()
            .map_err(ParserError::from)?
            .ok_or(ParserError::InvalidInstruction)?;

        println!("instruction: {:?}", instruction);

        let push_bytes = instruction
            .push_bytes()
            .ok_or(ParserError::InvalidEmbeddedData)?;

        // Create a cursor to read through the push_bytes sequentially
        let bytes = push_bytes.as_bytes();
        let mut cursor = 0;

        // Read hash (tag)
        let tag = bytes[cursor..cursor + TAG_HASH_SIZE].to_vec();
        cursor += TAG_HASH_SIZE;

        // Read service_tag_hash
        let service_tag = bytes[cursor..cursor + SERVICE_TAG_HASH_SIZE].to_vec();
        cursor += SERVICE_TAG_HASH_SIZE;

        // Read version
        let version = bytes[cursor];
        cursor += VERSION_SIZE;

        // Read network_id
        let network_id = bytes[cursor];
        cursor += NETWORK_ID_SIZE;

        // Read have_only_covenants
        let have_only_covenants = bytes[cursor] != 0;
        cursor += HAVE_ONLY_COVENANTS_SIZE;

        // Read covenant_quorum
        let covenant_quorum = bytes[cursor];
        cursor += COVENANT_QUORUM_SIZE;

        // Read destination_chain_id
        let destination_chain_id = bytes[cursor..cursor + CHAIN_ID_SIZE]
            .try_into()
            .map_err(|_| ParserError::InvalidScript)?;
        cursor += CHAIN_ID_SIZE;

        // Read destination_contract_address
        let destination_contract_address = bytes[cursor..cursor + ADDRESS_SIZE]
            .try_into()
            .map_err(|_| ParserError::InvalidScript)?;
        cursor += ADDRESS_SIZE;

        // Read destination_recipient_address
        let destination_recipient_address = bytes[cursor..cursor + ADDRESS_SIZE]
            .try_into()
            .map_err(|_| ParserError::InvalidScript)?;

        Ok(VaultReturnTxOutput {
            tag,
            service_tag,
            version,
            network_id,
            have_only_covenants,
            covenant_quorum,
            destination_chain_id,
            destination_contract_address,
            destination_recipient_address,
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
        let tx_hex = "020000000001013c4935c4274905047166060a54cf8b1e30b8dacf3dd9e27fd6b1586bfa3f59170200000000fdffffff03102700000000000022512067bff357780a93826a444646aec681c4ff1f4316244478c0d611f91a75c93b8a0000000000000000416a3f5343414c41526c69676874000100030000000000aa36a7b91e3a8ef862567026d6f376c9f3d6b814ca433724a1db57fa3ecafcbad91d6ef068439aceeae090b6d17f000000000016001450dceca158a9c872eb405d52293d351110572c9e0247304402200bb35f5944c2189bd61d71f8d35d4499c58290d2f87f94f0236fff61381c7d550220202a1bb7f437768582ebb5906d323d7c2b4d0139d63987660ba01e1ca57e1ec10121022ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350a00000000";

        // Decode hex string to bytes
        let tx_bytes = hex::decode(tx_hex).unwrap();

        // Parse bytes into Transaction
        let tx: Transaction = bitcoin::consensus::deserialize(&tx_bytes).unwrap();

        // Now you can test VaultReturnTxOutput conversion
        let vault_return = VaultReturnTxOutput::try_from(&tx.output[1]).unwrap();

        println!("vault_return: {:?}", vault_return);
    }
}
