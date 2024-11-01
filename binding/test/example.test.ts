// import ECPairFactory from "ecpair";
// import * as ecc from "tiny-secp256k1";
// import * as bitcoin from "bitcoinjs-lib";
// import { describe, it, beforeEach } from "bun:test";
// import { createVaultWasm } from "@/utils";

// describe("Bitcoin-Vault", () => {
//   const ECPair = ECPairFactory(ecc);
//   const stakerKeyPair = ECPair.makeRandom();
//   const protocolKeyPair = ECPair.makeRandom();
//   const customdialKeypairs = [];
//   const tag = "01020304";
//   const version = 1;
//   const privkeys = [];
//   const pubkeys = [];
//   const vaultWasm = createVaultWasm(tag, version);
//   // we will use them during MPC

//   const custodial_number = 4;
//   const custodial_pubkeys = new Uint8Array(33 * custodial_number);
//   // Destination info
//   const dst_chain_id = BigInt(11155111);
//   const dst_user_address = "130C4810D57140e1E62967cBF742CaEaE91b6ecE";
//   const dst_smart_contract_address = "1F98C06D8734D5A9FF0b53e3294626E62e4d232C";
//   beforeEach(() => {
//     for (let i = 0; i < custodial_number; i++) {
//       const custodialKeyPair = ECPair.makeRandom();
//       customdialKeypairs[i] = custodialKeyPair;
//       privkeys[i] = custodialKeyPair.privateKey;
//       pubkeys[i] = custodialKeyPair.publicKey;
//       custodial_pubkeys.set(custodialKeyPair.publicKey, i * 33);
//     }
//   });
//   it("should return exact staker address by decoding", () => {
//     const { address: stakerAddress } = bitcoin.payments.p2pkh({
//       pubkey: stakerKeyPair.publicKey,
//     });
//     console.log("Staker public key:", stakerKeyPair.publicKey);
//     console.log("Staker public key length:", stakerKeyPair.publicKey.length);
//     const utxos = new Uint8Array(0);
//     try {
//       if (!stakerAddress) {
//         throw new Error("Staker address is undefined");
//       }
//       const psbt = vaultWasm.create_unsigned_vault_psbt(
//         new Uint8Array(Buffer.from(stakerAddress)),
//         stakerKeyPair.publicKey,
//         protocolKeyPair.publicKey,
//         custodial_pubkeys,
//         custodial_number,
//         utxos,
//         BigInt(11155111),
//         new Uint8Array(Buffer.from(dst_user_address)),
//         new Uint8Array(Buffer.from(dst_smart_contract_address))
//       );
//       console.log("Output psbt:", psbt);
//     } catch (e) {
//       console.log(e);
//     }
//   });
// });
