import { bytesToHex, hexToBytes, signPsbt } from "@/utils";
import { Psbt } from "bitcoinjs-lib";
import { sleep } from "bun";
import { describe, it } from "bun:test";
import {
  buildUnsignedUnstakingUserProtocolPsbt,
  sendrawtransaction,
} from "../src";
import { setupStakingTx, StaticEnv } from "./util";
import * as bitcoin from "bitcoinjs-lib";

const TIMEOUT = 100_000;

describe("Vault-Unstaking", () => {
  it(
    "should unstake for user",
    async () => {
      const { txid, TestSuite, scriptPubkeyOfLocking } = await setupStakingTx();
      //loop until the tx is confirmed
      while (true) {
        console.log(new Date().toISOString(), "waiting for tx to be confirmed");
        await sleep(5000);
        const tx = await TestSuite.btcClient.command(
          "getrawtransaction",
          txid,
          true
        );
        console.log("tx.confirmations", tx.confirmations);
        if (tx.confirmations > 0) {
          break;
        }
      }
      console.log("scriptPubkeyOfLocking", bytesToHex(scriptPubkeyOfLocking));
      console.log("txid", txid);

      const p2wpkhScript = bitcoin.payments.p2wpkh({
        pubkey: TestSuite.stakerPubKey,
      }).output;

      if (!p2wpkhScript) {
        throw new Error("p2wpkhScript is undefined");
      }
      console.log("p2wpkhScript", bytesToHex(p2wpkhScript));
      const psbtHex = buildUnsignedUnstakingUserProtocolPsbt(
        StaticEnv.TAG,
        StaticEnv.VERSION,
        {
          txid,
          vout: 0,
          value: StaticEnv.STAKING_AMOUNT, // 10_000
          script_pubkey: scriptPubkeyOfLocking,
        },
        {
          script: p2wpkhScript,
          value: StaticEnv.STAKING_AMOUNT - BigInt(1_000), // 9_000
        },
        TestSuite.stakerPubKey,
        TestSuite.protocolPubkey,
        TestSuite.custodialPubkeys,
        StaticEnv.CUSTODIAL_QUORUM,
        StaticEnv.HAVE_ONLY_CUSTODIAL
      );

      const psbtStr = bytesToHex(psbtHex);

      console.log("psbtStr", psbtStr);

      const psbtFromHex = Psbt.fromBuffer(hexToBytes(psbtStr));

      const stakerSignedPsbt = signPsbt(
        TestSuite.network,
        TestSuite.stakerWif,
        psbtFromHex,
        false
      );

      const serviceSignedPsbt = signPsbt(
        TestSuite.network,
        TestSuite.protocolKeyPair.toWIF(),
        stakerSignedPsbt.signedPsbt,
        true
      );
      const hexTxfromPsbt = serviceSignedPsbt.signedPsbt
        .extractTransaction()
        .toHex();

      const unstakedTxid = await sendrawtransaction(
        hexTxfromPsbt,
        TestSuite.btcClient
      );

      console.log("unstakedTxid", unstakedTxid);
    },
    TIMEOUT
  );
});
