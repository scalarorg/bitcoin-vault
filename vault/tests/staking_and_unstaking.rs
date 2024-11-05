use std::collections::BTreeMap;
use std::str::FromStr;
use std::time::Duration;

use bitcoin::bip32::DerivationPath;
use bitcoin::psbt::Input;
use bitcoin::{
    absolute, address::NetworkChecked, key::Secp256k1, secp256k1::All, transaction, Address,
    NetworkKind, PrivateKey, Psbt, PublicKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut,
    Witness,
};
use bitcoin::{OutPoint, Txid};
use bitcoin_vault::{
    BuildStakingParams, BuildUnstakingParams, PreviousStakingUTXO, Signing, Staking, Unstaking,
    VaultManager,
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
    user_privkey: PrivateKey,
    user_pubkey: PublicKey,
    user_address: Address<NetworkChecked>,
    protocol_privkey: PrivateKey,
    protocol_pubkey: PublicKey,
    covenant_pubkeys: Vec<PublicKey>,
}

#[test]
fn test_user_protocol_unstaking() {
    // prepare staking tx
    let (staking_tx, txid) = TestSuite::new().prepare_staking_tx();

    // prepare unstaking tx
    let mut reversed_tx: [u8; 32] = txid.as_raw_hash().to_byte_array();
    reversed_tx.reverse();

    let vout: usize = 0;

    let mut unstaked_psbt = <VaultManager as Unstaking>::build(
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
        bitcoin_vault::UnstakingType::UserProtocol,
    )
    .unwrap();

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
    let finalized_tx = unstaked_psbt.extract_tx().unwrap();

    let txid = SUITE.get_rpc().send_raw_transaction(&finalized_tx).unwrap();

    let mut unstaked_tx: Option<GetTransactionResult> = None;

    let retry = 3;
    for _ in 0..retry {
        unstaked_tx = SUITE.get_rpc().get_transaction(&txid, None).ok();
        sleep(Duration::from_secs(5));
    }

    if unstaked_tx.is_none() {
        panic!("unstaked tx not found");
    }

    println!("unstaked txid: {}", txid);
    println!("unstaked_tx: {:?}", unstaked_tx.unwrap());
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

        let (user_privkey, user_pubkey) = TestSuite::key_from_wif(&env.user_private_key, &secp);
        let (protocol_privkey, protocol_pubkey) =
            TestSuite::key_from_wif(&env.protocol_private_key, &secp);
        let covenant_pubkeys: Vec<PublicKey> = env
            .covenant_private_keys
            .iter()
            .map(|s| TestSuite::key_from_wif(s, &secp).1)
            .collect();

        Self {
            rpc,
            env,
            user_privkey,
            user_pubkey,
            user_address,
            protocol_privkey,
            protocol_pubkey,
            covenant_pubkeys,
        }
    }

    fn prepare_staking_tx(&self) -> (Transaction, Txid) {
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

        let finalized_tx = psbt.extract_tx().unwrap();

        let txid = self.rpc.send_raw_transaction(&finalized_tx).unwrap();

        let mut staking_tx: Option<GetTransactionResult> = None;

        println!("waiting for staking tx to be confirmed");
        println!("txid: {}", txid);

        let retry = 3;
        for _ in 0..retry {
            staking_tx = self.rpc.get_transaction(&txid, None).ok();
            sleep(Duration::from_secs(5));
        }

        if staking_tx.is_none() {
            panic!("staking tx not found");
        }

        let staking_tx_hex = staking_tx.unwrap().hex;

        let staking_tx: Transaction = bitcoin::consensus::deserialize(&staking_tx_hex).unwrap();

        (staking_tx, txid)
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
        self.user_pubkey
    }

    fn get_protocol_pubkey(&self) -> PublicKey {
        self.protocol_pubkey
    }

    fn get_covenant_pubkeys(&self) -> Vec<PublicKey> {
        self.covenant_pubkeys.clone()
    }

    fn get_user_address(&self) -> Address<NetworkChecked> {
        self.user_address.clone()
    }

    fn get_rpc(&self) -> &Client {
        &self.rpc
    }

    fn get_user_privkey_bytes(&self) -> Vec<u8> {
        self.user_privkey.to_bytes()
    }

    fn get_protocol_privkey_bytes(&self) -> Vec<u8> {
        self.protocol_privkey.to_bytes()
    }

    fn get_fee(&self, n_outputs: u64) -> u64 {
        (148 + (34 * n_outputs) + 10) * self.get_fee_rate()
    }
}
