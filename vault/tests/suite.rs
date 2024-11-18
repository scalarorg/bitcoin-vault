use std::collections::BTreeMap;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use bitcoin::bip32::DerivationPath;
use bitcoin::psbt::Input;
use bitcoin::{
    absolute, address::NetworkChecked, key::Secp256k1, secp256k1::All, transaction, Address,
    NetworkKind, PrivateKey, Psbt, PublicKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut,
    Witness,
};
use bitcoin::{Amount, OutPoint};
use bitcoin_vault::{
    BuildStakingParams, BuildStakingWithOnlyCovenantsParams, BuildUnstakingParams,
    BuildUnstakingWithOnlyCovenantsParams, PreviousStakingUTXO, Signing, Staking, Unstaking,
    UnstakingType, VaultManager,
};
use bitcoincore_rpc::json::{
    GetTransactionResult, ListUnspentQueryOptions, ListUnspentResultEntry,
};

use bitcoincore_rpc::{Auth, Client, RpcApi};

use crate::{env::get_env, hex_to_vec, Env, MANAGER};

#[derive(Debug)]
pub struct TestSuite<'a> {
    rpc: Client,
    env: &'a Env,
    user_pair: (PrivateKey, PublicKey),
    protocol_pair: (PrivateKey, PublicKey),
    covenant_pairs: BTreeMap<PublicKey, (PrivateKey, PublicKey)>,
    user_address: Address<NetworkChecked>,
}

impl<'a> TestSuite<'a> {
    pub fn new() -> Self {
        let env = get_env();

        let secp = Secp256k1::new();

        let rpc = Client::new(
            &format!("{}/wallet/{}", env.btc_node_address, "staker"),
            Auth::UserPass(env.btc_node_user.clone(), env.btc_node_password.clone()),
        )
        .unwrap();

        let network = match env.network.as_str() {
            "testnet4" => bitcoin::Network::Testnet4,
            "regtest" => bitcoin::Network::Regtest,
            _ => panic!("Invalid network"),
        };

        let user_address: Address<NetworkChecked> = Address::from_str(&env.user_address)
            .unwrap()
            .require_network(network)
            .unwrap();

        let mut covenant_pairs: BTreeMap<PublicKey, (PrivateKey, PublicKey)> = BTreeMap::new();

        for (_, s) in env.covenant_private_keys.iter().enumerate() {
            let (privkey, pubkey) = TestSuite::key_from_wif(s, &secp);
            covenant_pairs.insert(pubkey, (privkey, pubkey));
        }

        Self {
            rpc,
            env,
            user_pair: TestSuite::key_from_wif(&env.user_private_key, &secp),
            protocol_pair: TestSuite::key_from_wif(&env.protocol_private_key, &secp),
            covenant_pairs,
            user_address,
        }
    }

    pub fn set_rpc(&mut self, wallet_name: &str) {
        self.rpc = self.create_rpc(wallet_name);
    }

    pub fn create_rpc(&self, wallet_name: &str) -> Client {
        Client::new(
            &format!("{}/wallet/{}", self.env.btc_node_address, wallet_name),
            Auth::UserPass(
                self.env.btc_node_user.clone(),
                self.env.btc_node_password.clone(),
            ),
        )
        .unwrap()
    }

