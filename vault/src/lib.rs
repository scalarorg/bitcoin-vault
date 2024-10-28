mod staking;

pub use staking::*;

#[cfg(test)]
mod utils;
#[cfg(test)]
mod tests {
    // use std::str::FromStr;

    // use bitcoin::{key::Secp256k1, Amount, OutPoint, PrivateKey, PublicKey, ScriptBuf, Txid};

    // use super::*;
    // use crate::utils::*;

    // static STAKING_AMOUNT: u64 = 1000;
    // static COVENANT_QUORUM: u8 = 1;
    // static RBF: bool = true;
    // static FEE_RATE: u64 = 1;
    // static HAVE_ONLY_COVENANTS: bool = false;
    // static DESTINATION_CHAIN_ID: [u8; 8] = [3; 8];
    // static DESTINATION_CONTRACT_ADDRESS: [u8; 20] = [4; 20];
    // static DESTINATION_RECIPIENT_ADDRESS: [u8; 20] = [5; 20];

    // fn load_params() -> CreateStakingParams {
    //     let env = get_env();
    //     let secp = &Secp256k1::new();

    //     let user_privkey = PrivateKey::from_wif(&env.user_private_key).unwrap();
    //     let user_pub_key = user_privkey.public_key(secp);

    //     let protocol_privkey = PrivateKey::from_wif(&env.protocol_private_key).unwrap();
    //     let protocol_pub_key = protocol_privkey.public_key(secp);

    //     let covenant_pubkeys: Vec<PublicKey> = env
    //         .covenant_private_keys
    //         .iter()
    //         .map(|k| PrivateKey::from_wif(k).unwrap().public_key(secp))
    //         .collect();

    //     println!("===== KEYS =====");
    //     println!("user_pub_key: {:?}", user_pub_key.to_string());
    //     println!("protocol_pub_key: {:?}", protocol_pub_key.to_string());
    //     for (i, covenant_pubkey) in covenant_pubkeys.iter().enumerate() {
    //         println!("covenant_pubkey {}: {:?}", i, covenant_pubkey.to_string());
    //     }

    //     println!("===== UTXOS =====");
    //     println!("utxo_tx_id: {:?}", env.utxo_tx_id);
    //     println!("utxo_vout: {:?}", env.utxo_vout);
    //     println!("utxo_amount: {:?}", env.utxo_amount);
    //     println!("script_pubkey: {:?}", env.script_pubkey);
    //     println!("===== ---- =====");

    //     CreateStakingParams {
    //         user_pub_key,
    //         protocol_pub_key,
    //         covenant_pubkeys,
    //         covenant_quorum: COVENANT_QUORUM,
    //         staking_amount: STAKING_AMOUNT,
    //         utxos: vec![UTXO {
    //             outpoint: OutPoint {
    //                 txid: Txid::from_str(&env.utxo_tx_id).unwrap(),
    //                 vout: env.utxo_vout,
    //             },
    //             amount_in_sats: Amount::from_sat(env.utxo_amount),
    //         }],
    //         script_pubkey: ScriptBuf::from_hex(&env.script_pubkey).unwrap(),
    //         rbf: RBF,
    //         fee_rate: FEE_RATE,
    //         have_only_covenants: HAVE_ONLY_COVENANTS,
    //         destination_chain_id: DESTINATION_CHAIN_ID,
    //         destination_contract_address: DESTINATION_CONTRACT_ADDRESS,
    //         destination_recipient_address: DESTINATION_RECIPIENT_ADDRESS,
    //     }
    // }

    // #[test]
    // fn test_create_unsigned_psbt() {
    //     let params = load_params();

    //     let staking_manager = StakingManager::new(vec![7, 7, 7, 7], 1);

    //     let unsigned_psbt = staking_manager.create(&params).unwrap();

    //     println!("Unsigned PSBT: {:?}", unsigned_psbt);

    //     let output = unsigned_psbt.serialize();
    //     println!("Serialized PSBT: {:?}", output);
    //     let hex = unsigned_psbt.serialize_hex();
    //     println!("Serialized PSBT hex: {:?}", hex);
    // }
}
