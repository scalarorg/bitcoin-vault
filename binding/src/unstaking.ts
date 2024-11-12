// import { TxOutput } from "bitcoinjs-lib";
// import { PreviousStakingUTXO } from "./types1";
// import { createVaultWasm } from "./utils";

// export const buildUnsignedUnstakingUserProtocolPsbt = (
//   tag: string,
//   version: number,
//   input: PreviousStakingUTXO,
//   output: TxOutput,
//   stakerPubkey: Uint8Array,
//   protocolPubkey: Uint8Array,
//   covenantPubkeys: Uint8Array,
//   covenantQuorum: number,
//   haveOnlyCovenants = false,
//   rbf: boolean = true
// ): Uint8Array => {
//   const vaultWasm = createVaultWasm(tag, version);
//   return vaultWasm.build_user_protocol_spend(
//     input.script_pubkey,
//     Buffer.from(input.txid, "hex"),
//     input.vout,
//     input.value,
//     output.script,
//     output.value,
//     stakerPubkey,
//     protocolPubkey,
//     covenantPubkeys,
//     covenantQuorum,
//     haveOnlyCovenants,
//     rbf
//   );
// };
