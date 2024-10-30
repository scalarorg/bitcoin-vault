import { bytesToHex, hexToBytes, logToJSON, signPsbt } from "@/utils";
import { Psbt } from "bitcoinjs-lib";
import { describe, it } from "bun:test";
import { sendrawtransaction, testmempoolaccept } from "../src";
import { setUpTest } from "./util";

const TIMEOUT = 900_000;

describe("Vault-Signing", () => {
  it(
    "should sign psbt",
    async () => {
      const TestSuite = await setUpTest();
      const psbtHex =
        "70736274ff0100520200000001e0a68346c9118f584c22c9afa89b641e06127d1b1fa661788ea922261dee37600000000000fdffffff012823000000000000160014acd07b22adf2299c56909c9ca537fd2c58127ecc000000000001012b102700000000000022512054bfa5690019d09073d75d1094d6eb9a551a5d61b0fcfc1fd474da6bfea88627010304000000004215c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac007e94635a4727997d13497f6529f00a9ca291c2e6e10253eb995eecd130a9eeb4520f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256fad20992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbbacc02116992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbb25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a000000002116f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0011820867e83e93516ecde27680f5af69af0bd633f9918874b975c7e65c0b2419047ee0000";

      const psbtFromHex = Psbt.fromBuffer(hexToBytes(psbtHex));

      console.log("\n==== tapScriptSig before signPsbt ====\n");
      console.log(psbtFromHex.data.inputs[0].tapScriptSig);
      console.log("\n==== tapBip32Derivation ====\n");
      console.log(
        "derivation: ",
        psbtFromHex.data.inputs[0].tapBip32Derivation
      );
      console.log(
        "pubkey[1]: ",
        bytesToHex(psbtFromHex.data.inputs[0].tapBip32Derivation![1].pubkey)
      );
      console.log(
        "pubkey[1]: ",
        psbtFromHex.data.inputs[0].tapBip32Derivation![1].pubkey
      );

      const stakerSignedPsbt = signPsbt(
        TestSuite.network,
        TestSuite.stakerWif,
        psbtFromHex,
        false
      );

      // input: {(XOnlyPublicKey(6f254d00314876efd3adf5bef6267d8e193c7c2a2af199d93eaf0d25960d2ef0280025f800ee144fb83286c8fd545a4248134c99d74484e577f7b5e3b855eb59), 9e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a): Signature { signature: Signature(b21c79a3f1196e8d8d309eff56b4ca2f39cb2957c0a540f66aed88d1ca33bdcaea2434cc02c71c30bb2ceaa629dcdf2fd2b6a5efef019cd07bde292edeb2230d), sighash_type: Default }}

      // pubkey:  f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f
      // signature:  b21c79a3f1196e8d8d309eff56b4ca2f39cb2957c0a540f66aed88d1ca33bdcaea2434cc02c71c30bb2ceaa629dcdf2fd2b6a5efef019cd07bde292edeb2230d
      // leafHash:  9e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a

      console.log("\n==== tapScriptSig after signPsbt ====\n");
      for (const item of stakerSignedPsbt.signedPsbt.data.inputs[0]
        ?.tapScriptSig ?? []) {
        console.log("pubkey: ", bytesToHex(item.pubkey));
        console.log("signature: ", bytesToHex(item.signature));
        console.log("leafHash: ", bytesToHex(item.leafHash));
      }

      console.log("\n==== stakerSignedPsbt ====\n");
      console.log(stakerSignedPsbt.signedPsbt.toHex());

      const serviceSignedPsbt = signPsbt(
        TestSuite.network,
        TestSuite.protocolKeyPair.toWIF(),
        stakerSignedPsbt.signedPsbt,
        true
      );

      const hexTxfromPsbt = serviceSignedPsbt.signedPsbt
        .extractTransaction()
        .toHex();

      console.log("==== hexTxfromPsbt ====");
      console.log(hexTxfromPsbt);

      console.log("==== testmempoolaccept ====");
      const unstakedTxid = await testmempoolaccept(
        hexTxfromPsbt,
        TestSuite.btcClient
      );
      console.log("unstakedTxid", unstakedTxid);
    },
    TIMEOUT
  );
});
