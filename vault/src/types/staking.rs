use bitcoin::Txid;

use super::VaultTransaction;

pub struct StakingData {
    pub tx_id: Txid,
    pub vout: u32,
    pub amount: u64,
}
impl From<VaultTransaction> for StakingData {
    fn from(value: VaultTransaction) -> Self {
        StakingData {
            tx_id: value.tx.compute_txid(),
            vout: 0,
            amount: 0,
        }
    }
}
