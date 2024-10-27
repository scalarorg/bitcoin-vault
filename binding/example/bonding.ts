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
} from "../src";
import { getDefaultEthAddress } from "./eth";
import { globalParams } from "./params";
import { UTXO } from "../src/types/btc";
import { sendrawtransaction } from "../src/client/bitcoin";
import Client from "bitcoin-core-ts";

/*
 *  bondingAmount in shatoshi
 *  mintingAmount consider equal to bondingAmount
 */
async function createBondingTransaction(bondingAmount: number): Promise<{
  hexTxfromPsbt: string;
  fee: number;
}> {
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
  const outputs = buildStakingOutput(
    globalParams.tag,
    globalParams.version,
    BigInt(bondingAmount),
    globalParams.bondHolderPublicKey,
    globalParams.bondHolderPublicKey,
    globalParams.covenantPublicKeys,
    globalParams.covenantQuorum,
    false,
    globalParams.destChainId,
    globalParams.destSmartContractAddress,
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
  const btcClient = new Client({
    network: "regtest",
    host: "localhost",
    port: "18332",
    wallet: "legacy",
    username: "user",
    password: "password"
  });
  for (let i = 0; i < numberTxs; i++) {
    // Create a bonding transaction
    await createBondingTransaction(
      bondingAmount + Math.min(1000, Math.ceil(bondingAmount * i * 0.1))
    )
      .then(async ({ hexTxfromPsbt, fee }) => {
        console.log(
          `Signed Tx in Hex: ${hexTxfromPsbt} with estimated fee ${fee}`
        );
        const testRes = await btcClient.command("testmempoolaccept", [
          hexTxfromPsbt,
        ]);
        console.log(testRes);
        return sendrawtransaction(hexTxfromPsbt);
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
