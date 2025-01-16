use bitcoin::taproot::TaprootBuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Insufficient UTXOs: required {required}, available {available}")]
    InsufficientUTXOs { required: u64, available: u64 },
    #[error("Invalid tag")]
    InvalidTag,
    #[error("Invalid service tag")]
    InvalidServiceTag,
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
    #[error("Invalid embedded data")]
    InvalidEmbeddedData,
    #[error("Invalid control block")]
    InvalidControlBlock,
    #[error("Invalid script")]
    InvalidScript,
    #[error("No embedded data")]
    NoEmbeddedData,
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),
    #[error("Invalid secp256k1 public key")]
    InvalidSecp256k1PublicKey,
    #[error("Signing psbt failed: {0}")]
    SigningPSBTFailed(String),
    #[error("Failed to extract tx")]
    FailedToExtractTx,
    #[error("Invalid unstaking type")]
    InvalidUnstakingType,
    #[error("Cannot convert op_return data to slice")]
    CannotConvertOpReturnDataToSlice,
    #[error("Invalid taproot tree type")]
    InvalidTaprootTreeType,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Mismatch between number of inputs and tap script sigs")]
    MismatchBetweenNumberOfInputsAndTapScriptSigs,
    #[error("Invalid signature size, just support Sighash Default")]
    InvalidSignatureSize,
    #[error("Failed to encode leaf hash")]
    FailedToEncodeLeafHash,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid leaf hash")]
    InvalidLeafHash,
}
