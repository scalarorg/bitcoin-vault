use crate::{
    CustodianOnlyStakingParams, CustodianOnlyUnstakingParams, FeeParams, PreviousStakingUTXO,
    Signing, Staking, TaprootTreeType, UPCStakingParams, UPCUnstakingParams, Unstaking,
    UnstakingOutput, UnstakingType, VaultManager,
};
use bitcoin::bip32::DerivationPath;
use bitcoin::hex::DisplayHex;
use bitcoin::key::rand;
use bitcoin::psbt::Input;
use bitcoin::{
    absolute, key::Secp256k1, transaction, NetworkKind, PrivateKey, Psbt, PublicKey, ScriptBuf,
    Sequence, Transaction, TxIn, TxOut, Witness,
};
use bitcoin::{AddressType, Amount, OutPoint};
use bitcoincore_rpc::json::GetTransactionResult;

use std::collections::BTreeMap;

use bitcoin::secp256k1::rand::prelude::SliceRandom;
use bitcoincore_rpc::Client;
use bitcoincore_rpc::RpcApi;

use crate::helper::{create_rpc, get_approvable_utxos, get_network_id_from_str, key_from_wif};

use super::helper::{get_fee_rate, hex_to_vec};
use super::{Env, SuiteAccount};

#[derive(Debug, Clone)]
pub enum TestEnv {
    Regtest,
    Testnet4,
    Custom(String),
}

const VOUT: usize = 1;

impl TestEnv {
    pub fn from_env() -> Self {
        match std::env::var("TEST_ENV").as_deref() {
            Ok("regtest") => TestEnv::Regtest,
            Ok("testnet4") => TestEnv::Testnet4,
            Ok(custom_env) => TestEnv::Custom(custom_env.to_string()),
            Err(_) => {
                println!("TEST_ENV is not set, using .env file");
                TestEnv::Custom(".env".to_string())
            }
        }
    }
}

#[derive(Debug)]
pub struct TestSuite {
    manager: VaultManager,
    env: Env,
    rpc: Client,
    protocol_pair: (PrivateKey, PublicKey),
    custodian_pairs: BTreeMap<PublicKey, (PrivateKey, PublicKey)>,
    network_id: NetworkKind,
    env_path: String,
}

impl TestSuite {}

impl TestSuite {
    pub fn new() -> Self {
        let env = TestEnv::from_env();
        println!("\n=================================================================");
        println!(
            "                     RUNNING TEST ON {:?}                     ",
            env
        );
        println!("=================================================================\n");

        let path = match env {
            TestEnv::Regtest => ".env.test.regtest".to_string(),
            TestEnv::Testnet4 => ".env.test.testnet4".to_string(),
            TestEnv::Custom(custom_env) => custom_env,
        };

        let env = Env::new(Some(&path)).unwrap();

        let cloned_env = env.clone();

        let secp = Secp256k1::new();

        let rpc = create_rpc(
            &env.btc_node_address,
            &env.btc_node_user,
            &env.btc_node_password,
            &env.btc_node_wallet,
        );

        let mut custodian_pairs: BTreeMap<PublicKey, (PrivateKey, PublicKey)> = BTreeMap::new();

        for (_, s) in env.custodian_private_keys.iter().enumerate() {
            let (privkey, pubkey) = key_from_wif(s, &secp);
            custodian_pairs.insert(pubkey, (privkey, pubkey));
        }

        let network_id = get_network_id_from_str(&env.network);

        let manager = VaultManager::new(
            env.tag.as_bytes().to_vec(),
            env.service_tag.as_bytes().to_vec(),
            env.version,
            network_id as u8,
        );

        let protocol_privkey = env.protocol_private_key;

        Self {
            rpc,
            env: cloned_env,
            protocol_pair: key_from_wif(&protocol_privkey, &secp),
            custodian_pairs,
            manager,
            network_id,
            env_path: path,
        }
    }

