use bitcoin::{
    hashes::{sha256d::Hash as Sha256dHash, Hash},
    key::Secp256k1,
    opcodes::all::*,
    script,
    secp256k1::All,
    ScriptBuf, XOnlyPublicKey,
};

use super::{
    CoreError, DestinationChain, DestinationRecipientAddress, DestinationTokenAddress, TaprootTree,
    EMBEDDED_DATA_SCRIPT_SIZE, SERVICE_TAG_HASH_SIZE, TAG_HASH_SIZE,
    UNSTAKING_EMBEDDED_DATA_SCRIPT_SIZE,
};

#[derive(Debug)]
pub struct LockingScriptParams<'a> {
    pub user_pub_key: &'a XOnlyPublicKey,
    pub protocol_pub_key: &'a XOnlyPublicKey,
    pub covenant_pub_keys: &'a [XOnlyPublicKey],
    pub covenant_quorum: u8,
}

#[derive(Debug)]
pub struct LockingScriptWithOnlyCovenantsParams<'a> {
    pub covenant_pub_keys: &'a [XOnlyPublicKey],
    pub covenant_quorum: u8,
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
        )?;

        Ok(LockingScript(tree.into_script(secp)))
    }

    pub fn new_with_only_covenants(
        secp: &Secp256k1<All>,
        params: &LockingScriptWithOnlyCovenantsParams,
    ) -> Result<Self, CoreError> {
        let tree = TaprootTree::new_with_only_covenants(
            secp,
            params.covenant_pub_keys,
            params.covenant_quorum,
        )?;

        Ok(LockingScript(tree.into_script(secp)))
    }

    pub fn into_script(self) -> ScriptBuf {
        self.0
    }
}

pub struct DataScriptParams<'a> {
    pub tag: &'a Vec<u8>,
    pub version: u8,
    pub network_id: u8,
    pub covenant_quorum: u8,
    pub destination_chain_id: &'a DestinationChain,
    pub destination_token_address: &'a DestinationTokenAddress,
    pub destination_recipient_address: &'a DestinationRecipientAddress,
    pub service_tag: &'a Vec<u8>,
}

pub struct DataScriptParamsWithOnlyCovenants<'a> {
    pub tag: &'a Vec<u8>,
    pub version: u8,
    pub network_id: u8,
    pub service_tag: &'a Vec<u8>,
    pub covenant_quorum: u8,
    pub destination_chain_id: &'a DestinationChain,
    pub destination_token_address: &'a DestinationTokenAddress,
    pub destination_recipient_address: &'a DestinationRecipientAddress,
}

pub struct DataScriptParamsWithOnlyCovenantsUnstaking<'a> {
    pub tag: &'a Vec<u8>,
    pub version: u8,
    pub network_id: u8,
    pub service_tag: &'a Vec<u8>,
}

#[derive(Debug)]
pub struct DataScript(ScriptBuf);

/**
 * Taproot tree type
 * Ref: [docs/op_return.md](docs/op_return.md)
 */
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TaprootTreeType {
    /**
     * For future use
     */
    OnlyKeys = 0b00000000,

    /**
     * Only covenants
     */
    CovenantOnly = 0b01000000,

    /**
     * User - Protocol, Covenant - Protocol, User - Covenant
     */
    MultiBranch = 0b10000000,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnstakingTaprootTreeType {
    CovenantOnly = 0b01000001,
}

impl TryFrom<u8> for TaprootTreeType {
    type Error = CoreError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00000000 => Ok(Self::OnlyKeys),
            0b01000000 => Ok(Self::CovenantOnly),
            0b10000000 => Ok(Self::MultiBranch),
            _ => Err(CoreError::InvalidTaprootTreeType),
        }
    }
}

impl TryFrom<u8> for UnstakingTaprootTreeType {
    type Error = CoreError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b01000001 => Ok(Self::CovenantOnly),
            _ => Err(CoreError::InvalidTaprootTreeType),
        }
    }
}

