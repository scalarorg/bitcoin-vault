
import ECPairFactory from "ecpair";
import * as bitcoin from "bitcoinjs-lib";
import * as ecc from "tiny-secp256k1";
import { InputByAddress } from "@/types";
import { DEFAULT_INPUT_SIZE, P2TR_INPUT_SIZE, P2WPKH_INPUT_SIZE } from "./constants";

bitcoin.initEccLib(ecc);
export const ECPair = ECPairFactory(ecc);

const BASE_BYTES = 10.5;
const INPUT_BYTES_BASE = 57.5;
const OUTPUT_BYTES_BASE = 43;


export const extractPublicKeyFromWIF = function (WIF: string, network?: bitcoin.Network): string {
    const keyPair = ECPair.fromWIF(WIF, network);
    return keyPair.publicKey.toString();
}

//https://github.com/bitcoinjs/bitcoinjs-lib/blob/master/ts_src/address.ts#L211
export const prepareExtraInputByAddress = (address: string, publicKey: string, network: bitcoin.Network): InputByAddress => {
  let decodeBase58;
  let decodeBech32;
  try {
    decodeBase58 = bitcoin.address.fromBase58Check(address);
  } catch (e) {
    try {
      decodeBech32 = bitcoin.address.fromBech32(address);
    } catch (e) {}
  }
  let outputScript;
  let outputScriptSize = DEFAULT_INPUT_SIZE;
  if (decodeBase58 && decodeBase58.version === network.pubKeyHash) {
    // P2PKH
    // This code get from Nguyen Ba Hoang
    // Todo: check if this is correct
    const addressDecodedSub = Buffer.from(decodeBase58.hash).toString("hex");
    outputScript = new Uint8Array(Buffer.from(`76a914${addressDecodedSub}88ac`, "hex"));
  } else {
    outputScript = bitcoin.address.toOutputScript(address, network);
  }
  //For P2TR address, we need to get tapInternalKey
  let tapInternalKey;
  if (decodeBech32
    && decodeBech32.prefix === network.bech32
    && decodeBech32.version === 1
    && decodeBech32.data.length === 32
  ) {
    // xonly public key
    tapInternalKey = Buffer.from(publicKey, "hex").subarray(1, 33);
    outputScriptSize = P2TR_INPUT_SIZE;
  }

  // For P2SH-P2WPKH address, we need to get redeemScript
  let redeemScript;
  if (decodeBech32
    && decodeBech32.prefix === network.bech32
    && decodeBech32.version === 0
    && decodeBech32.data.length === 20
  ) {
    redeemScript = bitcoin.payments.p2wpkh({ pubkey: Buffer.from(publicKey, "hex") }).output;
    outputScriptSize = P2WPKH_INPUT_SIZE;
  }
  return {
    outputScript,
    outputScriptSize,
    tapInternalKey,
    redeemScript
  }
}
// export const getPublicKeyNoCoord = (pkHex: string): Buffer => {
//   const publicKey = Buffer.from(pkHex, "hex");
//   return publicKey.subarray(1, 33);
// };
// export const publicKeyToP2trScript = (publicKey: string, network: bitcoin.Network): Uint8Array => {
//   const p2pktr = bitcoin.payments.p2tr({
//     pubkey: getPublicKeyNoCoord(publicKey),
//     network
//   });
//   if (!p2pktr.output) {
//     throw new Error("Failed to create p2tr script");
//   }
//   return p2pktr.output;   
// };
// export const addressToOutputScript = (address: string, network: bitcoin.Network): Uint8Array => {
//   return bitcoin.address.toOutputScript(address, network);
// };

export const signPsbt = (network: bitcoin.Network, privkey: string, unsignedPsbt: bitcoin.Psbt, finalize: boolean = true): bitcoin.Psbt => {
  console.log("signPsbt", unsignedPsbt);
  const keyPair = ECPair.fromWIF(privkey, network);
  const signedPsbt = unsignedPsbt.signAllInputs(keyPair);
  if (finalize) {
      signedPsbt.finalizeAllInputs();
  }
  return signedPsbt;
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
      2,
    ),
  );
}

