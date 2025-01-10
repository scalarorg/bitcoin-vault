import { describe, it, expect } from "bun:test";
import { calculateUnstakingPayloadHash } from "./encode/contractCallWithToken";
import { BTCFeeOpts } from "../src/types/fee";

describe("encode", () => {
  it("should calculate unstaking payload hash", () => {
    const example_string =
      "0x1234567890123456789012345678901234567890123456789012345678901234567890";
    const tmp_amount = 7n;
    const tmp_fee_opts = BTCFeeOpts.FastestFee;

    const payload = calculateUnstakingPayloadHash(
      example_string,
      tmp_amount,
      tmp_fee_opts
    );

    console.log("payload", payload);

    const expected_payload =
      "0x000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000070400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002312345678901234567890123456789012345678901234567890123456789012345678900000000000000000000000000000000000000000000000000000000000";
    expect(payload).toBe(expected_payload);
  });
});
