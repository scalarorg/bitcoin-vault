use bitcoin::{
    key::constants::{PUBLIC_KEY_SIZE, SCHNORR_PUBLIC_KEY_SIZE},
    Address, Network, PublicKey, XOnlyPublicKey,
};

use super::errors::VaultABIError;

pub struct Decoder;

/// ABI Decoder for input data
impl Decoder {
    pub fn decode_address(input: &[u8]) -> Result<Address, VaultABIError> {
        let str = std::str::from_utf8(input)
            .map_err(|err| VaultABIError::DecodingError(format!("{}", err)))?;
        str.parse::<Address<_>>()
            .expect("a valid address")
            .require_network(Network::Bitcoin)
            .map_err(|e| VaultABIError::DecodingError(format!("{}", e)))
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
    pub fn decode_xonly_pubkey(input: &[u8]) -> Result<XOnlyPublicKey, VaultABIError> {
        XOnlyPublicKey::from_slice(input)
            .map_err(|e| VaultABIError::DecodingError(format!("{}", e)))
    }

    pub fn decode_xonly_pubkey_list(input: &[u8]) -> Result<Vec<XOnlyPublicKey>, VaultABIError> {
        let key_len = SCHNORR_PUBLIC_KEY_SIZE;
        if input.is_empty() || input.len() % key_len != 0 {
            return Err(VaultABIError::InvalidInputData);
        }
        let number_of_pubkeys = input.len() / key_len;
        let mut pubkeys = Vec::with_capacity(number_of_pubkeys);
        for i in 0..number_of_pubkeys {
            let offset = i * key_len;
            match Self::decode_xonly_pubkey(&input[offset..(offset + key_len)]) {
                Ok(pubkey) => pubkeys.push(pubkey),
                Err(e) => return Err(VaultABIError::DecodingError(format!("{}", e))),
            }
        }
        Ok(pubkeys)
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
    #[test]
    fn test_decode_address() {
        let secp = Secp256k1::new();
        let zero = ChildNumber::from_normal_idx(0).unwrap();
        let (sk, pk) = generate_secp256k1_keys(&secp);
        let public_key = pk.derive_pub(&secp, &[zero, zero]).unwrap().public_key;
        let address = Address::p2wpkh(&CompressedPublicKey(public_key), KnownHrp::Mainnet);
        let address_str = address.to_string();
        let address_bytes = address_str.as_bytes();
        println!(
            "Address as bytes: {:?} with length {}",
            address_bytes,
            address_bytes.len()
        );
        let decoded_address = Decoder::decode_address(address_str.as_bytes()).unwrap();
        let decoded_address_str = decoded_address.to_string();
        println!("{}", decoded_address_str);
        assert_eq!(address_str, decoded_address_str);
    }
    #[test]
    fn test_decode_public_key() {
        let secp = Secp256k1::new();
        let zero = ChildNumber::from_normal_idx(0).unwrap();
        let (sk, pk) = generate_secp256k1_keys(&secp);
        let xonly_pk = pk.derive_pub(&secp, &[zero, zero]).unwrap().to_x_only_pub();
        let pk_bytes = xonly_pk.serialize();
        println!(
            "Xonly pubkey as bytes: {:?} with length {}",
            pk_bytes,
            pk_bytes.len()
        );
        let decoded_xonly_pk = Decoder::decode_xonly_pubkey(&pk_bytes).unwrap();
        let decoded_xonly_pk_bytes = decoded_xonly_pk.serialize();
        println!("Decoded xonly pubkey: {:?}", decoded_xonly_pk_bytes);
        assert_eq!(xonly_pk, decoded_xonly_pk);
    }
}
