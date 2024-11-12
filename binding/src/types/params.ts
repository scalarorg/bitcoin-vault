import { TxOutput } from "bitcoinjs-lib";
import { UTXO } from "./bitcoin";

export type TBuildUnsignedStakingPsbt = {
  stakingAmount: bigint;
  stakerPubkey: Uint8Array;
  stakerAddress: string;
  protocolPubkey: Uint8Array;
  custodialPubkeys: Uint8Array;
  covenantQuorum: number;
  haveOnlyCovenants: boolean;
  destinationChainId: bigint;
  destinationSmartContractAddress: Uint8Array;
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
  rbf: boolean;
};
