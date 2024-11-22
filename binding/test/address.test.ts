import * as bitcoin from "bitcoinjs-lib";

import { describe, expect, it, test } from "bun:test";
import { AddressType } from "../src/types";
import { defineAddressType } from "../src/utils";

import * as ec from "tiny-secp256k1";
import { fromOutputScript } from "bitcoinjs-lib/src/address";

bitcoin.initEccLib(ec);

//Start local regtest bitcoin node before running the test
describe("Vault-AddressType", () => {
  it("should be regtest P2WPKH", () => {
    const network = bitcoin.networks.regtest;
    const address = "bcrt1q27ply66u77athpuw6xtwy7nj40wmjfjwrwts07";
    const addressType = defineAddressType(address, network);
    expect(addressType).toBe(AddressType.P2WPKH);
  });

  test("should be p2tr", () => {
    const network = bitcoin.networks.testnet;
    const output = Buffer.from(
      "51207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d5",
      "hex"
    );
    const script = bitcoin.payments.p2tr({
      network,
      output,
    });

    console.log({ address: script.address });

    const address2 = fromOutputScript(output, network);
    console.log({ address2 });
  });
});
