use bitcoin::{Amount, OutPoint};

pub struct UTXO {
    pub outpoint: OutPoint, // txid, vout
    pub amount_in_sats: Amount,
}
