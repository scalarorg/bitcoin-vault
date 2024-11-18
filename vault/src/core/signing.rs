use super::SignByKeyMap;
use bitcoin::{consensus::serialize, key::Secp256k1, secp256k1::All, NetworkKind, Psbt};

use lazy_static::lazy_static;

use super::{CoreError, Signing, SigningKeyMap, VaultManager};

lazy_static! {
    static ref SECP: Secp256k1<All> = Secp256k1::new();
}

impl Signing for VaultManager {
    type PsbtHex = Vec<u8>;

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
        return Ok(serialize(&tx));
    }
}
