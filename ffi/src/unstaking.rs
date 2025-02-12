use std::slice;

use bitcoin::PublicKey;
use vault::{CustodianOnlyUnstakingParams, PreviousStakingUTXO, UnstakingOutput, VaultManager};

use vault::Unstaking;

use crate::{
    create_null_buffer, ByteBuffer, PreviousStakingUTXOFFI, PublicKeyFFI, UnstakingOutputFFI,
};

/// # Safety
///
/// This function is unsafe because it uses raw pointers and assumes that the caller has
/// provided valid pointers and lengths for the inputs and outputs.
#[no_mangle]
pub unsafe extern "C" fn build_custodian_only(
    tag: *const u8,
    tag_len: usize,
    service_tag: *const u8,
    service_tag_len: usize,
    version: u8,
    network_kind: u8,

    inputs_ptr: *const PreviousStakingUTXOFFI,
    inputs_len: usize,
    outputs_ptr: *const UnstakingOutputFFI,
    outputs_len: usize,
    custodian_pub_keys_ptr: *const PublicKeyFFI,
    custodian_pub_keys_len: usize,
    custodian_quorum: u8,
    rbf: bool,
    fee_rate: u64,
) -> ByteBuffer {
    // Safety checks for null pointers
    if inputs_ptr.is_null() || outputs_ptr.is_null() || custodian_pub_keys_ptr.is_null() {
        return create_null_buffer();
    }

    // Convert raw pointers to slices
    let tag = slice::from_raw_parts(tag, tag_len);
    let service_tag = slice::from_raw_parts(service_tag, service_tag_len);

    let inputs = slice::from_raw_parts(inputs_ptr, inputs_len);
    let outputs = slice::from_raw_parts(outputs_ptr, outputs_len);
    let custodian_pub_keys = slice::from_raw_parts(custodian_pub_keys_ptr, custodian_pub_keys_len);

    let inputs: Vec<PreviousStakingUTXO> = inputs
        .iter()
        .map(|input| input.try_into().unwrap())
        .collect();

    let outputs: Vec<UnstakingOutput> = outputs.iter().map(|output| output.into()).collect();

    let custodian_pub_keys: Vec<PublicKey> = custodian_pub_keys
        .iter()
        .map(|key| PublicKey::from_slice(key.as_slice()).unwrap())
        .collect();

    // Create parameters for the unstaking function
    let params = CustodianOnlyUnstakingParams {
        inputs: inputs.to_vec(),
        unstaking_outputs: outputs.to_vec(),
        custodian_pub_keys: custodian_pub_keys.to_vec(),
        custodian_quorum,
        rbf,
        fee_rate,
    };

    // Create a VaultManager instance
    let vault_manager =
        VaultManager::new(tag.to_vec(), service_tag.to_vec(), version, network_kind); // Assuming a constructor exists

    // Call the build_custodian_only function
    match vault_manager.build_custodian_only(&params) {
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
