use bitcoin::{
    absolute,
    hashes::{sha256d::Hash as Sha256dHash, Hash},
    key::Secp256k1,
    opcodes::all::{
        OP_CHECKSIG, OP_CHECKSIGADD, OP_CHECKSIGVERIFY, OP_GREATERTHANOREQUAL, OP_RETURN,
    },
    psbt::{Input, PsbtSighashType},
    script,
    secp256k1::All,
    taproot::TaprootBuilder,
    transaction, Amount, Psbt, PublicKey, ScriptBuf, Sequence, TapSighashType, Transaction, TxIn,
    TxOut, Witness, XOnlyPublicKey,
};

use lazy_static::lazy_static;

use super::{
    CreateStakingParams, DestinationAddress, DestinationChainId, StakingError,
    EMBEDDED_DATA_SCRIPT_SIZE, NUM_OUTPUTS, TAG_HASH_SIZE, UTXO,
};

lazy_static! {
    pub static ref NUMS_BIP_341: XOnlyPublicKey = XOnlyPublicKey::from_slice(&[
        0x50, 0x92, 0x9b, 0x74, 0xc1, 0xa0, 0x49, 0x54, 0xb7, 0x8b, 0x4b, 0x60, 0x35, 0xe9, 0x7a,
        0x5e, 0x07, 0x8a, 0x5a, 0x0f, 0x28, 0xec, 0x96, 0xd5, 0x47, 0xbf, 0xee, 0x9a, 0xce, 0x80,
        0x3a, 0xc0,
    ])
    .unwrap();
}

pub trait Staking {
    type Error;

    fn new(tag: Vec<u8>, version: u8) -> Self;
    fn create(&self, params: &CreateStakingParams) -> Result<Psbt, Self::Error>;
}

pub struct StakingManager {
    secp: Secp256k1<All>,
    tag: Vec<u8>,
    version: u8,
}

impl Staking for StakingManager {
    type Error = StakingError;

    fn new(tag: Vec<u8>, version: u8) -> Self {
        let secp = Secp256k1::new();
        Self { secp, tag, version }
    }

    // This function is used to create an unsigned PSBT for staking
    fn create(&self, params: &CreateStakingParams) -> Result<Psbt, Self::Error> {
        // TODO: 0.validate params
        let user_pub_key_x_only = params.user_pub_key.inner.x_only_public_key().0;
        let protocol_pub_key_x_only = params.protocol_pub_key.inner.x_only_public_key().0;
        let covenant_pubkeys_x_only = params
            .covenant_pubkeys
            .iter()
            .map(|pk| pk.inner.x_only_public_key().0)
            .collect();

        // 1a. Construct the locking script with Taproot
        let lock_script = Self::create_locking_script(
            &self.secp,
            &user_pub_key_x_only,
            &protocol_pub_key_x_only,
            &covenant_pubkeys_x_only,
            params.covenant_quorum,
            params.have_only_covenants,
        )?;

        // 1b. Create the embedded_data_script
        let embedded_data_script = Self::create_embedded_data_script(
            &self.tag,
            self.version,
            &params.destination_chain_id,
            &params.destination_address,
            &params.destination_recipient_address,
        )?;

        // 2. Calculate the total input amount
        let total_input_amount: u64 =
            Self::calculate_total_input_amount(&params.utxos, params.staking_amount)?;

        // 3. Calculate the fee and create the outputs, including the lock and change if necessary
        let num_inputs = params.utxos.len();
        let tx_outputs = Self::create_tx_outputs(
            total_input_amount,
            params.staking_amount,
            num_inputs,
            params.fee_rate,
            lock_script,
            embedded_data_script,
        )?;

        // 4. Construct the transaction
        let tx_inputs: Vec<TxIn> = params
            .utxos
            .iter()
            .map(|utxo| TxIn {
                previous_output: utxo.outpoint,
                script_sig: ScriptBuf::default(),
                sequence: match params.rbf {
                    true => Sequence::ENABLE_RBF_NO_LOCKTIME,
                    false => Sequence::MAX,
                },
                witness: Witness::default(),
            })
            .collect();

        let unsigned_tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: tx_inputs,
            output: tx_outputs,
        };

