import Client from "bitcoin-core-ts";
import { sleep } from "bun";
import { beforeAll, describe, test } from "bun:test";
import {
  bytesToHex,
  ChainType,
  DestinationChain,
  getAddressUtxos,
  hexToBytes,
  sendrawtransaction,
  TBuildUPCUnstakingPsbt,
} from "../src";
import { logToJSON, setUpTest, StaticEnv, TTestSuite } from "./util";
import * as bitcoin from "bitcoinjs-lib";
import { Psbt } from "bitcoinjs-lib";
import { AddressTxsUtxo } from "@mempool/mempool.js/lib/interfaces/bitcoin/addresses";

const TIMEOUT = 900_000;
const VOUT_INDEX_OF_LOCKING_OUTPUT = 1;

describe("UPC E2E", () => {
  let TestSuite: TTestSuite | null;
  beforeAll(async () => {
    TestSuite = await setUpTest();
  });

  test(
    "e2e-upc",
    async () => {
      if (!TestSuite) {
        throw new Error("TestSuite is not set");
      }

      const mockBtcClient = new Client({
        network: "testnet",
        host: "testnet4.btc.scalar.org",
        port: 80,
        username: "scalar",
        password: "scalartestnet4",
      });

      const txs = await prepareStakingTxs(TestSuite, mockBtcClient, 2);

      console.log({ txs });

      if (txs.length !== 2) {
        throw new Error("Failed to prepare staking txs");
      }

      let scriptPubkeyOfLocking = Buffer.from(
        txs[0].vout[VOUT_INDEX_OF_LOCKING_OUTPUT].scriptPubKey.hex,
        "hex"
      );

      const params: TBuildUPCUnstakingPsbt = {
        inputs: txs.map((tx) => ({
          txid: tx.txid,
          vout: VOUT_INDEX_OF_LOCKING_OUTPUT,
          value: BigInt(
            Math.floor(tx.vout[VOUT_INDEX_OF_LOCKING_OUTPUT].value * 1e8)
          ),
          script_pubkey: scriptPubkeyOfLocking,
        })),
        output: {
          script: bitcoin.address.toOutputScript(
            TestSuite.stakerAddress,
            TestSuite.network
          ),
          value: (() => {
            const amount = Math.floor(Number(StaticEnv.STAKING_AMOUNT) * 1.5);
            return BigInt(amount);
          })(),
        },
        stakerPubkey: TestSuite.stakerPubKey,
        protocolPubkey: TestSuite.protocolPubkey,
        custodianPubkeys: TestSuite.custodialPubkeys,
        custodianQuorum: StaticEnv.CUSTODIAL_QUORUM,
        feeRate: BigInt(1),
        rbf: true,
        type: "user_custodian",
      };

      // // Build the unsigned psbt
      const psbtHex = TestSuite.vaultUtils.buildUPCUnstakingPsbt(params);

      const psbtStr = bytesToHex(psbtHex);

      const psbtFromHex = Psbt.fromBuffer(hexToBytes(psbtStr));

      // staker signs the psbt
      const stakerSignedPsbt = TestSuite.vaultUtils.signPsbt({
        psbt: psbtFromHex,
        wif: TestSuite.stakerWif,
        finalize: false,
      });

      const psbtBase64 = stakerSignedPsbt.toBase64();
      console.log("psbtBase64", psbtBase64);
      console.log("===============");
      console.log("userSignedPsbt", stakerSignedPsbt.toHex());

      let custodianSignedPsbt = stakerSignedPsbt;

      for (let i = 0; i < TestSuite.custodianPrivateKeys.length; i++) {
        custodianSignedPsbt = TestSuite.vaultUtils.signPsbt({
          psbt: stakerSignedPsbt,
          wif: TestSuite.custodianPrivateKeys[i],
          finalize: i === TestSuite.custodianPrivateKeys.length - 1,
        });
      }

      console.log("===============");
      console.log("psbtServicesHex", custodianSignedPsbt.toHex());

      const hexTxfromPsbt = custodianSignedPsbt.extractTransaction().toHex();

      console.log("===============");
      console.log("hexTxfromPsbt", hexTxfromPsbt);

      const unstakedTxid = await sendrawtransaction(
        hexTxfromPsbt,
        mockBtcClient
      );
      console.log("unstakedTxid", unstakedTxid);
    },
    TIMEOUT
  );
});

async function prepareStakingTxs(
  TestSuite: TTestSuite,
  client: Client,
  count: number
) {
  const txs = [];
  let neededUtxos = [];

  const utxos = await getAddressUtxos({
    address: TestSuite.stakerAddress,
    mempoolClient: TestSuite.mempoolClient,
  });

  let cursor = 0;

  for (let i = 0; i < count; i++) {
    console.log("\n The cursor is at: ", cursor);

    while (cursor < utxos.length) {
      if (utxos[cursor].value >= StaticEnv.STAKING_AMOUNT) {
        neededUtxos.push(utxos[cursor]);
        cursor++; // Increment cursor after finding a valid UTXO
        break;
      }
      cursor++;
    }

    if (neededUtxos.length <= i) {
      throw new Error("Not enough utxos");
    }

    console.log("Using UTXO:", neededUtxos[i]);
    const tx = await prepareStakingTx(TestSuite, [neededUtxos[i]]);
    txs.push(tx);
  }

  for (let i = 0; i < count; i++) {
    console.log("TX: ", txs[i].txid);
  }

  return txs;
}

async function prepareStakingTx(
  TestSuite: TTestSuite,
  utxos: AddressTxsUtxo[]
) {
  const { fastestFee: feeRate } =
    await TestSuite.mempoolClient.fees.getFeesRecommended(); // Get this from Mempool API

  const { psbt: unsignedVaultPsbt, fee: estimatedFee } =
    TestSuite.vaultUtils.buildUPCStakingPsbt({
      stakingAmount: StaticEnv.STAKING_AMOUNT,
      stakerPubkey: TestSuite.stakerPubKey,
      stakerAddress: TestSuite.stakerAddress,
      protocolPubkey: TestSuite.protocolPubkey,
      custodianPubkeys: TestSuite.custodialPubkeys,
      custodianQuorum: StaticEnv.CUSTODIAL_QUORUM,
      destinationChain: new DestinationChain(
        ChainType.EVM,
        StaticEnv.DEST_CHAIN_ID
      ),
      destinationContractAddress: hexToBytes(StaticEnv.DEST_TOKEN_ADDRESS),
      destinationRecipientAddress: hexToBytes(StaticEnv.DEST_USER_ADDRESS),
      availableUTXOs: utxos,
      feeRate,
      rbf: true,
    });

  console.log({ estimatedFee });

  const signedPsbt = TestSuite.vaultUtils.signPsbt({
    psbt: unsignedVaultPsbt,
    wif: TestSuite.stakerWif,
    finalize: true,
  });

  let transaction = signedPsbt.extractTransaction(false);
  const txHexfromPsbt = transaction.toHex();
  logToJSON({ txHexfromPsbt, fee: estimatedFee });

  console.log("txHexfromPsbt", txHexfromPsbt);
  const txid = await sendrawtransaction(txHexfromPsbt, TestSuite.btcClient);
  console.log("Successfully broadcasted txid", txid);

  while (true) {
    try {
      const tx = await TestSuite.btcClient.command(
        "getrawtransaction",
        txid,
        true
      );

      if (Boolean(tx)) {
        return tx;
      }
      await sleep(5000);
    } catch (e) {
      console.log("error", e);
    }
  }
}
