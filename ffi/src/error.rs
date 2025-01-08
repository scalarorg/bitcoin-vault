use thiserror::Error;

#[derive(Error, Debug)]
pub enum FFIError {
    #[error("Invalid Txid")]
    InvalidTxid,
}