        // 5. Create the PSBT
        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| StakingError::FailedToCreatePSBT)?;

        let ty: PsbtSighashType = TapSighashType::All.into();

        psbt.inputs = params
            .utxos
            .iter()
            .map(|utxo| Self::utxo_to_input(utxo, &params.user_pub_key, &params.script_pubkey, &ty))
            .collect();

        Ok(psbt)
    }
}

impl StakingManager {
    fn create_tx_outputs(
        total_input_amount: u64,
        staking_amount: u64,
        num_inputs: usize,
        fee_rate: u64,
        lock_script: ScriptBuf,
        embedded_data_script: ScriptBuf,
    ) -> Result<Vec<TxOut>, StakingError> {
        let fee_amount = Self::calculate_fee_amount(num_inputs, NUM_OUTPUTS, fee_rate)?;
        let change_amount = total_input_amount - staking_amount - fee_amount;

        Ok(vec![
            TxOut {
                value: Amount::from_sat(staking_amount),
                script_pubkey: lock_script,
            },
            TxOut {
                value: Amount::from_sat(change_amount),
                script_pubkey: ScriptBuf::default(),
            },
            TxOut {
                value: Amount::from_sat(fee_amount),
                script_pubkey: embedded_data_script,
            },
        ])
    }

    fn create_locking_script(
        secp: &Secp256k1<All>,
        user_pub_key: &XOnlyPublicKey,
        protocol_pub_key: &XOnlyPublicKey,
        covenant_pubkeys: &Vec<XOnlyPublicKey>,
        covenant_quorum: u8,
        have_only_covenants: bool,
    ) -> Result<ScriptBuf, StakingError> {
        let mut builder = TaprootBuilder::new();

        let user_protocol_branch = Self::user_protocol_banch(user_pub_key, protocol_pub_key);
        let covenants_protocol_branch =
            Self::covenants_protocol_branch(covenant_pubkeys, covenant_quorum, protocol_pub_key)?;
        let covenants_user_branch =
            Self::covenants_user_branch(covenant_pubkeys, covenant_quorum, user_pub_key)?;

        builder = builder.add_leaf(0, user_protocol_branch)?;
        builder = builder.add_leaf(1, covenants_protocol_branch)?;

        if have_only_covenants {
            builder = builder.add_leaf(2, covenants_user_branch)?;
            let only_covenants_branch =
                Self::only_covenants_branch(covenant_pubkeys, covenant_quorum)?;
            builder = builder.add_leaf(2, only_covenants_branch)?;
        } else {
            builder = builder.add_leaf(1, covenants_user_branch)?;
        }

        let taproot_spend_info = builder
            .finalize(secp, *NUMS_BIP_341)
            .map_err(|_| StakingError::TaprootFinalizationFailed)?;

        Ok(ScriptBuf::new_p2tr(
            secp,
            taproot_spend_info.internal_key(),
            taproot_spend_info.merkle_root(),
        ))
    }

    fn user_protocol_banch(
        user_pub_key: &XOnlyPublicKey,
        service_pub_key: &XOnlyPublicKey,
    ) -> ScriptBuf {
        script::Builder::new()
            .push_x_only_key(&user_pub_key)
            .push_opcode(OP_CHECKSIGVERIFY)
            .push_x_only_key(&service_pub_key)
            .push_opcode(OP_CHECKSIG)
            .into_script()
    }

    fn covenants_protocol_branch(
        covenant_pubkeys: &Vec<XOnlyPublicKey>,
        covenant_quorum: u8,
        protocol_pub_key: &XOnlyPublicKey,
    ) -> Result<ScriptBuf, StakingError> {
        Self::create_covenant_branch(covenant_pubkeys, covenant_quorum, Some(protocol_pub_key))
    }

    fn covenants_user_branch(
        covenant_pubkeys: &Vec<XOnlyPublicKey>,
        covenant_quorum: u8,
        user_pub_key: &XOnlyPublicKey,
    ) -> Result<ScriptBuf, StakingError> {
        Self::create_covenant_branch(covenant_pubkeys, covenant_quorum, Some(user_pub_key))
    }

    fn only_covenants_branch(
        covenant_pubkeys: &Vec<XOnlyPublicKey>,
        covenant_quorum: u8,
    ) -> Result<ScriptBuf, StakingError> {
        Self::create_covenant_branch(covenant_pubkeys, covenant_quorum, None)
    }

