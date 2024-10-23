use bitcoin::{
    address::NetworkChecked,
    bip32::Xpub,
    key::Secp256k1,
    secp256k1::{All, PublicKey, SecretKey},
    taproot::TaprootBuilder,
    Address, OutPoint, Psbt, Script, ScriptBuf, Sequence, TxIn, TxOut, Witness, XOnlyPublicKey,
};

use crate::{error::StakingError, utxo::UTXO};

pub struct StakingManager {
    secp: Secp256k1<All>,
}

pub struct CreateStakingParams {
    user_priv_key: SecretKey,
    protocol_pub_key: PublicKey,
    covenant_pubkeys: Vec<PublicKey>,
    covenant_quorum: u8,
    staking_amount: u64,
    reciever_address: Address<NetworkChecked>,
    utxos: Vec<UTXO>,
    rbf: bool,
    fee_rate: u64,
}

pub trait Staking {
    type Error;

    fn new(secp: Secp256k1<All>) -> Self;
    // Create a PSBT for staking
    fn create(&self, params: &CreateStakingParams) -> Result<Psbt, Self::Error>;
}

impl Staking for StakingManager {
    type Error = StakingError;

    fn new(secp: Secp256k1<All>) -> Self {
        Self { secp }
    }

    // TODO: validate params
    fn create(&self, params: &CreateStakingParams) -> Result<Psbt, Self::Error> {
        // 1.Calculate Fees:
        // Estimate the size of the transaction to calculate the fee using a fee rate.
        // Account for the number of inputs (UTXOs), outputs (stake, OP_RETURN, and change), and the witness data.
        let total_input_amount: u64 =
            Self::calculate_total_input_amount(&params.utxos, params.staking_amount)?;

        let secp = Secp256k1::new();

        // 2. Construct the locking script with Taproot
        let user_pub_key = PublicKey::from_secret_key(&secp, &params.user_priv_key);
        let user_pub_key_xonly = XOnlyPublicKey::from(user_pub_key);
        let lock_script = Self::create_locking_script(
            &self.secp,
            &user_pub_key,
            &params.protocol_pub_key,
            &params.covenant_pubkeys,
        )?;

        // 3. Calculate the fee and create the outputs (lock and change if necessary)
        let tx_outputs = Self::create_tx_outputs(
            params.staking_amount,
            params.fee_rate,
            params.utxos.len(),
            &lock_script,
        )?;

        // 4. Construct the transaction
        let inputs: Vec<TxIn> = params
            .utxos
            .iter()
            .map(|utxo| TxIn {
                previous_output: utxo.outpoint,
                script_sig: ScriptBuf::default(),
                sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::default(),
            })
            .collect();

        let unsigned_tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: inputs,
            output: tx_outputs,
        };

        let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).expect("could not create PSBT");

        todo!()

        //     let ty = TapSighashType::All.into();
        // psbt.inputs = vec![
        //     Input {
        //         witness_utxo: Some(utxos[0].clone()),
        //         tap_key_origins: origins[0].clone(),
        //         tap_internal_key: Some(pk_input_1),
        //         sighash_type: Some(ty),
        //         ..Default::default()
        //     },
        //     Input {
        //         witness_utxo: Some(utxos[1].clone()),
        //         tap_key_origins: origins[1].clone(),
        //         tap_internal_key: Some(pk_input_2),
        //         sighash_type: Some(ty),
        //         ..Default::default()
        //     },
        // ];

        // // // Step 3: Signer role; that signs the PSBT.
        // // psbt.sign(&master_xpriv, &secp).expect("valid signature");

        // // // Step 4: Finalizer role; that finalizes the PSBT.
        // // psbt.inputs.iter_mut().for_each(|input| {
        // //     let script_witness = Witness::p2tr_key_spend(&input.tap_key_sig.unwrap());
        // //     input.final_script_witness = Some(script_witness);

        // //     // Clear all the data fields as per the spec.
        // //     input.partial_sigs = BTreeMap::new();
        // //     input.sighash_type = None;
        // //     input.redeem_script = None;
        // //     input.witness_script = None;
        // //     input.bip32_derivation = BTreeMap::new();
        // // });

        // // // BOOM! Transaction signed and ready to broadcast.
        // // let signed_tx = psbt.extract_tx().expect("valid transaction");
        // // let serialized_signed_tx = consensus::encode::serialize_hex(&signed_tx);
        // // println!("Transaction Details: {:#?}", signed_tx);
        // // // check with:
        // // // bitcoin-cli decoderawtransaction <RAW_TX> true
        // // println!("Raw Transaction: {}", serialized_signed_tx);
    }
}

impl StakingManager {
    fn create_tx_outputs(
        staking_amount: u64,
        fee_rate: u64,
        num_inputs: usize,
        lock_script: &Script,
    ) -> Result<Vec<TxOut>, StakingError> {
        todo!()
    }

    fn create_locking_script(
        secp: &Secp256k1<All>,
        user_pub_key: &PublicKey,
        protocol_pub_key: &PublicKey,
        covenant_pubkeys: &Vec<PublicKey>,
    ) -> Result<ScriptBuf, StakingError> {
        let mut builder = TaprootBuilder::new();

        // Define the Taproot tree (3 branches).
        // 1. User and 3rd-party service can unlock.
        // builder = builder.add_leaf(0, user_protocol_banch(user_pub_key, service_pub_key))?;

        // 2. User and 3 out of 5 covenants can unlock.
        // builder = builder.add_leaf(0, user_covenant_branch(user_pub_key, covenant_pubkeys, 3))?;

        // // 3. 3rd-party service and 3 out of 5 covenants can unlock.
        // builder = builder.add_leaf(
        //     0,
        //     service_covenant_branch(service_pub_key, covenant_pubkeys, 3),
        // )?;

        // let taproot_output_key = builder.finalize(&Secp256k1::new())?;

        todo!()
        // Ok(ScriptBuf::new_p2tr(
        //     &Secp256k1::new(),
        //     taproot_output_key.output_key(),
        //     None,
        // ))
    }

    fn user_protocol_banch(user_pub_key: &PublicKey, service_pub_key: &PublicKey) -> Box<Script> {
        let musig = MuSig2::new(secp, user_pub_key, service_pub_key);
        todo!()
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

    fn estimate_tx_size(num_inputs: usize, num_outputs: usize) -> usize {
        // Estimate size: 68 vbytes per input, 31 vbytes per output
        // TODO: Add witness size
        num_inputs * 68 + num_outputs * 31
    }
}
