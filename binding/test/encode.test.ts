import { describe, it, expect } from "bun:test";
import { calculateContractCallWithTokenPayload } from "../src/encode/contractCallWithToken";
import { BTCFeeOpts } from "../src/types/fee";
import { decodeAbiParameters } from "viem";
import { contractCallWithTokenAbi } from "../src/encode/abi";

describe("encode", () => {
  it("should calculate unstaking payload hash", () => {
    const lockingScript = "0x001450dceca158a9c872eb405d52293d351110572c9e";
    const payload = calculateContractCallWithTokenPayload(
      BTCFeeOpts.MinimumFee,
      true,
      lockingScript
    );
    console.log("payload", payload);

    const decoded = decodeAbiParameters(contractCallWithTokenAbi, payload);
    console.log("decoded", decoded);

    expect(decoded[0]).toBe(BTCFeeOpts.MinimumFee);
    expect(decoded[1]).toBe(true);
    expect(decoded[2]).toBe(lockingScript);
  });
});
