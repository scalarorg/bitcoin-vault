use bip39::Mnemonic;
use bitcoin::{
    absolute::LockTime,
    bip32::{ChildNumber, DerivationPath, Xpriv},
    key::Secp256k1,
    psbt::Input,
    secp256k1::All,
    transaction::Version,
    Address, AddressType, Amount, CompressedPublicKey, Network, OutPoint, PrivateKey, Psbt,
    ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};
use clap::{Parser, Subcommand};
use rust_mempool::MempoolClient;
use serde::{Deserialize, Serialize};
use vault::{
    get_fee_rate, get_network_from_str, log_tx_result, Env, FeeParams, NeededUtxo, Signing,
    TaprootTreeType, TestSuite, VaultManager,
};

use crate::{
    commands::BridgeCmdParams,
    db::{CommandHistory, Config, Querier},
    executors::BridgeExecutor,
    TvlMaker,
};

use std::{collections::BTreeMap, str::FromStr};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct BatchCommand {
    /// Name for the unique config
    #[arg(short, long)]
    pub name: String,

    /// Amount
    #[arg(short, long)]
    pub amount: u64,

    #[arg(short, long)]
    pub count: u64,

    #[command(subcommand)]
    command: BatchSubCommands,
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
enum BatchSubCommands {
    /// Run bridge command
    BridgeUpc(BatchBridgeParams),
    // ///// Run redeem command
    BridgeCustodianOnly(BatchBridgeParams),
}

#[derive(Parser, Debug, Serialize, Deserialize)]

struct BatchBridgeParams {
    /// Service tag
    #[arg(short, long)]
    pub service_tag: String,
}

type BridgeResult<T> = anyhow::Result<T>;

impl BatchCommand {
    pub fn execute(&self, db_querier: &Querier) -> BridgeResult<()> {
        let (params, command_name, tree_type) = self.get_command_params();
        let config = self.load_config(db_querier)?;
        let suite = self.create_test_suite(&config, &params);
        let tvl_maker = TvlMaker::new_with_suite(db_querier, suite);

        let (xprv, base_path) = self.setup_wallet(&config)?;
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let network = get_network_from_str(&config.network);

        let command_histories = self.process_batch(
            &xprv,
            &base_path,
            &secp,
            network,
            &tvl_maker,
            command_name,
            tree_type,
            &config,
        )?;

        let n = tvl_maker
            .db_querier
            .batch_save(&command_histories)
            .map_err(|e| anyhow::anyhow!("Failed to create batch command history: {:?}", e))?;

        println!("n: {:?}", n);

        Ok(())
    }

    fn get_command_params(&self) -> (&BatchBridgeParams, &str, TaprootTreeType) {
        match &self.command {
            BatchSubCommands::BridgeUpc(params) => {
                (params, "batch_bridge_upc", TaprootTreeType::UPCBranch)
            }
            BatchSubCommands::BridgeCustodianOnly(params) => (
                params,
                "batch_bridge_custodian_only",
                TaprootTreeType::CustodianOnly,
            ),
        }
    }

    fn load_config(&self, db_querier: &Querier) -> BridgeResult<Config> {
        db_querier
            .get_config_by_name(&self.name)
            .map_err(|_| anyhow::anyhow!("Failed to get config"))
    }

    fn setup_wallet(&self, config: &Config) -> BridgeResult<(Xpriv, DerivationPath)> {
        let mnemonic = Mnemonic::parse_normalized(&config.mnemonic)
            .map_err(|e| anyhow::anyhow!("Invalid mnemonic: {}", e))?;

        let seed = mnemonic.to_seed("");
        let network = get_network_from_str(&config.network);
        let xprv = Xpriv::new_master(network, &seed)
            .map_err(|e| anyhow::anyhow!("Failed to create master key: {}", e))?;

        let base_path = DerivationPath::from_str("m/84'/0'/0'/0")
            .map_err(|e| anyhow::anyhow!("Invalid derivation path: {}", e))?;

        Ok((xprv, base_path))
    }

    #[tokio::main]
    async fn process_batch(
        &self,
        xprv: &Xpriv,
        base_path: &DerivationPath,
        secp: &Secp256k1<All>,
        network: Network,
        tvl_maker: &TvlMaker,
        command_name: &str,
        tree_type: TaprootTreeType,
        config: &Config,
    ) -> BridgeResult<Vec<CommandHistory>> {
        let mut command_histories = Vec::with_capacity(self.count as usize);

        // this accounts includes the first account and <count> accounts
        let accounts = (0..self.count + 1)
            .map(|i| get_account(base_path, i as u32, *xprv, secp, network))
            .collect::<Vec<(PrivateKey, Address)>>();

        let total_output_value = self.amount * self.count;

        let client = MempoolClient::new(network);

        let first_account = &accounts[0];

        let (utxos, total_utxos_amount): (Vec<NeededUtxo>, u64) =
            get_utxo_by_address(&client, &first_account.1, total_output_value)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get utxo: {}", e))?;

        let mut unsigned_tx = Transaction {
            version: Version::TWO,
            lock_time: LockTime::ZERO,
            input: utxos
                .iter()
                .map(|utxo| TxIn {
                    previous_output: OutPoint::new(utxo.txid, utxo.vout),
                    script_sig: ScriptBuf::default(),
                    sequence: Sequence::MAX,
                    witness: Witness::new(),
                })
                .collect(),
            output: (1..self.count + 1)
                .map(|i| {
                    let account = &accounts[i as usize];
                    TxOut {
                        value: Amount::from_sat(self.amount),
                        script_pubkey: account.1.script_pubkey(),
                    }
                })
                .collect(),
        };

        let fee = tvl_maker
            .suite
            .manager()
            .calculate_transaction_fee(FeeParams {
                n_inputs: unsigned_tx.input.len() as u64,
                n_outputs: unsigned_tx.output.len() as u64,
                fee_rate: get_fee_rate(),
            });

        let change =
            Amount::from_sat(total_utxos_amount) - Amount::from_sat(total_output_value) - fee;

        if change > Amount::ZERO {
            unsigned_tx.output.push(TxOut {
                value: change,
                script_pubkey: first_account.1.script_pubkey(),
            });
        }

        let mut psbt = Psbt::from_unsigned_tx(unsigned_tx)
            .map_err(|e| anyhow::anyhow!("Failed to create psbt: {}", e))?;

        for (i, utxo) in utxos.iter().enumerate() {
            let xonly_pubkey = first_account.0.public_key(secp).inner.x_only_public_key().0;

            psbt.inputs[i] = Input {
                witness_utxo: Some(TxOut {
                    value: utxo.amount,
                    script_pubkey: first_account.1.script_pubkey(),
                }),

                // TODO: fix this, taproot address: leaf hash, no key origin
                // TODO: fix this, segwit address: no leaf hash, key origin
                tap_internal_key: match first_account.1.address_type() {
                    Some(AddressType::P2tr) => Some(xonly_pubkey),
                    _ => None,
                },
                tap_key_origins: {
                    let mut map = BTreeMap::new();
                    // Note: no need leaf hash when staking
                    map.insert(
                        xonly_pubkey,
                        (vec![], ([0u8; 4].into(), DerivationPath::default())),
                    );
                    map
                },
                ..Default::default()
            }
        }

        <VaultManager as Signing>::sign_psbt_by_single_key(
            &mut psbt,
            &first_account.0.to_bytes(),
            tvl_maker.suite.network_id(),
            true,
        )
        .map_err(|e| anyhow::anyhow!("Failed to sign PSBT: {}", e))?;

        let result = TestSuite::send_psbt(&tvl_maker.suite.rpc, psbt)
            .ok_or(anyhow::anyhow!("Failed to send PSBT"))?;

        println!("====== FAUCET BTC ======");
        log_tx_result(&result);

        for i in 1..self.count + 1 {
            let (private_key, address) = get_account(base_path, i as u32, *xprv, secp, network);

            let bridge_params = self.create_bridge_params(&address, &private_key, config);

            let utxo = NeededUtxo {
                txid: result.txid,
                vout: (i - 1) as u32, // because we run from 1
                amount: Amount::from_sat(self.amount),
            };

            let command_history = BridgeExecutor::execute_bridge(
                tvl_maker,
                command_name,
                &bridge_params,
                tree_type,
                utxo,
            )?;

            command_histories.push(command_history);
        }

        Ok(command_histories)
    }

    fn create_bridge_params(
        &self,
        address: &Address,
        private_key: &PrivateKey,
        config: &Config,
    ) -> BridgeCmdParams {
        BridgeCmdParams {
            amount: self.amount - 259, // 259 is the fee for the bridge
            wallet_address: address.to_string(),
            private_key: private_key.to_wif(),
            destination_chain: config.destination_chain.clone(),
            destination_token_address: config.destination_token_address.clone(),
            destination_recipient_address: config.destination_recipient_address.clone(),
        }
    }

    fn create_test_suite(&self, config: &Config, params: &BatchBridgeParams) -> TestSuite {
        TestSuite::new(
            params.service_tag.as_str(),
            Env {
                btc_node_address: config.btc_node_address.clone(),
                btc_node_user: config.btc_node_user.clone(),
                btc_node_password: config.btc_node_password.clone(),
                btc_node_wallet: config.btc_node_wallet.clone(),
                protocol_private_key: config.protocol_private_key.clone(),
                custodian_private_keys: config
                    .custodian_private_keys
                    .split(",")
                    .map(|s| s.to_string())
                    .collect(),
                custodian_quorum: config.custodian_quorum as u8,
                network: config.network.clone(),
                tag: config.tag.clone(),
                version: config.version as u8,
            },
            None,
        )
    }
}

fn get_account(
    base_derivation_path: &DerivationPath,
    index: u32,
    xprv: Xpriv,
    secp: &Secp256k1<All>,
    network: Network,
) -> (PrivateKey, Address) {
    let derivation_path =
        base_derivation_path.child(ChildNumber::from_normal_idx(index as u32).unwrap());

    let child_xprv = xprv.derive_priv(secp, &derivation_path).unwrap();

    let private_key: PrivateKey = PrivateKey::new(child_xprv.private_key, network);

    let public_key = CompressedPublicKey::from_private_key(secp, &private_key).unwrap();

    let address = Address::p2wpkh(&public_key, network);

    (private_key, address)
}

async fn get_utxo_by_address(
    client: &MempoolClient,
    address: &Address,
    amount: u64,
) -> BridgeResult<(Vec<NeededUtxo>, u64)> {
    let utxos = client
        .get_address_utxo(address.to_string().as_str())
        .await?;

    let mut total = 0;

    let mut filter_utxos: Vec<NeededUtxo> = Vec::new();

    for utxo in utxos {
        if !utxo.status.confirmed {
            continue;
        }

        if total > amount {
            filter_utxos.push(NeededUtxo {
                txid: Txid::from_str(&utxo.txid).unwrap(),
                vout: utxo.vout,
                amount: Amount::from_sat(utxo.value),
            }); // this utxo ensures fee is covered
            total += utxo.value;
            break;
        }

        total += utxo.value;

        filter_utxos.push(NeededUtxo {
            txid: Txid::from_str(&utxo.txid).unwrap(),
            vout: utxo.vout,
            amount: Amount::from_sat(utxo.value),
        });
    }

    if total < amount {
        return Err(anyhow::anyhow!("Not enough utxos"));
    }

    Ok((filter_utxos, total))
}
