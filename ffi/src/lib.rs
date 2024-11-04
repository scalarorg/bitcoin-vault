use bitcoin::{NetworkKind, Psbt};
use bitcoin_vault::{Signing, StakingManager};
use std::slice;
mod staking;
use staking::*;

const EVM_ADDRESS_LENGTH: usize = 20;
#[repr(C)]
pub struct ByteBuffer {
    data: *mut u8,
    len: usize,
}
impl Default for ByteBuffer {
    fn default() -> Self {
        ByteBuffer {
            data: std::ptr::null_mut(),
            len: 0,
        }
    }
}
impl From<Vec<u8>> for ByteBuffer {
    fn from(mut value: Vec<u8>) -> Self {
        ByteBuffer {
            data: value.as_mut_ptr(),
            len: value.len(),
        }
    }
}
#[no_mangle]
pub extern "C" fn sign_psbt_by_single_key(
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
    let signed_psbt = match StakingManager::sign_psbt_by_single_key(
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
pub extern "C" fn free_byte_buffer(buffer: ByteBuffer) {
    if !buffer.data.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(buffer.data, buffer.len, buffer.len);
        }
    }
}

#[no_mangle]
pub extern "C" fn create_staking_psbt(
    tag: *const u8,
    tag_len: usize,
    version: u8,
    staking_amount: u64,
    staker_pubkey: *const u8,
    staker_pubkey_len: usize,
    protocol_pubkey: *const u8,
    protocol_pubkey_len: usize,
    custodial_pubkeys: *const u8,
    custodial_pubkeys_len: usize,
    covenant_quorum: i32,
    have_only_covenants: bool,
    destination_chain_id: u64,
    destination_smart_contract_address: *const u8,
    destination_recipient_address: *const u8,
) -> ByteBuffer {
    // Convert raw pointers to slices
    let tag_slice = unsafe { slice::from_raw_parts(tag, tag_len) };
    let staker_pubkey_slice = unsafe { slice::from_raw_parts(staker_pubkey, staker_pubkey_len) };
    let protocol_pubkey_slice =
        unsafe { slice::from_raw_parts(protocol_pubkey, protocol_pubkey_len) };
    let custodial_pubkeys_slice =
        unsafe { slice::from_raw_parts(custodial_pubkeys, custodial_pubkeys_len) };
    let destination_smart_contract_address_slice =
        unsafe { slice::from_raw_parts(destination_smart_contract_address, EVM_ADDRESS_LENGTH) };
    let destination_recipient_address_slice =
        unsafe { slice::from_raw_parts(destination_recipient_address, EVM_ADDRESS_LENGTH) };

    build_staking_outputs(
        tag_slice,
        version,
        staking_amount,
        staker_pubkey_slice,
        protocol_pubkey_slice,
        custodial_pubkeys_slice,
        covenant_quorum,
        have_only_covenants,
        destination_chain_id,
        destination_smart_contract_address_slice,
        destination_recipient_address_slice,
    )
    .map(ByteBuffer::from)
    .unwrap_or_default()
}