    fn create_covenant_branch(
        covenant_pubkeys: &Vec<XOnlyPublicKey>,
        covenant_quorum: u8,
        initial_key: Option<&XOnlyPublicKey>,
    ) -> Result<ScriptBuf, StakingError> {
        let mut builder = script::Builder::new();

        // Initial key check
        if let Some(initial_key) = initial_key {
            builder = builder
                .push_x_only_key(initial_key)
                .push_opcode(OP_CHECKSIGVERIFY);
        }

        // Sort covenant public keys
        let mut sorted_pks = covenant_pubkeys.clone();
        sorted_pks.sort_by(|a, b| a.cmp(b));

        // Check for duplicates
        for i in 0..sorted_pks.len() - 1 {
            if sorted_pks[i] == sorted_pks[i + 1] {
                return Err(StakingError::DuplicateCovenantKeys);
            }
        }

        // Add covenant keys to the script
        builder = builder.push_x_only_key(&sorted_pks[0]);
        builder = builder.push_opcode(OP_CHECKSIG);

        for pk in sorted_pks.iter().skip(1) {
            builder = builder.push_x_only_key(pk);
            builder = builder.push_opcode(OP_CHECKSIGADD);
        }

        // Add quorum check
        builder = builder
            .push_int(covenant_quorum as i64)
            .push_opcode(OP_GREATERTHANOREQUAL);

        Ok(builder.into_script())
    }

    fn create_embedded_data_script(
        tag: &Vec<u8>,
        version: u8,
        destination_chain_id: &DestinationChainId,
        destination_address: &DestinationAddress,
        destination_recipient_address: &DestinationAddress,
    ) -> Result<ScriptBuf, StakingError> {
        let hash: [u8; TAG_HASH_SIZE];
        let tag_bytes = tag.as_slice();
        if tag.len() <= TAG_HASH_SIZE {
            hash = tag_bytes[0..TAG_HASH_SIZE]
                .try_into()
                .map_err(|_| StakingError::InvalidTag)?;
        } else {
            hash = Sha256dHash::hash(tag_bytes)[0..TAG_HASH_SIZE]
                .try_into()
                .map_err(|_| StakingError::InvalidTag)?;
        }

        let embedded_data_script = script::Builder::new()
            .push_opcode(OP_RETURN)
            .push_int(EMBEDDED_DATA_SCRIPT_SIZE as i64)
            .push_slice(hash)
            .push_slice(&version.to_le_bytes())
            .push_slice(destination_chain_id)
            .push_slice(destination_address)
            .push_slice(destination_recipient_address)
            .into_script();

        Ok(embedded_data_script)
    }

    fn calculate_total_input_amount(
        utxos: &Vec<UTXO>,
        staking_amount: u64,
    ) -> Result<u64, StakingError> {
        let total_input_amount: u64 = utxos.iter().map(|utxo| utxo.amount_in_sats.to_sat()).sum();
        if total_input_amount < staking_amount {
            return Err(StakingError::InsufficientUTXOs {
                required: staking_amount,
                available: total_input_amount,
            });
        }
        Ok(total_input_amount)
    }

    fn utxo_to_input(
        utxo: &UTXO,
        pubkey: &PublicKey,
        script_pubkey: &ScriptBuf,
        ty: &PsbtSighashType,
    ) -> Input {
        let mut input = Input {
            witness_utxo: Some(TxOut {
                value: utxo.amount_in_sats,
                script_pubkey: script_pubkey.clone(),
            }),
            sighash_type: Some(*ty),
            ..Default::default()
        };

        let is_p2tr = script_pubkey.is_p2tr();

        match is_p2tr {
            true => {
                input.tap_internal_key = Some(pubkey.inner.x_only_public_key().0);
            }

            false => {
                // TODO: handle p2sh, p2wpkh, p2pkh
            }
        }

        input
    }

    fn calculate_fee_amount(
        num_inputs: usize,
        num_outputs: usize,
        fee_rate: u64,
    ) -> Result<u64, StakingError> {
        // Estimate vsize for a Taproot transaction
        // This is a simplified estimation and may need further refinement
        let vsize = 10.25 + (num_inputs as f64 * 57.75) + (num_outputs as f64 * 43.0);

        // Convert vsize to weight units (1 vbyte = 4 weight units)
        let weight = (vsize * 4.0).ceil() as u64;

        // Calculate the fee (weight * fee_rate / 4)
        let fee_amount = weight * fee_rate / 4;

        Ok(fee_amount)
    }
}
