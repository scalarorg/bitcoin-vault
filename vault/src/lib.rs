mod parser;
mod staking;
pub use parser::*;
pub use staking::*;
#[cfg(test)]
mod utils;
#[cfg(test)]
mod tests {
    use bitcoin::{
        hashes::Hash, key::Secp256k1, Address, Amount, CompressedPublicKey, Network, OutPoint,
        PrivateKey, PublicKey, ScriptBuf, TxOut, Txid,
    };

    use crate::{
        hex_to_vec, utils::get_env, BuildUserProtocolSpendParams, Parsing, PreviousStakingUTXO,
        StakingManager, Unstaking,
    };

    static TEST_CUSTODIAL_QUORUM: u8 = 1;

    #[test]
    fn test_parse_embedded_data() {
        let tx_hex = "020000000001017161373459156dc2e40548b86a2d0818a8459c409355538278ad05ed46c5c3cd0000000000fdffffff03e4969800000000002251206ed59921fda3e5a9b2490dac5aea47f734432a5d2dbe5883cbb69df4796f882c00000000000000003d6a013504010203040100080000000000aa36a7141f98c06d8734d5a9ff0b53e3294626e62e4d232c14130c4810d57140e1e62967cbf742caeae91b6ece9b94b708000000001600141302a4ea98285baefb2d290de541d069356d88e90247304402205de8b44cceae9cdf6add698051f7ee171607a4e36c7df60d811f2a339263e398022072b873381018d79fd82b07ba8be52012cd9990491c6bc7274f48e5638c439d0d01210369f8edcde3c4e5e5082f7d772170bbd9803b8d4e0c788830c7227bcea8a5653400000000";
        let raw_tx = hex_to_vec!(tx_hex);

        let result = StakingManager::parse_embedded_data(raw_tx).unwrap();
        println!("{:?}", result);
    }

    fn load_build_user_protocol_spend_params() -> BuildUserProtocolSpendParams {
        let env = get_env();
        let secp = &Secp256k1::new();

        let user_privkey = PrivateKey::from_wif(&env.user_private_key).unwrap();
        let user_pub_key = user_privkey.public_key(secp);

        let compressed_pubkey = CompressedPublicKey::from_private_key(secp, &user_privkey).unwrap();

        let user_address = Address::p2wpkh(&compressed_pubkey, Network::Regtest);

        let protocol_privkey = PrivateKey::from_wif(&env.protocol_private_key).unwrap();
        let protocol_pub_key = protocol_privkey.public_key(secp);

        let covenant_pubkeys: Vec<PublicKey> = env
            .covenant_private_keys
            .iter()
            .map(|k| PrivateKey::from_wif(k).unwrap().public_key(secp))
            .collect();

        println!("===== KEYS =====");
        println!("user_pub_key: {:?}", user_pub_key.to_string());
        println!("protocol_pub_key: {:?}", protocol_pub_key.to_string());
        for (i, covenant_pubkey) in covenant_pubkeys.iter().enumerate() {
            println!("covenant_pubkey {}: {:?}", i, covenant_pubkey.to_string());
        }

        println!("===== UTXOS =====");
        println!("utxo_tx_id: {:?}", env.utxo_tx_id);
        println!("utxo_vout: {:?}", env.utxo_vout);
        println!("utxo_amount: {:?}", env.utxo_amount);
        println!("script_pubkey: {:?}", env.script_pubkey);
        println!("===== ---- =====");

        BuildUserProtocolSpendParams {
            input_utxo: PreviousStakingUTXO {
                script_pubkey: ScriptBuf::from_hex(&env.script_pubkey).unwrap(),
                outpoint: OutPoint {
                    txid: {
                        let tx_bytes = hex_to_vec!(env.utxo_tx_id);
                        let txid = Txid::from_slice(&tx_bytes).unwrap();
                        txid
                    },
                    vout: env.utxo_vout,
                },
                amount_in_sats: Amount::from_sat(env.utxo_amount),
            },
            unstaking_output: TxOut {
                value: Amount::from_sat(env.utxo_amount - 1000), // 1000 sats for fees
                script_pubkey: ScriptBuf::from_hex(&env.script_pubkey).unwrap(),
            },
            user_pub_key: user_pub_key,
            protocol_pub_key: protocol_pub_key,
            covenant_pubkeys,
            covenant_quorum: TEST_CUSTODIAL_QUORUM,
            have_only_covenants: false,
            rbf: true,
        }
    }

