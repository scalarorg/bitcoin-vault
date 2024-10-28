use bitcoin::{Amount, OutPoint, ScriptBuf};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct UTXO {
    pub outpoint: OutPoint, // txid, vout
    pub amount_in_sats: Amount,
}

#[derive(Debug, Validate)]
pub struct PreviousStakingUTXO {
    pub outpoint: OutPoint, // txid, vout
    pub amount_in_sats: Amount,
    pub script_pubkey: ScriptBuf,
}
