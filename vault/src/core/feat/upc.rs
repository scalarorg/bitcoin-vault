use bitcoin::{opcodes::all::OP_RETURN, script::Builder, Psbt, PublicKey, XOnlyPublicKey};

use crate::{
    convert_pubkey_to_x_only_key, convert_pubkeys_to_x_only_keys, get_global_secp, CoreError,
    DataScript, DestinationChain, DestinationRecipientAddress, DestinationTokenAddress,
    LockingOutput, LockingScript, TaprootTree, TaprootTreeType, UPCLockingParams, UPCTaprootTree,
    UPCUnlockingParams, UnlockingParams, UnlockingTaprootTreeType, UnlockingType, VaultManager,
    EMBEDDED_DATA_SCRIPT_SIZE, HASH_SIZE, UPC,
};

impl UPC for VaultManager {
    type Error = CoreError;

    fn build_locking_output(
        &self,
        params: &UPCLockingParams,
    ) -> Result<LockingOutput, Self::Error> {
        let locking_script = <Self as UPC>::locking_script(
            &params.user_pub_key,
            &params.protocol_pub_key,
            &params.custodian_pub_keys,
            params.custodian_quorum,
        )?;

        let data_script = <Self as UPC>::data_script(
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
        user_pub_key: &PublicKey,
        protocol_pub_key: &PublicKey,
        custodian_pub_keys: &[PublicKey],
        custodian_quorum: u8,
    ) -> Result<LockingScript, Self::Error> {
        let secp = get_global_secp();
        let (user, protocol, custodians) =
            convert_upc_to_x_only_keys(user_pub_key, protocol_pub_key, custodian_pub_keys);

        let tree =
            TaprootTree::<UPCTaprootTree>::new(secp, user, protocol, custodians, custodian_quorum)?;

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

        let flags = TaprootTreeType::UPCBranch as u8;

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

        Ok(DataScript(
            Builder::new()
                .push_opcode(OP_RETURN)
                .push_slice(data_slice)
                .into_script(),
        ))
    }

    fn build_unlocking_psbt(
        &self,
        params: &UPCUnlockingParams,
    ) -> Result<bitcoin::Psbt, Self::Error> {
        let (total_input_value, total_output_value) = params.validate()?;

        let secp = get_global_secp();

        let (user, protocol, custodians) = convert_upc_to_x_only_keys(
            &params.user_pub_key,
            &params.protocol_pub_key,
            &params.custodian_pub_keys,
        );

        let tree = TaprootTree::<UPCTaprootTree>::new(
            secp,
            user,
            protocol,
            custodians.clone(),
            params.custodian_quorum,
        )?;

        let unsigned_tx = self.build_unlocking_transaction(&UnlockingParams {
            total_input_value,
            total_output_value,
            inputs: &params.inputs,
            outputs: &[params.output.clone()],
            tree_type: UnlockingTaprootTreeType::UPCBranch,
            script: &tree.clone().into_script(secp),
            rbf: params.rbf,
            fee_rate: params.fee_rate,
            custodian_quorum: params.custodian_quorum,
            session_sequence: 0,
            custodian_group_uid: [0u8; HASH_SIZE],
        })?;

        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

        let (branch, keys) = match params.typ {
            UnlockingType::UserProtocol => (&tree.raw.user_protocol_branch, vec![user, protocol]),
            UnlockingType::CustodianProtocol => {
                let mut keys = vec![protocol];
                keys.extend_from_slice(&custodians);
                (&tree.raw.custodian_protocol_branch, keys)
            }
            UnlockingType::CustodianUser => {
                let mut keys = vec![user];
                keys.extend_from_slice(&custodians);
                (&tree.raw.custodian_user_branch, keys)
            }
        };

        psbt.inputs = self.prepare_psbt_inputs(&params.inputs, &tree.root, branch, &keys);

        Ok(psbt)
    }
}

fn convert_upc_to_x_only_keys(
    user_pub_key: &PublicKey,
    protocol_pub_key: &PublicKey,
    custodian_pub_keys: &[PublicKey],
) -> (XOnlyPublicKey, XOnlyPublicKey, Vec<XOnlyPublicKey>) {
    let user_x_only = convert_pubkey_to_x_only_key(user_pub_key);
    let protocol_x_only = convert_pubkey_to_x_only_key(protocol_pub_key);
    let custodian_x_only = convert_pubkeys_to_x_only_keys(custodian_pub_keys);

    (user_x_only, protocol_x_only, custodian_x_only)
}
