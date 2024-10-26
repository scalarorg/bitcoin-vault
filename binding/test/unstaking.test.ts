
import * as bitcoin from "bitcoinjs-lib";
import { buildStakingOutput, createStakingPsbt, createVaultWasm, decodeStakingOutput, getStakingTxInputUTXOsAndFees, logToJSON, signPsbt } from "@/utils";
import { hexToBytes } from "@/utils/encode";
import { defaultMempoolClient, getAddressUtxos } from "@/client";
import { UTXO } from "@/types";
import { PsbtOutputExtended } from "bip174";
import Client from "bitcoin-core-ts";
import { AddressTxsUtxo } from "@mempool/mempool.js/lib/interfaces/bitcoin/addresses";

import { describe, it, beforeEach, expect } from "bun:test";
describe("Vault-Unstaking", () => {
    const tag = "01020304";
    const version = 1;
    const network = bitcoin.networks.regtest;
    const btcRegtestClient = new Client({
        network: "regtest",
        host: "localhost",
        port: "18332",
        wallet: "legacy",
        username: "user",
        password: "password"
    });
  it("should create a psbt for unstaking", () => {
    const amount_bytes = new Uint8Array([0, 0, 0, 2, 84, 13, 106, 160]); // 10_000_100_000
    const amount = new DataView(amount_bytes.buffer).getBigUint64(0);
    expect(amount).toBe(BigInt(10000100000))
  });
});
