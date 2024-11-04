use bitcoin::{
    hashes::{sha256d::Hash as Sha256dHash, Hash},
    key::Secp256k1,
    opcodes::all::*,
    script,
    secp256k1::All,
    ScriptBuf, XOnlyPublicKey,
};

use super::{
    CoreError, DestinationAddress, DestinationChainId, TaprootManager, EMBEDDED_DATA_SCRIPT_SIZE,
    TAG_HASH_SIZE,
};

pub struct ScriptBuilder;

impl ScriptBuilder {
    pub fn create_locking_script(
        secp: &Secp256k1<All>,
        user_pub_key: &XOnlyPublicKey,
        protocol_pub_key: &XOnlyPublicKey,
        covenant_pubkeys: &[XOnlyPublicKey],
        covenant_quorum: u8,
        have_only_covenants: bool,
    ) -> Result<ScriptBuf, CoreError> {
        let taproot_spend_info = TaprootManager::build_taproot_tree(
            secp,
            user_pub_key,
            protocol_pub_key,
            covenant_pubkeys,
            covenant_quorum,
            have_only_covenants,
        )?;

        Ok(ScriptBuf::new_p2tr(
            secp,
            taproot_spend_info.internal_key(),
            taproot_spend_info.merkle_root(),
        ))
    }

    /// Creates an embedded data script for the staking transaction.
    ///
    /// This script is used to embed additional data in the staking transaction.
    ///
    /// # Arguments
    /// * `tag` - The tag for the embedded data: 4 bytes
    /// * `version` - The version of the embedded data: 1 byte
    /// * `destination_chain_id` - The destination chain ID: 8 bytes
    /// * `destination_contract_address` - The destination address: 20 bytes
    /// * `destination_recipient_address` - The destination recipient address: 20 bytes
    ///
    /// # The script is constructed as follows:
    /// ```text
    /// OP_RETURN <embedded_data_script_size> <hash> <version> <destination_chain_id> <destination_contract_address> <destination_recipient_address>
    /// ```
    ///
    /// # Returns
    /// * `Result<ScriptBuf, CoreError>` - The resulting embedded data script or an error
    ///
    pub fn create_embedded_data_script(
        tag: &Vec<u8>,
        version: u8,
        destination_chain_id: &DestinationChainId,
        destination_contract_address: &DestinationAddress,
        destination_recipient_address: &DestinationAddress,
    ) -> Result<ScriptBuf, CoreError> {
        let tag_bytes = tag.as_slice();

        let hash: [u8; TAG_HASH_SIZE] = if tag.len() <= TAG_HASH_SIZE {
            tag_bytes[0..TAG_HASH_SIZE]
                .try_into()
                .map_err(|_| CoreError::InvalidTag)?
        } else {
            Sha256dHash::hash(tag_bytes)[0..TAG_HASH_SIZE]
                .try_into()
                .map_err(|_| CoreError::InvalidTag)?
        };

        let embedded_data_script = script::Builder::new()
            .push_opcode(OP_RETURN)
            .push_int(EMBEDDED_DATA_SCRIPT_SIZE as i64)
            .push_slice(hash)
            .push_slice(version.to_le_bytes())
            .push_slice(destination_chain_id)
            .push_slice(destination_contract_address)
            .push_slice(destination_recipient_address)
            .into_script();

        Ok(embedded_data_script)
    }
}
