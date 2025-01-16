use bitcoin::taproot::TaprootBuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Insufficient UTXOs: required {required}, available {available}")]
    InsufficientUTXOs { required: u64, available: u64 },
    #[error("Invalid tag")]
    InvalidTag,
    #[error("Duplicate custodian keys")]
    DuplicateCustodianKeys,
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
    #[error("Taproot builder error: {0}")]
    TaprootBuilderError(#[from] TaprootBuilderError),
    #[error("Taproot finalization failed")]
    TaprootFinalizationFailed,
    #[error("Failed to create PSBT")]
    FailedToCreatePSBT,
    #[error("Control block not found")]
    ControlBlockNotFound,
    #[error("Invalid transaction hex")]
    InvalidTransactionHex,
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Invalid embedded data")]
    InvalidEmbeddedData,
    #[error("Invalid control block")]
    InvalidControlBlock,
    #[error("Invalid script")]
    InvalidScript,
    #[error("No embedded data")]
    NoEmbeddedData,
}

impl From<bitcoin::script::Error> for ParserError {
    fn from(_err: bitcoin::script::Error) -> Self {
        ParserError::InvalidScript
    }
}
