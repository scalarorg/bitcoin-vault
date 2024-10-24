use bitcoin::{Amount, OutPoint};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct UTXO {
    pub outpoint: OutPoint, // txid, vout
    pub amount_in_sats: Amount,
}
