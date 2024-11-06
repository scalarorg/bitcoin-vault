use std::collections::BTreeMap;
use std::str::FromStr;
use std::time::Duration;

use bitcoin::bip32::DerivationPath;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::psbt::Input;
use bitcoin::{
    absolute, address::NetworkChecked, consensus::serialize, key::Secp256k1, secp256k1::All,
    transaction, Address, NetworkKind, PrivateKey, Psbt, PublicKey, ScriptBuf, Sequence,
    Transaction, TxIn, TxOut, Witness,
};
use bitcoin::{OutPoint, Txid};
use bitcoin_vault::{
    BuildStakingParams, BuildUnstakingParams, PreviousStakingUTXO, SignByKeyMap, Signing, Staking,
    Unstaking, UnstakingType, VaultManager,
};
use bitcoincore_rpc::json::{GetTransactionResult, ListUnspentResultEntry};

use std::thread::sleep;

use bitcoin::hashes::Hash;

use crate::{get_env, hex_to_vec, Env, MANAGER};

use lazy_static::lazy_static;

use bitcoincore_rpc::{Auth, Client, RpcApi};

lazy_static! {
    static ref SUITE: TestSuite<'static> = TestSuite::new();
}

pub struct TestSuite<'a> {
    rpc: Client,
    env: &'a Env,
    user_pair: (PrivateKey, PublicKey),
    protocol_pair: (PrivateKey, PublicKey),
    covenant_pairs: BTreeMap<PublicKey, (PrivateKey, PublicKey)>,
    user_address: Address<NetworkChecked>,
}

#[test]
fn test_user_protocol_unstaking() {
    // prepare staking tx
    let staking_tx = TestSuite::new().prepare_staking_tx();

    // prepare unstaking tx
    let mut unstaked_psbt =
        TestSuite::new().build_unstaking_tx(&staking_tx, UnstakingType::UserProtocol);

    // sign unstaking psbt
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &SUITE.get_user_privkey_bytes(),
        NetworkKind::Test,
        false,
    )
    .unwrap();

    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &SUITE.get_protocol_privkey_bytes(),
        NetworkKind::Test,
        true,
    )
    .unwrap();

    //  send unstaking tx
    let result = TestSuite::new().send_psbt(unstaked_psbt);

    println!("unstaked tx result: {:?}", result);
}

#[test]
fn test_covenants_user_unstaking() {
    let staking_tx = TestSuite::new().prepare_staking_tx();
    let mut unstaked_psbt =
        TestSuite::new().build_unstaking_tx(&staking_tx, UnstakingType::CovenantsUser);

    // Sign with user key first
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        &SUITE.get_user_privkey_bytes(),
        NetworkKind::Test,
        false,
    )
    .unwrap();

    // Get sorted covenant keys and sign in order
    let sorted_covenants_privkey_bytes = SUITE
        .get_sorted_covenant_privkeys()
        .iter()
        .map(|p| p.to_bytes())
        .collect::<Vec<Vec<u8>>>();

    // Sign with each covenant key in order
    for privkey_bytes in sorted_covenants_privkey_bytes {
        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut unstaked_psbt,
            &privkey_bytes,
            NetworkKind::Test,
            false,
        )
        .unwrap();
    }

    // Finalize the PSBT
    <Psbt as SignByKeyMap<All>>::finalize(&mut unstaked_psbt);

    // Extract and send
    let result = TestSuite::new().send_psbt(unstaked_psbt);
    println!("unstaked tx result: {:?}", result);
}

