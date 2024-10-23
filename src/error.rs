use thiserror::Error;

#[derive(Error, Debug)]
pub enum StakingError {
    #[error("Insufficient UTXOs: required {required}, available {available}")]
    InsufficientUTXOs { required: u64, available: u64 },
    #[error("Invalid quorum: {0}")]
    InvalidQuorum(u8),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}