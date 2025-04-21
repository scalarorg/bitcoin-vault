use std::slice;

use bitcoin::PublicKey;
use vault::{PreviousStakingUTXO, UnstakingOutput};

#[derive(Default, Debug)]
pub struct PoolingRedeemParams<'a> {
    pub tag: &'a [u8],
    pub service_tag: &'a [u8],
    pub version: u8,
    pub network_id: u8,
    pub inputs: Vec<PreviousStakingUTXO>,
    pub outputs: Vec<UnstakingOutput>,
    pub custodian_pub_keys: Vec<PublicKey>,
    pub custodian_quorum: u8,
    pub rbf: bool,
    pub fee_rate: u64,
    pub session_sequence: u64,
    pub custodian_group_uid: &'a [u8],
}

impl PoolingRedeemParams<'_> {
    /// # Safety
    ///
    ///
    pub unsafe fn from_buffer(buffer: *const u8, len: usize) -> Result<Self, anyhow::Error> {
        let mut ptr = 0_usize;
        // Convert *const u8 to a slice reference
        let data: &[u8] = unsafe { slice::from_raw_parts(buffer, len) };
        //Tag
        if ptr + 1 > len {
            return Err(anyhow::anyhow!("Tag is not defined"));
        }
        let tag_size = data[ptr] as usize;
        ptr += 1;
        if ptr + tag_size > len {
            return Err(anyhow::anyhow!("Tag is not defined"));
        }
        let tag = &data[ptr..ptr + tag_size];
        ptr += tag_size * size_of::<u8>();
        //Service tag
        if ptr + 1 > len {
            return Err(anyhow::anyhow!("Service tag is not found"));
        }
        let service_tag_size = data[ptr] as usize;
        ptr += 1;
        let service_tag = &data[ptr..ptr + service_tag_size];
        ptr += service_tag_size * size_of::<u8>();
        //Version
        if ptr + 1 > len {
            return Err(anyhow::anyhow!("Version is not found"));
        }
        let version = data[ptr] as u8;
        ptr += 1;
        //Network id
        if ptr + 1 > len {
            return Err(anyhow::anyhow!("Network id is not found"));
        }
        let network_id = data[ptr] as u8;
        ptr += 1;
        //Inputs
        if ptr + 4 > len {
            return Err(anyhow::anyhow!("Inputs are not found"));
        }
        let inputs_len = u32::from_be_bytes(data[ptr..ptr + 4].try_into().unwrap()) as usize;
        ptr += 4;
        let mut inputs = Vec::new();
        for _ in 0..inputs_len {
            let input_size = u32::from_be_bytes(data[ptr..ptr + 4].try_into().unwrap()) as usize;
            ptr += 4;
            let input = PreviousStakingUTXO::try_from(&data[ptr..ptr + input_size])?;
            inputs.push(input);
            ptr += input_size;
        }
        //Unstaking outputs
        if ptr + 4 > len {
            return Err(anyhow::anyhow!("redeem outputs are not found"));
        }
        let outputs_len = u32::from_be_bytes(data[ptr..ptr + 4].try_into().unwrap()) as usize;
        ptr += 4;
        let mut outputs = Vec::new();
        for _ in 0..outputs_len {
            let output_size = u32::from_be_bytes(data[ptr..ptr + 4].try_into().unwrap()) as usize;
            ptr += 4;
            let output = UnstakingOutput::try_from(&data[ptr..ptr + output_size])?;
            outputs.push(output);
            ptr += output_size;
        }
        //Custodian pub keys
        if ptr + 4 > len {
            return Err(anyhow::anyhow!("Custodian pub keys are not found"));
        }
        let pub_keys_len = u32::from_be_bytes(data[ptr..ptr + 4].try_into().unwrap()) as usize;
        ptr += 4;
        let mut custodian_pub_keys = Vec::new();
        for _ in 0..pub_keys_len {
            let pub_key = PublicKey::from_slice(&data[ptr..ptr + 33])?;
            custodian_pub_keys.push(pub_key);
            ptr += 33;
        }
        //Custodian quorum
        if ptr + 1 > len {
            return Err(anyhow::anyhow!("Custodian quorum is not found"));
        }
        let custodian_quorum = data[ptr] as u8;
        ptr += 1;
        //RBF
        if ptr + 1 > len {
            return Err(anyhow::anyhow!("RBF is not found"));
        }
        let rbf = data[ptr] as u8;
        ptr += 1;
        //Fee rate
        if ptr + 8 > len {
            return Err(anyhow::anyhow!("Fee rate is not found"));
        }
        let fee_rate = u64::from_be_bytes(data[ptr..ptr + 8].try_into().unwrap());
        ptr += 8;
        //Session sequence
        if ptr + 8 > len {
            return Err(anyhow::anyhow!("Session sequence is not found"));
        }
        let session_sequence = u64::from_be_bytes(data[ptr..ptr + 8].try_into().unwrap());
        ptr += 8;
        //custodian_group_uid
        if ptr + 32 > len {
            return Err(anyhow::anyhow!("Custodian group uid is not found"));
        }
        let custodian_group_uid = &data[ptr..ptr + 32];

        Ok(PoolingRedeemParams {
            tag,
            service_tag,
            version,
            network_id,
            inputs,
            outputs,
            custodian_pub_keys,
            custodian_quorum,
            rbf: rbf == 1,
            fee_rate,
            session_sequence,
            custodian_group_uid,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ByteBuffer;

    use super::*;
    use bitcoin::consensus::serde::Hex;
    use vault::{hex_to_vec, CustodianOnlyUnstakingParams, Unstaking, VaultManager};

    #[test]
    fn test_pooling_redeem_params_parse() {
        //let params_hex = "065343414c415205706f6f6c730301000000010000004e52c0173d62c0c6a79ab2da183f059580fd996c24727894d3e0f6cf36a3cb77730000000000000000000003e85120a8fc50d87f16d892b4d4d087d259c0ab417e106b044b291a7728d2ae1343de7f000000020000001e00000000000003e8001463dc22751d9a7778aa4450ceeb0b5c3ee214401c0000001e00000000000003e8001463dc22751d9a7778aa4450ceeb0b5c3ee214401c000000050215da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e164148802f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb503594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb781103b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc610203e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff0300000000000000000100000000000000013e79326a9493896e13af62194e694dff4c9300700407449363564b0eaeaf07e8";
        let params_hex = "065343414c415205706f6f6c730301000000010000004e52c0173d62c0c6a79ab2da183f059580fd996c24727894d3e0f6cf36a3cb77730000000000000000000003e85120a8fc50d87f16d892b4d4d087d259c0ab417e106b044b291a7728d2ae1343de7f000000010000001e00000000000003e8001463dc22751d9a7778aa4450ceeb0b5c3ee214401c000000050215da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e164148802f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb503594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb781103b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc610203e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff030000000000000000010000000000000001bffb71bf819ae4cb65188905ac54763a09144bc3a0629808d7142dd5dbd98693";
        let params_bytes = hex_to_vec(params_hex);
        let params_bytes_ptr = params_bytes.as_ptr();
        let params = unsafe {
            PoolingRedeemParams::from_buffer(params_bytes_ptr, params_bytes.len()).unwrap()
        };
        println!("tag: {:?}", params);
    }
    #[test]
    fn test_build_pooling_redeem_tx() {
        //let params_hex = "065343414c415205706f6f6c730301000000010000004e52c0173d62c0c6a79ab2da183f059580fd996c24727894d3e0f6cf36a3cb77730000000000000000000003e85120a8fc50d87f16d892b4d4d087d259c0ab417e106b044b291a7728d2ae1343de7f000000020000001e00000000000003e8001463dc22751d9a7778aa4450ceeb0b5c3ee214401c0000001e00000000000003e8001463dc22751d9a7778aa4450ceeb0b5c3ee214401c000000050215da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e164148802f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb503594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb781103b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc610203e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff0300000000000000000100000000000000013e79326a9493896e13af62194e694dff4c9300700407449363564b0eaeaf07e8";
        let params_hex = "065343414c415205706f6f6c730301000000010000004e52c0173d62c0c6a79ab2da183f059580fd996c24727894d3e0f6cf36a3cb77730000000000000000000003e85120a8fc50d87f16d892b4d4d087d259c0ab417e106b044b291a7728d2ae1343de7f000000010000001e00000000000003e8001463dc22751d9a7778aa4450ceeb0b5c3ee214401c000000050215da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e164148802f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb503594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb781103b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc610203e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff030000000000000000010000000000000001bffb71bf819ae4cb65188905ac54763a09144bc3a0629808d7142dd5dbd98693";
        let params_bytes = hex_to_vec(params_hex);
        let params_bytes_ptr = params_bytes.as_ptr();
        let PoolingRedeemParams {
            tag,
            service_tag,
            version,
            network_id,
            inputs,
            outputs,
            custodian_pub_keys,
            custodian_quorum,
            rbf,
            fee_rate,
            session_sequence,
            custodian_group_uid,
        } = unsafe {
            PoolingRedeemParams::from_buffer(params_bytes_ptr, params_bytes.len()).unwrap()
        };
        // println!("tag: {:?}", tag);
        // println!("service_tag: {:?}", service_tag);
        // println!("version: {:?}", version);
        // println!("network_id: {:?}", network_id);
        // println!("inputs: {:?}", inputs);
        // println!("outputs: {:?}", outputs);
        // println!("custodian_pub_keys: {:?}", custodian_pub_keys);
        // println!("custodian_quorum: {:?}", custodian_quorum);
        // println!("rbf: {:?}", rbf);
        // println!("fee_rate: {:?}", fee_rate);
        // println!("session_sequence: {:?}", session_sequence);
        // println!("custodian_group_uid: {:?}", custodian_group_uid);

        // Create a VaultManager instance
        let vault_manager =
            VaultManager::new(tag.to_vec(), service_tag.to_vec(), version, network_id);

        // Create parameters for the unstaking function
        let params = CustodianOnlyUnstakingParams {
            inputs,
            unstaking_outputs: outputs,
            custodian_pub_keys,
            custodian_quorum,
            rbf,
            fee_rate,
            session_sequence,
            custodian_group_uid: custodian_group_uid.try_into().unwrap(),
        };
        // Call the build_custodian_only function
        match vault_manager.build_custodian_only(&params) {
            Ok(psbt) => {
                // Serialize the PSBT and return it as a ByteBuffer
                let psbt_bytes = psbt.serialize();
                let mut output = Vec::with_capacity(psbt_bytes.len());
                output.extend_from_slice(&psbt_bytes);
                // let buffer = ByteBuffer {
                //     data: output.as_mut_ptr(),
                //     len: output.len(),
                // };
                let hex_string: String = output
                    .iter()
                    .map(|b| format!("{:02x}", b)) // use `{:02X}` for uppercase
                    .collect();
                println!("Psbt {}", hex_string); // Outputs: baadf00d
                std::mem::forget(output); // Prevent deallocation
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