impl<'a> TestSuite<'a> {
    pub fn new() -> Self {
        let env = get_env();

        let secp = Secp256k1::new();

        let rpc = Client::new(
            &format!("{}/wallet/{}", env.btc_node_address, "legacy"),
            Auth::UserPass(env.btc_node_user.clone(), env.btc_node_password.clone()),
        )
        .expect("Failed to create RPC client");

        let user_address: Address<NetworkChecked> = Address::from_str(&env.user_address)
            .unwrap()
            .require_network(bitcoin::Network::Regtest)
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

    fn prepare_staking_tx(&self) -> Transaction {
        let params = self.get_staking_params();
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

        let result = self.send_psbt(psbt);

        let staking_tx_hex = result.hex;

        let staking_tx: Transaction = bitcoin::consensus::deserialize(&staking_tx_hex).unwrap();

        staking_tx
    }

    fn build_unstaking_tx(&self, staking_tx: &Transaction, unstaking_type: UnstakingType) -> Psbt {
        let mut reversed_tx: [u8; 32] = staking_tx.compute_txid().as_raw_hash().to_byte_array();
        reversed_tx.reverse();

        let vout: usize = 0;

        <VaultManager as Unstaking>::build(
            &MANAGER,
            &BuildUnstakingParams {
                input_utxo: PreviousStakingUTXO {
                    outpoint: OutPoint::new(Txid::from_byte_array(reversed_tx), vout as u32),
                    amount_in_sats: staking_tx.output[vout].value,
                    script_pubkey: staking_tx.output[vout].script_pubkey.clone(),
                },

                unstaking_output: TxOut {
                    value: staking_tx.output[vout].value - bitcoin::Amount::from_sat(1_000),
                    script_pubkey: SUITE.get_user_address().script_pubkey(),
                },
                user_pub_key: SUITE.get_user_pubkey(),
                protocol_pub_key: SUITE.get_protocol_pubkey(),
                covenant_pub_keys: SUITE.get_covenant_pubkeys(),
                covenant_quorum: SUITE.get_covenant_quorum(),
                have_only_covenants: SUITE.get_have_only_covenants(),
                rbf: true,
            },
            unstaking_type,
        )
        .unwrap()
    }

    fn send_psbt(&self, psbt: Psbt) -> GetTransactionResult {
        let finalized_tx = psbt.extract_tx().unwrap();

        let txid = SUITE.get_rpc().send_raw_transaction(&finalized_tx).unwrap();

        let mut tx_result: Option<GetTransactionResult> = None;

        let retry = 3;
        for _ in 0..retry {
            tx_result = SUITE.get_rpc().get_transaction(&txid, None).ok();
            sleep(Duration::from_secs(5));
        }

        if tx_result.is_none() {
            panic!("tx not found");
        }

        let tx = tx_result.unwrap();

        println!("tx result: {:?}", tx);

        tx
    }

    fn get_staking_params(&self) -> BuildStakingParams {
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

    fn get_approvable_utxos(&self, btc_amount: u64) -> ListUnspentResultEntry {
        let utxos = self
            .rpc
            .list_unspent(Some(0), None, Some(&[&self.user_address]), None, None)
            .unwrap();

        let seleted_utxo = utxos
            .into_iter()
            .find(|u| u.amount >= bitcoin::Amount::from_sat(btc_amount))
            .unwrap();

        seleted_utxo
    }

    fn key_from_wif(wif: &str, secp: &Secp256k1<All>) -> (PrivateKey, PublicKey) {
        let privkey = PrivateKey::from_wif(wif).unwrap();
        let pubkey = privkey.public_key(secp);
        (privkey, pubkey)
    }

    fn get_destination_chain_id(&self) -> [u8; 8] {
        self.env.destination_chain_id.to_le_bytes()
    }

    fn get_destination_contract_address(&self) -> [u8; 20] {
        hex_to_vec!(self.env.destination_contract_address)
            .try_into()
            .unwrap()
    }

    fn get_destination_recipient_address(&self) -> [u8; 20] {
        hex_to_vec!(self.env.destination_recipient_address)
            .try_into()
            .unwrap()
    }

    fn get_staking_amount(&self) -> u64 {
        self.env.staking_amount
    }

    fn get_covenant_quorum(&self) -> u8 {
        self.env.covenant_quorum
    }

    fn get_have_only_covenants(&self) -> bool {
        self.env.have_only_covenants
    }

    fn get_fee_rate(&self) -> u64 {
        1
    }

    fn get_user_pubkey(&self) -> PublicKey {
        self.user_pair.1
    }

    fn get_protocol_pubkey(&self) -> PublicKey {
        self.protocol_pair.1
    }

    fn get_covenant_pubkeys(&self) -> Vec<PublicKey> {
        self.covenant_pairs.values().map(|p| p.1).collect()
    }

    fn get_sorted_covenant_privkeys(&self) -> Vec<PrivateKey> {
        let mut pubkey_privkey_pairs: Vec<(PublicKey, PrivateKey)> = self
            .covenant_pairs
            .iter()
            .map(|(_, pair)| (pair.1, pair.0))
            .collect();

        pubkey_privkey_pairs.sort_by(|a, b| a.0.cmp(&b.0));

        pubkey_privkey_pairs
            .into_iter()
            .map(|(_, privkey)| privkey)
            .collect()
    }

    fn get_user_address(&self) -> Address<NetworkChecked> {
        self.user_address.clone()
    }

    fn get_rpc(&self) -> &Client {
        &self.rpc
    }

    fn get_user_privkey_bytes(&self) -> Vec<u8> {
        self.user_pair.0.to_bytes()
    }

    fn get_protocol_privkey_bytes(&self) -> Vec<u8> {
        self.protocol_pair.0.to_bytes()
    }

    fn get_fee(&self, n_outputs: u64) -> u64 {
        (148 + (34 * n_outputs) + 10) * self.get_fee_rate()
    }
}