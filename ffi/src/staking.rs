use bitcoin_vault::{
    BuildStakingOutputParams, Decoder, DestinationAddress, Encoder, Staking, StakingManager,
    VaultABIError,
};

pub fn build_staking_outputs(
    tag: &[u8],
    version: u8,
    staking_amount: u64,
    staker_pubkey: &[u8],
    protocol_pubkey: &[u8],
    custodial_pubkeys: &[u8],
    covenant_quorum: i32,
    have_only_covenants: bool,
    destination_chain_id: u64,
    destination_smartcontract_address: &[u8],
    destination_recipient_address: &[u8],
) -> Result<Vec<u8>, VaultABIError> {
    let destination_contract_address: DestinationAddress = destination_smartcontract_address
        .try_into()
        .map_err(|e| VaultABIError::InvalidInputData)?;
    // console::debug_1(&"Wasm#parsed destination_contract_address".into());
    let destination_recipient_address: DestinationAddress = destination_recipient_address
        .try_into()
        .map_err(|e| VaultABIError::InvalidInputData)?;

    let params = BuildStakingOutputParams {
        user_pub_key: Decoder::decode_33bytes_pubkey(staker_pubkey)?,
        protocol_pub_key: Decoder::decode_33bytes_pubkey(protocol_pubkey)?,
        covenant_pubkeys: Decoder::decode_33bytes_pubkey_list(custodial_pubkeys)?,
        covenant_quorum: covenant_quorum as u8,
        staking_amount,
        have_only_covenants,
        destination_chain_id: destination_chain_id.to_be_bytes(),
        destination_contract_address,
        destination_recipient_address,
    };
    let staking = StakingManager::new(tag.to_vec(), version);
    staking
        .build_staking_outputs(&params)
        .map(|tx_outs| Encoder::serialize_tx_outs(&tx_outs))
        .map_err(|_| VaultABIError::StakingError)
}
