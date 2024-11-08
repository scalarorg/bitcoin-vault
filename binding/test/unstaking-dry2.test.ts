import { bytesToHex, hexToBytes, signPsbt } from "@/utils";
import Client from "bitcoin-core-ts";
import * as bitcoin from "bitcoinjs-lib";
import { Psbt } from "bitcoinjs-lib";
import { sleep } from "bun";
import { describe, it } from "bun:test";
import { StaticEnv } from "./util";
import { buildUnsignedUnstakingUserProtocolPsbt } from "../src";

const TIMEOUT = 900_000;

describe("Vault-Unstaking", () => {
  it(
    "should unstake for user",
    async () => {
      const txid =
        "9e612815522243e794cbe79acabea666af32d41a3e4c33c3a10d24b9972362df";

      if (!txid) {
        throw new Error("txid is required");
      }

      console.log("txid", txid);

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
          console.log(
            new Date().toISOString(),
            "waiting for tx to be confirmed"
          );
          tx = await mockBtcClient.command("getrawtransaction", txid, true);
          console.log("tx.confirmations", tx.confirmations);
          if (tx.confirmations > 0) {
            console.log("tx", tx);
            break;
          }
          await sleep(5000);
        } catch (e) {
          console.log("error", e);
        }
      }

      let scriptPubkeyOfLocking = Buffer.from(
        tx.vout[0].scriptPubKey.hex,
        "hex"
      );
      if (!scriptPubkeyOfLocking) {
        throw new Error("scriptPubkeyOfLocking is undefined");
      }

      console.log("scriptPubkeyOfLocking", bytesToHex(scriptPubkeyOfLocking));
      console.log("txid", txid);

      const stakerPubKey = Buffer.from(
        "022ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350a",
        "hex"
      );

      const protocolPubkey = Buffer.from(
        "021387aab21303782b17e760c670432559df3968e52cb82cc2d8f9be43a227d5dc",
        "hex"
      );

      const p2wpkhScript = bitcoin.payments.p2wpkh({
        pubkey: stakerPubKey,
      }).output;

      if (!p2wpkhScript) {
        throw new Error("p2wpkhScript is undefined");
      }
      console.log("p2wpkhScript", bytesToHex(p2wpkhScript));

      const custodialPubkeys = [
        "02a60824d85942a8bdf63daa3cbbc816e3346705ac474c691852511a1c2c8326c0",
        "0271d9443d1937264d465424ff63b2eac0d2d9827f109161198715d84cf2c968ba",
        "022b818929ec7968225ea024b7c7d77cca2f164d99a55874a487d51a84c7754a22",
        "0259fee67d67dfb2015d1554f6fe2ed121ad4dd477e7457ea831ad0193893a8ab5",
        "03e85d4238ca54e23dbcb93d53f25bebd039f393c6f185ce7a301870d6a98374e0",
      ];

      const custodialPubkeysBuffer = new Uint8Array(33 * 5);

      for (let i = 0; i < StaticEnv.CUSTODIAL_NUMBER; i++) {
        custodialPubkeysBuffer.set(hexToBytes(custodialPubkeys[i]), i * 33);
      }

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
        stakerPubKey,
        protocolPubkey,
        custodialPubkeysBuffer,
        StaticEnv.CUSTODIAL_QUORUM,
        StaticEnv.HAVE_ONLY_CUSTODIAL
      );

      const psbtStr = bytesToHex(psbtHex);

      console.log("psbtStr", psbtStr);

      const psbtFromHex = Psbt.fromBuffer(hexToBytes(psbtStr));

      const stakerSignedPsbt = signPsbt(
        bitcoin.networks.testnet,
        "cQ7kMt56n8GeKkshaiCt3Lh2ChuaD3tWdSrH37MwU93PA4qZs9JR",
        psbtFromHex,
        false
      );

      console.log(
        "stakerSignedPsbt.signedPsbt.inputs[0].tap_script_sigs",
        stakerSignedPsbt.signedPsbt.data.inputs[0].tapScriptSig
      );

      const psbtBase64 = stakerSignedPsbt.signedPsbt.toBase64();

      console.log("psbtBase64: \n", psbtBase64);
      console.log("psbtHex: \n", bytesToHex(psbtHex));

      const serviceSignedPsbt = signPsbt(
        bitcoin.networks.testnet,
        "cVpL6mBRYV3Dmkx87wfbtZ4R3FTD6g58VkTt1ERkqGTMzTcDVw5M",
        stakerSignedPsbt.signedPsbt,
        true
      );

      const psbtServicesHex = bytesToHex(
        serviceSignedPsbt.signedPsbt.toBuffer()
      );

      console.log("psbtServicesHex: \n", psbtServicesHex);

      const hexTxfromPsbt = serviceSignedPsbt.signedPsbt
        .extractTransaction()
        .toHex();

      console.log("==== hexTxfromPsbt ====");
      console.log(hexTxfromPsbt);

      //   console.log("==== sendrawtransaction ====");
      //   const unstakedTxid = await sendrawtransaction(
      //     hexTxfromPsbt,
      //     TestSuite.btcClient
      // );

      // 02000000000101df622397b9240da1c3334c3e1ad432af66a6beca9ae7cb94e74322521528619e0000000000fdffffff01282300000000000016001450dceca158a9c872eb405d52293d351110572c9e0440e7757536ce5cf4246485d74cfc14d74821acefd8ccba32c92a1c6852b1219d82320ea9868bb57e552be3bc8fea5f9a214006d4d2e87476df815bce6a18f68d4e400addd97a3e1e1aa7daf64b77e4356d629c33918afea40c9c10227eba3425a258e32a606ec141005a9b4db68f5ebcdc9d2c7165d272269d817b1433a9ed78d2f944202ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350aad201387aab21303782b17e760c670432559df3968e52cb82cc2d8f9be43a227d5dcac41c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0b03e4f11ba594a5e348a85f4c2d16f3b9b19be3eeff494c12c3050153585255000000000

      // 02000000000101df622397b9240da1c3334c3e1ad432af66a6beca9ae7cb94e74322521528619e0000000000fdffffff01282300000000000016001450dceca158a9c872eb405d52293d351110572c9e03400addd97a3e1e1aa7daf64b77e4356d629c33918afea40c9c10227eba3425a258e32a606ec141005a9b4db68f5ebcdc9d2c7165d272269d817b1433a9ed78d2f944202ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350aad201387aab21303782b17e760c670432559df3968e52cb82cc2d8f9be43a227d5dcac41c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0b03e4f11ba594a5e348a85f4c2d16f3b9b19be3eeff494c12c3050153585255000000000
    },
    TIMEOUT
  );
});
