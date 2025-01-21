export const CustodianOnly_contractCallWithTokenPayloadAbi = [
  { type: "uint8", name: "feeOpts" },
  { type: "bool", name: "rbf" },
  { type: "bytes", name: "recipientChainIdentifier" },
] as const;

export const UPC_contractCallWithTokenPayloadAbi = [
  { type: "bytes", name: "psbt" },
] as const;
