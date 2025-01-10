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
import { contractCallWithTokenAbi } from "./abi";

export const calculateContractCallWithTokenPayload = (
  feeOpts: BTCFeeOpts,
  rbf: boolean,
  recipientChainIdentifier: `0x${string}`
): `0x${string}` => {
  return encodeAbiParameters(contractCallWithTokenAbi, [
    feeOpts,
    rbf,
    recipientChainIdentifier,
  ]);
};
