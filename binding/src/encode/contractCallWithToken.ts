/**
 * @description: typescript version of CalculateUnstakingPayloadHash
 * @description: Check go-utils/encode/unstaking.go for the go version
 * @param {BTCFeeOpts} feeOpts - The fee options
 * @param {boolean} rbf - The rbf flag
 * @param {`0x${string}`} recipientChainIdentifier - The recipient chain identifier
 * @returns {Uint8Array} encodedPayload - The encoded payload
 */

import { BTCFeeOpts } from "@/types/fee";
import { encodeAbiParameters } from "viem";
import {
  CustodianOnly_contractCallWithTokenPayloadAbi,
  UPC_contractCallWithTokenPayloadAbi,
} from "./abi";

type TContractCallWithTokenPayloadType = "custodianOnly" | "upc";

const ContractCallWithTokenPayloadTypeToBytes: Record<
  TContractCallWithTokenPayloadType,
  Buffer
> = {
  custodianOnly: Buffer.from([0]),
  upc: Buffer.from([1]),
} as const;

type CustodianOnlyPayloadArgs = {
  type: "custodianOnly" | "upc";
  custodianOnly?: {
    feeOpts: BTCFeeOpts;
    rbf: boolean;
    recipientChainIdentifier: `0x${string}`;
  };
  upc?: {
    psbt: `0x${string}`;
  };
};

export const calculateContractCallWithTokenPayload = (
  payloadArgs: CustodianOnlyPayloadArgs
): `0x${string}` => {
  let encodedPayload: `0x${string}`;
  if (payloadArgs.type === "custodianOnly") {
    if (!payloadArgs.custodianOnly) {
      throw new Error("CustodianOnly payload is required");
    }
    encodedPayload = encodeAbiParameters(
      CustodianOnly_contractCallWithTokenPayloadAbi,
      [
        payloadArgs.custodianOnly.feeOpts,
        payloadArgs.custodianOnly.rbf,
        payloadArgs.custodianOnly.recipientChainIdentifier,
      ]
    );
  } else if (payloadArgs.type === "upc") {
    if (!payloadArgs.upc) {
      throw new Error("UPC payload is required");
    }
    encodedPayload = encodeAbiParameters(UPC_contractCallWithTokenPayloadAbi, [
      payloadArgs.upc.psbt,
    ]);
  } else {
    throw new Error("Invalid payload type");
  }

  let typeBytes = ContractCallWithTokenPayloadTypeToBytes[payloadArgs.type];

  let encodedPayloadBytes = Buffer.from(
    encodedPayload.replace("0x", ""),
    "hex"
  );

  let finalPayload = Buffer.concat([typeBytes, encodedPayloadBytes]).toString(
    "hex"
  );

  return `0x${finalPayload}`;
};
