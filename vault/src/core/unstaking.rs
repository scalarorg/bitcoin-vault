use std::collections::BTreeMap;

use crate::core::fee::UnstakingFeeParams;

use super::{
    get_global_secp, manager, CoreError, CustodianOnlyUnstakingParams, DataScript, TaprootTree,
    UPCUnlockingParams, UnstakingDataScriptParams, UnstakingOutput, UnstakingTaprootTreeType,
    VaultManager, XOnlyKeys, HASH_SIZE,
};

use super::PreviousStakingUTXO;
use bitcoin::bip32::{DerivationPath, Fingerprint};
use bitcoin::psbt::{Input, PsbtSighashType};
use bitcoin::taproot::{LeafVersion, TaprootSpendInfo};
use bitcoin::{
    absolute, transaction, Amount, OutPoint, Psbt, PublicKey, ScriptBuf, Sequence, TapLeafHash,
    TapSighashType, Transaction, TxIn, TxOut, Witness, XOnlyPublicKey,
};

#[derive(Debug, PartialEq)]
pub enum UnlockingType {
    UserProtocol,
    CustodianProtocol,
    CustodianUser,
}

// impl Unstaking for VaultManager {
//     type Error = CoreError;

//     fn build_custodian_only(
//         &self,
//         params: &CustodianOnlyUnstakingParams,
//     ) -> Result<Psbt, Self::Error> {
//         let (total_input_value, total_output_value) = params.validate()?;
//         let secp = get_global_secp();

//         let x_only_pub_keys = self.convert_to_x_only_keys(&params.custodian_pub_keys);
//         let tree =
//             TaprootTree::new_custodian_only(secp, &x_only_pub_keys, params.custodian_quorum)?;
//         let custodian_only_script = tree.clone().into_script(secp);

//         let unsigned_tx = self.build_unstaking_transaction(
//             total_input_value,
//             total_output_value,
//             &params.inputs,
//             &params.unstaking_outputs,
//             UnstakingTaprootTreeType::CustodianOnly,
//             &custodian_only_script,
//             params.rbf,
//             params.fee_rate,
//             params.custodian_quorum,
//             params.session_sequence,
//             params.custodian_group_uid,
//         )?;

//         let mut psbt =
//             Psbt::from_unsigned_tx(unsigned_tx).map_err(|_| CoreError::FailedToCreatePSBT)?;

//         let (branch, keys) = (tree.only_custodian_tree.as_ref().unwrap(), x_only_pub_keys);

//         psbt.inputs = self.prepare_psbt_inputs(&params.inputs, &tree.root, branch, &keys);

//         Ok(psbt)
//     }
// }


pub struct UnstakingTransactionBuilder {
    version: transaction::Version,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    rbf: bool,
}

impl UnstakingTransactionBuilder {
    pub fn new(rbf: bool) -> Self {
        Self {
            version: transaction::Version::TWO,
            inputs: Vec::new(),
            outputs: Vec::new(),
            rbf,
        }
    }

    pub fn add_input(&mut self, outpoint: OutPoint) {
        self.inputs.push(TxIn {
            previous_output: outpoint,
            script_sig: ScriptBuf::default(),
            sequence: match self.rbf {
                true => Sequence::ENABLE_RBF_NO_LOCKTIME,
                false => Sequence::MAX,
            },
            witness: Witness::default(),
        });
    }

    pub fn add_input_with_sequence(&mut self, outpoint: OutPoint, sequence: Sequence) {
        self.inputs.push(TxIn {
            previous_output: outpoint,
            script_sig: ScriptBuf::default(),
            sequence,
            witness: Witness::default(),
        });
    }

    pub fn add_output(&mut self, value: Amount, script_pubkey: ScriptBuf) {
        self.outputs.push(TxOut {
            value,
            script_pubkey,
        });
    }

    pub fn add_raw_output(&mut self, output: TxOut) {
        self.outputs.push(output);
    }

    pub fn build(self) -> Transaction {
        Transaction {
            version: self.version,
            lock_time: absolute::LockTime::ZERO,
            input: self.inputs,
            output: self.outputs,
        }
    }
}

// struct UnstakingKeys;

// impl UnstakingKeys {
   
// }

// Helper trait for Amount calculations
pub trait AmountExt {
    fn checked_sub_fee(&self, fee: Amount) -> Result<Amount, CoreError>;
}

impl AmountExt for Amount {
    fn checked_sub_fee(&self, fee: Amount) -> Result<Amount, CoreError> {
        self.checked_sub(fee).ok_or(CoreError::InsufficientFunds)
    }
}
