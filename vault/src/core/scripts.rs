use bitcoin::{
    hashes::{sha256d::Hash as Sha256dHash, Hash},
    ScriptBuf, XOnlyPublicKey,
};

use super::{
    CoreError, DestinationChain, DestinationRecipientAddress, DestinationTokenAddress,
    SERVICE_TAG_HASH_SIZE, TAG_HASH_SIZE,
};

#[derive(Debug)]
pub struct UPCLockingScriptParams<'a> {
    pub user_pub_key: &'a XOnlyPublicKey,
    pub protocol_pub_key: &'a XOnlyPublicKey,
    pub custodian_pub_keys: &'a [XOnlyPublicKey],
    pub custodian_quorum: u8,
}

#[derive(Debug)]
pub struct CustodianOnlyLockingScriptParams<'a> {
    pub custodian_pub_keys: &'a [XOnlyPublicKey],
    pub custodian_quorum: u8,
}

#[derive(Debug, Clone)]
pub struct LockingScript(pub ScriptBuf);

impl LockingScript {
    pub fn into_script(self) -> ScriptBuf {
        self.0
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

pub struct DataScriptParams<'a> {
    pub tag: &'a Vec<u8>,
    pub version: u8,
    pub network_id: u8,
    pub custodian_quorum: u8,
    pub destination_chain_id: &'a DestinationChain,
    pub destination_token_address: &'a DestinationTokenAddress,
    pub destination_recipient_address: &'a DestinationRecipientAddress,
    pub service_tag: &'a Vec<u8>,
}

pub struct CustodianOnlyDataParams<'a> {
    pub tag: &'a Vec<u8>,
    pub version: u8,
    pub network_id: u8,
    pub service_tag: &'a Vec<u8>,
    pub custodian_quorum: u8,
    pub destination_chain_id: &'a DestinationChain,
    pub destination_token_address: &'a DestinationTokenAddress,
    pub destination_recipient_address: &'a DestinationRecipientAddress,
}

#[derive(Debug, Clone)]
pub struct DataScript(pub ScriptBuf);

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
     * Only custodians
     */
    CustodianOnly = 0b01000000,

    /**
     * User - Protocol, Custodian - Protocol, User - Custodian
     */
    UPCBranch = 0b10000000,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnlockingTaprootTreeType {
    CustodianOnlyBranch = 0b01000001,
    UPCBranch = 0b10000001,
}

impl TryFrom<u8> for TaprootTreeType {
    type Error = CoreError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00000000 => Ok(Self::OnlyKeys),
            0b01000000 => Ok(Self::CustodianOnly),
            0b10000000 => Ok(Self::UPCBranch),
            _ => Err(CoreError::InvalidTaprootTreeType),
        }
    }
}

impl TryFrom<u8> for UnlockingTaprootTreeType {
    type Error = CoreError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b01000001 => Ok(Self::CustodianOnlyBranch),
            0b10000001 => Ok(Self::UPCBranch),
            _ => Err(CoreError::InvalidTaprootTreeType),
        }
    }
}

impl DataScript {
    pub fn compute_tag_hash(tag: &[u8]) -> Result<[u8; TAG_HASH_SIZE], CoreError> {
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

    pub fn compute_service_tag_hash(
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
