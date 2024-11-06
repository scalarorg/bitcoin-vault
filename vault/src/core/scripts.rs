use bitcoin::{
    hashes::{sha256d::Hash as Sha256dHash, Hash},
    key::Secp256k1,
    opcodes::all::*,
    script,
    secp256k1::All,
    ScriptBuf, XOnlyPublicKey,
};

use super::{
    CoreError, DestinationAddress, DestinationChainId, TaprootTree, EMBEDDED_DATA_SCRIPT_SIZE,
    TAG_HASH_SIZE,
};

#[derive(Debug)]
pub struct LockingScriptParams<'a> {
    pub user_pub_key: &'a XOnlyPublicKey,
    pub protocol_pub_key: &'a XOnlyPublicKey,
    pub covenant_pub_keys: &'a [XOnlyPublicKey],
    pub covenant_quorum: u8,
    pub have_only_covenants: bool,
}

#[derive(Debug)]
pub struct LockingScript(ScriptBuf);

impl LockingScript {
    pub fn new(secp: &Secp256k1<All>, params: &LockingScriptParams) -> Result<Self, CoreError> {
        let tree = TaprootTree::new(
            secp,
            params.user_pub_key,
            params.protocol_pub_key,
            params.covenant_pub_keys,
            params.covenant_quorum,
            params.have_only_covenants,
        )?;

        Ok(LockingScript(ScriptBuf::new_p2tr(
            secp,
            tree.internal_key(),
            tree.merkle_root(),
        )))
    }

    pub fn into_script(self) -> ScriptBuf {
        self.0
    }
}

pub struct DataScriptParams<'a> {
    pub tag: &'a Vec<u8>,
    pub version: u8,
    pub destination_chain_id: &'a DestinationChainId,
    pub destination_contract_address: &'a DestinationAddress,
    pub destination_recipient_address: &'a DestinationAddress,
}

#[derive(Debug)]
pub struct DataScript(ScriptBuf);

impl DataScript {
    pub fn new(params: &DataScriptParams) -> Result<Self, CoreError> {
        let tag_bytes = params.tag.as_slice();

        let hash: [u8; TAG_HASH_SIZE] = if params.tag.len() <= TAG_HASH_SIZE {
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
            .push_slice(params.version.to_le_bytes())
            .push_slice(params.destination_chain_id)
            .push_slice(params.destination_contract_address)
            .push_slice(params.destination_recipient_address)
            .into_script();

        Ok(DataScript(embedded_data_script))
    }

    pub fn into_script(self) -> ScriptBuf {
        self.0
    }
}