import { describe, expect, it } from "bun:test";
describe("Vault-Decoder", () => {
  const tag = "01020304";
  const version = 1;
  it("should decode", () => {
    const amount_bytes = new Uint8Array([0, 0, 0, 2, 84, 13, 106, 160]); // 10_000_100_000
    const amount = new DataView(amount_bytes.buffer).getBigUint64(0);
    expect(amount).toBe(BigInt(10000100000));
  });
});
