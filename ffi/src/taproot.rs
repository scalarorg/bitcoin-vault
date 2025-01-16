use std::slice;

use bitcoin::{PublicKey, XOnlyPublicKey};
use bitcoin_vault::{CustodianOnlyLockingScriptParams, LockingScript};

use crate::{create_null_buffer, ByteBuffer, PublicKeyFFI};

#[no_mangle]
pub unsafe extern "C" fn only_covenants_locking_script(
    covenant_pub_keys_ptr: *const PublicKeyFFI,
    covenant_pub_keys_len: usize,
    covenant_quorum: u8,
) -> ByteBuffer {
    // Safety checks for null pointers
    if covenant_pub_keys_ptr.is_null() {
        return create_null_buffer();
    }

    let covenant_pub_keys = slice::from_raw_parts(covenant_pub_keys_ptr, covenant_pub_keys_len);

    let covenant_pub_keys: Vec<PublicKey> = covenant_pub_keys
        .iter()
        .map(|key| PublicKey::from_slice(key.as_slice()).unwrap())
        .collect();

    let covenant_x_only_pubkeys: Vec<XOnlyPublicKey> = covenant_pub_keys
        .iter()
        .map(|p| XOnlyPublicKey::from(*p))
        .collect::<Vec<_>>();

    // Create parameters for the unstaking function
    let result = LockingScript::get_custodian_only(&CustodianOnlyLockingScriptParams {
        custodian_pub_keys: &covenant_x_only_pubkeys,
        custodian_quorum: covenant_quorum,
    });

    // Call the build_with_only_covenants function
    match result {
        Ok(script) => {
            let script_bytes = script.to_bytes();
            let mut output = Vec::with_capacity(script_bytes.len());
            output.extend_from_slice(&script_bytes);
            let buffer = ByteBuffer {
                data: output.as_mut_ptr(),
                len: output.len(),
            };
            std::mem::forget(output); // Prevent deallocation
            buffer
        }
        Err(_) => create_null_buffer(),
    }
}
