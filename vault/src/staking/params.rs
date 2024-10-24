use bitcoin::{PublicKey, ScriptBuf};

use super::{ADDRESS_SIZE, CHAIN_ID_SIZE, UTXO};

pub type DestinationAddress = [u8; ADDRESS_SIZE];

pub type DestinationChainId = [u8; CHAIN_ID_SIZE];

pub struct CreateStakingParams {
    pub user_pub_key: PublicKey,
    pub protocol_pub_key: PublicKey,
    pub covenant_pubkeys: Vec<PublicKey>,
    pub covenant_quorum: u8,
    pub staking_amount: u64,
    pub utxos: Vec<UTXO>,
    pub script_pubkey: ScriptBuf,
    pub rbf: bool,
    pub fee_rate: u64, // in sat/vbyte
    pub have_only_covenants: bool,
    pub destination_chain_id: DestinationChainId,
    pub destination_address: DestinationAddress,
    pub destination_recipient_address: DestinationAddress,
}
