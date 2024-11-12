// import * as bitcoin from "bitcoinjs-lib";
// import { Psbt } from "bitcoinjs-lib";
// import { sleep } from "bun";
// import { describe, it } from "bun:test";

// import { setUpTest, StaticEnv } from "./util";

// const TIMEOUT = 900_000;

// // txid=<txid> bun test unstaking-dry
// // Eg:
// // txid=10f5d2f7167428cfd983bfbaad566adce246f98d3a0ca8ab590844bcab9b2c81 bun test test/unstaking-dry.test.ts

// describe("Vault-Unstaking", () => {
//   it(
//     "should unstake for user",
//     async () => {
//       const TestSuite = await setUpTest();
//       //loop until the tx is confirmed
//       const txid = process.env.txid;

//       if (!txid) {
//         throw new Error("txid is required");
//       }

//       console.log("txid", txid);

//       let tx = null;

//       while (true) {
//         console.log(new Date().toISOString(), "waiting for tx to be confirmed");
//         tx = await TestSuite.btcClient.command("getrawtransaction", txid, true);
//         console.log("tx.confirmations", tx.confirmations);
//         if (tx.confirmations > 0) {
//           console.log("tx", tx);
//           break;
//         }
//         await sleep(5000);
//       }

//       let scriptPubkeyOfLocking = Buffer.from(
//         tx.vout[0].scriptPubKey.hex,
//         "hex"
//       );
//       if (!scriptPubkeyOfLocking) {
//         throw new Error("scriptPubkeyOfLocking is undefined");
//       }

//       console.log("scriptPubkeyOfLocking", bytesToHex(scriptPubkeyOfLocking));
//       console.log("txid", txid);

//       const p2wpkhScript = bitcoin.payments.p2wpkh({
//         pubkey: TestSuite.stakerPubKey,
//       }).output;

//       if (!p2wpkhScript) {
//         throw new Error("p2wpkhScript is undefined");
//       }
//       console.log("p2wpkhScript", bytesToHex(p2wpkhScript));

//       // Build the unsigned psbt
//       const psbtHex = buildUnsignedUnstakingUserProtocolPsbt(
//         StaticEnv.TAG,
//         StaticEnv.VERSION,
//         {
//           txid,
//           vout: 0,
//           value: StaticEnv.STAKING_AMOUNT, // 10_000
//           script_pubkey: scriptPubkeyOfLocking,
//         },
//         {
//           script: p2wpkhScript,
//           value: StaticEnv.STAKING_AMOUNT - BigInt(1_000), // 9_000
//         },
//         TestSuite.stakerPubKey,
//         TestSuite.protocolPubkey,
//         TestSuite.custodialPubkeys,
//         StaticEnv.CUSTODIAL_QUORUM,
//         StaticEnv.HAVE_ONLY_CUSTODIAL
//       );

//       const psbtStr = bytesToHex(psbtHex);

//       console.log("psbtStr", psbtStr);

//       const psbtFromHex = Psbt.fromBuffer(hexToBytes(psbtStr));

//       // User signs the psbt
//       const stakerSignedPsbt = signPsbt(
//         TestSuite.network,
//         TestSuite.stakerWif,
//         psbtFromHex,
//         false
//       );

//       // Protocol signs the psbt
//       const serviceSignedPsbt = signPsbt(
//         TestSuite.network,
//         TestSuite.protocolKeyPair.toWIF(),
//         stakerSignedPsbt.signedPsbt,
//         true
//       );

//       const hexTxfromPsbt = serviceSignedPsbt.signedPsbt
//         .extractTransaction()
//         .toHex();

//       console.log("==== hexTxfromPsbt ====");
//       console.log(hexTxfromPsbt);

//       console.log("==== sendrawtransaction ====");
//       // Broadcast the transaction
//       const unstakedTxid = await sendrawtransaction(
//         hexTxfromPsbt,
//         TestSuite.btcClient
//       );
//       console.log("unstakedTxid", unstakedTxid);
//     },
//     TIMEOUT
//   );
// });
