use crate::{
    DestinationChain, DestinationRecipientAddress, DestinationTokenAddress, TaprootTreeType,
    UnlockingTaprootTreeType, CUSTODIAN_QUORUM_SIZE, DEST_CHAIN_SIZE, DEST_RECIPIENT_ADDRESS_SIZE,
    DEST_TOKEN_ADDRESS_SIZE, FLAGS_SIZE, HASH_SIZE, NETWORK_ID_SIZE, SEQUENCE_SIZE,
    SERVICE_TAG_HASH_SIZE, TAG_HASH_SIZE, VERSION_SIZE,
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
    Unlocking,
    Locking,
}

impl Default for VaultReturnTxOutputType {
    fn default() -> Self {
        Self::Unlocking
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
    pub session_sequence: u64,
    pub custodian_group_uid: [u8; HASH_SIZE],
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

        match UnlockingTaprootTreeType::try_from(flags) {
            Ok(UnlockingTaprootTreeType::CustodianOnlyBranch) => {
                let service_tag = read_bytes(bytes, &mut cursor, SERVICE_TAG_HASH_SIZE)?;
                let session_sequence = read_bytes(bytes, &mut cursor, SEQUENCE_SIZE)?;
                let session_sequence = u64::from_be_bytes(session_sequence.try_into().unwrap());
                let custodian_group_uid = read_bytes(bytes, &mut cursor, HASH_SIZE)?;
                Ok(VaultReturnTxOutput {
                    tag: tag.try_into().unwrap(),
                    version,
                    network_id,
                    flags,
                    service_tag: service_tag.try_into().unwrap(),
                    transaction_type: VaultReturnTxOutputType::Unlocking,
                    session_sequence,
                    custodian_group_uid: custodian_group_uid.try_into().unwrap(),
                    ..Default::default()
                })
            }
            Ok(UnlockingTaprootTreeType::UPCBranch) => {
                let service_tag = read_bytes(bytes, &mut cursor, SERVICE_TAG_HASH_SIZE)?;
                Ok(VaultReturnTxOutput {
                    tag: tag.try_into().unwrap(),
                    version,
                    network_id,
                    flags,
                    service_tag: service_tag.try_into().unwrap(),
                    transaction_type: VaultReturnTxOutputType::Unlocking,
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
                    transaction_type: VaultReturnTxOutputType::Locking,
                    custodian_quorum,
                    destination_chain,
                    destination_token_address,
                    destination_recipient_address,
                    ..Default::default()
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
            VaultReturnTxOutputType::Unlocking => Ok(VaultTransaction {
                txid,
                tx_content: hex::encode(tx_content),
                inputs: tx.input.clone(),
                return_tx,
                lock_tx: None,
                change_tx: None,
            }),
            VaultReturnTxOutputType::Locking => {
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
    fn test_op_return_data() {
        let script_pubkey = "6a365343414c4152030141706f6f6c730000000000000001bffb71bf819ae4cb65188905ac54763a09144bc3a0629808d7142dd5dbd98693";

        let script_pubkey = hex::decode(script_pubkey).unwrap();

        let vault_return_tx = VaultReturnTxOutput::try_from_script_pubkey(&script_pubkey).unwrap();
        println!("Vault return tx: {:?}", vault_return_tx);
    }

    #[test]
    fn test_upc_vault_transaction() {
        let tx: Transaction = bitcoin::consensus::encode::deserialize_hex("020000000001011713e20bd169b9fe7afd16831989b4a893945150c40f252047cf58b7acaffcfa0000000000fdffffff020000000000000000106a0e5343414c41520101816c696768740d2600000000000016001450dceca158a9c872eb405d52293d351110572c9e044016deab9d5ceeea9869c16cb4b45db9df30cff5b3aca61f36edf59efbc055eb4f66776cf6cde51737041e27b978ea17459ea6b07e36fc55bfea6ac5240245e9d440ea00e8a6f2e2ba01839405c1c9e5ee192a659fa35505019f330d4447dfd7be3e3c36fe68b018faa7a13c58b6ad5e0d259625907023b081a7e09728012a1c371f44202ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350aad201387aab21303782b17e760c670432559df3968e52cb82cc2d8f9be43a227d5dcac41c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac02e1a575a04d7b56bd92189dd89ac259caf7bc23f45035afab9fa81e45c45443b00000000").unwrap();

        println!("Tx: {:?}", tx);

        let vault_tx = VaultTransaction::try_from(&tx).unwrap();
        println!("Vault tx: {:?}", vault_tx);
    }

    #[test]
    fn test_parser() {
        struct TestSuite {
            hex: String,
            name: String,
        }

        let suites = vec![
            TestSuite{
                name: "Bridge".to_string(),
                hex: "020000000001024de91ad32144227ebf8eb967df972cd688723c1509c71e718b7349dc82a8ac590200000000ffffffffc5f0f4f755c06d31fae9b78a96b2caece6e43e035bf357ddfccc01886ecd8e4f0500000000ffffffff030000000000000000416a3f5343414c4152030140706f6f6c73030100000000aa36a7e2eac602f56b2f50685f2c76496c8068f0ed11334fab6cb4c6e8b72f1529eda3e71f45127a85d4440d260000000000002251207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d55e9701000000000016001463dc22751d9a7778aa4450ceeb0b5c3ee214401c024730440220795993f015500bfa1ad952691e2177428a98b24fe02d82ee4ed0535f9677a9dd0220558677cbcdf44d719e5bfaa3948aae01e20203cbae54bae0c91ddcf65773cdc90121020d1acabc9af43d39d064effd67fcec44125d7b5cf20ff23e34406fae475fc7aa02483045022100c0f2f86fa3ba7bc603c542f92c4ff73bd55eddd90004f23012284703f678493d02207f92c0eeafabcde1c2b4ea555c324da982e794f72ec46ab524cd05044038af0f0121020d1acabc9af43d39d064effd67fcec44125d7b5cf20ff23e34406fae475fc7aa00000000".to_string(),
            },
            TestSuite{
                name: "Redeem".to_string(),
                hex: "0200000000010152c0173d62c0c6a79ab2da183f059580fd996c24727894d3e0f6cf36a3cb77730000000000ffffffff020000000000000000386a365343414c4152030141706f6f6c730000000000000001bffb71bf819ae4cb65188905ac54763a09144bc3a0629808d7142dd5dbd98693ed0200000000000016001450dceca158a9c872eb405d52293d351110572c9e0640036c9c2a5cdbf05b14f61a03cd3d698646f0fedc141832f2055762c51e1b1a45f9232cdbe6f037be69f2269d9eb7ec6ea938572903438625976c648fffeb4355405a9045de24ae00f7c71a0f0d4939ee3243f4b865594c419559a618473fafac89e41ad91f1ea6866735ebeeaf43488a6882167e43db03adf47bee24c29c32ac0040e015388ad24fd8a4b67bf05c49a05502ca80dfa2a0befb45655a99a4f5be0e3bbbcfb1861f04c880da45b6680c8c807897cefea53112451e117bfc9554d35f04008a2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a221c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac000000000".to_string(),
            }
        ];

        for suite in suites {
            println!("TEST: {}", suite.name);
            let tx = bitcoin::consensus::encode::deserialize_hex(&suite.hex).unwrap();
            let vault_tx = VaultTransaction::try_from(&tx).unwrap();
            println!("Vault tx: {:?}", vault_tx);
        }
    }
}
