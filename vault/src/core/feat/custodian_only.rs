use bitcoin::{opcodes::all::OP_RETURN, script::Builder, Psbt, PublicKey};

use crate::{
    convert_pubkeys_to_x_only_keys, get_global_secp, CoreError, CustodianOnly,
    CustodianOnlyLockingParams, CustodianOnlyTree, CustodianOnlyUnlockingParams, DataScript,
    DestinationChain, DestinationRecipientAddress, DestinationTokenAddress, LockingOutput,
    LockingScript, TaprootTree, TaprootTreeType, UnlockingParams, UnlockingTaprootTreeType,
    VaultManager, EMBEDDED_DATA_SCRIPT_SIZE,
};

impl CustodianOnly for VaultManager {
    type Error = CoreError;

    fn build_locking_output(
        &self,
        params: &CustodianOnlyLockingParams,
    ) -> Result<LockingOutput, Self::Error> {
        let locking_script = <Self as CustodianOnly>::locking_script(
            &params.custodian_pubkeys,
            params.custodian_quorum,
        )?;

        let data_script = <Self as CustodianOnly>::data_script(
            self,
            params.custodian_quorum,
            &params.destination_chain,
            &params.destination_token_address,
            &params.destination_recipient_address,
        )?;

        Ok(LockingOutput::new(
            params.locking_amount,
            locking_script,
            Some(data_script),
        ))
    }

    fn locking_script(
        custodian_pubkeys: &[PublicKey],
        custodian_quorum: u8,
    ) -> Result<LockingScript, Self::Error> {
        let secp = get_global_secp();
        let keys = convert_pubkeys_to_x_only_keys(custodian_pubkeys);

        let tree = TaprootTree::<CustodianOnlyTree>::new(secp, &keys, custodian_quorum)?;

        Ok(LockingScript(tree.into_script(secp)))
    }

    fn data_script<'a>(
        &self,
        custodian_quorum: u8,
        destination_chain_id: &'a DestinationChain,
        destination_token_address: &'a DestinationTokenAddress,
        destination_recipient_address: &'a DestinationRecipientAddress,
    ) -> Result<DataScript, Self::Error> {
        let tag_hash = DataScript::compute_tag_hash(self.tag().as_slice())?;
        let service_tag_hash = DataScript::compute_service_tag_hash(self.service_tag().as_slice())?;
        let flags = TaprootTreeType::CustodianOnly as u8;

        let mut data = Vec::<u8>::with_capacity(EMBEDDED_DATA_SCRIPT_SIZE);
        data.extend_from_slice(&tag_hash);
        data.push(self.version());
        data.push(self.network_id());
        data.push(flags);
        data.extend_from_slice(&service_tag_hash);
        data.push(custodian_quorum);
        data.extend_from_slice(destination_chain_id);
        data.extend_from_slice(destination_token_address);
        data.extend_from_slice(destination_recipient_address);

        let data_slice: &[u8; EMBEDDED_DATA_SCRIPT_SIZE] = data
            .as_slice()
            .try_into()
            .map_err(|_| CoreError::CannotConvertOpReturnDataToSlice)?;

        let embedded_data_script = Builder::new()
            .push_opcode(OP_RETURN)
            .push_slice(data_slice)
            .into_script();

        Ok(DataScript(embedded_data_script))
    }

    fn build_unlocking_psbt(
        &self,
        params: &CustodianOnlyUnlockingParams,
    ) -> Result<bitcoin::Psbt, Self::Error> {
        let (total_input_value, total_output_value) = params.validate()?;
        let secp = get_global_secp();

        let x_only_pubkeys = convert_pubkeys_to_x_only_keys(&params.custodian_pubkeys);
        let tree =
            TaprootTree::<CustodianOnlyTree>::new(secp, &x_only_pubkeys, params.custodian_quorum)?;

        let unsigned_tx = self.build_unlocking_transaction(&UnlockingParams {
            total_input_value,
            total_output_value,
            inputs: &params.inputs,
            outputs: &params.outputs,
            tree_type: UnlockingTaprootTreeType::CustodianOnlyBranch,
            script: &tree.clone().into_script(secp),
            rbf: params.rbf,
            fee_rate: params.fee_rate,
            custodian_quorum: params.custodian_quorum,
            session_sequence: params.session_sequence,
            custodian_group_uid: params.custodian_group_uid,
        })?;

        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

        let (branch, keys) = (tree.raw.custodian_only_branch, x_only_pubkeys);

        psbt.inputs = self.prepare_psbt_inputs(&params.inputs, &tree.root, &branch, &keys);

        Ok(psbt)
    }
}
