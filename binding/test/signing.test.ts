// import { bytesToHex, hexToBytes } from "../src/utils";
// import { Psbt } from "bitcoinjs-lib";
// import { describe, expect, it } from "bun:test";
// import { setUpTest, StaticEnv } from "./util";
// import { isTestnet } from "../src";

// const TIMEOUT = 900_000;

// const UNSIGNED_PSBT_HEX =
//   "70736274ff0100520200000001e0a68346c9118f584c22c9afa89b641e06127d1b1fa661788ea922261dee37600000000000fdffffff012823000000000000160014acd07b22adf2299c56909c9ca537fd2c58127ecc000000000001012b102700000000000022512054bfa5690019d09073d75d1094d6eb9a551a5d61b0fcfc1fd474da6bfea88627010304000000004215c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac007e94635a4727997d13497f6529f00a9ca291c2e6e10253eb995eecd130a9eeb4520f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256fad20992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbbacc02116992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbb25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a000000002116f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0011820867e83e93516ecde27680f5af69af0bd633f9918874b975c7e65c0b2419047ee0000";

// const EXPECTED_STAKER_SIGNED_PSBT_HEX =
//   "70736274ff0100520200000001e0a68346c9118f584c22c9afa89b641e06127d1b1fa661788ea922261dee37600000000000fdffffff012823000000000000160014acd07b22adf2299c56909c9ca537fd2c58127ecc000000000001012b102700000000000022512054bfa5690019d09073d75d1094d6eb9a551a5d61b0fcfc1fd474da6bfea88627010304000000004114f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f9e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a40b21c79a3f1196e8d8d309eff56b4ca2f39cb2957c0a540f66aed88d1ca33bdcaea2434cc02c71c30bb2ceaa629dcdf2fd2b6a5efef019cd07bde292edeb2230d4215c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac007e94635a4727997d13497f6529f00a9ca291c2e6e10253eb995eecd130a9eeb4520f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256fad20992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbbacc02116992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbb25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a000000002116f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0011820867e83e93516ecde27680f5af69af0bd633f9918874b975c7e65c0b2419047ee0000";

// const EXPECTED_TX_HEX =
//   "02000000000101e0a68346c9118f584c22c9afa89b641e06127d1b1fa661788ea922261dee37600000000000fdffffff012823000000000000160014acd07b22adf2299c56909c9ca537fd2c58127ecc04406b665c5660454029a0dd164b076159e1a53f4d891199246329b5fa9d738d2fe9035fb3ea8ea82416b18d6fae118740e8cfbda706dccbaecf14a6bc70a69bda0e40b21c79a3f1196e8d8d309eff56b4ca2f39cb2957c0a540f66aed88d1ca33bdcaea2434cc02c71c30bb2ceaa629dcdf2fd2b6a5efef019cd07bde292edeb2230d4420f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256fad20992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbbac41c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac007e94635a4727997d13497f6529f00a9ca291c2e6e10253eb995eecd130a9eeb00000000";

// describe("Vault-Signing", () => {
//   it(
//     "should sign psbt",
//     async () => {
//       const TestSuite = await setUpTest();
//       console.log(
//         "Staker privkey: ",
//         bytesToHex(TestSuite.stakerKeyPair.privateKey!)
//       );

//       const stakerSignedPsbt = TestSuite.vaultWasm.sign_psbt_by_single_key(
//         hexToBytes(UNSIGNED_PSBT_HEX),
//         TestSuite.stakerKeyPair.privateKey!,
//         isTestnet(StaticEnv.NETWORK),
//         false
//       );

//       const signedPsbt = Psbt.fromBuffer(stakerSignedPsbt);

//       const item = signedPsbt.data.inputs[0]?.tapScriptSig![0];

//       expect(
//         "f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f"
//       ).toBe(bytesToHex(item.pubkey));

//       expect(
//         "b21c79a3f1196e8d8d309eff56b4ca2f39cb2957c0a540f66aed88d1ca33bdcaea2434cc02c71c30bb2ceaa629dcdf2fd2b6a5efef019cd07bde292edeb2230d"
//       ).toBe(bytesToHex(item.signature));

//       expect(
//         "9e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a"
//       ).toBe(bytesToHex(item.leafHash));

//       expect(signedPsbt.toHex()).toBe(EXPECTED_STAKER_SIGNED_PSBT_HEX);

//       const finalizedPsbt = TestSuite.vaultWasm.sign_psbt_by_single_key(
//         stakerSignedPsbt,
//         TestSuite.protocolKeyPair.privateKey!,
//         isTestnet(StaticEnv.NETWORK),
//         true
//       );

//       expect(bytesToHex(finalizedPsbt)).toBe(EXPECTED_TX_HEX);
//     },
//     TIMEOUT
//   );
// });
