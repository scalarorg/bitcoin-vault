use bitcoin::{Amount, OutPoint, Script};

pub struct UTXO {
    pub outpoint: OutPoint, // txid, vout
    pub amount_in_sats: Amount,
}
