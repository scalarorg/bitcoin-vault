import { bytesToHex, hexToBytes, signPsbt } from "@/utils";
import Client from "bitcoin-core-ts";
import * as bitcoin from "bitcoinjs-lib";
import { Psbt } from "bitcoinjs-lib";
import { sleep } from "bun";
import { describe, it } from "bun:test";
import { StaticEnv } from "./util";
import {
  buildUnsignedUnstakingUserProtocolPsbt,
  sendrawtransaction,
} from "../src";

const TIMEOUT = 900_000;

describe("Vault-Unstaking", () => {
  it(
    "should unstake for user",
    async () => {
      const txid =
        "10f5d2f7167428cfd983bfbaad566adce246f98d3a0ca8ab590844bcab9b2c81";

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

      let scriptPubkeyOfLocking = Buffer.from(
        tx.vout[0].scriptPubKey.hex,
        "hex"
      );
      if (!scriptPubkeyOfLocking) {
        throw new Error("scriptPubkeyOfLocking is undefined");
      }

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

      // Build the unsigned psbt
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
          value: BigInt(900), // 9_000
        },
        stakerPubKey,
        protocolPubkey,
        custodialPubkeysBuffer,
        StaticEnv.CUSTODIAL_QUORUM,
        StaticEnv.HAVE_ONLY_CUSTODIAL
      );

      const psbtStr = bytesToHex(psbtHex);

      const psbtFromHex = Psbt.fromBuffer(hexToBytes(psbtStr));


      // staker signs the psbt
      const stakerSignedPsbt = signPsbt(
        bitcoin.networks.testnet,
        "cQ7kMt56n8GeKkshaiCt3Lh2ChuaD3tWdSrH37MwU93PA4qZs9JR",
        psbtFromHex,
        false
      );

      const psbtBase64 = stakerSignedPsbt.signedPsbt.toBase64();
      const userSignedPsbt = Psbt.fromHex(stakerSignedPsbt.signedPsbt.toHex());

      console.log("psbtBase64", psbtBase64);
      console.log("===============");
      console.log("userSignedPsbt", userSignedPsbt.toHex());


      return;

      const serviceSignedPsbt = signPsbt(
        bitcoin.networks.testnet,
        "cVpL6mBRYV3Dmkx87wfbtZ4R3FTD6g58VkTt1ERkqGTMzTcDVw5M",
        stakerSignedPsbt.signedPsbt,
        true
      );

      const psbtServicesHex = bytesToHex(
        serviceSignedPsbt.signedPsbt.toBuffer()
      );

      console.log("===============");
      console.log("psbtServicesHex", psbtServicesHex);

      const hexTxfromPsbt = serviceSignedPsbt.signedPsbt
        .extractTransaction()
        .toHex();

      console.log("===============");
      console.log("hexTxfromPsbt", hexTxfromPsbt);

      const fakedTxhex =
        "02000000000101812c9babbc440859aba80c3a8df946e2dc6a56adbabf83d9cf287416f7d2f5100000000000fdffffff01840300000000000016001450dceca158a9c872eb405d52293d351110572c9e0440af5e1fdfd21e81ba69a814a363cf47a2f467c109f007255c39df218f547328b8ca34db4528ca3f467d593df214be9f316964fec9d7d58856e6302b94dc84799f409eb6f67acb5035e1ab63f0cddd377c186a16f1695eee2013c3219750f729bfbdf37c8b69d8609f5133a1f593421f5bb4c40d4214e3b45f7ba02e38ac5f902a1444202ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350aad201387aab21303782b17e760c670432559df3968e52cb82cc2d8f9be43a227d5dcac41c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0b03e4f11ba594a5e348a85f4c2d16f3b9b19be3eeff494c12c3050153585255000000000";

      const unstakedTxid = await sendrawtransaction(fakedTxhex, mockBtcClient);
      console.log("unstakedTxid", unstakedTxid);
    },
    TIMEOUT
  );
});
