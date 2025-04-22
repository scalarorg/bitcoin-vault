use bitcoin::{Psbt, PublicKey, TxOut};

use crate::{
    convert_pubkey_to_x_only_key, convert_pubkeys_to_x_only_keys, get_global_secp, CoreError,
    LockingOutput, LockingScript, TaprootTree, TimeGated, TimeGatedLockingParams, TimeGatedTree,
    TimeGatedUnlockingParams, TransactionBuilder, UnlockingParams, VaultManager,
};

impl TimeGated for VaultManager {
    type Error = CoreError;

    fn build_locking_output(
        &self,
        params: &TimeGatedLockingParams,
    ) -> Result<LockingOutput, Self::Error> {
        let locking_script = <Self as TimeGated>::locking_script(
            &params.party_pubkey,
            &params.custodian_pub_keys,
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
        custodian_pub_keys: &[PublicKey],
        custodian_quorum: u8,
        sequence: i64,
    ) -> Result<LockingScript, Self::Error> {
        let secp = get_global_secp();
        let keys = convert_pubkeys_to_x_only_keys(custodian_pub_keys);
        let party = convert_pubkey_to_x_only_key(party);

        let tree =
            TaprootTree::<TimeGatedTree>::new(secp, &party, &keys, custodian_quorum, sequence)?;

        Ok(LockingScript(tree.into_script(secp)))
    }

    fn build_unlocking_psbt(
        &self,
        params: &TimeGatedUnlockingParams,
    ) -> Result<bitcoin::Psbt, Self::Error> {
        unimplemented!()
        // let secp = get_global_secp();
        // let (total_input_value, total_output_value) = params.validate()?;

        // let party = convert_pubkey_to_x_only_key(&params.party_pubkey);
        // let keys = convert_pubkeys_to_x_only_keys(&params.custodian_pub_keys);

        // let tree = TaprootTree::<TimeGatedTree>::new(
        //     secp,
        //     &party,
        //     &keys,
        //     params.custodian_quorum,
        //     params.sequence,
        // )?;

        // let mut tx_builder = TransactionBuilder::new(params.rbf);

        // self.add_inputs_to_builder(&mut tx_builder, &params.inputs);

        // tx_builder.add_raw_output(&TxOut {
        //     script_pubkey:
        //     value:
        // });

        // let mut unsigned_tx = tx_builder.build();

        // let fee = self.calculate_unlocking_fee(UnlockingFeeParams {
        //     n_inputs: unsigned_tx.input.len() as u64,
        //     n_outputs: unsigned_tx.output.len() as u64,
        //     fee_rate: params.fee_rate,
        //     quorum: params.custodian_quorum,
        // });

        // self.distribute_fee(&mut unsigned_tx, params.total_output_value, fee)?;

        // if change > Amount::ZERO {
        //     self.replace_change_output(&mut unsigned_tx, change, params.script);
        // }

        // let mut psbt =
        //     Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

        // let (branch, keys) = (tree.raw, x_only_pub_keys);

        // psbt.inputs = self.prepare_psbt_inputs(
        //     &params.inputs,
        //     &tree.root,
        //     &branch.only_custodian_branch,
        //     &keys,
        // );

        // Ok(psbt)
    }
}
