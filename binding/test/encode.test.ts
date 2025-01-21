import { describe, it, expect } from "bun:test";
import { calculateContractCallWithTokenPayload } from "../src/encode/contractCallWithToken";
import { BTCFeeOpts } from "../src/types/fee";
import { decodeAbiParameters } from "viem";
import { CustodianOnly_contractCallWithTokenPayloadAbi } from "../src/encode/abi";

describe("encode", () => {
  it("should calculate unstaking payload hash", () => {
    const lockingScript = "0x001450dceca158a9c872eb405d52293d351110572c9e";
    const payload = calculateContractCallWithTokenPayload({
      type: "custodianOnly",
      custodianOnly: {
        feeOpts: BTCFeeOpts.MinimumFee,
        rbf: true,
        recipientChainIdentifier: lockingScript,
      },
    });
    console.log("payload", payload);

    if (
      payload !=
      "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000016001450dceca158a9c872eb405d52293d351110572c9e00000000000000000000"
    ) {
      throw new Error("Payload mismatch");
    }

    const decoded = decodeAbiParameters(
      CustodianOnly_contractCallWithTokenPayloadAbi,
      `0x${payload.replace("0x", "").slice(2)}`
    );
    console.log("decoded", decoded);

    expect(decoded[0]).toBe(BTCFeeOpts.MinimumFee);
    expect(decoded[1]).toBe(true);
    expect(decoded[2]).toBe(lockingScript);
  });
});
