
import * as bitcoin from "bitcoinjs-lib";
import {
    buildStakingOutput,
    createStakingPsbt,
    createVaultWasm,
    decodeStakingOutput,
    defineAddressType,
    ECPair,
    getStakingTxInputUTXOsAndFees,
    logToJSON,
    prepareExtraInputByAddress,
    signPsbt
} from "@/utils";
import { bytesToHex, hexToBytes } from "@/utils/encode";
import { defaultMempoolClient, getAddressUtxos, sendrawtransaction } from "@/client";
import { AddressType, UTXO } from "@/types";
import { PsbtOutputExtended } from "bip174";
import Client from "bitcoin-core-ts";
import { AddressTxsUtxo } from "@mempool/mempool.js/lib/interfaces/bitcoin/addresses";

import { describe, it, beforeEach, expect } from "bun:test";
import { buildUnsignedStakingPsbt } from "@/staking";

//Start local regtest bitcoin node before running the test
describe("Vault-AddressType", () => {
    it("should be regtest P2WPKH", () => {
        const network = bitcoin.networks.regtest;
        const address = "bcrt1q27ply66u77athpuw6xtwy7nj40wmjfjwrwts07";
        const addressType = defineAddressType(address, network);
        expect(addressType).toBe(AddressType.P2WPKH);
    })
})  