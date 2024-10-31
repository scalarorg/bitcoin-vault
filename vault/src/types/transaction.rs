use bitcoin::{Amount, ScriptBuf, Transaction, TxIn, TxOut, Txid};
use serde::{Deserialize, Serialize};

use crate::{DestinationAddress, DestinationChainId};

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
    pub version: u8,
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

        // Skip embedded data size
        instructions.next();
        // 4. Parse each field
        let tag = instructions
            .next()
            .transpose()
            .map_err(ParserError::from)?
            .as_ref()
            .and_then(|value| value.push_bytes())
            .map(|push_bytes| push_bytes.to_owned().as_bytes().to_vec())
            .unwrap_or_default();

        let version = instructions
            .next()
            .transpose()
            .map_err(ParserError::from)?
            .as_ref()
            .and_then(|value| value.push_bytes())
            .ok_or(ParserError::InvalidEmbeddedData)?[0];

        let destination_chain_id = instructions
            .next()
            .transpose()
            .map_err(ParserError::from)?
            .as_ref()
            .map(|value| value.push_bytes())
            .flatten()
            .and_then(|push_bytes| push_bytes.as_bytes().try_into().ok())
            .ok_or(ParserError::InvalidScript)?;

        let destination_contract_address = instructions
            .next()
            .transpose()
            .map_err(ParserError::from)?
            .as_ref()
            .map(|value| value.push_bytes())
            .flatten()
            .and_then(|push_bytes| push_bytes.as_bytes().try_into().ok())
            .ok_or(ParserError::InvalidScript)?;

        let destination_recipient_address = instructions
            .next()
            .transpose()
            .map_err(ParserError::from)?
            .as_ref()
            .map(|value| value.push_bytes())
            .flatten()
            .and_then(|push_bytes| push_bytes.as_bytes().try_into().ok())
            .ok_or(ParserError::InvalidScript)?;

        Ok(VaultReturnTxOutput {
            tag,
            version,
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
    //Block height when the transaction is confirmed, is set in parser
    pub confirmed_height: u32,
    //Index of the transaction in the block, is set in parser
    pub tx_position: u32,
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
        for (_index, _txi) in tx.input.iter().enumerate() {
            //Todo: parse the transaction inputs if needed
        }
        Ok(VaultTransaction {
            txid,
            confirmed_height: 0,
            tx_position: 0,
            inputs: tx.input.clone(),
            lock_tx,
            return_tx,
            change_tx,
        })
    }
}
