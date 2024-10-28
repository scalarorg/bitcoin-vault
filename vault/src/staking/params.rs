use super::{PreviousStakingUTXO, ADDRESS_SIZE, CHAIN_ID_SIZE, UTXO};
use bitcoin::{PublicKey, TxOut};
use validator::Validate;

pub type DestinationAddress = [u8; ADDRESS_SIZE];

pub type DestinationChainId = [u8; CHAIN_ID_SIZE];

#[derive(Debug, Validate)]
pub struct BuildStakingOutputParams {
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub covenant_pubkeys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub staking_amount: u64,
    pub have_only_covenants: bool,
    pub destination_chain_id: DestinationChainId,
    pub destination_contract_address: DestinationAddress,
    pub destination_recipient_address: DestinationAddress,
}

#[derive(Debug, Validate)]
pub struct BuildUserProtocolSpendParams {
    pub input_utxo: PreviousStakingUTXO,
    pub unstaking_output: TxOut,
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub covenant_pubkeys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub have_only_covenants: bool,
    pub rbf: bool,
}

#[derive(Debug, Validate)]
pub struct BuildCovenantsProtocolSpendParams<'a> {
    pub input_utxo: &'a UTXO,
    pub unstaking_output: &'a TxOut,
    pub protocol_pub_key: &'a PublicKey,
    pub covenant_pubkeys: &'a [PublicKey],
    pub covenant_quorum: u8,
}

#[derive(Debug, Validate)]
pub struct BuildCovenantsUserSpendParams<'a> {
    pub input_utxo: &'a UTXO,
    pub unstaking_output: &'a TxOut,
    pub protocol_pub_key: &'a PublicKey,
    pub covenant_pubkeys: &'a [PublicKey],
    pub covenant_quorum: u8,
}

/// Struct to hold the parsed embedded data
#[derive(Debug, Clone, PartialEq)]
pub struct EmbeddedData {
    pub tag: Vec<u8>,
    pub version: u8,
    pub destination_chain_id: DestinationChainId,
    pub destination_contract_address: DestinationAddress,
    pub destination_recipient_address: DestinationAddress,
}
