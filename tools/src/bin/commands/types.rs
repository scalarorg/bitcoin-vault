use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommandStatus {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandResult {
    pub txid: Option<String>,
    pub status: CommandStatus,
    pub error: Option<String>,
}

impl CommandResult {
    pub fn new(txid: Option<String>, status: CommandStatus, error: Option<String>) -> Self {
        Self {
            txid,
            status,
            error,
        }
    }
}

impl Default for CommandResult {
    fn default() -> Self {
        Self {
            txid: None,
            status: CommandStatus::Success,
            error: None,
        }
    }
}

