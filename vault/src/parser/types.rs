use bitcoin::Network;

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
