import { TNetwork } from "@/types/network";
import * as bitcoin from "bitcoinjs-lib";

export const getNetwork = (name: TNetwork) => {
  switch (name) {
    case "bitcoin":
      return bitcoin.networks.bitcoin;
    case "testnet":
      return bitcoin.networks.testnet;
    case "testnet4":
      return bitcoin.networks.testnet;
    case "regtest":
      return bitcoin.networks.regtest;
    default:
      throw new Error(`Unknown network: ${name}`);
  }
};
