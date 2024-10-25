import { AddressTxsUtxo } from "@mempool/mempool.js/lib/interfaces/bitcoin/addresses";
import {
  addressToOutputScript,
  buildStakingOutput,
  createVaultWasm,
  getPublicKeyNoCoord,
  getStakingTxInputUTXOsAndFees,
  logToJSON,
  signPsbt,
  defaultMempoolClient,
  getAddressUtxos,
  createStakingPsbt,
  btcClient,
} from "../src";
import { getDefaultEthAddress } from "./eth";
import { globalParams } from "./params";
import { UTXO } from "../src/types/btc";

/*
 *  bondingAmount in shatoshi
 *  mintingAmount consider equal to bondingAmount
 */
async function createBondingTransaction(bondingAmount: number): Promise<{
  hexTxfromPsbt: string;
  fee: number;
}> {
  if (!globalParams.bondHolderAddress) {
    throw new Error("Bond holder address is not provided");
  }
  if (!globalParams.bondHolderPrivKey) {
    throw new Error("Bond holder private key is not provided");
  }
  if (!globalParams.bondHolderPublicKey) {
    throw new Error("Bond holder public key is not provided");
  }
  if (!globalParams.protocolPublicKey) {
    throw new Error("Protocol public key is not provided");
  }
  if (!globalParams.covenantPublicKeys) {
    throw new Error("Covenant public keys are not provided");
  }
  // const stakerPubKey = "032b122fd36a9db2698c39fb47df3e3fa615e70e368acb874ec5494e4236722b2d";
  // const stakerPrivKey = "cUKxQGWboxiXBW3iXQ9pTzwCdMK7Un9mdLeDKepZkdVf5V7JNd9a"
  // const bondHolderPublicKey = "032b122fd36a9db2698c39fb47df3e3fa615e70e368acb874ec5494e4236722b2d";
  const userAddress = globalParams.destUserAddress || getDefaultEthAddress();
  console.log(`User Address: ${userAddress}`);

  // const staker = new vault.Staker(
  //     globalParams.bondHolderAddress,
  //     globalParams.bondHolderPublicKey,
  //     globalParams.protocolPublicKey,
  //     globalParams.covenantPublicKeys,
  //     globalParams.covenantQuorum,
  //     globalParams.tag,
  //     globalParams.version,
  //     globalParams.destChainId || '1',
  //     userAddress,
  //     globalParams.destSmartContractAddress || '',
  //     bondingAmount,
  // );
  // --- Get UTXOs

  const addressUtxo: AddressTxsUtxo[] = await getAddressUtxos(
    globalParams.bondHolderAddress
  );
  const regularUTXOs: UTXO[] = addressUtxo.map(
    ({ txid, vout, status, value }: AddressTxsUtxo) => ({
      txid,
      vout,
      value,
      status: {
        block_hash: status.block_hash,
        block_height: status.block_height,
        block_time: status.block_time,
        confirmed: status.confirmed,
      },
    })
  );

  //const regularUTXOs = await addresses.getAddressTxsUtxo({address: globalParams.bondHolderAddress});
  const { fees } = defaultMempoolClient;
  const { fastestFee: feeRate } = await fees.getFeesRecommended(); // Get this from Mempool API
  const rbf = true; // Replace by fee, need to be true if we want to replace the transaction when the fee is low
  let vaultWasm = createVaultWasm(globalParams.tag, globalParams.version);
  const outputs = buildStakingOutput(
    vaultWasm,
    BigInt(bondingAmount),
    globalParams.bondHolderPublicKey,
    globalParams.bondHolderPublicKey,
    globalParams.covenantPublicKeys,
    globalParams.covenantQuorum,
    false,
    globalParams.destChainId || "1",
    globalParams.destSmartContractAddress || "",
    userAddress
  );
  const scriptPubKey = addressToOutputScript(
    globalParams.bondHolderAddress,
    globalParams.network
  );
  const { selectedUTXOs, fee } = getStakingTxInputUTXOsAndFees(
    globalParams.network,
    regularUTXOs,
    Buffer.from(scriptPubKey),
    bondingAmount,
    feeRate,
    outputs
  );
  const publicKeyNoCoord = getPublicKeyNoCoord(
    globalParams.bondHolderPublicKey
  );
  const { psbt: unsignedVaultPsbt, fee: estimatedFee } = createStakingPsbt(
    globalParams.network,
    publicKeyNoCoord,
    selectedUTXOs,
    scriptPubKey,
    outputs,
    bondingAmount,
    fee,
    globalParams.bondHolderAddress
  );
  // const { psbt: unsignedVaultPsbt, feeEstimate: fee } =
  //     await staker.getUnsignedVaultPsbt(
  //         regularUTXOs,
  //         bondingAmount,
  //         feeRate,
  //         rbf,
  //     );
  console.log(unsignedVaultPsbt);
  // Simulate signing
  const signedPsbt = signPsbt(
    globalParams.network,
    globalParams.bondHolderPrivKey,
    unsignedVaultPsbt
  );
  // --- Sign with staker
  const hexTxfromPsbt = signedPsbt.extractTransaction().toHex();
  return {
    hexTxfromPsbt,
    fee,
  };
}

async function createBondingTransactions(
  bondingAmount: number,
  numberTxs: number
) {
  for (let i = 0; i < numberTxs; i++) {
    // Create a bonding transaction
    await createBondingTransaction(
      bondingAmount + Math.min(1000, Math.ceil(bondingAmount * i * 0.1))
    )
      .then(async ({ hexTxfromPsbt, fee }) => {
        console.log(
          `Signed Tx in Hex: ${hexTxfromPsbt} with estimated fee ${fee}`
        );
        const testRes = await btcClient.rpcClient.command("testmempoolaccept", [
          hexTxfromPsbt,
        ]);
        console.log(testRes);
        return btcClient.sendrawtransaction(hexTxfromPsbt);
      })
      .then((txid) => {
        console.log(`Transaction ID: ${txid}`);
      })
      .catch((error) => {
        console.error(error);
      });
  }
}

const bondingAmount = 10000; // in satoshis
const numberTxs = 2;
logToJSON(globalParams);
createBondingTransactions(bondingAmount, numberTxs);
