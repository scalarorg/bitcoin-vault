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

// /// ReversedPreviousStakingUTXO
// /// ### Description
// /// This struct is used to store the previous staking UTXO in reverse byte order of the txid.
// /// ### References: https://learnmeabitcoin.com/technical/general/byte-order/#natural-byte-order
// #[derive(Debug, Validate)]
// pub struct ReversedPreviousStakingUTXO {
//     pub outpoint: OutPoint, // txid, vout
//     pub amount_in_sats: Amount,
//     pub script_pubkey: ScriptBuf,
// }

// impl From<PreviousStakingUTXO> for ReversedPreviousStakingUTXO {
//     fn from(value: PreviousStakingUTXO) -> Self {
//         let mut txid: [u8; 32] = value.outpoint.txid.as_raw_hash().to_byte_array();

//         txid.reverse();

//         let new_outpoint = OutPoint {
//             txid: Txid::from_byte_array(txid),
//             vout: value.outpoint.vout,
//         };
//         Self {
//             outpoint: new_outpoint,
//             amount_in_sats: value.amount_in_sats,
//             script_pubkey: value.script_pubkey,
//         }
//     }
// }
