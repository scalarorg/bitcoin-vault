
import ECPairFactory from "ecpair";
import * as bitcoin from "bitcoinjs-lib";
import * as ecc from "tiny-secp256k1";
import { AddressTxsUtxo } from "@mempool/mempool.js/lib/interfaces/bitcoin/addresses";

export const ECPair = ECPairFactory(ecc);


const BASE_BYTES = 10.5;
const INPUT_BYTES_BASE = 57.5;
const OUTPUT_BYTES_BASE = 43;


export const extractPublicKeyFromWIF = function (WIF: string, network?: bitcoin.Network): string {
    const keyPair = ECPair.fromWIF(WIF, network);
    return keyPair.publicKey.toString();
}
export const getPublicKeyNoCoord = (pkHex: string): Buffer => {
  const publicKey = Buffer.from(pkHex, "hex");
  return publicKey.subarray(1, 33);
};

export const addressToOutputScript = (address: string, network: bitcoin.Network): Uint8Array => {
  return bitcoin.address.toOutputScript(address, network);
};
export const signPsbt = (network: bitcoin.Network, privkey: string, unsignedPsbt: bitcoin.Psbt, finalize: boolean = true): bitcoin.Psbt => {
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

