use bitcoin::Amount;

use super::VaultManager;

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
        println!("num_inputs: {}", num_inputs);
        println!("num_outputs: {}", num_outputs);
        println!("fee_rate: {}", fee_rate);
        bitcoin::Amount::from_sat(VaultManager::get_fee(num_inputs, num_outputs, fee_rate))
    }
}
