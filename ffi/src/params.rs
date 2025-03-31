use std::slice;

use bitcoin::PublicKey;
use vault::{PreviousStakingUTXO, UnstakingOutput};

#[derive(Default)]
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
