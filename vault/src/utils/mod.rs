pub mod account;
pub mod dest_info;
pub mod env;
pub mod helper;
pub mod suite;

pub use account::*;
use bitcoin::Amount;
pub use dest_info::*;
pub use env::*;
pub use helper::*;
use serde::{Deserialize, Serialize};
pub use suite::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeededUtxo {
    pub txid: bitcoin::Txid,
    pub vout: u32,
    pub amount: Amount,
}
