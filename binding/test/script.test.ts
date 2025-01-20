import { describe, it } from "bun:test";
import { setUpTest, StaticEnv } from "./util";
import { fromOutputScript } from "bitcoinjs-lib/src/address";
import * as bitcoin from "bitcoinjs-lib";
import { getAddressUtxos, sendrawtransaction } from "../src/client";
import { TBuildUPCUntakingPsbt } from "../src";
import { getEstimatedFee } from "../src/utils";
import { Psbt } from "bitcoinjs-lib";
//Start local regtest bitcoin node before running the test
describe("Vault Script", async () => {
  const TestSuite = await setUpTest();
  it("should create, signed and broadcast staking psbt", async () => {
    const script = TestSuite.vaultUtils.upcLockingScript({
      userPubkey: TestSuite.stakerPubKey,
      protocolPubkey: TestSuite.protocolPubkey,
      custodianPubkeys: TestSuite.custodialPubkeys,
      custodianQuorum: StaticEnv.CUSTODIAL_QUORUM,
    });

    console.log({ script: Buffer.from(script).toString("hex") });

    const address = fromOutputScript(script, bitcoin.networks.testnet);
    console.log({ address });

    const addressUtxos = await getAddressUtxos({
      address: address,
      mempoolClient: TestSuite.mempoolClient,
    });

    console.log({ availableUtxos: addressUtxos.length });

    if (addressUtxos.length === 0) {
      throw new Error("No utxos found for the address");
    }

    const choosenUtxo = addressUtxos[0];

    console.log({ choosenUtxo });

    const params: TBuildUPCUntakingPsbt = {
      input: {
        txid: choosenUtxo.txid,
        vout: choosenUtxo.vout,
        value: BigInt(choosenUtxo.value),
        script_pubkey: script,
      },
      output: {
        script: bitcoin.address.toOutputScript(
          TestSuite.stakerAddress,
          TestSuite.network
        ),
        value: BigInt(choosenUtxo.value),
      },
      stakerPubkey: TestSuite.stakerPubKey,
      protocolPubkey: TestSuite.protocolPubkey,
      custodianPubkeys: TestSuite.custodialPubkeys,
      custodianQuorum: StaticEnv.CUSTODIAL_QUORUM,
      feeRate: BigInt(1),
      rbf: true,
      type: "user_protocol",
    };

    // Build the unsigned psbt
    const psbtHex = TestSuite.vaultUtils.buildUPCUnstakingPsbt(params);
    const psbtFromHex = Psbt.fromBuffer(psbtHex);

    console.log("===============");
    console.log("psbtFromHex, input value", choosenUtxo.value);
    console.log("===============");
    console.log("psbtFromHex, output value", psbtFromHex.txOutputs[1].value);

    console.log("===============");
    console.log("unstaked psbt base64", psbtFromHex.toBase64());
    console.log("===============");

    // staker signs the psbt
    const stakerSignedPsbt = TestSuite.vaultUtils.signPsbt({
      psbt: psbtFromHex,
      wif: TestSuite.stakerWif,
      finalize: false,
    });

    const psbtBase64 = stakerSignedPsbt.toBase64();
    console.log("===============");
    console.log("psbtBase64", psbtBase64);
    console.log("===============");
    console.log("userSignedPsbt", stakerSignedPsbt.toHex());

    const serviceSignedPsbt = TestSuite.vaultUtils.signPsbt({
      psbt: stakerSignedPsbt,
      wif: TestSuite.protocolKeyPair.toWIF(),
      finalize: true,
    });

    // check the output value

    console.log("===============");
    console.log("psbtServicesHex", serviceSignedPsbt.toHex());

    const hexTxfromPsbt = serviceSignedPsbt.extractTransaction().toHex();

    console.log("===============");
    console.log("hexTxfromPsbt", hexTxfromPsbt);

    const unstakedTxid = await sendrawtransaction(
      hexTxfromPsbt,
      TestSuite.btcClient
    );
    console.log("\nunstakedTxid", unstakedTxid);
  });
});
