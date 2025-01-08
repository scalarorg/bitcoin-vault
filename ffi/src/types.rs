use bitcoin::{hashes::Hash, Amount, OutPoint, ScriptBuf, Txid};
use bitcoin_vault::{PreviousStakingUTXO, UnstakingOutput};
use std::slice;

use crate::FFIError;

#[repr(C)]
pub struct ByteBuffer {
    pub data: *mut u8,
    pub len: usize,
}

#[repr(C)]
pub struct TapScriptSigFFI {
    pub key_x_only: [u8; 32],
    pub leaf_hash: [u8; 32],
    pub signature: [u8; 64],
}

#[repr(C)]
pub struct TapScriptSigFFIArray {
    pub data: *mut TapScriptSigFFI,
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

type AmountFFI = u64;

#[repr(C)]
pub struct PreviousStakingUTXOFFI {
    pub outpoint: OutPointFFI,
    pub amount_in_sats: AmountFFI,
    pub script_pubkey: ScriptBufFFI,
}

impl TryInto<PreviousStakingUTXO> for &PreviousStakingUTXOFFI {
    type Error = FFIError;

    fn try_into(self) -> Result<PreviousStakingUTXO, Self::Error> {
        Ok(PreviousStakingUTXO {
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
pub struct UnstakingOutputFFI {
    pub locking_script: ScriptBufFFI,
    pub amount_in_sats: AmountFFI,
}

impl Into<UnstakingOutput> for &UnstakingOutputFFI {
    fn into(self) -> UnstakingOutput {
        UnstakingOutput {
            locking_script: ScriptBuf::from_bytes(self.locking_script.to_vec()),
            amount_in_sats: Amount::from_sat(self.amount_in_sats),
        }
    }
}

pub type PublicKeyFFI = [u8; 33];

impl ScriptBufFFI {
    pub fn to_vec(&self) -> Vec<u8> {
        unsafe { slice::from_raw_parts(self.data, self.len).to_vec() }
    }
}
