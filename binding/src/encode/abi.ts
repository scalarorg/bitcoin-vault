export const unstakingPayloadAbi = [
  { type: "bytes", name: "lockingScript" },
  { type: "uint64", name: "amount" },
  { type: "bytes1", name: "feeOpts" },
] as const;
