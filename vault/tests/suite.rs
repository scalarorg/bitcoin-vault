use bitcoin::bip32::DerivationPath;
use bitcoin::hex::DisplayHex;
use bitcoin::key::rand;
use bitcoin::psbt::Input;
use bitcoin::{
    absolute, address::NetworkChecked, key::Secp256k1, transaction, Address, NetworkKind,
    PrivateKey, Psbt, PublicKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
};
use bitcoin::{AddressType, OutPoint, Txid};
use bitcoin_vault::{
    BuildStakingParams, BuildStakingWithOnlyCovenantsParams, BuildUnstakingParams,
    BuildUnstakingWithOnlyCovenantsParams, PreviousStakingUTXO, Signing, Staking, TaprootTreeType,
    Unstaking, UnstakingType, VaultManager,
};
use bitcoincore_rpc::json::GetTransactionResult;

use std::collections::BTreeMap;

use bitcoin::secp256k1::rand::Rng;
use bitcoincore_rpc::Client;
use bitcoincore_rpc::RpcApi;

use crate::env::*;
use crate::helper::*;

#[derive(Debug)]
pub struct TestSuite<'a> {
    rpc: Client,
    env: &'a Env,
    user_pair: (PrivateKey, PublicKey),
    protocol_pair: (PrivateKey, PublicKey),
    covenant_pairs: BTreeMap<PublicKey, (PrivateKey, PublicKey)>,
    user_address: Address<NetworkChecked>,
    manager: VaultManager,
    network_id: NetworkKind,
}

