use bitcoin_vault::types::VaultReturnTxOutput;
use std::slice;

use crate::ByteBuffer;

/// # Safety
///
/// This function is unsafe because it uses raw pointers and assumes that the caller has
/// provided valid pointers and lengths for the inputs and outputs.
#[no_mangle]
pub unsafe extern "C" fn parse_vault_embedded_data(
    script_pubkey: *const u8,
    script_pubkey_len: usize,
) -> ByteBuffer {
    if script_pubkey.is_null() {
        return ByteBuffer {
            data: std::ptr::null_mut(),
            len: 0,
        };
    }

    let script_slice = unsafe { slice::from_raw_parts(script_pubkey, script_pubkey_len) };

    let result = match VaultReturnTxOutput::try_from_script_pubkey(script_slice) {
        Ok(output) => output,
        Err(_) => {
            return ByteBuffer {
                data: std::ptr::null_mut(),
                len: 0,
            }
        }
    };

    let json = match serde_json::to_vec(&result) {
        Ok(json) => json,
        Err(_) => {
            return ByteBuffer {
                data: std::ptr::null_mut(),
                len: 0,
            }
        }
    };

    let mut output = Vec::with_capacity(json.len());
    output.extend_from_slice(&json);
    let buffer = ByteBuffer {
        data: output.as_mut_ptr(),
        len: output.len(),
    };
    std::mem::forget(output);
    buffer
}
