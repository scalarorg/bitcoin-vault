#[cfg(test)]
mod test_script {

    use std::str::FromStr;

    use bitcoin::{hex::DisplayHex, PrivateKey, PublicKey};
    use lazy_static::lazy_static;
    use vault::{get_global_secp, TestSuite, VaultManager, UPC};

    lazy_static! {
        static ref TEST_SUITE: TestSuite = TestSuite::new_with_loaded_env("PEPE");
    }

    #[test]
    fn test_upc() {
        let priv_key = std::env::var("USER_PRIVKEY").unwrap();
        let secp = get_global_secp();

        for priv_key in TEST_SUITE.env().custodian_private_keys.iter() {
            let priv_key = PrivateKey::from_wif(&priv_key).unwrap();
            // println!("priv_key: {:?}", priv_key.to_bytes().to_lower_hex_string());
            println!(
                "pub_key: {:?}",
                priv_key.public_key(secp).to_bytes().to_lower_hex_string()
            );
        }

        let user_privkey = PrivateKey::from_wif(&priv_key).unwrap();
        let user_pubkey = user_privkey.public_key(secp);
        println!(
            "user_pubkey: {:?}",
            user_pubkey.to_bytes().to_lower_hex_string()
        );

        println!(
            "protocol_pubkey: {:?}",
            TEST_SUITE
                .protocol_pubkey()
                .to_bytes()
                .to_lower_hex_string()
        );

        let protocol_pubkey = PublicKey::from_str(
            "03a9a3ec96a1051310a80ea9eaaed56cc68b5d7dbe3caa6f145014da88b897e9fa",
        )
        .unwrap();

        let script = <VaultManager as UPC>::locking_script(
            &user_pubkey,
            &protocol_pubkey,
            &TEST_SUITE.custodian_pubkeys(),
            TEST_SUITE.env().custodian_quorum,
        )
        .unwrap();

        println!(
            "script: {:?}",
            script.into_script().as_bytes().to_lower_hex_string()
        );
    }
}
