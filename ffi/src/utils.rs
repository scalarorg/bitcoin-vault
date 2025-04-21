use bitcoin::{Amount, NetworkKind, ScriptBuf, TxOut};

use crate::ByteBuffer;

pub(crate) fn network_from_byte(network: u8) -> Option<NetworkKind> {
    match network {
        0 => Some(NetworkKind::Main),
        1 => Some(NetworkKind::Test),
        _ => None,
    }
}

pub(crate) fn create_null_buffer() -> ByteBuffer {
    ByteBuffer {
        data: std::ptr::null_mut(),
        len: 0,
    }
}

pub(crate) fn convert_vec_to_txout(value: &[u8]) -> Result<TxOut, anyhow::Error> {
    let amount = u64::from_be_bytes(value[0..8].try_into().unwrap());
    Ok(TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: ScriptBuf::from_bytes(value[8..].to_vec()),
    })
}
