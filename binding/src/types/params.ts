import { TxOutput } from "bitcoinjs-lib";
import { UTXO } from "./bitcoin";
import { DestinationChain } from "@/chain";

export type TBuildUnsignedStakingPsbt = {
  stakingAmount: bigint;
  stakerPubkey: Uint8Array;
  stakerAddress: string;
  protocolPubkey: Uint8Array;
  custodialPubkeys: Uint8Array;
  covenantQuorum: number;
  haveOnlyCovenants: boolean;
  destinationChain: DestinationChain;
  destinationContractAddress: Uint8Array;
  destinationRecipientAddress: Uint8Array;
  availableUTXOs: UTXO[];
  feeRate: number;
  rbf?: boolean;
};

export type TBuildUnsignedStakingWithOnlyCovenantsPsbt = {
  stakingAmount: bigint;
  stakerPubkey: Uint8Array;
  stakerAddress: string;
  custodialPubkeys: Uint8Array;
  covenantQuorum: number;
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

export type TBuildUnsignedUnstakingUserProtocolPsbt = {
  input: PreviousStakingUTXO;
  output: TxOutput;
  stakerPubkey: Uint8Array;
  protocolPubkey: Uint8Array;
  covenantPubkeys: Uint8Array;
  covenantQuorum: number;
  haveOnlyCovenants: boolean;
  feeRate: bigint;
  rbf: boolean;
};

export type TBuildUnsignedUnstakingWithOnlyCovenantsPsbt = {
  inputs: PreviousStakingUTXO[];
  output: TxOutput;
  stakerPubkey: Uint8Array;
  protocolPubkey: Uint8Array;
  covenantPubkeys: Uint8Array;
  covenantQuorum: number;
  haveOnlyCovenants: boolean;
  feeRate: bigint;
  rbf: boolean;
};
