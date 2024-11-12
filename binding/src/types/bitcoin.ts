export type TNetwork = "bitcoin" | "testnet" | "testnet4" | "regtest";

export enum AddressType {
  P2PKH, //decodeBase58, version = pubKeyHash
  P2SH, //decodeBase58, version = scriptHash
  P2WPKH, //decodeBech32, version = 0, data.length = 20
  P2WSH, //decodeBech32, version = 0, data.length = 32
  P2TR, //decodeBech32, version = 1, data.length = 32
}

export type InputByAddress = {
  addressType: AddressType;
  outputScript: Uint8Array;
  outputScriptSize: number;
  tapInternalKey?: Uint8Array;
  redeemScript?: Uint8Array;
};

import { PsbtOutput } from "bip174";

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