    // 70736274ff01005d0200000001df0404d9a9baea8b6e71bfb3566517f1765300f6603f158bc530c3323f6dbd340000000000fdffffff01e49698000000000021036fb1229de5f62d82db8c93d4963d9824e6f3f32ed418fe62c2600f3771215207000000000001012be4969800000000002251206ed59921fda3e5a9b2490dac5aea47f734432a5d2dbe5883cbb69df4796f882c010304010000004215c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0261f514bfc5d3d11cbcda229805f301ba525a11f8e977991191defa12e52f7d0452069f8edcde3c4e5e5082f7d772170bbd9803b8d4e0c788830c7227bcea8a56534ad206fb1229de5f62d82db8c93d4963d9824e6f3f32ed418fe62c2600f3771215207acc0211669f8edcde3c4e5e5082f7d772170bbd9803b8d4e0c788830c7227bcea8a5653425019d8a6a2d6991ddcecbaa320f0eb4caa2eae9ffe36e0694787767ccebab7ebe7b0000000021166fb1229de5f62d82db8c93d4963d9824e6f3f32ed418fe62c2600f377121520725019d8a6a2d6991ddcecbaa320f0eb4caa2eae9ffe36e0694787767ccebab7ebe7b0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac00118209edfc38420215722665dd8c9e79bbb99d546096a4062b2e3f0b565a53514d8420000

    #[test]
    fn test_build_user_protocol_spend() {
        let params = load_build_user_protocol_spend_params();

        println!("===== PARAMS =====");
        println!("{:?}", params);
        println!("===== ---- =====");

        let manager = StakingManager::new(vec![1, 2, 3, 4], 1);

        let psbt = manager.build_user_protocol_spend(&params).unwrap();
        println!("{:?}", psbt);

        // Verify PSBT structure
        assert_eq!(psbt.inputs.len(), 1);
        assert_eq!(psbt.outputs.len(), 1);

        let input = &psbt.inputs[0];

        // Verify witness UTXO
        assert!(input.witness_utxo.is_some());
        assert_eq!(
            input.witness_utxo.as_ref().unwrap().value,
            Amount::from_sat(10_000_100)
        );

        // Verify Taproot-specific fields
        assert!(input.tap_internal_key.is_some());
        assert!(input.tap_merkle_root.is_some());
        assert!(!input.tap_scripts.is_empty());
        assert!(!input.tap_key_origins.is_empty());
        assert_eq!(input.tap_key_origins.len(), 2); // Should have both user and protocol keys

        println!("Tx hex: {:?}", psbt.serialize_hex());
    }
}

// 70736274ff0100520200000001df0404d9a9baea8b6e71bfb3566517f1765300f6603f158bc530c3323f6dbd340000000000fdffffff01fc929800000000001600141302a4ea98285baefb2d290de541d069356d88e9000000000001011fe4969800000000001600141302a4ea98285baefb2d290de541d069356d88e9010304010000004215c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0261f514bfc5d3d11cbcda229805f301ba525a11f8e977991191defa12e52f7d0452069f8edcde3c4e5e5082f7d772170bbd9803b8d4e0c788830c7227bcea8a56534ad206fb1229de5f62d82db8c93d4963d9824e6f3f32ed418fe62c2600f3771215207acc0211669f8edcde3c4e5e5082f7d772170bbd9803b8d4e0c788830c7227bcea8a5653425019d8a6a2d6991ddcecbaa320f0eb4caa2eae9ffe36e0694787767ccebab7ebe7b0000000021166fb1229de5f62d82db8c93d4963d9824e6f3f32ed418fe62c2600f377121520725019d8a6a2d6991ddcecbaa320f0eb4caa2eae9ffe36e0694787767ccebab7ebe7b0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac00118209edfc38420215722665dd8c9e79bbb99d546096a4062b2e3f0b565a53514d8420000

// 70736274ff0100520200000001473491f8456f7c77caf22a793c3c92d6c5b79650c6ebea818c18437bc6e3dd210000000000fdffffff01fc929800000000001600141302a4ea98285baefb2d290de541d069356d88e9000000000001012be4969800000000002251206ed59921fda3e5a9b2490dac5aea47f734432a5d2dbe5883cbb69df4796f882c010304010000004215c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0261f514bfc5d3d11cbcda229805f301ba525a11f8e977991191defa12e52f7d0452069f8edcde3c4e5e5082f7d772170bbd9803b8d4e0c788830c7227bcea8a56534ad206fb1229de5f62d82db8c93d4963d9824e6f3f32ed418fe62c2600f3771215207acc0211669f8edcde3c4e5e5082f7d772170bbd9803b8d4e0c788830c7227bcea8a5653425019d8a6a2d6991ddcecbaa320f0eb4caa2eae9ffe36e0694787767ccebab7ebe7b0000000021166fb1229de5f62d82db8c93d4963d9824e6f3f32ed418fe62c2600f377121520725019d8a6a2d6991ddcecbaa320f0eb4caa2eae9ffe36e0694787767ccebab7ebe7b0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac00118209edfc38420215722665dd8c9e79bbb99d546096a4062b2e3f0b565a53514d8420000
