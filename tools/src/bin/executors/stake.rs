use bitcoin::Transaction;
use bitcoin_vault::{AccountEnv, SuiteAccount, TaprootTreeType, TestSuite};

use crate::commands::StakingCmdParams;

pub struct StakingExecutor;

impl StakingExecutor {
    pub fn execute_staking(
        suite: &TestSuite,
        network: String,
        params: &StakingCmdParams,
        tree_type: TaprootTreeType,
    ) -> Result<Transaction, String> {
        let raw_account = SuiteAccount::new(AccountEnv {
            private_key: params.private_key.clone(),
            address: params.address.clone(),
            network: network,
        });

        suite.prepare_staking_tx(params.amount, tree_type, raw_account)
    }
}