impl DataScript {
    pub fn new(params: &DataScriptParams) -> Result<Self, CoreError> {
        let tag_hash = Self::compute_tag_hash(params.tag.as_slice())?;
        let service_tag_hash = Self::compute_service_tag_hash(params.service_tag.as_slice())?;
        // let flags = if !params.have_only_covenants {
        //     TaprootTreeType::ManyBranchNoCovenants as u8
        // } else {
        //     TaprootTreeType::ManyBranchWithCovenants as u8
        // };

        let flags = TaprootTreeType::MultiBranch as u8;

        let mut data = Vec::<u8>::with_capacity(EMBEDDED_DATA_SCRIPT_SIZE);
        data.extend_from_slice(&tag_hash);
        data.push(params.version);
        data.push(params.network_id);
        data.push(flags);
        data.extend_from_slice(&service_tag_hash);
        data.push(params.covenant_quorum);
        data.extend_from_slice(params.destination_chain_id);
        data.extend_from_slice(params.destination_token_address);
        data.extend_from_slice(params.destination_recipient_address);

        let data_slice: &[u8; EMBEDDED_DATA_SCRIPT_SIZE] = data
            .as_slice()
            .try_into()
            .map_err(|_| CoreError::CannotConvertOpReturnDataToSlice)?;

        let embedded_data_script = script::Builder::new()
            .push_opcode(OP_RETURN)
            .push_slice(data_slice)
            .into_script();

        Ok(DataScript(embedded_data_script))
    }

    pub fn new_with_only_covenants(
        params: &DataScriptParamsWithOnlyCovenants,
    ) -> Result<Self, CoreError> {
        let tag_hash = Self::compute_tag_hash(params.tag.as_slice())?;
        let service_tag_hash = Self::compute_service_tag_hash(params.service_tag.as_slice())?;
        let flags = TaprootTreeType::CovenantOnly as u8;

        let mut data = Vec::<u8>::with_capacity(EMBEDDED_DATA_SCRIPT_SIZE);
        data.extend_from_slice(&tag_hash);
        data.push(params.version);
        data.push(params.network_id);
        data.push(flags);
        data.extend_from_slice(&service_tag_hash);
        data.push(params.covenant_quorum);
        data.extend_from_slice(params.destination_chain_id);
        data.extend_from_slice(params.destination_token_address);
        data.extend_from_slice(params.destination_recipient_address);

        let data_slice: &[u8; EMBEDDED_DATA_SCRIPT_SIZE] = data
            .as_slice()
            .try_into()
            .map_err(|_| CoreError::CannotConvertOpReturnDataToSlice)?;

        let embedded_data_script = script::Builder::new()
            .push_opcode(OP_RETURN)
            .push_slice(data_slice)
            .into_script();

        Ok(DataScript(embedded_data_script))
    }

    pub fn new_unstaking_with_only_covenants(
        params: &DataScriptParamsWithOnlyCovenantsUnstaking,
    ) -> Result<Self, CoreError> {
        let tag_hash = Self::compute_tag_hash(params.tag.as_slice())?;
        let service_tag_hash = Self::compute_service_tag_hash(params.service_tag.as_slice())?;

        let flags = UnstakingTaprootTreeType::CovenantOnly as u8;

        let mut data = Vec::<u8>::with_capacity(UNSTAKING_EMBEDDED_DATA_SCRIPT_SIZE);
        data.extend_from_slice(&tag_hash);
        data.push(params.version);
        data.push(params.network_id);
        data.push(flags);
        data.extend_from_slice(&service_tag_hash);
        let data_slice: &[u8; UNSTAKING_EMBEDDED_DATA_SCRIPT_SIZE] = data
            .as_slice()
            .try_into()
            .map_err(|_| CoreError::CannotConvertOpReturnDataToSlice)?;

        let embedded_data_script = script::Builder::new()
            .push_opcode(OP_RETURN)
            .push_slice(data_slice)
            .into_script();

        Ok(DataScript(embedded_data_script))
    }

    fn compute_tag_hash(tag: &[u8]) -> Result<[u8; TAG_HASH_SIZE], CoreError> {
        let mut new_hash = [0u8; TAG_HASH_SIZE];
        if tag.len() <= TAG_HASH_SIZE {
            new_hash[TAG_HASH_SIZE - tag.len()..].copy_from_slice(tag);
            Ok(new_hash)
        } else {
            Sha256dHash::hash(tag)[0..TAG_HASH_SIZE]
                .try_into()
                .map_err(|_| CoreError::InvalidTag)
        }
    }

    fn compute_service_tag_hash(
        service_tag: &[u8],
    ) -> Result<[u8; SERVICE_TAG_HASH_SIZE], CoreError> {
        if service_tag.len() <= SERVICE_TAG_HASH_SIZE {
            let mut tag = [0u8; SERVICE_TAG_HASH_SIZE];
            let len = service_tag.len();
            tag[SERVICE_TAG_HASH_SIZE - len..].copy_from_slice(service_tag);
            Ok(tag)
        } else {
            Sha256dHash::hash(service_tag)[0..SERVICE_TAG_HASH_SIZE]
                .try_into()
                .map_err(|_| CoreError::InvalidServiceTag)
        }
    }

    pub fn into_script(self) -> ScriptBuf {
        self.0
    }
}
