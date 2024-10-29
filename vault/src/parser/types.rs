use bitcoin::{Network, Txid};

pub struct ParserConfig {
    network: Network,
    tag: Vec<u8>,
    version: u8,
}
impl ParserConfig {
    pub fn new(network: Network, tag: Vec<u8>, version: u8) -> Self {
        Self {
            network,
            tag,
            version,
        }
    }
}

pub struct StakingData {
    pub tx_id: Txid,
    pub vout: u32,
    pub amount: u64,
}
