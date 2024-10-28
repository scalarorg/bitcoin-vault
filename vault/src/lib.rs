mod staking;

pub use staking::*;

#[cfg(test)]
mod utils;
#[cfg(test)]
mod tests {
    use crate::{hex_to_vec, Parsing, StakingManager};

    #[test]
    fn test_parse_embedded_data() {
        let tx_hex = "020000000001017161373459156dc2e40548b86a2d0818a8459c409355538278ad05ed46c5c3cd0000000000fdffffff03e4969800000000002251206ed59921fda3e5a9b2490dac5aea47f734432a5d2dbe5883cbb69df4796f882c00000000000000003d6a013504010203040100080000000000aa36a7141f98c06d8734d5a9ff0b53e3294626e62e4d232c14130c4810d57140e1e62967cbf742caeae91b6ece9b94b708000000001600141302a4ea98285baefb2d290de541d069356d88e90247304402205de8b44cceae9cdf6add698051f7ee171607a4e36c7df60d811f2a339263e398022072b873381018d79fd82b07ba8be52012cd9990491c6bc7274f48e5638c439d0d01210369f8edcde3c4e5e5082f7d772170bbd9803b8d4e0c788830c7227bcea8a5653400000000";
        let raw_tx = hex_to_vec!(tx_hex);

        let result = StakingManager::parse_embedded_data(raw_tx).unwrap();
        println!("{:?}", result);
    }
}
