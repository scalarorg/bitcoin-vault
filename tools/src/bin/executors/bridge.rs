use vault::{
    AccountEnv, DestinationInfo, DestinationInfoEnv, NeededUtxo, SuiteAccount, TaprootTreeType,
};

use crate::{
    commands::{BridgeCmdParams, CommandResult, CommandStatus},
    db::CommandHistory,
    TvlMaker,
};

pub struct BridgeExecutor;

impl BridgeExecutor {
    pub fn execute_bridge(
        tvl_maker: &TvlMaker,
        command_name: &str,
        params: &BridgeCmdParams,
        tree_type: TaprootTreeType,
        utxo: NeededUtxo,
    ) -> anyhow::Result<CommandHistory> {
        let raw_account = SuiteAccount::new(AccountEnv {
            private_key: params.private_key.clone(),
            address: params.wallet_address.clone(),
            network: tvl_maker.suite.env().network.to_string(),
        });

        let destination_info = DestinationInfo::new(DestinationInfoEnv {
            destination_chain: params.destination_chain.clone(),
            destination_token_address: params.destination_token_address.clone(),
            destination_recipient_address: params.destination_recipient_address.clone(),
        });

        let result = tvl_maker.suite.prepare_staking_tx(
            params.amount,
            tree_type,
            raw_account,
            destination_info,
            utxo,
        );

        let result = result
            .map(|tx| {
                CommandResult::new(
                    Some(tx.compute_txid().to_string()),
                    CommandStatus::Success,
                    None,
                )
            })
            .unwrap_or_else(|e| CommandResult::new(None, CommandStatus::Error, Some(e)));

        // Serialize params once for command history
        let command_history_params = serde_json::to_string(params).unwrap();

        // Create and store command history
        let command_history = CommandHistory::new(
            command_name.to_string(),
            Some(serde_json::to_string(tvl_maker.suite.env()).unwrap()),
            Some(command_history_params),
            Some(serde_json::to_string(&result).unwrap()),
        );

        Ok(command_history)
    }
}
