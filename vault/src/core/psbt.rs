use std::{borrow::Borrow, collections::BTreeMap};

use bitcoin::{
    ecdsa,
    hashes::{hash160, Hash},
    key::{Keypair, Parity, Secp256k1, TapTweak, Verification},
    psbt::{
        GetKey, GetKeyError, IndexOutOfBoundsError, Input, KeyRequest, OutputType, PsbtSighashType,
        SignError, SigningAlgorithm, SigningErrors, SigningKeys, SigningKeysMap,
    },
    secp256k1::{Error as Secp256k1Error, Message, PublicKey as Secp256k1PublicKey, Signing},
    sighash::{Prevouts, SighashCache},
    taproot::{self, ControlBlock},
    NetworkKind, PrivateKey, Psbt, PublicKey, ScriptBuf, TapLeafHash, TapSighashType, Transaction,
    Witness, XOnlyPublicKey,
};

pub struct SigningKeyMap(BTreeMap<XOnlyPublicKey, PrivateKey>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct AliasPushBytes(Vec<u8>);

impl SigningKeyMap {
    fn inner(&self) -> &BTreeMap<XOnlyPublicKey, PrivateKey> {
        &self.0
    }

    pub fn from_privkey_slice<C: Signing + Verification>(
        secp: &Secp256k1<C>,
        privkey_slice: &[u8],
        network_kind: NetworkKind,
    ) -> Result<Self, Secp256k1Error> {
        let privkey = PrivateKey::from_slice(privkey_slice, network_kind)?;
        let x_only_pubkey = privkey.public_key(secp).into();
        Ok(Self(BTreeMap::from([(x_only_pubkey, privkey)])))
    }

    pub fn get_x_only_pubkey(&self) -> Option<&XOnlyPublicKey> {
        if let Some((x_only_pubkey, _)) = self.inner().first_key_value() {
            return Some(x_only_pubkey);
        }
        None
    }
}

impl GetKey for SigningKeyMap {
    type Error = GetKeyError;

    fn get_key<C: Signing>(
        &self,
        key_request: KeyRequest,
        _: &Secp256k1<C>,
    ) -> Result<Option<PrivateKey>, Self::Error> {
        match key_request {
            KeyRequest::Pubkey(pk) => {
                let pubkey: XOnlyPublicKey = pk.into();
                Ok(self.inner().get(&pubkey).cloned())
            }
            KeyRequest::Bip32(_) => Err(GetKeyError::NotSupported),
            _ => Err(GetKeyError::NotSupported),
        }
    }
}

/// Sign a PSBT by a key map.
///
/// ### Description
///
/// This trait is used to sign a PSBT by a key map.
///
/// ### Errors
///
/// Returns an error if the PSBT is already signed.
///
/// ### Examples
///
pub trait SignByKeyMap<C> {
    fn sign_by_key_map(
        &mut self,
        key_map: &SigningKeyMap,
        secp: &Secp256k1<C>,
    ) -> Result<SigningKeysMap, (SigningKeysMap, SigningErrors)>
    where
        C: Signing + Verification;
    fn finalize(&mut self);
}

impl<C> SignByKeyMap<C> for Psbt {
    fn sign_by_key_map(
        &mut self,
        key_map: &SigningKeyMap,
        secp: &Secp256k1<C>,
    ) -> Result<SigningKeysMap, (SigningKeysMap, SigningErrors)>
    where
        C: Signing + Verification,
    {
        let tx = self.unsigned_tx.clone(); // clone because we need to mutably borrow when signing.
        let mut cache = SighashCache::new(&tx);

        let mut used = BTreeMap::new();
        let mut errors = BTreeMap::new();

        for i in 0..self.inputs.len() {
            match self.signing_algorithm(i) {
                Ok(SigningAlgorithm::Ecdsa) => {
                    match self.key_map_sign_ecdsa(key_map, i, &mut cache, secp) {
                        Ok(v) => {
                            used.insert(i, SigningKeys::Ecdsa(v));
                        }
                        Err(e) => {
                            errors.insert(i, e);
                        }
                    }
                }

                Ok(SigningAlgorithm::Schnorr) => {
                    match self.key_map_sign_schnorr(key_map, i, &mut cache, secp) {
                        Ok(v) => {
                            used.insert(i, SigningKeys::Schnorr(v));
                        }
                        Err(e) => {
                            errors.insert(i, e);
                        }
                    }
                }

                _ => {
                    errors.insert(i, SignError::WrongSigningAlgorithm);
                    return Err((used, errors));
                }
            }
        }

        if errors.is_empty() {
            Ok(used)
        } else {
            Err((used, errors))
        }
    }

    fn finalize(&mut self) {
        for i in 0..self.inputs.len() {
            if !self.is_taproot_input(i) {
                self.finalize_ecdsa_input(i);
                continue;
            }

            self.finalize_taproot_input(i);
        }
    }
}

pub trait Utils {
    fn signing_algorithm(&self, input_index: usize) -> Result<SigningAlgorithm, SignError>;
    fn output_type(&self, input_index: usize) -> Result<OutputType, SignError>;
    fn checked_input(&self, input_index: usize) -> Result<&Input, IndexOutOfBoundsError>;
    fn check_index_is_within_bounds(&self, input_index: usize)
        -> Result<(), IndexOutOfBoundsError>;
    fn key_map_sign_schnorr<C, T>(
        &mut self,
        key_map: &SigningKeyMap,
        input_index: usize,
        cache: &mut SighashCache<T>,
        secp: &Secp256k1<C>,
    ) -> Result<Vec<XOnlyPublicKey>, SignError>
    where
        C: Signing + Verification,
        T: Borrow<Transaction>;
    fn sighash_taproot<T: Borrow<Transaction>>(
        &self,
        input_index: usize,
        cache: &mut SighashCache<T>,
        leaf_hash: Option<TapLeafHash>,
    ) -> Result<(Message, TapSighashType), SignError>;
    fn key_map_sign_ecdsa<C, T>(
        &mut self,
        key_map: &SigningKeyMap,
        input_index: usize,
        cache: &mut SighashCache<T>,
        secp: &Secp256k1<C>,
    ) -> Result<Vec<PublicKey>, SignError>
    where
        C: Signing + Verification,
        T: Borrow<Transaction>;
    fn is_taproot_input(&self, input_index: usize) -> bool;
    fn finalize_taproot_input(&mut self, input_index: usize);
    fn finalize_ecdsa_input(&mut self, input_index: usize);

    fn find_tap_leaf_to_finalize(
        &self,
        input_index: usize,
    ) -> Option<(
        ScriptBuf,
        ControlBlock,
        TapLeafHash,
        BTreeMap<&XOnlyPublicKey, &taproot::Signature>,
    )>;

    // fn calculate_pubkey_position_in_script(
    //     &self,
    //     script: &ScriptBuf,
    //     pubkeys: &[XOnlyPublicKey],
    // ) -> Option<BTreeMap<XOnlyPublicKey, usize>>;

    // fn calculate_x_only_pubkey_position_in_script(
    //     &self,
    //     script: &ScriptBuf,
    // ) -> Option<BTreeMap<XOnlyPublicKey, usize>>;

    fn calculate_push_bytes_positions(&self, script: &ScriptBuf)
        -> BTreeMap<AliasPushBytes, usize>;
    fn get_push_bytes_position(
        &self,
        push_bytes_positions: &BTreeMap<AliasPushBytes, usize>,
        key: &XOnlyPublicKey,
    ) -> Option<usize>;

    fn reset_taproot_input(&mut self, input_index: usize);
}

impl Utils for Psbt {
    /// Finalizes a taproot input by adding the signatures to the input.
    fn finalize_taproot_input(&mut self, input_index: usize) {
        let input = &mut self.inputs[input_index];

        if let Some(signature) = input.tap_key_sig {
            let mut script_witness: Witness = Witness::new();
            script_witness.push(signature.to_vec());
            input.final_script_witness = Some(script_witness);
            self.reset_taproot_input(input_index);
            return;
        }

        let result = self.find_tap_leaf_to_finalize(input_index);

        if result.is_none() {
            return;
        }

        let (tap_script, control_block, _, sigs_map) = result.unwrap();

        let push_bytes_positions = self.calculate_push_bytes_positions(&tap_script);

        if push_bytes_positions.is_empty() {
            return;
        }

        let mut raw_witness = vec![Vec::new(); push_bytes_positions.len()];

        for (key, signature) in sigs_map.iter() {
            let pos = self.get_push_bytes_position(&push_bytes_positions, key);
            if let Some(pos) = pos {
                raw_witness[pos] = signature.to_vec();
            }
        }

        let mut script_witness: Witness = Witness::new();
        for witness in raw_witness {
            script_witness.push(witness);
        }

        script_witness.push(tap_script.to_bytes());
        script_witness.push(control_block.serialize());

        self.inputs[input_index].final_script_witness = Some(script_witness);

        self.reset_taproot_input(input_index);
    }

    fn finalize_ecdsa_input(&mut self, input_index: usize) {
        let input = &mut self.inputs[input_index];
        let sigs = input.partial_sigs.values().collect::<Vec<_>>();
        let pubkeys = input.partial_sigs.keys().collect::<Vec<_>>();
        let mut script_witness: Witness = Witness::new();
        for i in 0..sigs.len() {
            script_witness.push(sigs[i].to_vec());
            script_witness.push(pubkeys[i].to_bytes());
        }
        input.final_script_witness = Some(script_witness);
        self.reset_taproot_input(input_index);
    }

    fn reset_taproot_input(&mut self, input_index: usize) {
        let input = &mut self.inputs[input_index];
        input.partial_sigs = BTreeMap::new();
        input.sighash_type = None;
        input.redeem_script = None;
        input.witness_script = None;
        input.bip32_derivation = BTreeMap::new();
        input.tap_script_sigs = BTreeMap::new();
        input.tap_scripts = BTreeMap::new();
        input.tap_key_sig = None;
        input.tap_internal_key = None;
        input.tap_merkle_root = None;
        input.tap_key_origins = BTreeMap::new();
    }

    fn find_tap_leaf_to_finalize(
        &self,
        input_index: usize,
    ) -> Option<(
        ScriptBuf,
        ControlBlock,
        TapLeafHash,
        BTreeMap<&XOnlyPublicKey, &taproot::Signature>,
    )> {
        let input = &self.inputs[input_index];

        let mut tap_scripts: Vec<_> = input.tap_scripts.iter().collect();
        tap_scripts.sort_by(|a, b| a.0.serialize().len().cmp(&b.0.serialize().len()));

        tap_scripts
            .into_iter()
            .find_map(|(control_block, (script, leaf_version))| {
                let leaf_hash = TapLeafHash::from_script(script, *leaf_version);

                let sigs_map: BTreeMap<&XOnlyPublicKey, &taproot::Signature> = input
                    .tap_script_sigs
                    .iter()
                    .filter(|((_, hash), _)| *hash == leaf_hash)
                    .collect::<BTreeMap<_, _>>()
                    .into_iter()
                    .map(|(k, v)| (&k.0, v))
                    .collect::<BTreeMap<_, _>>();

                if sigs_map.is_empty() {
                    None
                } else {
                    Some((script.clone(), control_block.clone(), leaf_hash, sigs_map))
                }
            })
    }

    fn calculate_push_bytes_positions(
        &self,
        script: &ScriptBuf,
    ) -> BTreeMap<AliasPushBytes, usize> {
        let mut push_bytes: Vec<AliasPushBytes> = vec![];

        for ins in script.instructions().flatten() {
            if let Some(data_push) = ins.push_bytes() {
                push_bytes.push(AliasPushBytes(data_push.as_bytes().to_vec()));
            }
        }

        push_bytes.reverse();

        let mut positions: BTreeMap<AliasPushBytes, usize> = BTreeMap::new();

        for (pos, push_byte) in push_bytes.iter().enumerate() {
            positions.insert(push_byte.clone(), pos);
        }

        positions
    }

    fn get_push_bytes_position(
        &self,
        push_bytes_positions: &BTreeMap<AliasPushBytes, usize>,
        key: &XOnlyPublicKey,
    ) -> Option<usize> {
        let result = push_bytes_positions.get(&AliasPushBytes(key.serialize().to_vec()));
        if result.is_some() {
            return result.copied();
        }

        let pubkey_hash = hash160::Hash::hash(&key.serialize());
        let result =
            push_bytes_positions.get(&AliasPushBytes(pubkey_hash.to_byte_array().to_vec()));

        if result.is_some() {
            return result.copied();
        }

        None
    }

    /// Attempts to create all signatures required by this PSBT's `tap_key_origins` field, adding
    /// them to `tap_key_sig` or `tap_script_sigs`.
    ///
    /// # Returns
    ///
    /// - Ok: A list of the xonly public keys used in signing. When signing a key path spend we
    ///   return the internal key.
    /// - Err: Error encountered trying to calculate the sighash AND we had the signing key.
    fn key_map_sign_schnorr<C, T>(
        &mut self,
        key_map: &SigningKeyMap,
        input_index: usize,
        cache: &mut SighashCache<T>,
        secp: &Secp256k1<C>,
    ) -> Result<Vec<XOnlyPublicKey>, SignError>
    where
        C: Signing + Verification,
        T: Borrow<Transaction>,
    {
        let mut input = self.checked_input(input_index)?.clone();

        let mut used = vec![]; // List of pubkeys used to sign the input.

        for (&xonly, (leaf_hashes, _)) in input.tap_key_origins.iter() {
            let key: Secp256k1PublicKey =
                Secp256k1PublicKey::from_x_only_public_key(xonly, Parity::Even); // even or odd is not relevant for signing, just needs to be consistent with the KeyRequest::Pubkey
            let pubkey: PublicKey = key.into();
            let sk = if let Ok(Some(secret_key)) = key_map.get_key(KeyRequest::Pubkey(pubkey), secp)
            {
                secret_key
            } else {
                continue;
            };

            // Considering the responsibility of the PSBT's finalizer to extract valid signatures,
            // the goal of this algorithm is to provide signatures to the best of our ability:
            // 1) If the conditions for key path spend are met, proceed to provide the signature for key path spend
            // 2) If the conditions for script path spend are met, proceed to provide the signature for script path spend

            // key path spend
            if let Some(internal_key) = input.tap_internal_key {
                // BIP 371: The internal key does not have leaf hashes, so can be indicated with a hashes len of 0.
                // Based on input.tap_internal_key.is_some() alone, it is not sufficient to determine whether it is a key path spend.
                // According to BIP 371, we also need to consider the condition leaf_hashes.is_empty() for a more accurate determination.
                if internal_key == xonly && leaf_hashes.is_empty() && input.tap_key_sig.is_none() {
                    let (msg, sighash_type) = self.sighash_taproot(input_index, cache, None)?;
                    let key_pair = Keypair::from_secret_key(secp, &sk.inner)
                        .tap_tweak(secp, input.tap_merkle_root)
                        .to_inner();

                    #[cfg(feature = "rand-std")]
                    let signature = secp.sign_schnorr(&msg, &key_pair);
                    #[cfg(not(feature = "rand-std"))]
                    let signature = secp.sign_schnorr_no_aux_rand(&msg, &key_pair);

                    let signature = taproot::Signature {
                        signature,
                        sighash_type,
                    };

                    input.tap_key_sig = Some(signature);

                    used.push(internal_key);
                }
            }

            // script path spend
            if let Some((leaf_hashes, _)) = input.tap_key_origins.get(&xonly) {
                let leaf_hashes = leaf_hashes
                    .iter()
                    .filter(|lh| !input.tap_script_sigs.contains_key(&(xonly, **lh)))
                    .cloned()
                    .collect::<Vec<_>>();

                if !leaf_hashes.is_empty() {
                    let key_pair = Keypair::from_secret_key(secp, &sk.inner);

                    for lh in leaf_hashes {
                        let (msg, sighash_type) =
                            self.sighash_taproot(input_index, cache, Some(lh))?;

                        #[cfg(feature = "rand-std")]
                        let signature = secp.sign_schnorr(&msg, &key_pair);
                        #[cfg(not(feature = "rand-std"))]
                        let signature = secp.sign_schnorr_no_aux_rand(&msg, &key_pair);

                        let signature = taproot::Signature {
                            signature,
                            sighash_type,
                        };

                        input.tap_script_sigs.insert((xonly, lh), signature);
                    }

                    used.push(sk.public_key(secp).into());
                }
            }
        }

        self.inputs[input_index] = input; // input_index is checked above.

        Ok(used)
    }

    /// Returns the sighash message to sign an SCHNORR input along with the sighash type.
    ///
    /// Uses the [`TapSighashType`] from this input if one is specified. If no sighash type is
    /// specified uses [`TapSighashType::Default`].
    fn sighash_taproot<T: Borrow<Transaction>>(
        &self,
        input_index: usize,
        cache: &mut SighashCache<T>,
        leaf_hash: Option<TapLeafHash>,
    ) -> Result<(Message, TapSighashType), SignError> {
        use OutputType::*;

        if self.signing_algorithm(input_index)? != SigningAlgorithm::Schnorr {
            return Err(SignError::WrongSigningAlgorithm);
        }

        let input = self.checked_input(input_index)?;

        match self.output_type(input_index)? {
            Tr => {
                let hash_ty = input
                    .sighash_type
                    .unwrap_or_else(|| TapSighashType::Default.into())
                    .taproot_hash_ty()
                    .map_err(|_| SignError::InvalidSighashType)?;

                let spend_utxos = (0..self.inputs.len())
                    .map(|i| self.spend_utxo(i).ok())
                    .collect::<Vec<_>>();
                let all_spend_utxos;

                let is_anyone_can_pay = PsbtSighashType::from(hash_ty).to_u32() & 0x80 != 0;

                let prev_outs = if is_anyone_can_pay {
                    Prevouts::One(
                        input_index,
                        spend_utxos[input_index].ok_or(SignError::MissingSpendUtxo)?,
                    )
                } else if spend_utxos.iter().all(Option::is_some) {
                    all_spend_utxos = spend_utxos.iter().filter_map(|x| *x).collect::<Vec<_>>();
                    Prevouts::All(&all_spend_utxos)
                } else {
                    return Err(SignError::MissingSpendUtxo);
                };

                let sighash = if let Some(leaf_hash) = leaf_hash {
                    cache.taproot_script_spend_signature_hash(
                        input_index,
                        &prev_outs,
                        leaf_hash,
                        hash_ty,
                    )?
                } else {
                    cache.taproot_key_spend_signature_hash(input_index, &prev_outs, hash_ty)?
                };
                Ok((Message::from(sighash), hash_ty))
            }
            _ => Err(SignError::Unsupported),
        }
    }

    fn signing_algorithm(&self, input_index: usize) -> Result<SigningAlgorithm, SignError> {
        let output_type = self.output_type(input_index)?;
        let signing_algorithm = output_type.signing_algorithm();
        Ok(signing_algorithm)
    }

    /// Returns the [`OutputType`] of the spend utxo for this PBST's input at `input_index`.
    fn output_type(&self, input_index: usize) -> Result<OutputType, SignError> {
        let input = self.checked_input(input_index)?;
        let utxo = self.spend_utxo(input_index)?;
        let spk = utxo.script_pubkey.clone();

        // Anything that is not segwit and is not p2sh is `Bare`.
        if !(spk.is_witness_program() || spk.is_p2sh()) {
            return Ok(OutputType::Bare);
        }

        if spk.is_p2wpkh() {
            return Ok(OutputType::Wpkh);
        }

        if spk.is_p2wsh() {
            return Ok(OutputType::Wsh);
        }

        if spk.is_p2sh() {
            if input
                .redeem_script
                .as_ref()
                .map(|s| s.is_p2wpkh())
                .unwrap_or(false)
            {
                return Ok(OutputType::ShWpkh);
            }
            if input
                .redeem_script
                .as_ref()
                .map(|x| x.is_p2wsh())
                .unwrap_or(false)
            {
                return Ok(OutputType::ShWsh);
            }
            return Ok(OutputType::Sh);
        }

        if spk.is_p2tr() {
            return Ok(OutputType::Tr);
        }

        // Something is wrong with the input scriptPubkey or we do not know how to sign
        // because there has been a new softfork that we do not yet support.
        Err(SignError::UnknownOutputType)
    }

    /// Gets the input at `input_index` after checking that it is a valid index.
    fn checked_input(&self, input_index: usize) -> Result<&Input, IndexOutOfBoundsError> {
        self.check_index_is_within_bounds(input_index)?;
        Ok(&self.inputs[input_index])
    }

    /// Checks `input_index` is within bounds for the PSBT `inputs` array and
    /// for the PSBT `unsigned_tx` `input` array.
    fn check_index_is_within_bounds(
        &self,
        input_index: usize,
    ) -> Result<(), IndexOutOfBoundsError> {
        if input_index >= self.inputs.len() {
            return Err(IndexOutOfBoundsError::Inputs {
                index: input_index,
                length: self.inputs.len(),
            });
        }

        if input_index >= self.unsigned_tx.input.len() {
            return Err(IndexOutOfBoundsError::TxInput {
                index: input_index,
                length: self.unsigned_tx.input.len(),
            });
        }

        Ok(())
    }

    fn is_taproot_input(&self, input_index: usize) -> bool {
        let input = &self.inputs[input_index];
        input.tap_internal_key.is_some()
            || input.tap_merkle_root.is_some()
            || (!input.tap_scripts.is_empty())
            || (input.witness_utxo.is_some()
                && input.witness_utxo.as_ref().unwrap().script_pubkey.is_p2tr())
    }

    /// Note: This method only works for ECDSA inputs (not for Schnorr inputs).
    /// Signs an ECDSA input by a key map.
    fn key_map_sign_ecdsa<C, T>(
        &mut self,
        key_map: &SigningKeyMap,
        input_index: usize,
        cache: &mut SighashCache<T>,
        secp: &Secp256k1<C>,
    ) -> Result<Vec<PublicKey>, SignError>
    where
        C: Signing + Verification,
        T: Borrow<Transaction>,
    {
        let msg_sighash_ty_res = self.sighash_ecdsa(input_index, cache);

        let input = &mut self.inputs[input_index]; // Index checked in call to `sighash_ecdsa`.

        let mut used = vec![]; // List of pubkeys used to sign the input.

        for (pk, _) in input.tap_key_origins.iter() {
            let key: Secp256k1PublicKey =
                Secp256k1PublicKey::from_x_only_public_key(*pk, Parity::Even); // even or odd is not relevant for signing, just needs to be consistent with the KeyRequest::Pubkey
            let pubkey: PublicKey = key.into();

            let sk = if let Ok(Some(sk)) = key_map.get_key(KeyRequest::Pubkey(pubkey), secp) {
                sk
            } else {
                continue;
            };

            // // Only return the error if we have a secret key to sign this input.
            let (msg, sighash_ty) = match msg_sighash_ty_res {
                Err(e) => return Err(e),
                Ok((msg, sighash_ty)) => (msg, sighash_ty),
            };

            let sig = ecdsa::Signature {
                signature: secp.sign_ecdsa(&msg, &sk.inner),
                sighash_type: sighash_ty,
            };

            let pk = sk.public_key(secp);

            input.partial_sigs.insert(pk, sig);
            used.push(pk);
        }

        Ok(used)
    }
}
