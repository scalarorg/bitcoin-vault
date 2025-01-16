import { getAddressUtxos, sendrawtransaction } from "../src/client";

import { describe, it } from "bun:test";
import { bytesToHex, ChainType, DestinationChain, hexToBytes } from "../src";
import { logToJSON, setUpTest, StaticEnv } from "./util";

//Start local regtest bitcoin node before running the test
describe("Vault-Staking", async () => {
  const TestSuite = await setUpTest();
  it("should create, signed and broadcast staking psbt", async () => {
    const addressUtxos = await getAddressUtxos({
      address: TestSuite.stakerAddress,
      mempoolClient: TestSuite.mempoolClient,
    });
    const { fastestFee: feeRate } =
      await TestSuite.mempoolClient.fees.getFeesRecommended(); // Get this from Mempool API

    console.log("buildUnsignedStakingPsbt");
    console.log("TAG", StaticEnv.TAG);
    console.log("VERSION", StaticEnv.VERSION);
    console.log("network", TestSuite.network);
    console.log("stakerAddress", TestSuite.stakerAddress);
    console.log("stakerPubKey", bytesToHex(TestSuite.stakerPubKey));
    console.log("protocolPubkey", bytesToHex(TestSuite.protocolPubkey));
    console.log("custodialPubkeys", TestSuite.custodialPubkeys);
    console.log("CUSTODIAL_QUORUM", StaticEnv.CUSTODIAL_QUORUM);
    console.log("DEST_CHAIN_ID", StaticEnv.DEST_CHAIN_ID);
    console.log("DEST_SMART_CONTRACT_ADDRESS", StaticEnv.DEST_TOKEN_ADDRESS);
    console.log("DEST_USER_ADDRESS", StaticEnv.DEST_USER_ADDRESS);
    console.log("addressUtxos", addressUtxos);
    console.log("feeRate", feeRate);
    console.log("STAKING_AMOUNT", StaticEnv.STAKING_AMOUNT);

    const { psbt: unsignedVaultPsbt, fee: estimatedFee } =
      TestSuite.vaultUtils.buildUPCStakingPsbt({
        stakingAmount: StaticEnv.STAKING_AMOUNT,
        stakerPubkey: TestSuite.stakerPubKey,
        stakerAddress: TestSuite.stakerAddress,
        protocolPubkey: TestSuite.protocolPubkey,
        custodianPubkeys: TestSuite.custodialPubkeys,
        custodianQuorum: StaticEnv.CUSTODIAL_QUORUM,
        destinationChain: new DestinationChain(
          ChainType.EVM,
          StaticEnv.DEST_CHAIN_ID
        ),
        destinationContractAddress: hexToBytes(StaticEnv.DEST_TOKEN_ADDRESS),
        destinationRecipientAddress: hexToBytes(StaticEnv.DEST_USER_ADDRESS),
        availableUTXOs: addressUtxos,
        feeRate,
        rbf: true,
      });

    console.log({ estimatedFee });

    const signedPsbt = TestSuite.vaultUtils.signPsbt({
      psbt: unsignedVaultPsbt,
      wif: TestSuite.stakerWif,
      finalize: true,
    });

    let transaction = signedPsbt.extractTransaction(false);
    const txHexfromPsbt = transaction.toHex();
    logToJSON({ txHexfromPsbt, fee: estimatedFee });

    console.log("txHexfromPsbt", txHexfromPsbt);
    const txid = await sendrawtransaction(txHexfromPsbt, TestSuite.btcClient);
    console.log("Successfully broadcasted txid", txid);
  });
});
