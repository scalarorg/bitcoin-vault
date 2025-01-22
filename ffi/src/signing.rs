use bitcoin::Psbt;
use bitcoin_vault::TapScriptSigsMap;
use bitcoin_vault::{Signing, VaultManager};
use std::slice;

use crate::create_null_buffer;
use crate::network_from_byte;
use crate::ByteBuffer;

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
        return create_null_buffer();
    }

    // Convert raw pointers to slices
    let psbt_slice = unsafe { slice::from_raw_parts(psbt_bytes, psbt_len) };

    let privkey_slice = unsafe { slice::from_raw_parts(privkey_bytes, privkey_len) };

    // Parse PSBT
    let mut psbt = match Psbt::deserialize(psbt_slice) {
        Ok(psbt) => psbt,
        Err(_) => return create_null_buffer(),
    };

    // Convert network byte to NetworkKind
    let network_kind = match network_from_byte(network) {
        Some(n) => n,
        None => return create_null_buffer(),
    };

    // Sign PSBT
    let (signed_psbt, _) = match VaultManager::sign_psbt_by_single_key(
        &mut psbt,
        privkey_slice,
        network_kind,
        finalize,
    ) {
        Ok(signed_psbt) => signed_psbt,
        Err(_) => return create_null_buffer(),
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

/// Signs a PSBT and collects all Taproot script signatures
///
/// # Safety
///
/// This function is unsafe because it:
/// - Dereferences raw pointers (`psbt_bytes` and `privkey_bytes`)
/// - Assumes the provided lengths match the actual data
/// - Returns a pointer that must be freed using `free_tap_script_sig_array`
#[no_mangle]
pub unsafe extern "C" fn sign_psbt_and_collect_sigs(
    psbt_bytes: *const u8,
    psbt_len: usize,
    privkey_bytes: *const u8,
    privkey_len: usize,
    network: u8,
) -> ByteBuffer {
    // Safety checks
    if psbt_bytes.is_null() || privkey_bytes.is_null() {
        return create_null_buffer();
    }

    // Convert raw pointers to slices
    let psbt_slice = slice::from_raw_parts(psbt_bytes, psbt_len);

    let privkey_slice = slice::from_raw_parts(privkey_bytes, privkey_len);

    // Parse PSBT
    let mut psbt = match Psbt::deserialize(psbt_slice) {
        Ok(psbt) => psbt,
        Err(_) => return create_null_buffer(),
    };

    // Convert network byte
    let network_kind = match network_from_byte(network) {
        Some(n) => n,
        None => return create_null_buffer(),
    };

    // Sign and collect signatures
    let tap_script_sigs = match VaultManager::sign_psbt_and_collect_tap_script_sigs(
        &mut psbt,
        privkey_slice,
        network_kind,
    ) {
        Ok(sigs) => sigs,
        Err(_) => return create_null_buffer(),
    };

    let json = match serde_json::to_vec(&tap_script_sigs) {
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

/// # Safety
///
/// This function is unsafe because it uses raw pointers and assumes that the caller has
/// provided valid pointers and lengths for the inputs and outputs.
#[no_mangle]
pub unsafe extern "C" fn aggregate_tap_script_sigs(
    psbt_bytes: *const u8,
    psbt_len: usize,
    tap_script_sigs_map_bytes: *const u8,
    tap_script_sigs_map_len: usize,
) -> ByteBuffer {
    // Safety checks for null pointers
    if psbt_bytes.is_null() || tap_script_sigs_map_bytes.is_null() {
        return create_null_buffer();
    }

    // Convert raw pointers to slices
    let psbt_slice = slice::from_raw_parts(psbt_bytes, psbt_len);
    let tap_script_sigs_map_slice =
        slice::from_raw_parts(tap_script_sigs_map_bytes, tap_script_sigs_map_len);

    // Parse PSBT
    let mut psbt = match Psbt::deserialize(psbt_slice) {
        Ok(psbt) => psbt,
        Err(_) => return create_null_buffer(),
    };

    // Convert FFI TapScriptSigs to internal TapScriptSig format
    let tap_script_sigs_map: TapScriptSigsMap =
        serde_json::from_slice(tap_script_sigs_map_slice).unwrap_or_default();

    // Aggregate signatures
    if VaultManager::aggregate_tap_script_sigs(&mut psbt, &tap_script_sigs_map).is_err() {
        return create_null_buffer();
    }

    let psbt_hex = match VaultManager::aggregate_tap_script_sigs(&mut psbt, &tap_script_sigs_map) {
        Ok(psbt_hex) => psbt_hex,
        Err(_) => return create_null_buffer(),
    };

    // Allocate and copy the result
    let mut output = Vec::with_capacity(psbt_hex.len());
    output.extend_from_slice(&psbt_hex);
    let buffer = ByteBuffer {
        data: output.as_mut_ptr(),
        len: output.len(),
    };
    std::mem::forget(output); // Prevent deallocation
    buffer
}

/// # Safety
///
/// This function is unsafe because it uses raw pointers and assumes that the caller has
/// provided valid pointers and lengths for the inputs and outputs.
#[no_mangle]
pub unsafe extern "C" fn finalize_psbt_and_extract_tx(
    psbt_bytes: *const u8,
    psbt_len: usize,
) -> ByteBuffer {
    if psbt_bytes.is_null() {
        return create_null_buffer();
    }

    let psbt_slice = slice::from_raw_parts(psbt_bytes, psbt_len);

    let mut psbt = match Psbt::deserialize(psbt_slice) {
        Ok(psbt) => psbt,
        Err(_) => return create_null_buffer(),
    };

    let tx_hex = match VaultManager::finalize_psbt_and_extract_tx(&mut psbt) {
        Ok(tx_hex) => tx_hex,
        Err(_) => return create_null_buffer(),
    };

    // Allocate and copy the result
    let mut output = Vec::with_capacity(tx_hex.len());
    output.extend_from_slice(&tx_hex);
    let buffer = ByteBuffer {
        data: output.as_mut_ptr(),
        len: output.len(),
    };
    std::mem::forget(output); // Prevent deallocation
    buffer
}
