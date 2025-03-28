use std::collections::BTreeMap;

use bitcoin::{consensus::Encodable, hashes::Hash, taproot, TapLeafHash, XOnlyPublicKey};
#[allow(unused_imports)]
use bitcoincore_rpc::jsonrpc::serde_json;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{serde_as, Bytes};

use super::CoreError;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TapScriptSigSerialized {
    #[serde_as(as = "Bytes")]
    pub key_x_only: [u8; 32],
    #[serde_as(as = "Bytes")]
    pub leaf_hash: [u8; 32],
    #[serde_as(as = "Bytes")]
    pub signature: [u8; 64],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TapScriptSig {
    key_and_leaf_hash: (XOnlyPublicKey, TapLeafHash),
    sig: taproot::Signature,
}

impl TapScriptSig {
    pub fn new(key_and_leaf_hash: (XOnlyPublicKey, TapLeafHash), sig: taproot::Signature) -> Self {
        Self {
            key_and_leaf_hash,
            sig,
        }
    }

    pub fn key_and_leaf_hash(&self) -> &(XOnlyPublicKey, TapLeafHash) {
        &self.key_and_leaf_hash
    }

    pub fn sig(&self) -> &taproot::Signature {
        &self.sig
    }

    pub fn key(&self) -> &XOnlyPublicKey {
        &self.key_and_leaf_hash.0
    }

    pub fn leaf_hash(&self) -> &TapLeafHash {
        &self.key_and_leaf_hash.1
    }

    pub fn serialize(&self) -> Result<TapScriptSigSerialized, CoreError> {
        let key = self.key().serialize();
        let signature: [u8; 64] = self
            .sig()
            .to_vec()
            .try_into()
            .map_err(|_| CoreError::InvalidSignatureSize)?;
        let mut leaf_hash_bytes = vec![];
        self.leaf_hash()
            .consensus_encode(&mut leaf_hash_bytes)
            .map_err(|_| CoreError::FailedToEncodeLeafHash)?;
        let leaf_hash_bytes: [u8; 32] = leaf_hash_bytes
            .try_into()
            .map_err(|_| CoreError::FailedToEncodeLeafHash)?;
        Ok(TapScriptSigSerialized {
            key_x_only: key,
            leaf_hash: leaf_hash_bytes,
            signature,
        })
    }

    pub fn from_serialized(serialized: TapScriptSigSerialized) -> Result<Self, CoreError> {
        let key =
            XOnlyPublicKey::from_slice(&serialized.key_x_only).map_err(|_| CoreError::InvalidPublicKey)?;
        let leaf_hash = TapLeafHash::from_slice(&serialized.leaf_hash)
            .map_err(|_| CoreError::InvalidLeafHash)?;
        let sig = taproot::Signature::from_slice(&serialized.signature)
            .map_err(|_| CoreError::InvalidSignatureSize)?;

        Ok(Self {
            key_and_leaf_hash: (key, leaf_hash),
            sig,
        })
    }
}

impl Serialize for TapScriptSig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized = TapScriptSigSerialized {
            key_x_only: self.key().serialize(),
            leaf_hash: {
                let mut leaf_hash_bytes = vec![];
                self.leaf_hash()
                    .consensus_encode(&mut leaf_hash_bytes)
                    .map_err(|e| serde::ser::Error::custom(format!("{:?}", e)))?;
                leaf_hash_bytes
                    .try_into()
                    .map_err(|e| serde::ser::Error::custom(format!("{:?}", e)))?
            },
            signature: self
                .sig()
                .to_vec()
                .try_into()
                .map_err(|e| serde::ser::Error::custom(format!("{:?}", e)))?,
        };
        serialized.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TapScriptSig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serialized = TapScriptSigSerialized::deserialize(deserializer)?;
        let key = XOnlyPublicKey::from_slice(&serialized.key_x_only).map_err(de::Error::custom)?;
        let leaf_hash =
            TapLeafHash::from_slice(&serialized.leaf_hash).map_err(de::Error::custom)?;
        let sig =
            taproot::Signature::from_slice(&serialized.signature).map_err(de::Error::custom)?;

        Ok(TapScriptSig {
            key_and_leaf_hash: (key, leaf_hash),
            sig,
        })
    }
}

pub type InputIndex = u64;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TapScriptSigsMap(BTreeMap<InputIndex, Vec<TapScriptSig>>);

impl TapScriptSigsMap {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, index: InputIndex, tap_script_sig: Vec<TapScriptSig>) {
        self.0.insert(index, tap_script_sig);
    }

    pub fn get(&self, index: InputIndex) -> Option<&Vec<TapScriptSig>> {
        self.0.get(&index)
    }
}

impl Serialize for TapScriptSigsMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TapScriptSigsMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = BTreeMap::deserialize(deserializer)?;
        Ok(TapScriptSigsMap(map))
    }
}

#[test]
fn test_tap_script_sigs_map() {
    let mut map = TapScriptSigsMap::default();
    map.insert(
        0,
        vec![TapScriptSig::new(
            (
                XOnlyPublicKey::from_slice(&[1; 32]).unwrap(),
                TapLeafHash::from_slice(&[2; 32]).unwrap(),
            ),
            taproot::Signature::from_slice(&[3; 64]).unwrap(),
        )],
    );
    let serialized = serde_json::to_string(&map).unwrap();

    // write to file
    std::fs::write("tap_script_sigs_map.json", serialized.clone()).unwrap();

    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(map, deserialized);
}