    pub fn prepare_staking_tx(
        &self,
        amount: u64,
        taproot_tree_type: TaprootTreeType,
        account: SuiteAccount,
    ) -> Transaction {
        let destination_chain = self.hex_to_destination(&self.env.destination_chain);
        let destination_token_address =
            self.hex_to_destination(&self.env.destination_token_address);
        let destination_recipient_address =
            self.hex_to_destination(&self.env.destination_recipient_address);

        let outputs: Vec<TxOut> = match taproot_tree_type {
            TaprootTreeType::OnlyKeys => {
                panic!("not implemented");
            }
            TaprootTreeType::CustodianOnly => <VaultManager as Staking>::build_custodian_only(
                &self.manager,
                &CustodianOnlyStakingParams {
                    custodian_pub_keys: self.custodian_pubkeys(),
                    custodian_quorum: self.env.custodian_quorum,
                    staking_amount: amount,
                    destination_chain,
                    destination_token_address,
                    destination_recipient_address,
                },
            )
            .unwrap()
            .into_tx_outs(),
            TaprootTreeType::UPCBranch => <VaultManager as Staking>::build_upc(
                &self.manager,
                &UPCStakingParams {
                    user_pub_key: account.public_key(),
                    protocol_pub_key: self.protocol_pubkey(),
                    custodian_pub_keys: self.custodian_pubkeys(),
                    custodian_quorum: self.env.custodian_quorum,
                    staking_amount: amount,
                    destination_chain: self.hex_to_destination(&self.env.destination_chain),
                    destination_token_address: self
                        .hex_to_destination(&self.env.destination_token_address),
                    destination_recipient_address: self
                        .hex_to_destination(&self.env.destination_recipient_address),
                },
            )
            .unwrap()
            .into_tx_outs(),
        };

        let utxo = get_approvable_utxos(&self.rpc, &account.address(), amount);

        let mut unsigned_tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: OutPoint::new(utxo.txid, utxo.vout),
                script_sig: ScriptBuf::default(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }],
            output: outputs,
        };

        let fee = self.manager.calculate_transaction_fee(FeeParams {
            n_inputs: unsigned_tx.input.len() as u64,
            n_outputs: unsigned_tx.output.len() as u64,
            fee_rate: get_fee_rate(),
        });

        println!("Staking Fee: {:?}", fee);

        let total_output_value = unsigned_tx
            .output
            .iter()
            .map(|o| o.value.to_sat())
            .sum::<u64>();

        let change = utxo.amount - Amount::from_sat(total_output_value) - fee;

        if change > Amount::ZERO {
            unsigned_tx.output.push(TxOut {
                value: change,
                script_pubkey: account.address().script_pubkey(),
            });
        }

        let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();

        psbt.inputs[0] = Input {
            witness_utxo: Some(TxOut {
                value: utxo.amount,
                script_pubkey: utxo.script_pub_key.clone(),
            }),

            // TODO: fix this, taproot address: leaf hash, no key origin
            // TODO: fix this, segwit address: no leaf hash, key origin
            tap_internal_key: match account.address().address_type() {
                Some(AddressType::P2tr) => Some(account.public_key().inner.x_only_public_key().0),
                _ => None,
            },
            tap_key_origins: {
                let mut map = BTreeMap::new();
                // Note: no need leaf hash when staking
                map.insert(
                    account.public_key().inner.x_only_public_key().0,
                    (vec![], ([0u8; 4].into(), DerivationPath::default())),
                );
                map
            },
            ..Default::default()
        };

        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut psbt,
            &account.private_key().to_bytes(),
            self.network_id,
            true,
        )
        .unwrap();

        let result = Self::send_psbt(&self.rpc, psbt).unwrap();

        let staking_tx_hex = result.hex;

        let staking_tx: Transaction = bitcoin::consensus::deserialize(&staking_tx_hex).unwrap();

        println!("\nSTAKING TXID: {:?}", staking_tx.compute_txid());

        staking_tx
    }

    pub fn build_upc_unstaking_tx(
        &self,
        staking_tx: &Transaction,
        unstaking_type: UnstakingType,
        account: SuiteAccount,
    ) -> Psbt {
        <VaultManager as Unstaking>::build_upc(
            &self.manager,
            &UPCUnstakingParams {
                input: PreviousStakingUTXO {
                    outpoint: OutPoint::new(staking_tx.compute_txid(), VOUT as u32),
                    amount_in_sats: staking_tx.output[VOUT].value,
                    script_pubkey: staking_tx.output[VOUT].script_pubkey.clone(),
                },
                locking_script: account.address().script_pubkey(),
                user_pub_key: account.public_key(),
                protocol_pub_key: self.protocol_pubkey(),
                custodian_pub_keys: self.custodian_pubkeys(),
                custodian_quorum: self.env.custodian_quorum,
                fee_rate: get_fee_rate(),
                rbf: true,
            },
            unstaking_type,
        )
        .unwrap()
    }

    #[allow(dead_code)]
    pub fn build_batch_custodian_only_unstaking_tx(
        &self,
        staking_txs: &[Transaction],
        unstaking_outputs: Vec<UnstakingOutput>,
    ) -> Psbt {
        <VaultManager as Unstaking>::build_custodian_only(
            &self.manager,
            &CustodianOnlyUnstakingParams {
                inputs: staking_txs
                    .iter()
                    .map(|t| PreviousStakingUTXO {
                        outpoint: OutPoint::new(t.compute_txid(), VOUT as u32),
                        amount_in_sats: t.output[VOUT].value,
                        script_pubkey: t.output[VOUT].script_pubkey.clone(),
                    })
                    .collect(),
                unstaking_outputs,
                custodian_pub_keys: self.custodian_pubkeys(),
                custodian_quorum: self.env.custodian_quorum,
                fee_rate: get_fee_rate(),
                rbf: true,
            },
        )
        .unwrap()
    }

    pub fn send_psbt_by_rpc(&self, psbt: Psbt) -> Option<GetTransactionResult> {
        Self::send_psbt(&self.rpc, psbt)
    }

    pub fn send_psbt(rpc: &Client, psbt: Psbt) -> Option<GetTransactionResult> {
        let finalized_tx = psbt.extract_tx().unwrap();

        let tx_hex = bitcoin::consensus::serialize(&finalized_tx);

        println!("TX_HEX: {:?}", tx_hex.to_lower_hex_string());

        let txid = rpc.send_raw_transaction(&finalized_tx).unwrap();

        let mut retry_count = 0;

        let tx_result: Option<GetTransactionResult> = loop {
            let tx_result = rpc.get_transaction(&txid, None).ok();

            if tx_result.is_none() {
                retry_count += 1;
            } else {
                break tx_result;
            }

            if retry_count > 10 {
                panic!("tx not found");
            }
        };

        let tx = tx_result.unwrap();

        Some(tx)
    }
}

