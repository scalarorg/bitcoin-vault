use bitcoin::{
    absolute, transaction, Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
};

use super::CoreError;

pub struct TransactionBuilder {
    version: transaction::Version,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    rbf: bool,
}

impl TransactionBuilder {
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

    pub fn add_outputs(&mut self, outputs: &[TxOut]) {
        for output in outputs {
            self.add_output(output.value, output.script_pubkey.clone());
        }
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

// Helper trait for Amount calculations
pub trait AmountExt {
    fn checked_sub_fee(&self, fee: Amount) -> Result<Amount, CoreError>;
}

impl AmountExt for Amount {
    fn checked_sub_fee(&self, fee: Amount) -> Result<Amount, CoreError> {
        self.checked_sub(fee).ok_or(CoreError::InsufficientFunds)
    }
}
