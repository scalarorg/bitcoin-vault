use super::SignByKeyMap;
use bitcoin::{
    consensus::{serialize, Encodable},
    hashes::Hash,
    key::Secp256k1,
    secp256k1::All,
    taproot, NetworkKind, Psbt, TapLeafHash, XOnlyPublicKey,
};

use lazy_static::lazy_static;

use super::{CoreError, Signing, SigningKeyMap, VaultManager};

lazy_static! {
    static ref SECP: Secp256k1<All> = Secp256k1::new();
}

#[derive(Debug, Clone)]

pub struct TapScriptSig {
    key_and_leaf_hash: (XOnlyPublicKey, TapLeafHash),
    sig: taproot::Signature,
}

#[derive(Debug, Clone)]
pub struct TapScriptSigSerialized {
    pub key: [u8; 32],
    pub leaf_hash: [u8; 32],
    pub signature: [u8; 64],
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
            key,
            leaf_hash: leaf_hash_bytes,
            signature,
        })
    }

    pub fn from_serialized(serialized: TapScriptSigSerialized) -> Result<Self, CoreError> {
        let key =
            XOnlyPublicKey::from_slice(&serialized.key).map_err(|_| CoreError::InvalidPublicKey)?;
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

impl Signing for VaultManager {
    type PsbtHex = Vec<u8>;

    type TxHex = Vec<u8>;

    fn sign_psbt_by_single_key(
        psbt: &mut Psbt,
        privkey: &[u8],
        network_kind: NetworkKind,
        finalize: bool,
    ) -> Result<Self::PsbtHex, CoreError> {
        let key_map = SigningKeyMap::from_privkey_slice(&SECP, privkey, network_kind)
            .map_err(|err| CoreError::InvalidPrivateKey(err.to_string()))?;

        psbt.sign_by_key_map(&key_map, &SECP).map_err(|err| {
            let (_, errors) = err;
            let error_messages: Vec<String> = errors.values().map(|e| e.to_string()).collect();
            CoreError::SigningPSBTFailed(error_messages.join(", "))
        })?;

        if !finalize {
            return Ok(psbt.serialize());
        }

        <Psbt as SignByKeyMap<All>>::finalize(psbt);
        let tx = psbt
            .clone()
            .extract_tx()
            .map_err(|_| CoreError::FailedToExtractTx)?;
        Ok(serialize(&tx))
    }

    fn sign_psbt_and_collect_tap_script_sigs(
        psbt: &mut Psbt,
        privkey: &[u8],
        network_kind: NetworkKind,
    ) -> Result<Vec<TapScriptSig>, CoreError> {
        <VaultManager as Signing>::sign_psbt_by_single_key(psbt, privkey, network_kind, false)?;

        let mut tap_script_sigs: Vec<TapScriptSig> = vec![];

        for input in psbt.inputs.iter() {
            if let Some((key, sig)) = input.tap_script_sigs.first_key_value() {
                tap_script_sigs.push(TapScriptSig::new(*key, *sig));
            }
        }

        Ok(tap_script_sigs)
    }

    fn aggregate_tap_script_sigs(
        psbt: &mut Psbt,
        tap_script_sigs: &[TapScriptSig],
    ) -> Result<Self::PsbtHex, CoreError> {
        if psbt.inputs.len() != tap_script_sigs.len() {
            return Err(CoreError::MismatchBetweenNumberOfInputsAndTapScriptSigs);
        }

        for (input, tap_script_sig) in psbt.inputs.iter_mut().zip(tap_script_sigs) {
            input
                .tap_script_sigs
                .insert(*tap_script_sig.key_and_leaf_hash(), *tap_script_sig.sig());
        }

        Ok(psbt.serialize())
    }

    fn finalize_psbt_and_extract_tx(psbt: &mut Psbt) -> Result<Self::TxHex, CoreError> {
        <Psbt as SignByKeyMap<All>>::finalize(psbt);
        let tx = psbt
            .clone()
            .extract_tx()
            .map_err(|_| CoreError::FailedToExtractTx)?;
        Ok(serialize(&tx))
    }
}
