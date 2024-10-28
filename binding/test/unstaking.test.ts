import { describe, it } from "bun:test";
import { setupStakingTx } from "./util";

describe("Vault-Unstaking", () => {
  it("should unstake for user", async () => {
    const { txid, txHexfromPsbt, TestSuite } = await setupStakingTx();
  });
});
