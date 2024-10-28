import { describe, it, expect } from "bun:test";
import { setupStakingTx, StaticEnv } from "./util";
import { bytesToHex, ECPair, hexToBytes, logToJSON, signPsbt } from "@/utils";
import {
  buildUnsignedUnstakingUserProtocolPsbt,
  sendrawtransaction,
} from "../src";
import { Psbt } from "bitcoinjs-lib";
import { sleep } from "bun";

describe("Vault-Unstaking", () => {
  it("should unstake for user", async () => {
    const { txid, TestSuite, scriptPubkeyOfLocking } = await setupStakingTx();
    //loop until the tx is confirmed
    while (true) {
      console.log(new Date().toISOString(), "waiting for tx to be confirmed");
      await sleep(15000);
      const tx = await TestSuite.btcClient.command("getrawtransaction", txid, true);
      console.log("tx.confirmations", tx.confirmations);  
      if (tx.confirmations > 1) {
        break;
      }
    }
    console.log("scriptPubkeyOfLocking", bytesToHex(scriptPubkeyOfLocking));
    console.log("txid", txid);

    const psbtHex = buildUnsignedUnstakingUserProtocolPsbt(
      StaticEnv.TAG,
      StaticEnv.VERSION,
      {
        txid,
        vout: 0,
        value: StaticEnv.STAKING_AMOUNT,
        script_pubkey: scriptPubkeyOfLocking,
      },
      {
        script: hexToBytes("00141302a4ea98285baefb2d290de541d069356d88e9"),
        value: BigInt(10000100) - BigInt(1000),
      },
      TestSuite.stakerPubKey,
      TestSuite.protocolPubkey,
      TestSuite.custodialPubkeys,
      StaticEnv.CUSTODIAL_QUORUM,
      StaticEnv.HAVE_ONLY_CUSTODIAL
    );

    const psbtStr = bytesToHex(psbtHex);

    const psbtFromHex = Psbt.fromBuffer(hexToBytes(psbtStr));

    console.log("========= sign by staker ==========");

    const stakerSignedPsbt = signPsbt(
      TestSuite.network,
      TestSuite.stakerWif,
      psbtFromHex,
      false
    );
    console.log("stakerSignedPsbt inputs", stakerSignedPsbt.signedPsbt.data.inputs);
    console.log("stakerSignedPsbt", stakerSignedPsbt.signedPsbt.toHex());

    // Step 2: this Psbt will be sent to bla bla ... then received by relayer of service dApp
    // the service dApp will sign the psbt, finalize it and send to bitcoin network
    // simulate service sign the psbt
    console.log("sign by protocol private key", TestSuite.protocolKeyPair.toWIF());
    const serviceSignedPsbt = signPsbt(
      TestSuite.network,
      TestSuite.protocolKeyPair.toWIF(),
      stakerSignedPsbt.signedPsbt,
      true  
    );
    console.log("serviceSignedPsbt tx", serviceSignedPsbt.signedPsbt.extractTransaction());
    const hexTxfromPsbt = serviceSignedPsbt.signedPsbt
      .extractTransaction()
      .toHex();

    console.log("hexTxfromPsbt", hexTxfromPsbt);

    const unstakedTxid = await sendrawtransaction(
      hexTxfromPsbt,
      TestSuite.btcClient
    );

    console.log("unstakedTxid", unstakedTxid);
  });
});
