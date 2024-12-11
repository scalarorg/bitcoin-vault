use bitcoin::NetworkKind;

use crate::{ByteBuffer, TapScriptSigFFIArray};

pub(crate) fn network_from_byte(network: u8) -> Option<NetworkKind> {
    match network {
        0 => Some(NetworkKind::Main),
        1 => Some(NetworkKind::Test),
        _ => None,
    }
}

pub(crate) fn create_null_tap_script_sig_array() -> TapScriptSigFFIArray {
    TapScriptSigFFIArray {
        data: std::ptr::null_mut(),
        len: 0,
    }
}

pub(crate) fn create_null_buffer() -> ByteBuffer {
    ByteBuffer {
        data: std::ptr::null_mut(),
        len: 0,
    }
}
