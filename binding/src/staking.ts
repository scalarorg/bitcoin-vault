import { Network } from "bitcoinjs-lib";
import { UTXO } from "./types";
import { AddressTxsUtxo } from "@mempool/mempool.js/lib/interfaces/bitcoin/addresses";
import { buildStakingOutput, createStakingPsbt, getStakingTxInputUTXOsAndFees, prepareExtraInputByAddress } from "./utils";

export const buildUnsignedStakingPsbt = (
  tag: string,
  version: number,
  network: Network,
  stakerAddress: string,
  stakerPubkey: string,
  protocolPubkey: string,
  custodialPubkeys: string[],
  custodialQuorum: number,
  dstChainId: bigint,
  dstSmartContractAddress: string,
  dstUserAddress: string,
  addressUtxos: AddressTxsUtxo[],
  feeRate: number,
  stakingAmount: bigint,
  rbf: boolean = true
) => {
  // create the p2tr script pubkey from public key
  //const p2trScriptPubKey = publicKeyToP2trScript(stakerPubkey, network);
  // 1. Create the staking output
  const outputs = buildStakingOutput(
      tag,
      version,
      stakingAmount,
      stakerPubkey,
      protocolPubkey,
      custodialPubkeys,
      custodialQuorum,
      false,
      dstChainId,
      dstSmartContractAddress,
      dstUserAddress
  );
  // 2. Get the selected utxos and fees
  const inputByAddress = prepareExtraInputByAddress(stakerAddress, stakerPubkey, network);
  // Todo: 
  // Taivv: 2024-10-27 - Testing with p2wpkh on regtest due to dumprivate key for signing
 
  console.log("inputByAddress", inputByAddress);
  const regularUTXOs: UTXO[] = addressUtxos.map(
      ({ txid, vout, value }: AddressTxsUtxo) => ({
          txid,
          vout,
          value
      })
  );
  const { selectedUTXOs, fee } = getStakingTxInputUTXOsAndFees(
      network,
      regularUTXOs,
      inputByAddress.outputScriptSize,
      Number(stakingAmount),
      feeRate,
      outputs
  );
  // 3. Create the psbt
  const { psbt, fee: estimatedFee } = createStakingPsbt(
    network,
    inputByAddress,
    selectedUTXOs,
    outputs,
    Number(stakingAmount),
    fee,
    stakerAddress,
    rbf
  );
  return {
    psbt,
    fee: estimatedFee
  }
}