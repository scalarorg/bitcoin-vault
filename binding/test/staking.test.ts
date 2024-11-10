import {
  BtcMempool,
  defaultMempoolClient,
  getAddressUtxos,
  sendrawtransaction,
} from "@/client";
import { decodeStakingOutput, logToJSON, signPsbt } from "@/utils";
import { hexToBytes } from "@/utils/encode";
import { PsbtOutputExtended } from "bip174";
import * as bitcoin from "bitcoinjs-lib";

import { buildUnsignedStakingPsbt } from "@/staking";
import { describe, expect, it } from "bun:test";
import { setUpTest, StaticEnv } from "./util";

//Start local regtest bitcoin node before running the test
describe("Vault-Staking", async () => {
  const TestSuite = await setUpTest();
  it("should create staking output", () => {
    let stakingOutputBuffer = TestSuite.vaultWasm.build_staking_output(
      StaticEnv.STAKING_AMOUNT,
      TestSuite.stakerPubKey,
      TestSuite.protocolPubkey,
      TestSuite.custodialPubkeys,
      StaticEnv.CUSTODIAL_QUORUM,
      StaticEnv.HAVE_ONLY_CUSTODIAL,
      StaticEnv.DEST_CHAIN_ID,
      hexToBytes(StaticEnv.DEST_SMART_CONTRACT_ADDRESS),
      hexToBytes(StaticEnv.DEST_USER_ADDRESS)
    );
    let stakingOutputs: PsbtOutputExtended[] =
      decodeStakingOutput(stakingOutputBuffer);
    logToJSON(stakingOutputs);
    expect(stakingOutputs.length).toBe(2);
    expect(stakingOutputs[0].value).toBe(StaticEnv.STAKING_AMOUNT);
    new bitcoin.Psbt({ network: TestSuite.network });
  });

  it("should create, signed and broadcast staking psbt", async () => {
    const addressUtxos = await getAddressUtxos({
      address: TestSuite.stakerAddress,
      mempoolClient: new BtcMempool("https://mempool.space/testnet4/api"),
    });
    const { fees } = defaultMempoolClient;
    const { fastestFee: feeRate } = await fees.getFeesRecommended(); // Get this from Mempool API
    //1. Build the unsigned psbt
    const { psbt: unsignedVaultPsbt, fee: estimatedFee } =
      buildUnsignedStakingPsbt(
        StaticEnv.TAG,
        StaticEnv.VERSION,
        TestSuite.network,
        TestSuite.stakerAddress,
        TestSuite.stakerPubKey,
        TestSuite.protocolPubkey,
        TestSuite.custodialPubkeys,
        StaticEnv.CUSTODIAL_QUORUM,
        StaticEnv.HAVE_ONLY_CUSTODIAL,
        StaticEnv.DEST_CHAIN_ID,
        hexToBytes(StaticEnv.DEST_SMART_CONTRACT_ADDRESS),
        hexToBytes(StaticEnv.DEST_USER_ADDRESS),
        addressUtxos,
        feeRate,
        StaticEnv.STAKING_AMOUNT
      );

    const { signedPsbt, isValid } = signPsbt(
      TestSuite.network,
      TestSuite.stakerWif,
      unsignedVaultPsbt
    );

    console.log({ signedPsbt: signedPsbt.data.inputs });

    expect(isValid).toBe(true);
    //3. Extract the transaction and broadcast
    let transaction = signedPsbt.extractTransaction(false);
    //console.log("inputs", signedPsbt.data.inputs);
    //console.log("transaction", transaction);
    const txHexfromPsbt = transaction.toHex();
    logToJSON({ txHexfromPsbt, fee: estimatedFee });
    //4. Broadcast the transaction
    const txid = await sendrawtransaction(txHexfromPsbt, TestSuite.btcClient);
    console.log("Successfully broadcasted txid", txid);
  });
});

// it("should create then sign staking psbt", async () => {
//     return;
//     const addressUtxos = await getAddressUtxos(stakerAddress, btcRegtestClient);
//     const regularUTXOs: UTXO[] = addressUtxos.map(
//       ({ txid, vout, value }: AddressTxsUtxo) => ({
//         txid,
//         vout,
//         value,
//       })
//     );
//     const { fees } = defaultMempoolClient;
//     const { fastestFee: feeRate } = await fees.getFeesRecommended(); // Get this from Mempool API
//     const rbf = true; // Replace by fee, need to be true if we want to replace the transaction when the fee is low
//     const outputs = buildStakingOutput(
//       tag,
//       version,
//       stakingAmount,
//       stakerPubkey,
//       protocolPubkey,
//       custodialPubkeys,
//       custodialQuorum,
//       false,
//       dstChainId,
//       dstSmartContractAddress,
//       dstUserAddress
//     );

//     //Create pay to taproot script pubkey
//     // const scriptPubKey = publicKeyToP2trScript(
//     //     stakerPubkey,
//     //     network
//     // );

//     // console.log("scriptPubKey", scriptPubKey);
//     const inputByAddress = prepareExtraInputByAddress(
//       stakerAddress,
//       stakerPubkey,
//       network
//     );
//     const { selectedUTXOs, fee } = getStakingTxInputUTXOsAndFees(
//       network,
//       regularUTXOs,
//       inputByAddress.outputScriptSize,
//       Number(stakingAmount),
//       feeRate,
//       outputs
//     );
//     console.log("selectedUTXOs:", selectedUTXOs);
//     console.log("fee", fee);
//     // const publicKeyNoCoord = getPublicKeyNoCoord(
//     //     stakerPubkey
//     // );

//     const { psbt: unsignedVaultPsbt, fee: estimatedFee } = createStakingPsbt(
//       network,
//       inputByAddress,
//       selectedUTXOs,
//       //scriptPubKey,
//       outputs,
//       Number(stakingAmount),
//       fee,
//       stakerAddress,
//       rbf
//     );
//     console.log("TxInputs", unsignedVaultPsbt.txInputs);
//     console.log("TxOutputs", unsignedVaultPsbt.txOutputs);
//     // logToJSON(unsignedVaultPsbt);
//     // Simulate signing
//     const signedPsbt = signPsbt(network, stakerPrivKey, unsignedVaultPsbt);
//   });
