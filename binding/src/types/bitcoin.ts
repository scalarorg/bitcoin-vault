import type { PsbtOutput } from "bip174";

export type TNetwork = "bitcoin" | "testnet" | "testnet4" | "regtest";

export enum AddressType {
  P2PKH = 0, //decodeBase58, version = pubKeyHash
  P2SH = 1, //decodeBase58, version = scriptHash
  P2WPKH = 2, //decodeBech32, version = 0, data.length = 20
  P2WSH = 3, //decodeBech32, version = 0, data.length = 32
  P2TR = 4, //decodeBech32, version = 1, data.length = 32
}

export type InputByAddress = {
  addressType: AddressType;
  outputScript: Uint8Array;
  outputScriptSize: number;
  tapInternalKey?: Uint8Array;
  redeemScript?: Uint8Array;
};


export type PsbtOutputExtended =
  | PsbtOutputExtendedAddress
  | PsbtOutputExtendedScript;

interface PsbtOutputExtendedAddress extends PsbtOutput {
  address: string;
  value: bigint;
}

interface PsbtOutputExtendedScript extends PsbtOutput {
  script: Buffer;
  value: bigint;
}

export interface UTXO {
  txid: string;
  vout: number;
  value: number;
}

export const isPsbtOutputExtendedAddress = (
  output: PsbtOutputExtended
): output is PsbtOutputExtendedAddress => {
  return (output as PsbtOutputExtendedAddress).address !== undefined;
};
