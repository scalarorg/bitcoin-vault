export type BtcUnspent = {
  txid: string;
  vout: number;
  address: string;
  label?: string;
  scriptPubKey: string;
  amount: number;
  confirmations: number;
  spendable: boolean;
  solvable: boolean;
  desc: string;
  parent_descs?: string[];
  safe: boolean;
}
// ScriptPubkey is only available in the unisat API, not in the mempool API
export interface UTXO {
  txid: string;
  vout: number;
  value: number;
  // scriptPubKey: string;
}

export type InputByAddress = {
  outputScript: Uint8Array,
  outputScriptSize: number,
  tapInternalKey?: Buffer,
  redeemScript?: Uint8Array
}

// export interface BuildTxOptions {
//   regularUTXOs: UTXO[];
//   inputs: UTXO[];
//   outputs: Output[];
//   feeRate: number;
//   address: string;
//   autoFinalized?: boolean;
// }

// export interface CalcFeeOptions {
//   inputs: UTXO[];
//   outputs: Output[];
//   addressType: AddressType;
//   feeRate: number;
//   network: Network;
//   autoFinalized?: boolean;
// }