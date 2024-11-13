use bitcoin::{NetworkKind, Psbt};
use bitcoin_vault::{types::VaultReturnTxOutput, Signing, VaultManager};
use std::slice;

#[repr(C)]
pub struct ByteBuffer {
    data: *mut u8,
    len: usize,
}

/// Signs a PSBT using a single private key
///
/// # Safety
///
/// This function is unsafe because it:
/// - Dereferences raw pointers (`psbt_bytes` and `privkey_bytes`)
/// - Assumes the provided lengths (`psbt_len` and `privkey_len`) match the actual data
/// - Caller must ensure that:
///   - The pointers are valid and properly aligned
///   - The memory they point to is valid for the given lengths
///   - The memory remains valid for the duration of the function call
///   - The lengths do not exceed the actual allocated memory
#[no_mangle]
pub unsafe extern "C" fn sign_psbt_by_single_key(
    psbt_bytes: *const u8,
    psbt_len: usize,
    privkey_bytes: *const u8,
    privkey_len: usize,
    network: u8,
    finalize: bool,
) -> ByteBuffer {
    // Safety checks for null pointers
    if psbt_bytes.is_null() || privkey_bytes.is_null() {
        return ByteBuffer {
            data: std::ptr::null_mut(),
            len: 0,
        };
    }

    // Convert raw pointers to slices
    let psbt_slice = unsafe { slice::from_raw_parts(psbt_bytes, psbt_len) };

    let privkey_slice = unsafe { slice::from_raw_parts(privkey_bytes, privkey_len) };

    // Parse PSBT
    let mut psbt = match Psbt::deserialize(psbt_slice) {
        Ok(psbt) => psbt,
        Err(_) => {
            return ByteBuffer {
                data: std::ptr::null_mut(),
                len: 0,
            }
        }
    };

    // Convert network byte to NetworkKind
    let network_kind = match network {
        0 => NetworkKind::Main,
        1 => NetworkKind::Test,
        _ => {
            return ByteBuffer {
                data: std::ptr::null_mut(),
                len: 0,
            }
        }
    };

    // Sign PSBT
    let signed_psbt = match VaultManager::sign_psbt_by_single_key(
        &mut psbt,
        privkey_slice,
        network_kind,
        finalize,
    ) {
        Ok(signed_psbt) => signed_psbt,
        Err(_) => {
            return ByteBuffer {
                data: std::ptr::null_mut(),
                len: 0,
            }
        }
    };

    // Allocate and copy the result
    let mut output = Vec::with_capacity(signed_psbt.len());
    output.extend_from_slice(&signed_psbt);
    let buffer = ByteBuffer {
        data: output.as_mut_ptr(),
        len: output.len(),
    };
    std::mem::forget(output); // Prevent deallocation
    buffer
}

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

#[no_mangle]
pub extern "C" fn free_byte_buffer(buffer: ByteBuffer) {
    if !buffer.data.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(buffer.data, buffer.len, buffer.len);
        }
    }
}
