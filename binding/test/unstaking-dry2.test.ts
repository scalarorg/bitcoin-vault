import Client from "bitcoin-core-ts";
import * as bitcoin from "bitcoinjs-lib";
import { Psbt } from "bitcoinjs-lib";
import { sleep } from "bun";
import { describe, it } from "bun:test";
import { setUpTest, StaticEnv } from "./util";
import {
  bytesToHex,
  getEstimatedFee,
  hexToBytes,
  sendrawtransaction,
  TBuildUPCUntakingPsbt,
} from "../src";

const TIMEOUT = 900_000;

describe("Vault-Unstaking", () => {
  it(
    "should unstake for user",
    async () => {
      const txid =
        "fafcafacb758cf4720250fc450519493a8b489198316fd7afeb969d10be21317";

      if (!txid) {
        throw new Error("txid is required");
      }

      let tx = null;

      const mockBtcClient = new Client({
        network: "testnet",
        host: "testnet4.btc.scalar.org",
        port: 80,
        username: "scalar",
        password: "scalartestnet4",
      });

      while (true) {
        try {
          tx = await mockBtcClient.command("getrawtransaction", txid, true);
          if (tx.confirmations > 0) {
            break;
          }
          await sleep(5000);
        } catch (e) {
          console.log("error", e);
        }
      }

      const testSuite = await setUpTest();

      let scriptPubkeyOfLocking = Buffer.from(
        tx.vout[0].scriptPubKey.hex,
        "hex"
      );
      if (!scriptPubkeyOfLocking) {
        throw new Error("scriptPubkeyOfLocking is undefined");
      }

      if (
        bytesToHex(testSuite.stakerPubKey) !==
        "022ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350a"
      ) {
        console.log("testSuite.stakerPubKey");
        throw new Error("stakerPubKey is not correct");
      }

      if (
        bytesToHex(testSuite.protocolPubkey) !==
        "021387aab21303782b17e760c670432559df3968e52cb82cc2d8f9be43a227d5dc"
      ) {
        throw new Error("protocolPubkey is not correct");
      }

      const p2wpkhScript = bitcoin.payments.p2wpkh({
        pubkey: testSuite.stakerPubKey,
      }).output;

      if (!p2wpkhScript) {
        throw new Error("p2wpkhScript is undefined");
      }

      const params: TBuildUPCUntakingPsbt = {
        input: {
          txid,
          vout: 0,
          value: BigInt(Math.floor(tx.vout[0].value * 1e8)),
          script_pubkey: scriptPubkeyOfLocking,
        },
        output: {
          script: bitcoin.address.toOutputScript(
            testSuite.stakerAddress,
            testSuite.network
          ),
          value: BigInt(Math.floor(tx.vout[0].value * 1e8)),
        },
        stakerPubkey: testSuite.stakerPubKey,
        protocolPubkey: testSuite.protocolPubkey,
        custodianPubkeys: testSuite.custodialPubkeys,
        custodianQuorum: StaticEnv.CUSTODIAL_QUORUM,
        feeRate: BigInt(1),
        rbf: true,
        type: "user_protocol",
      };

      // Build the unsigned psbt
      const psbtHex = testSuite.vaultUtils.buildUPCUnstakingPsbt(params);

      const psbtStr = bytesToHex(psbtHex);

      const psbtFromHex = Psbt.fromBuffer(hexToBytes(psbtStr));

      // staker signs the psbt
      const stakerSignedPsbt = testSuite.vaultUtils.signPsbt({
        psbt: psbtFromHex,
        wif: testSuite.stakerWif,
        finalize: false,
      });

      const psbtBase64 = stakerSignedPsbt.toBase64();
      console.log("psbtBase64", psbtBase64);
      console.log("===============");
      console.log("userSignedPsbt", stakerSignedPsbt.toHex());

      const serviceSignedPsbt = testSuite.vaultUtils.signPsbt({
        psbt: stakerSignedPsbt,
        wif: testSuite.protocolKeyPair.toWIF(),
        finalize: true,
      });

      console.log("===============");
      console.log("psbtServicesHex", serviceSignedPsbt.toHex());

      const hexTxfromPsbt = serviceSignedPsbt.extractTransaction().toHex();

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
