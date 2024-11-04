use bitcoin::{hashes::Hash, key::constants::PUBLIC_KEY_SIZE, Psbt, PublicKey, ScriptBuf, Txid};

use super::errors::VaultABIError;

pub struct Decoder;

/// ABI Decoder for input data
impl Decoder {
    pub fn decode_txid(input: &[u8]) -> Result<Txid, VaultABIError> {
        Txid::from_slice(input).map_err(|err| VaultABIError::DecodingError(format!("{:?}", err)))
    }

    pub fn decode_script_pubkey(input: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(input.to_vec())
    }
    pub fn decode_33bytes_pubkey(input: &[u8]) -> Result<PublicKey, VaultABIError> {
        PublicKey::from_slice(input).map_err(|e| VaultABIError::DecodingError(format!("{}", e)))
    }
    pub fn decode_33bytes_pubkey_list(input: &[u8]) -> Result<Vec<PublicKey>, VaultABIError> {
        let key_len = PUBLIC_KEY_SIZE;
        if input.is_empty() || input.len() % key_len != 0 {
            return Err(VaultABIError::InvalidInputData);
        }
        let number_of_pubkeys = input.len() / key_len;
        let mut pubkeys = Vec::with_capacity(number_of_pubkeys);
        for i in 0..number_of_pubkeys {
            let offset = i * key_len;
            match Self::decode_33bytes_pubkey(&input[offset..(offset + key_len)]) {
                Ok(pubkey) => pubkeys.push(pubkey),
                Err(e) => return Err(VaultABIError::DecodingError(format!("{}", e))),
            }
        }
        Ok(pubkeys)
    }

    pub fn decode_psbt(input: &[u8]) -> Result<Psbt, VaultABIError> {
        Psbt::deserialize(input).map_err(|e| VaultABIError::DecodingError(format!("{}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::Decoder;
    use bitcoin::{
        bip32::{ChildNumber, Xpriv, Xpub},
        key::Secp256k1,
        secp256k1::All,
        Address, CompressedPublicKey, KnownHrp, Network,
    };
    fn generate_secp256k1_keys(secp: &Secp256k1<All>) -> (Xpriv, Xpub) {
        let random_bytes = "000102030405060708090a0b0c0d0e0f".as_bytes();
        let sk = Xpriv::new_master(Network::Bitcoin, &random_bytes).unwrap();
        let pk = Xpub::from_priv(secp, &sk);
        (sk, pk)
    }
    #[test]
    fn test_pubkey() {
        let pubkey_bytes = [
            3, 100, 47, 100, 188, 43, 50, 91, 97, 225, 102, 38, 190, 80, 24, 76, 106, 168, 247, 85,
            33, 203, 61, 205, 31, 132, 175, 170, 8, 158, 47, 222, 63,
        ];
        // let pubkey_bytes = [
        //     2, 226, 10, 49, 146, 220, 97, 234, 184, 27, 85, 59, 169, 2, 45, 11, 0, 207, 125, 63,
        //     15, 101, 235, 214, 38, 95, 77, 210, 211, 50, 77, 132, 165,
        // ];
        let decoded_pubkey = Decoder::decode_33bytes_pubkey(&pubkey_bytes).unwrap();
        println!("Decoded address: {}", decoded_pubkey);
    }
}