impl<'staking> TestSuite<'staking> {
    pub fn prepare_staking_tx(
        &self,
        amount: u64,
        taproot_tree_type: TaprootTreeType,
        have_only_covenants: Option<bool>,
    ) -> Transaction {
        let destination_chain = self.hex_to_destination(&self.env.destination_chain);
        let destination_contract_address =
            self.hex_to_destination(&self.env.destination_contract_address);
        let destination_recipient_address =
            self.hex_to_destination(&self.env.destination_recipient_address);

        let outputs: Vec<TxOut> = match taproot_tree_type {
            TaprootTreeType::OneBranchOnlyCovenants => {
                <VaultManager as Staking>::build_with_only_covenants(
                    &self.manager,
                    &BuildStakingWithOnlyCovenantsParams {
                        covenant_pub_keys: self.covenant_pubkeys(),
                        covenant_quorum: self.env.covenant_quorum,
                        staking_amount: amount,
                        destination_chain,
                        destination_contract_address,
                        destination_recipient_address,
                    },
                )
                .unwrap()
                .into_tx_outs()
            }
            TaprootTreeType::OneBranchOnlyKeys => {
                panic!("not implemented");
            }
            _ => <VaultManager as Staking>::build(
                &self.manager,
                &BuildStakingParams {
                    user_pub_key: self.user_pubkey(),
                    protocol_pub_key: self.protocol_pubkey(),
                    covenant_pub_keys: self.covenant_pubkeys(),
                    covenant_quorum: self.env.covenant_quorum,
                    staking_amount: amount,
                    have_only_covenants: have_only_covenants
                        .unwrap_or(self.env.have_only_covenants),
                    destination_chain: self.hex_to_destination(&self.env.destination_chain),
                    destination_contract_address: self
                        .hex_to_destination(&self.env.destination_contract_address),
                    destination_recipient_address: self
                        .hex_to_destination(&self.env.destination_recipient_address),
                },
            )
            .unwrap()
            .into_tx_outs(),
        };

        let utxo = get_approvable_utxos(&self.rpc, &self.user_address, amount);

        let fee = get_fee(outputs.len() as u64);

        let change =
            utxo.amount.to_sat() - outputs.iter().map(|o| o.value.to_sat()).sum::<u64>() - fee;

        let mut unsigned_tx = Transaction {
            version: transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: OutPoint::new(utxo.txid, utxo.vout),
                script_sig: ScriptBuf::default(),
                sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::new(),
            }],
            output: outputs,
        };

        if change > 0 {
            unsigned_tx.output.push(TxOut {
                value: bitcoin::Amount::from_sat(change),
                script_pubkey: self.user_address().script_pubkey(),
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
            tap_internal_key: match self.user_address.address_type() {
                Some(AddressType::P2tr) => Some(self.user_pubkey().inner.x_only_public_key().0),
                _ => None,
            },
            tap_key_origins: {
                let mut map = BTreeMap::new();
                // Note: no need leaf hash when staking
                map.insert(
                    self.user_pubkey().inner.x_only_public_key().0,
                    (vec![], ([0u8; 4].into(), DerivationPath::default())),
                );
                map
            },
            ..Default::default()
        };

        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut psbt,
            &self.user_privkey().to_bytes(),
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
}

impl<'a> TestSuite<'a> {
    pub fn new() -> Self {
        let env = &get_env();

        let secp = Secp256k1::new();

        let rpc = create_rpc(
            &env.btc_node_address,
            &env.btc_node_user,
            &env.btc_node_password,
            &env.bond_holder_wallet,
        );

        let user_address = get_adress(&env.network, &env.bond_holder_address);

        let mut covenant_pairs: BTreeMap<PublicKey, (PrivateKey, PublicKey)> = BTreeMap::new();

        for (_, s) in env.covenant_private_keys.iter().enumerate() {
            let (privkey, pubkey) = key_from_wif(s, &secp);
            covenant_pairs.insert(pubkey, (privkey, pubkey));
        }

        let network_id = get_network_id_from_str(&env.network);

        let manager = VaultManager::new(
            env.tag.as_bytes().to_vec(),
            env.service_tag.as_bytes().to_vec(),
            env.version,
            network_id as u8,
        );

        Self {
            rpc,
            env,
            user_pair: key_from_wif(&env.bond_holder_private_key, &secp),
            protocol_pair: key_from_wif(&env.protocol_private_key, &secp),
            covenant_pairs,
            user_address,
            manager,
            network_id,
        }
    }

    pub fn build_unstaking_tx(
        &self,
        staking_tx: &Transaction,
        unstaking_type: UnstakingType,
        have_only_covenants: Option<bool>,
    ) -> Psbt {
        let vout: usize = 0;

        let fee = get_fee(3);

        <VaultManager as Unstaking>::build(
            &self.manager,
            &BuildUnstakingParams {
                inputs: vec![PreviousStakingUTXO {
                    outpoint: OutPoint::new(staking_tx.compute_txid(), vout as u32),
                    amount_in_sats: staking_tx.output[vout].value,
                    script_pubkey: staking_tx.output[vout].script_pubkey.clone(),
                }],

                unstaking_output: TxOut {
                    value: staking_tx.output[vout].value - bitcoin::Amount::from_sat(fee),
                    script_pubkey: self.user_address().script_pubkey(),
                },
                user_pub_key: self.user_pubkey(),
                protocol_pub_key: self.protocol_pubkey(),
                covenant_pub_keys: self.covenant_pubkeys(),
                covenant_quorum: self.env.covenant_quorum,
                have_only_covenants: have_only_covenants.unwrap_or(self.env.have_only_covenants),
                rbf: true,
            },
            unstaking_type,
        )
        .unwrap()
    }

    pub fn build_only_covenants_unstaking_tx(&self, staking_tx: &Transaction) -> Psbt {
        let vout: usize = 0;
        let fee = get_fee(3);

        <VaultManager as Unstaking>::build_with_only_covenants(
            &self.manager,
            &BuildUnstakingWithOnlyCovenantsParams {
                inputs: vec![PreviousStakingUTXO {
                    outpoint: OutPoint::new(staking_tx.compute_txid(), vout as u32),
                    amount_in_sats: staking_tx.output[vout].value,
                    script_pubkey: staking_tx.output[vout].script_pubkey.clone(),
                }],

                unstaking_output: TxOut {
                    value: staking_tx.output[vout].value - bitcoin::Amount::from_sat(fee),
                    script_pubkey: self.user_address().script_pubkey(),
                },
                covenant_pub_keys: self.covenant_pubkeys(),
                covenant_quorum: self.env.covenant_quorum,
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

    pub fn get_tx_by_id(&self, txid: &Txid) -> Result<Transaction, ()> {
        let result = self.rpc.get_transaction(txid, None).map_err(|_| ())?;

        let tx = bitcoin::consensus::deserialize(&result.hex).map_err(|_| ())?;

        Ok(tx)
    }
}

impl<'setter> TestSuite<'setter> {
    pub fn set_rpc(&mut self, wallet_name: &str) {
        self.rpc = create_rpc(
            &self.env.btc_node_address,
            &self.env.btc_node_user,
            &self.env.btc_node_password,
            wallet_name,
        );
    }
}

impl<'getter> TestSuite<'getter> {
    pub fn user_pubkey(&self) -> PublicKey {
        self.user_pair.1
    }

    pub fn user_privkey(&self) -> PrivateKey {
        self.user_pair.0
    }

    pub fn protocol_pubkey(&self) -> PublicKey {
        self.protocol_pair.1
    }

    pub fn protocol_privkey(&self) -> PrivateKey {
        self.protocol_pair.0
    }

    pub fn covenant_pubkeys(&self) -> Vec<PublicKey> {
        self.covenant_pairs.values().map(|p| p.1).collect()
    }

    pub fn covenant_privkeys(&self) -> Vec<Vec<u8>> {
        self.covenant_pairs
            .values()
            .map(|p| p.0.to_bytes())
            .collect()
    }

    pub fn n_quorum(&self) -> usize {
        self.env.covenant_quorum as usize
    }

    pub fn user_address(&self) -> Address<NetworkChecked> {
        self.user_address.clone()
    }

    fn hex_to_destination<T: TryFrom<Vec<u8>>>(&self, hex_str: &str) -> T
    where
        T::Error: std::fmt::Debug,
    {
        hex_to_vec(hex_str).try_into().unwrap()
    }

    pub fn rpc(&self) -> &Client {
        &self.rpc
    }

    pub fn network_id(&self) -> NetworkKind {
        self.network_id
    }

    pub fn get_random_covenant_privkeys(&self) -> Vec<Vec<u8>> {
        let rng = rand::thread_rng();
        rng.sample_iter(&rand::distributions::Uniform::new(
            0,
            self.covenant_privkeys().len(),
        ))
        .take(self.env.covenant_quorum as usize)
        .map(|i| self.covenant_privkeys()[i].clone())
        .collect()
    }
}

#[test]
fn test_send_tx() {
    let suite = TestSuite::new();

    let mock_tx = "02000000000101caf9348569401a90dd741816d021115bfbcfdecdb3263eaa5408e5e991f973460000000000fdffffff01da0200000000000022512095033d48b6029174ed3ba21390756c56e90c41eeeef41c172c81d1d09a167cda0840939c003ddcee24444c0606004e0c3414762c1a90ed0e9a412287583e6f4d9147c763aa1035500257d392c1581f7ca30bb51d45eaadcee8e903ccc259491d1ef30000400b8d316ffa0b05fead9893d55cef3ac4b42e1d156a9d1396536880a4dd30132ce26e69134d915d0b1ed8243da8e9cf70d59165f6862722c03eea6cd12208ff3940ce86b1545e93b7949cc3b7abaca64846487e8f9fd7b496537f5dee409b0ed4604f085b14148dfb5f7d8cacf8cfb141ba82bb6e0538d18ac8d7e7009427d58c6d4029d7ec238e73b488a781b380be8f9cd67ae1b3b9c3aeb279696d2b72eccf650ade088b987ca156e8a64bc2f2dd2ce810cf3ca0c0676748e18e2e07150764407bce202ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350aad2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a261c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac09c739c91af9ac0e06a9e0e463c7be2848d15c31b74938543337bcdd7145078018b212098a1c9f95fadf69babfe738c34897215e91707f1fdba99fa5474d93b1f00000000";

    let tx: Transaction = bitcoin::consensus::deserialize(hex_to_vec(mock_tx).as_slice()).unwrap();

    println!("tx: {:?}", tx);

    let result = suite.rpc.send_raw_transaction(&tx).unwrap();

    println!("result: {:?}", result);
}
