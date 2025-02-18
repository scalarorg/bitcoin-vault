import { TxOutput } from "bitcoinjs-lib";
import { UTXO } from "./bitcoin";
import { DestinationChain } from "@/chain";

export type TBuildUPCStakingPsbt = {
  stakingAmount: bigint;
  stakerPubkey: Uint8Array;
  stakerAddress: string;
  protocolPubkey: Uint8Array;
  custodianPubkeys: Uint8Array;
  custodianQuorum: number;
  destinationChain: DestinationChain;
  destinationContractAddress: Uint8Array;
  destinationRecipientAddress: Uint8Array;
  availableUTXOs: UTXO[];
  feeRate: number;
  rbf?: boolean;
};

export type TBuildCustodianOnlyStakingPsbt = {
  stakingAmount: bigint;
  stakerPubkey: Uint8Array;
  stakerAddress: string;
  custodianPubkeys: Uint8Array;
  custodianQuorum: number;
  destinationChain: DestinationChain;
  destinationContractAddress: Uint8Array;
  destinationRecipientAddress: Uint8Array;
  availableUTXOs: UTXO[];
  feeRate: number;
  rbf?: boolean;
};

interface PreviousStakingUTXO {
  script_pubkey: Uint8Array;
  txid: string;
  vout: number;
  value: bigint;
}

export type TBuildUPCUnstakingPsbt = {
  inputs: PreviousStakingUTXO[];
  output: TxOutput;
  stakerPubkey: Uint8Array;
  protocolPubkey: Uint8Array;
  custodianPubkeys: Uint8Array;
  custodianQuorum: number;
  feeRate: bigint;
  rbf: boolean;
  type: "user_protocol" | "user_custodian" | "protocol_custodian";
};
