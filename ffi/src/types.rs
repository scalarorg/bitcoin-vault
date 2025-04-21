use bitcoin::{hashes::Hash, Amount, OutPoint, ScriptBuf, TxOut, Txid};
use std::slice;
use vault::PreviousOutpoint;

use crate::FFIError;

type AmountFFI = u64;

pub type PublicKeyFFI = [u8; 33];

#[repr(C)]
pub struct ByteBuffer {
    pub data: *mut u8,
    pub len: usize,
}

#[repr(C)]
pub struct OutPointFFI {
    pub txid: [u8; 32], // Natural order
    pub vout: u32,
}

#[repr(C)]
pub struct ScriptBufFFI {
    pub data: *mut u8,
    pub len: usize,
}

#[repr(C)]
pub struct PreviousStakingUTXOFFI {
    pub outpoint: OutPointFFI,
    pub amount_in_sats: AmountFFI,
    pub script_pubkey: ScriptBufFFI,
}

impl TryInto<PreviousOutpoint> for &PreviousStakingUTXOFFI {
    type Error = FFIError;

    fn try_into(self) -> Result<PreviousOutpoint, Self::Error> {
        Ok(PreviousOutpoint {
            outpoint: OutPoint::new(
                Txid::from_slice(self.outpoint.txid.as_slice())
                    .map_err(|_| FFIError::InvalidTxid)?,
                self.outpoint.vout,
            ),
            amount_in_sats: Amount::from_sat(self.amount_in_sats),
            script_pubkey: ScriptBuf::from_bytes(self.script_pubkey.to_vec()),
        })
    }
}

#[repr(C)]
pub struct TxOutFFI {
    pub locking_script: ScriptBufFFI,
    pub amount_in_sats: AmountFFI,
}

impl From<&TxOutFFI> for TxOut {
    fn from(ffi: &TxOutFFI) -> TxOut {
        TxOut {
            script_pubkey: ScriptBuf::from_bytes(ffi.locking_script.to_vec()),
            value: Amount::from_sat(ffi.amount_in_sats),
        }
    }
}

impl ScriptBufFFI {
    pub fn to_vec(&self) -> Vec<u8> {
        unsafe { slice::from_raw_parts(self.data, self.len).to_vec() }
    }
}
