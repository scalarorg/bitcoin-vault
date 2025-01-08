use std::slice;

use bitcoin::PublicKey;
use bitcoin_vault::{
    BuildUnstakingWithOnlyCovenantsParams, PreviousStakingUTXO, UnstakingOutput, VaultManager,
};

use crate::{
    create_null_buffer, ByteBuffer, PreviousStakingUTXOFFI, PublicKeyFFI, UnstakingOutputFFI,
};

#[no_mangle]
pub unsafe extern "C" fn build_with_only_covenants(
    inputs_ptr: *const PreviousStakingUTXOFFI,
    inputs_len: usize,
    outputs_ptr: *const UnstakingOutputFFI,
    outputs_len: usize,
    covenant_pub_keys_ptr: *const PublicKeyFFI,
    covenant_pub_keys_len: usize,
    covenant_quorum: u8,
    rbf: bool,
    fee_rate: u64,
) -> ByteBuffer {
    // Safety checks for null pointers
    if inputs_ptr.is_null() || outputs_ptr.is_null() || covenant_pub_keys_ptr.is_null() {
        return create_null_buffer();
    }

    // Convert raw pointers to slices
    let inputs = slice::from_raw_parts(inputs_ptr, inputs_len);
    let outputs = slice::from_raw_parts(outputs_ptr, outputs_len);
    let covenant_pub_keys = slice::from_raw_parts(covenant_pub_keys_ptr, covenant_pub_keys_len);

    let inputs: Vec<PreviousStakingUTXO> = inputs
        .iter()
        .map(|input| input.try_into().unwrap())
        .collect();

    let outputs: Vec<UnstakingOutput> = outputs.iter().map(|output| output.into()).collect();

    let covenant_pub_keys: Vec<PublicKey> = covenant_pub_keys
        .iter()
        .map(|key| PublicKey::from_slice(key.as_slice()).unwrap())
        .collect();

    // Create parameters for the unstaking function
    let params = BuildUnstakingWithOnlyCovenantsParams {
        inputs: inputs.to_vec(),
        unstaking_outputs: outputs.to_vec(),
        covenant_pub_keys: covenant_pub_keys.to_vec(),
        covenant_quorum,
        rbf,
        fee_rate,
    };

    // Create a VaultManager instance
    let vault_manager = VaultManager::new(); // Assuming a constructor exists

    // Call the build_with_only_covenants function
    match vault_manager.build_with_only_covenants(&params) {
        Ok(psbt) => {
            // Serialize the PSBT and return it as a ByteBuffer
            let psbt_bytes = psbt.serialize();
            let mut output = Vec::with_capacity(psbt_bytes.len());
            output.extend_from_slice(&psbt_bytes);
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
