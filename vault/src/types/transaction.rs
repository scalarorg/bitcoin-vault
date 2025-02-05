use crate::{
    DestinationChain, DestinationRecipientAddress, DestinationTokenAddress, TaprootTreeType,
    UnstakingTaprootTreeType, CUSTODIAN_QUORUM_SIZE, DEST_CHAIN_SIZE, DEST_RECIPIENT_ADDRESS_SIZE,
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
    pub custodian_quorum: u8,
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
            Ok(UnstakingTaprootTreeType::CustodianOnly | UnstakingTaprootTreeType::UPCBranch) => {
                let service_tag = read_bytes(bytes, &mut cursor, SERVICE_TAG_HASH_SIZE)?;
                Ok(VaultReturnTxOutput {
                    tag: tag.try_into().unwrap(),
                    version,
                    network_id,
                    flags,
                    service_tag: service_tag.try_into().unwrap(),
                    transaction_type: VaultReturnTxOutputType::Unstaking,
                    ..Default::default()
                })
            }
            Err(_) => {
                let tree_type = TaprootTreeType::try_from(flags)
                    .map_err(|_| ParserError::InvalidScript(format!("Invalid flags: {}", flags)))?;

                let service_tag = read_bytes(bytes, &mut cursor, SERVICE_TAG_HASH_SIZE)?;

                // Read custodian_quorum
                let custodian_quorum = read_bytes(bytes, &mut cursor, CUSTODIAN_QUORUM_SIZE)?[0];

                // Read destination_chain_id
                let destination_chain = read_bytes(bytes, &mut cursor, DEST_CHAIN_SIZE)?
                    .try_into()
                    .map_err(|_| {
                        ParserError::InvalidScript("Invalid destination chain".to_string())
                    })?;

                // Read destination_contract_address
                let destination_token_address =
                    read_bytes(bytes, &mut cursor, DEST_TOKEN_ADDRESS_SIZE)?
                        .try_into()
                        .map_err(|_| {
                            ParserError::InvalidScript(
                                "Invalid destination token address:".to_string(),
                            )
                        })?;

                // Read destination_recipient_address
                let destination_recipient_address =
                    read_bytes(bytes, &mut cursor, DEST_RECIPIENT_ADDRESS_SIZE)?
                        .try_into()
                        .map_err(|_| {
                            ParserError::InvalidScript(
                                "Invalid destination recipient address".to_string(),
                            )
                        })?;
                //Check if no extra bytes left
                if cursor != bytes.len() {
                    return Err(ParserError::InvalidScript("Invalid script".to_string()));
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
                    custodian_quorum,
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
    pub return_tx: VaultReturnTxOutput,
    pub lock_tx: Option<VaultLockTxOutput>,
    pub change_tx: Option<VaultChangeTxOutput>,
}

impl TryFrom<&Transaction> for VaultTransaction {
    type Error = ParserError;
    fn try_from(tx: &Transaction) -> Result<Self, Self::Error> {
        //1. Validate the transaction if it's a staking transaction
        if tx.output.len() < 2 {
            return Err(ParserError::InvalidTransactionHex);
        }
        let txid = tx.compute_txid();
        let mut tx_content = vec![];
        tx.consensus_encode(&mut tx_content).unwrap();

        //2. Parse the op_return data
        let return_tx = VaultReturnTxOutput::try_from(&tx.output[0])?;

        match return_tx.transaction_type {
            VaultReturnTxOutputType::Unstaking => Ok(VaultTransaction {
                txid,
                tx_content: hex::encode(tx_content),
                inputs: tx.input.clone(),
                return_tx,
                lock_tx: None,
                change_tx: None,
            }),
            VaultReturnTxOutputType::Staking => {
                //2. Parse the transaction locking outputs
                let lock_tx = VaultLockTxOutput::from(&tx.output[1]);

                let change_tx = if tx.output.len() == 3 {
                    Some(VaultChangeTxOutput::from(&tx.output[2]))
                } else {
                    None
                };

                Ok(VaultTransaction {
                    txid,
                    tx_content: hex::encode(tx_content),
                    inputs: tx.input.clone(),
                    lock_tx: Some(lock_tx),
                    return_tx,
                    change_tx,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upc_vault_transaction() {
        let tx: Transaction = bitcoin::consensus::encode::deserialize_hex("020000000001011713e20bd169b9fe7afd16831989b4a893945150c40f252047cf58b7acaffcfa0000000000fdffffff020000000000000000106a0e5343414c41520101816c696768740d2600000000000016001450dceca158a9c872eb405d52293d351110572c9e044016deab9d5ceeea9869c16cb4b45db9df30cff5b3aca61f36edf59efbc055eb4f66776cf6cde51737041e27b978ea17459ea6b07e36fc55bfea6ac5240245e9d440ea00e8a6f2e2ba01839405c1c9e5ee192a659fa35505019f330d4447dfd7be3e3c36fe68b018faa7a13c58b6ad5e0d259625907023b081a7e09728012a1c371f44202ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350aad201387aab21303782b17e760c670432559df3968e52cb82cc2d8f9be43a227d5dcac41c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac02e1a575a04d7b56bd92189dd89ac259caf7bc23f45035afab9fa81e45c45443b00000000").unwrap();

        println!("Tx: {:?}", tx);

        let vault_tx = VaultTransaction::try_from(&tx).unwrap();
        println!("Vault tx: {:?}", vault_tx);
    }

    #[test]
    fn test_custodian_only_vault_transaction() {
        let tx = bitcoin::consensus::encode::deserialize_hex("02000000000102cbd475a2d89279afc418cf2662b3438d8f5e72b4453988001b6cad82d045ab1d0000000000fdffffff637e012bd3a2b45ce9431004b311f4346ef3060f771be084ede606adb56d2c1e0000000000fdffffff040000000000000000106a0e5343344c34520001410050335033151a00000000000016001450dceca158a9c872eb405d52293d351110572c9ee70e000000000000225120a5c3659224b2caa01b5b78fb0584731e406d3419375083ed018dd02a4425fda248e20200000000002251207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d507407a41238de7d029c8cae55666675170a559d301b7e3191b6e72b267cbbe4d3a7ab4f780a695047f93fe59b1694d55cd5deb8cd3c2e7e59ce03428348eb8161061402ae138e29862ea8766c74ba92cf4114e9185b7edc10c33c8a02165e69210f7170734f2d7272e25f715f19e482b6040b96b439ef7eab6ed177b6e18c4443fb373000040c4f3bde599185d6aaf5b0652fe3140bd349aa7a529d5e1eb0c0f19430fc03c024d2f2a6eb9ec1b3a9a168fc3272334ae46e5710ff36c528569fba5db86be9c73ac2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a221c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac007403151e26374f23e138969b18659df44cc00849c0d1b1c546734cb364c4fc87f9f7b7703b49a0e1dd032c61654df8c2444ad79e8caae8ecbbd9fe1dee9d75acbd84008a06bbc577e601ae084f85f262bd8df1eb393e3f95e967742febda328943fe0f11aae782c470287a905f8cc5cf573909778d98eabd59959ee8e093aea2a221b000040fa70b540d599bf3bfa05089e5e14f3d2c3d86f355696875a49ac9fe4506774d4f40ac4893658be6fed0459b4e992467feb01ce5c009730d3bf64ad38fdd8f9b5ac2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a221c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac000000000").unwrap();

        let vault_tx = VaultTransaction::try_from(&tx).unwrap();
        println!("Vault tx: {:?}", vault_tx);
    }
}
