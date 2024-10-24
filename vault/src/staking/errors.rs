use bitcoin::taproot::TaprootBuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StakingError {
    #[error("Insufficient UTXOs: required {required}, available {available}")]
    InsufficientUTXOs { required: u64, available: u64 },
    #[error("Invalid tag")]
    InvalidTag,
    #[error("Duplicate covenant keys")]
    DuplicateCovenantKeys,
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
    #[error("Taproot builder error: {0}")]
    TaprootBuilderError(#[from] TaprootBuilderError),
    #[error("Taproot finalization failed")]
    TaprootFinalizationFailed,
    #[error("Failed to create PSBT")]
    FailedToCreatePSBT,
}
