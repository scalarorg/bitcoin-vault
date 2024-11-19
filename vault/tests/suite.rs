use bitcoin::bip32::DerivationPath;
use bitcoin::psbt::Input;
use bitcoin::{
    absolute, address::NetworkChecked, key::Secp256k1, transaction, Address, NetworkKind,
    PrivateKey, Psbt, PublicKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
};
use bitcoin::{AddressType, OutPoint};
use bitcoin_vault::{
    BuildStakingParams, BuildStakingWithOnlyCovenantsParams, BuildUnstakingParams,
    BuildUnstakingWithOnlyCovenantsParams, PreviousStakingUTXO, Signing, Staking, TaprootTreeType,
    Unstaking, UnstakingType, VaultManager,
};
use bitcoincore_rpc::json::GetTransactionResult;
use log::debug;

use std::collections::BTreeMap;

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

        println!("staking tx_id: {:?}", staking_tx.compute_txid());

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
            "env.tag".as_bytes().to_vec(),
            "env.service_tag".as_bytes().to_vec(),
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

        let fee = get_fee(1);

        <VaultManager as Unstaking>::build(
            &self.manager,
            &BuildUnstakingParams {
                input_utxo: PreviousStakingUTXO {
                    outpoint: OutPoint::new(staking_tx.compute_txid(), vout as u32),
                    amount_in_sats: staking_tx.output[vout].value,
                    script_pubkey: staking_tx.output[vout].script_pubkey.clone(),
                },

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
        let fee = get_fee(1);

        <VaultManager as Unstaking>::build_with_only_covenants(
            &self.manager,
            &BuildUnstakingWithOnlyCovenantsParams {
                input_utxo: PreviousStakingUTXO {
                    outpoint: OutPoint::new(staking_tx.compute_txid(), vout as u32),
                    amount_in_sats: staking_tx.output[vout].value,
                    script_pubkey: staking_tx.output[vout].script_pubkey.clone(),
                },

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

        let txid = rpc.send_raw_transaction(&finalized_tx).unwrap();

        let tx_result = rpc.get_transaction(&txid, None).ok();

        if tx_result.is_none() {
            panic!("tx not found");
        }

        let tx = tx_result.unwrap();

        Some(tx)
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
}
