import ECPairFactory from "ecpair";
import * as bitcoin from "bitcoinjs-lib";
import * as ecc from "tiny-secp256k1";
import { AddressType, InputByAddress } from "@/types";
import {
  DEFAULT_INPUT_SIZE,
  P2TR_INPUT_SIZE,
  P2WPKH_INPUT_SIZE,
} from "./constants";

bitcoin.initEccLib(ecc);
export const ECPair = ECPairFactory(ecc);

const BASE_BYTES = 10.5;
const INPUT_BYTES_BASE = 57.5;
const OUTPUT_BYTES_BASE = 43;

export const extractPublicKeyFromWIF = function (
  WIF: string,
  network?: bitcoin.Network
): string {
  const keyPair = ECPair.fromWIF(WIF, network);
  return keyPair.publicKey.toString();
};

//https://github.com/bitcoinjs/bitcoinjs-lib/blob/master/ts_src/address.ts#L211
const prepareTaprootInput = (
  publicKey: string,
  network: bitcoin.Network
): InputByAddress => {
  const publicKeyNoCoord = Buffer.from(publicKey, "hex").subarray(1, 33);
  const outputScript = bitcoin.payments.p2tr({
    pubkey: publicKeyNoCoord,
    network,
  }).output;
  return {
    addressType: AddressType.P2TR,
    outputScript: new Uint8Array(outputScript!),
    outputScriptSize: P2TR_INPUT_SIZE,
    tapInternalKey: Buffer.from(publicKey, "hex").subarray(1, 33),
  };
};

//Default address type is P2WPKH
export const defineAddressType = (
  address: string,
  network: bitcoin.Network
): AddressType => {
  try {
    let decodeBase58 = bitcoin.address.fromBase58Check(address);
    if (decodeBase58.version === network.pubKeyHash) {
      return AddressType.P2PKH;
    } else if (decodeBase58.version === network.scriptHash) {
      return AddressType.P2SH;
    }
  } catch (e) {
    try {
      let decodeBech32 = bitcoin.address.fromBech32(address);
      if (decodeBech32.version === 0) {
        if (decodeBech32.data.length === 20) {
          return AddressType.P2WPKH;
        } else if (decodeBech32.data.length === 32) {
          return AddressType.P2WSH;
        }
      } else if (decodeBech32.version === 1) {
        if (decodeBech32.data.length === 32) {
          return AddressType.P2TR;
        }
      }
    } catch (e) {}
  }
  return AddressType.P2WPKH;
};
export const prepareExtraInputByAddress = (
  address: string,
  publicKey: Uint8Array,
  network: bitcoin.Network
): InputByAddress => {
  let decodeBase58;
  let decodeBech32;
  try {
    decodeBase58 = bitcoin.address.fromBase58Check(address);
  } catch (e) {
    try {
      decodeBech32 = bitcoin.address.fromBech32(address);
    } catch (e) {}
  }
  let addressType = defineAddressType(address, network);
  let outputScript;
  let outputScriptSize = DEFAULT_INPUT_SIZE;
  //Address type

  if (decodeBase58 && addressType === AddressType.P2PKH) {
    // This code get from Ba Hoang
    // Todo: check if this is correct
    const addressDecodedSub = Buffer.from(decodeBase58.hash).toString("hex");
    outputScript = new Uint8Array(
      Buffer.from(`76a914${addressDecodedSub}88ac`, "hex")
    );
  } else {
    outputScript = bitcoin.address.toOutputScript(address, network);
  }

  //For P2TR address, we need to get tapInternalKey
  let tapInternalKey;
  if (addressType === AddressType.P2TR) {
    // xonly public key
    tapInternalKey = publicKey.subarray(1, 33);
    outputScriptSize = P2TR_INPUT_SIZE;
  }

  let redeemScript;
  if (addressType == AddressType.P2WSH) {
    redeemScript = bitcoin.payments.p2wpkh({
      pubkey: publicKey,
    }).output;
    outputScriptSize = P2WPKH_INPUT_SIZE;
  }
  return {
    addressType,
    outputScript,
    outputScriptSize,
    tapInternalKey,
    redeemScript,
  };
};

export const signPsbt = (
  network: bitcoin.Network,
  privkey: string,
  unsignedPsbt: bitcoin.Psbt,
  finalize: boolean = true
): { signedPsbt: bitcoin.Psbt; isValid: boolean } => {
  const keyPair = ECPair.fromWIF(privkey, network);
  const signedPsbt = unsignedPsbt.signAllInputs(keyPair);
  
  if (finalize) {
    signedPsbt.finalizeAllInputs();
  }
  return {
    signedPsbt,
    isValid: true,
  };
};

export const psbtValidator = (
  pubkey: Uint8Array,
  msghash: Uint8Array,
  signature: Uint8Array
): boolean => {
  console.log("pubkey", pubkey);
  console.log("msghash", msghash);
  console.log("signature", signature);
  return true;
};

export function logToJSON(any: any) {
  console.log(
    JSON.stringify(
      any,
      (k, v) => {
        if (v.type === "Buffer") {
          return Buffer.from(v.data).toString("hex");
        }
        if (k === "network") {
          switch (v) {
            case bitcoin.networks.bitcoin:
              return "bitcoin";
            case bitcoin.networks.testnet:
              return "testnet";
            case bitcoin.networks.regtest:
              return "regtest";
          }
        }
        if (typeof v == "bigint") {
          return v.toString(10);
        }
        return v;
      },
      2
    )
  );
}