impl TestSuite {
    pub fn manager(&self) -> &VaultManager {
        &self.manager
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn env_path(&self) -> &str {
        &self.env_path
    }

    pub fn protocol_pubkey(&self) -> PublicKey {
        self.protocol_pair.1
    }

    #[allow(dead_code)]
    pub fn protocol_privkey(&self) -> PrivateKey {
        self.protocol_pair.0
    }

    pub fn custodian_pubkeys(&self) -> Vec<PublicKey> {
        self.custodian_pairs.values().map(|p| p.1).collect()
    }

    pub fn custodian_privkeys(&self) -> Vec<Vec<u8>> {
        self.custodian_pairs
            .values()
            .map(|p| p.0.to_bytes())
            .collect()
    }

    fn hex_to_destination<T: TryFrom<Vec<u8>>>(&self, hex_str: &str) -> T
    where
        T::Error: std::fmt::Debug,
    {
        hex_to_vec(hex_str).try_into().unwrap()
    }

    pub fn network_id(&self) -> NetworkKind {
        self.network_id
    }

    pub fn pick_random_custodian_privkeys(&self) -> Vec<Vec<u8>> {
        let custodian_privkeys = self.custodian_privkeys();
        let mut indices: Vec<usize> = (0..custodian_privkeys.len()).collect();

        // Shuffle the indices to ensure randomness
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);

        // Take the first `n` unique indices
        indices
            .into_iter()
            .take(self.env.custodian_quorum as usize)
            .map(|i| custodian_privkeys[i].clone())
            .collect()
    }
}
