import {
  bytesToHex,
  getEstimatedSize,
  getEstimatedTxFee,
  hexToBytes,
  signPsbt,
} from "@/utils";
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
        "f442ecc260c9ca52b5fc0efea3296accfed1da8640e5c828e3db677b712b2881";

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

      const input = {
        txid,
        vout: 0,
        value: StaticEnv.STAKING_AMOUNT,
        script_pubkey: scriptPubkeyOfLocking,
      };

      const output = {
        script: p2wpkhScript,
        value: StaticEnv.STAKING_AMOUNT,
      };

      const estimatedSize = getEstimatedSize(
        bitcoin.networks.testnet,
        [
          {
            ...input,
            value: Number(input.value),
          },
        ],
        p2wpkhScript.length,
        [
          {
            ...output,
            script: Buffer.from(output.script),
          },
        ]
      );

      console.log("estimatedSize", estimatedSize);

      const estimatedFee = getEstimatedTxFee(estimatedSize, 1);

      output.value = BigInt(Number(output.value) - estimatedFee);

      console.log("Staking Amount", StaticEnv.STAKING_AMOUNT);
      console.log("estimatedFee", estimatedFee);
      console.log("output.value", output.value);

      // Build the unsigned psbt
      const psbtHex = buildUnsignedUnstakingUserProtocolPsbt(
        StaticEnv.TAG,
        StaticEnv.VERSION,
        input,
        output,
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

      const unstakedTxid = await sendrawtransaction(
        hexTxfromPsbt,
        mockBtcClient
      );
      console.log("unstakedTxid", unstakedTxid);
    },
    TIMEOUT
  );
});
