export type PreviousStakingUTXO = {
  txid: string;
  vout: number;
  value: bigint;
  script_pubkey: Uint8Array;
};
