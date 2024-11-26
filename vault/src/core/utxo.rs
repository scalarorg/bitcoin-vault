use bitcoin::{Amount, OutPoint, ScriptBuf};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct UTXO {
    pub outpoint: OutPoint, // txid, vout
    pub amount_in_sats: Amount,
}

#[derive(Debug, Validate, Clone)]
pub struct PreviousStakingUTXO {
    pub outpoint: OutPoint, // txid, vout
    pub amount_in_sats: Amount,
    pub script_pubkey: ScriptBuf,
}

#[derive(Debug, Validate, Clone)]
pub struct UnstakingOutput {
    pub locking_script: ScriptBuf,
    pub amount_in_sats: Amount,
}
