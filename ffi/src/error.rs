use thiserror::Error;

#[derive(Error, Debug)]
pub enum FFIError {
    #[error("Invalid Txid")]
    InvalidTxid,
    #[error("Failed to parse xonly public keys")]
    FailedToBuildScript,
}
