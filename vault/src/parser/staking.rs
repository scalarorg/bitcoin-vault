use bitcoin::{Network, Transaction};

use super::{error::ParserError, types::StakingData};

pub trait ParsingStaking<Data> {
    fn parse(&self, tx_hex: &Transaction) -> Result<Data, ParserError>;
}

pub struct StakingParser {
    network: Network,
    tag: Vec<u8>,
    version: u8,
}
impl StakingParser {
    pub fn new(network: Network, tag: Vec<u8>, version: u8) -> Self {
        Self {
            network,
            tag,
            version,
        }
    }
}

impl ParsingStaking<StakingData> for StakingParser {
    fn parse(&self, tx_hex: &Transaction) -> Result<StakingData, ParserError> {
        Ok(StakingData {
            tx_id: tx_hex.compute_txid(),
            vout: 0,
            amount: 0,
        })
    }
}
