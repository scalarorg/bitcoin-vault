use bitcoin::Transaction;
use log::debug;

use crate::types::{error::ParserError, VaultTransaction};

pub trait ParsingStaking<Data> {
    fn parse(&self, tx_hex: &Transaction) -> Result<Data, ParserError>;
}

pub struct StakingParser {
    tag: Vec<u8>,
    version: u8,
}
impl StakingParser {
    pub fn new(tag: Vec<u8>, version: u8) -> Self {
        Self { tag, version }
    }
}

impl ParsingStaking<VaultTransaction> for StakingParser {
    fn parse(&self, tx: &Transaction) -> Result<VaultTransaction, ParserError> {
        let vault_tx = VaultTransaction::try_from(tx)?;
        if vault_tx.return_tx.tag != self.tag || vault_tx.return_tx.version != self.version {
            debug!(
                "Invalid tag or version. Found(tag:{:?}, version:{:?}) expected (tag: {:?}, version: {:?})",
                vault_tx.return_tx.tag, vault_tx.return_tx.version, self.tag,  self.version
            );
            return Err(ParserError::InvalidTag);
        }
        Ok(vault_tx)
    }
}
