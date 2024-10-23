use bitcoin::Network;
use lazy_static::lazy_static;
// use staking::StakingManager;

mod error;
mod musig2;
// mod staking;
// mod utxo;

lazy_static! {
    pub static ref CONFIG: Config = Config::new(None);
}
#[derive(Debug)]

pub struct Config {
    pub network: Network,
}

impl Config {
    pub fn new(network: Option<Network>) -> Self {
        Self {
            network: network.unwrap_or(Network::Testnet),
        }
    }
}

#[cfg(test)]
mod tests {

    // use staking::CreateStakingParams;

    // use super::*;

    // #[test]
    // fn test_staking() {
    //     let tx_hex = StakingManager::create(CreateStakingParams {
    //         user_priv_key: "".to_string(),
    //         protocol_pub_key: "".to_string(),
    //         covenant_pubkeys: vec![],
    //         covenant_quorum: 0,
    //         staking_amount: 0,
    //         utxos: vec![],
    //         fee_rate: 0,
    //         reciever_address: "".to_string(),
    //     });
    //     println!("txHex: {}", tx_hex);
    // }
}
