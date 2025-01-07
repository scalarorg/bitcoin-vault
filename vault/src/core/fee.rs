use bitcoin::{Amount, Transaction};

use super::{CoreError, VaultManager};

impl VaultManager {
    pub fn get_fee(n_inputs: u64, n_outputs: u64, fee_rate: u64) -> u64 {
        (11 + (68 + 112) * n_inputs + 34 * n_outputs) * fee_rate
    }

    pub fn calculate_transaction_fee(
        &self,
        num_inputs: u64,
        num_outputs: u64,
        fee_rate: u64,
    ) -> Amount {
        bitcoin::Amount::from_sat(VaultManager::get_fee(num_inputs, num_outputs, fee_rate))
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
