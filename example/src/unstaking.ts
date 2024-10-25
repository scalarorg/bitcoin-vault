import {
  btcClient,
  defaultMempoolClient as mempoolClient,
  addressToOutputScript,
  getPublicKeyNoCoord,
  signPsbt
} from "bitcoin-vault";
import * as vault from "bitcoin-vault";
import { globalParams } from "./params";

async function createUnbondingServiceTransaction(
  stakerAddress: string,
  receiveAddress: string,
  hexTx: string
): Promise<{
  hexTxfromPsbt: string;
  fee: number;
}> {
    const { fees } = mempoolClient;
    const { fastestFee: feeRate } = await fees.getFeesRecommended(); // Get this from Mempool API
    const rbf = true; // Replace by fee, need to be true if we want to replace the transaction when the fee is low
    let vaultWasm = vault.createVaultWasm(globalParams.tag, globalParams.version);
    const scriptPubKey = addressToOutputScript(globalParams.bondHolderAddress, globalParams.network);
    const publicKeyNoCoord = getPublicKeyNoCoord(globalParams.bondHolderPublicKey);

    const unstakingOuput = vault.buildUnstakingOutput(vaultWasm, receiveAddress, stakerAddress, globalParams.covenantPublicKeys!, globalParams.covenantQuorum, false, hexTx);
    
    const { psbt: unsignedPsbt } = vault.createUnStakingPsbt(globalParams.network, publicKeyNoCoord, scriptPubKey, unstakingOuput, feeRate, rbf);

    // // Step 1: staker create unbonding transaction,
    // // sign it and send with burning request to ETH
    // const unStaker = new vault.UnStaker(
    //     stakerAddress,
    //     hexTx,
    //     vaultWasm,
    //     globalParams.covenantPublicKeys!,
    //     globalParams.covenantQuorum
    // );
    // const {
    // psbt: unsignedPsbt,
    // feeEstimate: fee,
    //     burningLeaf,
    // } = await unStaker.getUnsignedBurningPsbt(receiveAddress, feeRate, rbf);

    //   // simulate staker sign the psbt
//   const stakerSignedPsbt = await psbt.signInputs(
//     globalParams.bondHolderPrivKey!,
//     globalParams.network,
//     unsignedPsbt.toBase64(),
//     false
//   );

//   // Step 2: this Psbt will be sent to bla bla ... then received by relayer of service dApp
//   // the service dApp will sign the psbt, finalize it and send to bitcoin network

//   // simulate service sign the psbt
//   const serviceSignedPsbt = await psbt.signInputs(
//     globalParams.protocolPrivKey!,
//     globalParams.network,
//     stakerSignedPsbt.toBase64(),
//     true
//   );
    const serviceSignedPsbt = signPsbt(globalParams.network, globalParams.protocolPrivKey, unsignedPsbt);
    const hexTxfromPsbt = serviceSignedPsbt.extractTransaction().toHex();
    //Todo: Calculate fee
  let fee = 0;
  return { hexTxfromPsbt, fee };
}

// -----------------
const receiveAddress = globalParams.bondHolderAddress;
// const receiveAddress = "bcrt1q7u9vq6zkmkkz3t536u3h9dxjhc8gry4987992j";
const stakerAddress = globalParams.bondHolderAddress;
// const stakerAddress = "bcrt1q7u9vq6zkmkkz3t536u3h9dxjhc8gry4987992j";
const hexBondTx =
  "020000000001019ce35d6e8a8a8dce48821eaff9a81dff297c42912b944ccad31a888519690eac0300000000fdffffff04e02e000000000000225120bcfd5db0727fb6f232baeadadc5f9f0f71135b3110951d51d528f87b2b0ece720000000000000000476a450102030400ee0e5795a901fde97451f5ab1d9b0639e5332c89739ac9260c963a20a6a71b8dada80c9d76057a5ca65e7af7b058c7926905ed548c766dee35ec0d27240957a900000000000000003a6a380000000000aa36a74f818beff37bced3238098b595220010c0a3504f26f761c408986e49693dde7f7e95987b5f8241b80000000000002ee0adc54f0900000000160014f70ac06856ddac28ae91d72372b4d2be0e8192a502483045022100eddfef9c345b573df5fd29e9c861410ac8202a0398968021eecb60148356385a022044dde165606a01775bd028eccd6eb2eea752f92d926aea1a245783f3a5fb760d012102ee0e5795a901fde97451f5ab1d9b0639e5332c89739ac9260c963a20a6a71b8d00000000";

createUnbondingServiceTransaction(stakerAddress, receiveAddress, hexBondTx)
  .then(async ({ hexTxfromPsbt, fee }) => {
    console.log(`Signed Tx in Hex: ${hexTxfromPsbt} with estimated fee ${fee}`);
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
