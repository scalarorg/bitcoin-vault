import * as bitcoin from "bitcoinjs-lib";

import { describe, expect, it } from "bun:test";
import { AddressType } from "../src/types";
import { defineAddressType } from "../src/utils";

//Start local regtest bitcoin node before running the test
describe("Vault-AddressType", () => {
  it("should be regtest P2WPKH", () => {
    const network = bitcoin.networks.regtest;
    const address = "bcrt1q27ply66u77athpuw6xtwy7nj40wmjfjwrwts07";
    const addressType = defineAddressType(address, network);
    expect(addressType).toBe(AddressType.P2WPKH);
  });
});