    pub fn prepare_staking_tx(&self, have_only_covenants: Option<bool>) -> Transaction {
        let mut params = self.get_staking_params();
        params.have_only_covenants = have_only_covenants.unwrap_or(params.have_only_covenants);

        let utxo = self.get_approvable_utxos(self.get_staking_amount());

        let outputs = <VaultManager as Staking>::build(&MANAGER, &params)
            .unwrap()
            .into_tx_outs();

        let fee = self.get_fee(outputs.len() as u64);

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
                script_pubkey: self.get_user_address().script_pubkey(),
            });
        }

        let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();

        psbt.inputs[0] = Input {
            witness_utxo: Some(TxOut {
                value: utxo.amount,
                script_pubkey: utxo.script_pub_key.clone(),
            }),
            tap_key_origins: {
                let mut map = BTreeMap::new();

                map.insert(
                    self.get_user_pubkey().inner.x_only_public_key().0,
                    (
                        vec![utxo.script_pub_key.tapscript_leaf_hash()],
                        ([0u8; 4].into(), DerivationPath::default()),
                    ),
                );
                map
            },
            ..Default::default()
        };

        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut psbt,
            &self.get_user_privkey_bytes(),
            NetworkKind::Test,
            true,
        )
        .unwrap();

        let result = self.send_psbt(psbt).unwrap();

        let staking_tx_hex = result.hex;

        let staking_tx: Transaction = bitcoin::consensus::deserialize(&staking_tx_hex).unwrap();

        staking_tx
    }

    pub fn prepare_only_covenants_staking_tx(&self) -> Transaction {
        let utxo = self.get_approvable_utxos(self.get_staking_amount());

        let outputs = <VaultManager as Staking>::build_with_only_covenants(
            &MANAGER,
            &BuildStakingWithOnlyCovenantsParams {
                covenant_pub_keys: self.get_covenant_pubkeys(),
                covenant_quorum: self.get_covenant_quorum(),
                staking_amount: self.get_staking_amount(),
                destination_chain_id: self.get_destination_chain_id(),
                destination_contract_address: self.get_destination_contract_address(),
                destination_recipient_address: self.get_destination_recipient_address(),
            },
        )
        .unwrap()
        .into_tx_outs();

        let fee = self.get_fee(outputs.len() as u64);

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
                script_pubkey: self.get_user_address().script_pubkey(),
            });
        }

        let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();

        psbt.inputs[0] = Input {
            witness_utxo: Some(TxOut {
                value: utxo.amount,
                script_pubkey: utxo.script_pub_key.clone(),
            }),
            tap_internal_key: Some(self.get_user_pubkey().inner.x_only_public_key().0),
            tap_key_origins: {
                let mut map = BTreeMap::new();

                // no leaf hashes, no derivation path
                map.insert(
                    self.get_user_pubkey().inner.x_only_public_key().0,
                    (vec![], ([0u8; 4].into(), DerivationPath::default())),
                );
                map
            },
            ..Default::default()
        };

        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut psbt,
            &self.get_user_privkey_bytes(),
            NetworkKind::Test,
            true,
        )
        .unwrap();

        let result = self.send_psbt(psbt).unwrap();

        let staking_tx_hex = result.hex;

        let staking_tx: Transaction = bitcoin::consensus::deserialize(&staking_tx_hex).unwrap();

        staking_tx
    }

    pub fn build_unstaking_tx(
        &self,
        staking_tx: &Transaction,
        unstaking_type: UnstakingType,
        have_only_covenants: Option<bool>,
    ) -> Psbt {
        let vout: usize = 0;

        let fee = self.get_fee(1);

        <VaultManager as Unstaking>::build(
            &MANAGER,
            &BuildUnstakingParams {
                input_utxo: PreviousStakingUTXO {
                    outpoint: OutPoint::new(staking_tx.compute_txid(), vout as u32),
                    amount_in_sats: staking_tx.output[vout].value,
                    script_pubkey: staking_tx.output[vout].script_pubkey.clone(),
                },

                unstaking_output: TxOut {
                    value: staking_tx.output[vout].value - bitcoin::Amount::from_sat(fee),
                    script_pubkey: self.get_user_address().script_pubkey(),
                },
                user_pub_key: self.get_user_pubkey(),
                protocol_pub_key: self.get_protocol_pubkey(),
                covenant_pub_keys: self.get_covenant_pubkeys(),
                covenant_quorum: self.get_covenant_quorum(),
                have_only_covenants: have_only_covenants.unwrap_or(self.get_have_only_covenants()),
                rbf: true,
            },
            unstaking_type,
        )
        .unwrap()
    }

    pub fn build_only_covenants_unstaking_tx(&self, staking_tx: &Transaction) -> Psbt {
        let vout: usize = 0;
        let fee = self.get_fee(3);

        <VaultManager as Unstaking>::build_with_only_covenants(
            &MANAGER,
            &BuildUnstakingWithOnlyCovenantsParams {
                input_utxo: PreviousStakingUTXO {
                    outpoint: OutPoint::new(staking_tx.compute_txid(), vout as u32),
                    amount_in_sats: staking_tx.output[vout].value,
                    script_pubkey: staking_tx.output[vout].script_pubkey.clone(),
                },

                unstaking_output: TxOut {
                    value: staking_tx.output[vout].value - bitcoin::Amount::from_sat(fee),
                    script_pubkey: self.get_user_address().script_pubkey(),
                },
                covenant_pub_keys: self.get_covenant_pubkeys(),
                covenant_quorum: self.get_covenant_quorum(),
                rbf: true,
            },
        )
        .unwrap()
    }

    pub fn send_psbt(&self, psbt: Psbt) -> Option<GetTransactionResult> {
        let finalized_tx = psbt.extract_tx().unwrap();

        let txid = self.rpc.send_raw_transaction(&finalized_tx).unwrap();

        let mut tx_result: Option<GetTransactionResult> = None;

        let retry = 3;
        for _ in 0..retry {
            tx_result = self.get_rpc().get_transaction(&txid, None).ok();
            sleep(Duration::from_secs(5));
        }

        if tx_result.is_none() {
            panic!("tx not found");
        }

        let tx = tx_result.unwrap();

        Some(tx)
    }

    pub fn get_staking_params(&self) -> BuildStakingParams {
        BuildStakingParams {
            user_pub_key: self.get_user_pubkey(),
            protocol_pub_key: self.get_protocol_pubkey(),
            covenant_pub_keys: self.get_covenant_pubkeys(),
            covenant_quorum: self.get_covenant_quorum(),
            staking_amount: self.get_staking_amount(),
            have_only_covenants: self.get_have_only_covenants(),
            destination_chain_id: self.get_destination_chain_id(),
            destination_contract_address: self.get_destination_contract_address(),
            destination_recipient_address: self.get_destination_recipient_address(),
        }
    }

    pub fn get_approvable_utxos(&self, btc_amount: u64) -> ListUnspentResultEntry {
        let utxos = self
            .rpc
            .list_unspent(
                Some(0),
                None,
                Some(&[&self.user_address]),
                Some(true),
                Some(ListUnspentQueryOptions {
                    minimum_amount: Some(Amount::from_sat(btc_amount)),
                    maximum_amount: None,
                    maximum_count: None,
                    minimum_sum_amount: None,
                }),
            )
            .unwrap();

        utxos[0].clone()
    }

    pub fn key_from_wif(wif: &str, secp: &Secp256k1<All>) -> (PrivateKey, PublicKey) {
        let privkey = PrivateKey::from_wif(wif).unwrap();
        let pubkey = privkey.public_key(secp);
        (privkey, pubkey)
    }

    pub fn get_destination_chain_id(&self) -> [u8; 8] {
        self.env.destination_chain_id.to_le_bytes()
    }

    pub fn get_destination_contract_address(&self) -> [u8; 20] {
        hex_to_vec(&self.env.destination_contract_address)
            .try_into()
            .unwrap()
    }

    pub fn get_destination_recipient_address(&self) -> [u8; 20] {
        hex_to_vec(&self.env.destination_recipient_address)
            .try_into()
            .unwrap()
    }

    pub fn get_staking_amount(&self) -> u64 {
        self.env.staking_amount
    }

    pub fn get_covenant_quorum(&self) -> u8 {
        self.env.covenant_quorum
    }

    pub fn get_have_only_covenants(&self) -> bool {
        self.env.have_only_covenants
    }

    pub fn get_fee_rate(&self) -> u64 {
        1
    }

    pub fn get_user_pubkey(&self) -> PublicKey {
        self.user_pair.1
    }

    pub fn get_protocol_pubkey(&self) -> PublicKey {
        self.protocol_pair.1
    }

    pub fn get_covenant_pubkeys(&self) -> Vec<PublicKey> {
        self.covenant_pairs.values().map(|p| p.1).collect()
    }

    pub fn get_covenant_privkeys(&self) -> Vec<Vec<u8>> {
        self.covenant_pairs
            .values()
            .map(|p| p.0.to_bytes())
            .collect()
    }

    pub fn get_user_address(&self) -> Address<NetworkChecked> {
        self.user_address.clone()
    }

    pub fn get_rpc(&self) -> &Client {
        let args = ["passphrase".into(), 60.into()];
        let result: Result<(), bitcoincore_rpc::Error> =
            bitcoincore_rpc::RpcApi::call(&self.rpc, "walletpassphrase", &args);
        result.unwrap();
        &self.rpc
    }

    pub fn get_user_privkey_bytes(&self) -> Vec<u8> {
        self.user_pair.0.to_bytes()
    }

    pub fn get_protocol_privkey_bytes(&self) -> Vec<u8> {
        self.protocol_pair.0.to_bytes()
    }

    // for one input
    pub fn get_fee(&self, n_outputs: u64) -> u64 {
        (148 + (34 * n_outputs) + 12) * self.get_fee_rate()

        // (148 * n_inputs + 34 * n_outputs + 10) * fee_rate
    }
}
