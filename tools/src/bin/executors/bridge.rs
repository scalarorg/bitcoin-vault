use bitcoin::Transaction;
use bitcoin_vault::{
    AccountEnv, DestinationInfo, DestinationInfoEnv, SuiteAccount, TaprootTreeType, TestSuite,
};

use crate::commands::BridgeCmdParams;

pub struct BridgeExecutor;

impl BridgeExecutor {
    pub fn execute_bridge(
        suite: &TestSuite,
        network: String,
        params: &BridgeCmdParams,
        tree_type: TaprootTreeType,
    ) -> Result<Transaction, String> {
        let raw_account = SuiteAccount::new(AccountEnv {
            private_key: params.private_key.clone(),
            address: params.wallet_address.clone(),
            network,
        });

        let destination_info = DestinationInfo::new(DestinationInfoEnv {
            destination_chain: params.destination_chain.clone(),
            destination_token_address: params.destination_token_address.clone(),
            destination_recipient_address: params.destination_recipient_address.clone(),
        });

        suite.prepare_staking_tx(params.amount, tree_type, raw_account, destination_info)
    }
}
