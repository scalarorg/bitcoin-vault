mod decoder;
mod encoder;
pub use decoder::*;
pub use encoder::*;

#[derive(Debug, PartialEq)]
pub enum VaultABIError {
    VaultError,
    InvalidInputData,
    EncodingError,
    DecodingError(String),
    StakingError,
}

impl VaultABIError {
    pub fn description(&self) -> &str {
        match self {
            VaultABIError::InvalidInputData => "Invalid input length",
            VaultABIError::VaultError => "Error propogated from original Vault",
            VaultABIError::EncodingError => "Can't encode output",
            VaultABIError::DecodingError(message) => message.as_str(),
            VaultABIError::StakingError => "Error building staking outputs",
        }
    }
}

impl std::error::Error for VaultABIError {}

impl std::fmt::Display for VaultABIError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.description())
    }
}
