use bitcoin::{consensus::Decodable, Amount, OutPoint, ScriptBuf, Txid};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct UTXO {
    pub outpoint: OutPoint, // txid, vout
    pub amount_in_sats: Amount,
}

#[derive(Debug, Validate, Clone)]
pub struct PreviousStakingUTXO {
    pub outpoint: OutPoint, // txid, vout
    pub amount_in_sats: Amount,
    pub script_pubkey: ScriptBuf,
}

impl TryFrom<&[u8]> for PreviousStakingUTXO {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut txid = [0u8; 32];
        txid.copy_from_slice(&value[0..32]);
        let txid = Txid::consensus_decode(&mut txid.as_slice())
            .map_err(|_| anyhow::anyhow!("Invalid txid"))?;
        let vout = u32::from_be_bytes(value[32..36].try_into().unwrap());
        let amount = u64::from_be_bytes(value[36..44].try_into().unwrap());
        Ok(PreviousStakingUTXO {
            outpoint: OutPoint::new(txid, vout),
            amount_in_sats: Amount::from_sat(amount),
            script_pubkey: ScriptBuf::from_bytes(value[44..].to_vec()),
        })
    }
}
#[derive(Debug, Validate, Clone)]
pub struct UnstakingOutput {
    pub locking_script: ScriptBuf,
    pub amount_in_sats: Amount,
}

impl TryFrom<&[u8]> for UnstakingOutput {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let amount = u64::from_be_bytes(value[0..8].try_into().unwrap());
        Ok(UnstakingOutput {
            locking_script: ScriptBuf::from_bytes(value[8..].to_vec()),
            amount_in_sats: Amount::from_sat(amount),
        })
    }
}
