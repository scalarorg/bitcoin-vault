use bitcoin::{Psbt, PublicKey, Sequence, TxOut};

use crate::{
    convert_pubkey_to_x_only_key, convert_pubkeys_to_x_only_keys, get_global_secp, CoreError,
    LockingOutput, LockingScript, TaprootTree, TimeGated, TimeGatedLockingParams, TimeGatedTree,
    TimeGatedUnlockingParams, TimeGatedUnlockingType, TransactionBuilder, UnlockingFeeParams,
    VaultManager,
};

impl TimeGated for VaultManager {
    type Error = CoreError;

    fn build_locking_output(
        &self,
        params: &TimeGatedLockingParams,
    ) -> Result<LockingOutput, Self::Error> {
        let locking_script = <Self as TimeGated>::locking_script(
            &params.party_pubkey,
            &params.custodian_pubkeys,
            params.custodian_quorum,
            params.sequence,
        )?;

        Ok(LockingOutput::new(
            params.locking_amount,
            locking_script,
            None,
        ))
    }

    fn locking_script(
        party: &PublicKey,
        custodian_pubkeys: &[PublicKey],
        custodian_quorum: u8,
        sequence: u16,
    ) -> Result<LockingScript, Self::Error> {
        let secp = get_global_secp();
        let party_x_only_pubkey = convert_pubkey_to_x_only_key(party);
        let x_only_pubkeys = convert_pubkeys_to_x_only_keys(custodian_pubkeys);

        let tree = TaprootTree::<TimeGatedTree>::new(
            secp,
            &party_x_only_pubkey,
            &x_only_pubkeys,
            custodian_quorum,
            sequence,
        )?;
        Ok(LockingScript(tree.into_script(secp)))
    }

    fn build_unlocking_psbt(
        &self,
        params: &TimeGatedUnlockingParams,
    ) -> Result<bitcoin::Psbt, Self::Error> {
        let secp = get_global_secp();
        let party_x_only_pubkey = convert_pubkey_to_x_only_key(&params.party_pubkey);
        let x_only_pubkeys = convert_pubkeys_to_x_only_keys(&params.custodian_pubkeys);

        let tree = TaprootTree::<TimeGatedTree>::new(
            secp,
            &party_x_only_pubkey,
            &x_only_pubkeys,
            params.custodian_quorum,
            params.sequence,
        )?;

        let mut tx_builder = TransactionBuilder::new(true);

        tx_builder.add_input_with_sequence(
            params.input.outpoint,
            Sequence::from_height(params.sequence),
        );

        tx_builder.add_raw_output(TxOut {
            script_pubkey: params.script_pubkey.clone(),
            value: params.input.amount_in_sats,
        });

        let mut unsigned_tx = tx_builder.build();

        let fee = self.calculate_unlocking_fee(UnlockingFeeParams {
            n_inputs: unsigned_tx.input.len() as u64,
            n_outputs: unsigned_tx.output.len() as u64,
            fee_rate: params.fee_rate,
            quorum: params.custodian_quorum,
        });

        self.distribute_fee(&mut unsigned_tx, params.input.amount_in_sats, fee)?;

        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

        let (branch, keys) = match params.typ {
            TimeGatedUnlockingType::CustodianOnly => {
                (&tree.raw.custodian_only_branch, x_only_pubkeys)
            }
            TimeGatedUnlockingType::PartyTimeGated => {
                (&tree.raw.csv_party_branch, vec![party_x_only_pubkey])
            }
        };

        psbt.inputs = self.prepare_psbt_inputs(&[params.input.clone()], &tree.root, branch, &keys);

        Ok(psbt)
    }
}
