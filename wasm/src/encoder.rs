use bitcoin::{Amount, TxOut};

pub struct Encoder;
impl Encoder {
    pub fn encode_tx_out(buffer: &mut Vec<u8>, tx_out: &TxOut) {
        //Put 2 bytes for the length of the script_pubkey
        let length = (tx_out.script_pubkey.len() + Amount::SIZE) as u16;
        buffer.extend_from_slice(&length.to_be_bytes());
        buffer.extend_from_slice(&tx_out.value.to_sat().to_be_bytes());
        buffer.extend_from_slice(tx_out.script_pubkey.as_bytes());
    }
    pub fn encode_tx_outs(buffer: &mut Vec<u8>, tx_outs: &[TxOut]) {
        for tx_out in tx_outs {
            Self::encode_tx_out(buffer, tx_out);
        }
    }
    pub fn serialize_tx_outs(tx_outs: &[TxOut]) -> Vec<u8> {
        let total_size = tx_outs
            .iter()
            .map(|tx_out| 2 + tx_out.script_pubkey.len() + Amount::SIZE)
            .reduce(|acc, size| acc + size)
            .unwrap_or(0);
        let mut buffer = Vec::with_capacity(total_size);
        for tx_out in tx_outs {
            Self::encode_tx_out(&mut buffer, tx_out);
        }
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::Encoder;
    use bitcoin::{Amount, ScriptBuf, TxOut};
    #[test]
    fn test_encode_tx_out() {
        let amount = 10_000_100_000u64;
        let amount_bytes = amount.to_be_bytes();
        println!("Amount bytes: {:?}", &amount.to_be_bytes());
        let recover_amount = u64::from_be_bytes(amount_bytes);
        assert_eq!(amount, recover_amount);
        let script_buf = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 15, 17, 18, 19, 20,
        ];
        let mut expected = Vec::with_capacity(script_buf.len() + 10);
        expected.push(0);
        expected.push(script_buf.len() as u8 + 8);
        expected.extend_from_slice(&amount_bytes);
        expected.extend_from_slice(&script_buf);

        let value = Amount::from_sat(amount);
        println!("Amount: {}, Satoshis: {}", amount, value.to_sat());
        let tx_out = TxOut {
            value,
            script_pubkey: ScriptBuf::from_bytes(script_buf),
        };

        let mut buffer = Vec::new();
        Encoder::encode_tx_out(&mut buffer, &tx_out);
        println!("Buffer: {:?} with length {}", buffer, buffer.len());
        println!("Expected: {:?} with length {}", expected, expected.len());
        assert_eq!(buffer, expected);
    }
}
