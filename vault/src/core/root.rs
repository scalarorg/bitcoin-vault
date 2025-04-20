use bitcoin::{
    opcodes::all::OP_RETURN, script::Builder, Amount, Psbt, PublicKey, ScriptBuf, TxOut,
    XOnlyPublicKey,
};

use super::{
    convert_pubkey_to_x_only_key, get_global_secp, CoreError, CustodianOnlyLockingScriptParams,
    CustodianOnlyStakingParams, DataScript, DataScriptParams, DestinationChain,
    DestinationRecipientAddress, DestinationTokenAddress, LockingOutput, LockingScript,
    TaprootTree, TaprootTreeType, UPCLockingParams, UPCLockingScriptParams, UPCTaprootTree,
    UnlockingType, VaultManager, XOnlyKeys, DEST_CHAIN_SIZE, DEST_RECIPIENT_ADDRESS_SIZE,
    DEST_TOKEN_ADDRESS_SIZE, EMBEDDED_DATA_SCRIPT_SIZE, UPC,
};

impl UPC for VaultManager {
    type Error = CoreError;

    fn build_locking_output(
        &self,
        params: &UPCLockingParams,
    ) -> Result<LockingOutput, Self::Error> {
        let locking_script = self.locking_script(
            &params.user_pub_key,
            &params.protocol_pub_key,
            &params.custodian_pub_keys,
            params.custodian_quorum,
        )?;

        let data_script = self.data_script(
            params.custodian_quorum,
            &params.destination_chain,
            &params.destination_token_address,
            &params.destination_recipient_address,
        )?;

        Ok(LockingOutput::new(
            params.staking_amount,
            locking_script,
            data_script,
        ))
    }

    fn locking_script(
        &self,
        user_pub_key: &PublicKey,
        protocol_pub_key: &PublicKey,
        custodian_pub_keys: &[PublicKey],
        custodian_quorum: u8,
    ) -> Result<LockingScript, Self::Error> {
        let secp = get_global_secp();
        let x_only_keys =
            convert_upc_to_x_only_keys(&user_pub_key, &protocol_pub_key, &custodian_pub_keys);

        let tree = TaprootTree::<UPCTaprootTree>::new(
            &secp,
            x_only_keys.user,
            x_only_keys.protocol,
            x_only_keys.custodians,
            custodian_quorum,
        )?;

        Ok(LockingScript(tree.into_script(&secp)))
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
        params: &super::UPCUnlockingParams,
        unstaking_type: super::UnlockingType,
    ) -> Result<bitcoin::Psbt, Self::Error> {
        let (total_input_value, total_output_value) = params.validate()?;

        let secp = get_global_secp();

        let x_only_keys = convert_upc_to_x_only_keys(
            &params.user_pub_key,
            &params.protocol_pub_key,
            &params.custodian_pub_keys,
        );

        let x_only_keys = convert_upc_to_x_only_keys(
            &params.user_pub_key,
            &params.protocol_pub_key,
            &params.custodian_pub_keys,
        );

        let tree = TaprootTree::<UPCTaprootTree>::new(
            &secp,
            x_only_keys.user,
            x_only_keys.protocol,
            x_only_keys.custodians,
            params.custodian_quorum,
        )?;

        let upc_script = tree.clone().into_script(secp);

        let unsigned_tx = self.build_unstaking_transaction(
            total_input_value,
            total_output_value,
            &params.inputs,
            &[params.unstaking_output.clone()],
            UnstakingTaprootTreeType::UPCBranch,
            &upc_script,
            params.rbf,
            params.fee_rate,
            params.custodian_quorum,
            0,
            [0u8; HASH_SIZE],
        )?;

        let mut psbt =
            Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

        let (branch, keys) = get_branch_and_keys_for_type(&x_only_keys, unstaking_type, &tree);

        psbt.inputs = self.prepare_psbt_inputs(&params.inputs, &tree.root, branch, &keys);

        Ok(psbt)
    }
}

pub fn convert_upc_to_x_only_keys(
    user_pub_key: &PublicKey,
    protocol_pub_key: &PublicKey,
    custodian_pub_keys: &[PublicKey],
) -> XOnlyKeys {
    let user_x_only = convert_pubkey_to_x_only_key(user_pub_key);
    let protocol_x_only = convert_pubkey_to_x_only_key(protocol_pub_key);
    let custodian_x_only = custodian_pub_keys
        .iter()
        .map(convert_pubkey_to_x_only_key)
        .collect();

    XOnlyKeys {
        user: user_x_only,
        protocol: protocol_x_only,
        custodians: custodian_x_only,
    }
}

pub fn get_branch_and_keys_for_type<'a>(
    x_only_keys: &XOnlyKeys,
    unstaking_type: UnlockingType,
    tree: &'a UPCTaprootTree,
) -> (&'a ScriptBuf, Vec<XOnlyPublicKey>) {
    match unstaking_type {
        UnlockingType::UserProtocol => (
            &tree.user_protocol_branch,
            vec![x_only_keys.user, x_only_keys.protocol],
        ),
        UnlockingType::CustodianProtocol => {
            let mut keys = vec![x_only_keys.protocol];
            keys.extend_from_slice(&x_only_keys.custodians);
            (&tree.protocol_custodian_branch, keys)
        }
        UnlockingType::CustodianUser => {
            let mut keys = vec![x_only_keys.user];
            keys.extend_from_slice(&x_only_keys.custodians);
            (&tree.user_custodian_branch, keys)
        }
    }
}

// fn build_custodian_only(
//     &self,
//     params: &CustodianOnlyStakingParams,
// ) -> Result<LockingOutput, Self::Error> {
//     let secp = get_global_secp();
//     // TODO: validate params
//     let custodians_x_only: Vec<XOnlyPublicKey> = params
//         .custodian_pub_keys
//         .iter()
//         .map(|pk| XOnlyPublicKey::from(*pk))
//         .collect();

//     let locking_script = LockingScript::new_custodian_only(
//         secp,
//         &CustodianOnlyLockingScriptParams {
//             custodian_pub_keys: &custodians_x_only,
//             custodian_quorum: params.custodian_quorum,
//         },
//     )?;

//     let embedded_data_script = DataScript::new_custodian_only(&CustodianOnlyDataParams {
//         tag: self.tag(),
//         version: self.version(),
//         network_id: self.network_id(),
//         service_tag: self.service_tag(),
//         custodian_quorum: params.custodian_quorum,
//         destination_chain_id: &params.destination_chain,
//         destination_token_address: &params.destination_token_address,
//         destination_recipient_address: &params.destination_recipient_address,
//     })?;

//     Ok(LockingOutput::new(
//         params.staking_amount,
//         locking_script,
//         embedded_data_script,
//     ))
// }
