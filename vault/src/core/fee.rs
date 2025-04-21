use bitcoin::{Amount, Transaction};

use super::{
    CoreError, VaultManager, ESTIMATE_ADDITIONAL_P2TR_SCRIPT_PATH_COST, ESTIMATE_SIGNATURE_COST,
    P2TR_BUFFER_SIZE, P2TR_INPUT_SIZE, P2TR_OUTPUT_SIZE,
};

#[derive(Debug)]
pub struct UnlockingFeeParams {
    pub n_inputs: u64,
    pub n_outputs: u64,
    pub quorum: u8,
    pub fee_rate: u64,
}

impl VaultManager {
    pub fn calculate_unlocking_fee(&self, params: UnlockingFeeParams) -> Amount {
        let witness_cost = ESTIMATE_SIGNATURE_COST * params.quorum as u64
            + ESTIMATE_ADDITIONAL_P2TR_SCRIPT_PATH_COST;
        let inputs_cost = (P2TR_INPUT_SIZE + witness_cost) * params.n_inputs;
        let outputs_cost = P2TR_OUTPUT_SIZE * params.n_outputs;
        let fee = (P2TR_BUFFER_SIZE + inputs_cost + outputs_cost) * params.fee_rate;
        bitcoin::Amount::from_sat(fee)
    }

    pub fn distribute_fee(
        &self,
        unsigned_tx: &mut Transaction,
        total_output_value: Amount,
        fee: Amount,
    ) -> Result<(), CoreError> {
        let fee_in_sats = fee.to_sat();
        let total_output_value_in_sats = total_output_value.to_sat();

        for output in unsigned_tx.output.iter_mut() {
            let sats = output.value.to_sat();
            if sats == 0 {
                continue;
            }

            let proportion = sats as f64 / total_output_value_in_sats as f64;
            let fee_share = (fee_in_sats as f64 * proportion).ceil() as u64; // Round up to the nearest integer
            let fee_share_in_sat = Amount::from_sat(fee_share);

            if let Some(new_value) = output.value.checked_sub(fee_share_in_sat) {
                output.value = new_value;
            } else {
                return Err(CoreError::InsufficientFunds);
            }
        }

        // Check if any output has a negative value after fee adjustment
        if unsigned_tx.output.iter().any(|o| o.value < Amount::ZERO) {
            return Err(CoreError::InsufficientFunds);
        }

        Ok(())
    }
}

// let base_size = 11 + 34 * params.n_outputs; // Base tx size
//     let witness_size_per_sig = 72; // Estimated per-signature witness size
//     let max_signatures = 5; // Assume worst case

//     let base_input_size = 68; // Base input size without witness
//     let witness_overhead = 2; // Taproot witness overhead
//     let total_witness_size = witness_overhead + (witness_size_per_sig * max_signatures);

//     let total_input_size = params.n_inputs * (base_input_size + total_witness_size);

//     (base_size + total_input_size) * params.fee_rate
