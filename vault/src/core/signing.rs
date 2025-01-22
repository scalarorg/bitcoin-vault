use super::{get_global_secp, SignByKeyMap, TapScriptSig, TapScriptSigsMap};
use bitcoin::{consensus::serialize, secp256k1::All, NetworkKind, Psbt};

use super::{CoreError, Signing, SigningKeyMap, VaultManager};

impl Signing for VaultManager {
    type PsbtHex = Vec<u8>;

    type TxHex = Vec<u8>;

    fn sign_psbt_by_single_key(
        psbt: &mut Psbt,
        privkey: &[u8],
        network_kind: NetworkKind,
        finalize: bool,
    ) -> Result<(Self::PsbtHex, SigningKeyMap), CoreError> {
        let secp = get_global_secp();
        let key_map = SigningKeyMap::from_privkey_slice(secp, privkey, network_kind)
            .map_err(|err| CoreError::InvalidPrivateKey(err.to_string()))?;

        psbt.sign_by_key_map(&key_map, secp).map_err(|err| {
            let (_, errors) = err;
            let error_messages: Vec<String> = errors.values().map(|e| e.to_string()).collect();
            CoreError::SigningPSBTFailed(error_messages.join(", "))
        })?;

        if !finalize {
            return Ok((psbt.serialize(), key_map));
        }

        <Psbt as SignByKeyMap<All>>::finalize(psbt);
        let tx = psbt
            .clone()
            .extract_tx()
            .map_err(|_| CoreError::FailedToExtractTx)?;
        Ok((serialize(&tx), key_map))
    }

    fn sign_psbt_and_collect_tap_script_sigs(
        psbt: &mut Psbt,
        privkey: &[u8],
        network_kind: NetworkKind,
    ) -> Result<TapScriptSigsMap, CoreError> {
        let (_, key_map) =
            <VaultManager as Signing>::sign_psbt_by_single_key(psbt, privkey, network_kind, false)?;

        let x_only_pubkey = key_map
            .get_x_only_pubkey()
            .ok_or(CoreError::SigningKeyMapIsEmpty)?;

        let mut tap_script_sigs: TapScriptSigsMap = TapScriptSigsMap::default();

        for (index, input) in psbt.inputs.iter().enumerate() {
            let mut tap_script_sig_vec: Vec<TapScriptSig> = vec![];

            for ((key, leaf_hash), sig) in input.tap_script_sigs.iter() {
                if key == x_only_pubkey {
                    tap_script_sig_vec.push(TapScriptSig::new((*key, *leaf_hash), *sig));
                }
            }
            tap_script_sigs.insert(index as u64, tap_script_sig_vec);
        }

        Ok(tap_script_sigs)
    }

    fn aggregate_tap_script_sigs(
        psbt: &mut Psbt,
        tap_script_sigs: &TapScriptSigsMap,
    ) -> Result<Self::PsbtHex, CoreError> {
        if psbt.inputs.len() != tap_script_sigs.len() {
            return Err(CoreError::MismatchBetweenNumberOfInputsAndTapScriptSigs);
        }

        for (index, input) in psbt.inputs.iter_mut().enumerate() {
            if let Some(sigs) = tap_script_sigs.get(index as u64) {
                for tap_script_sig in sigs {
                    input
                        .tap_script_sigs
                        .insert(*tap_script_sig.key_and_leaf_hash(), *tap_script_sig.sig());
                }
            }
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
