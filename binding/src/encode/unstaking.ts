/**
 * @description: typescript version of CalculateUnstakingPayloadHash
 * @description: Check go-utils/encode/unstaking.go for the go version
 * @param {Uint8Array} lockingScript - The locking script of the staker
 * @param {number} amount - The amount of satoshis to stake
 * @param {BTCFeeOpts} feeOpts - The fee options
 * @returns {Uint8Array} encodedPayload - The encoded payload
 */

import { BTCFeeOpts } from "@/types/fee";
import { encodeAbiParameters } from "viem";
import { unstakingPayloadAbi } from "./abi";

export const calculateUnstakingPayloadHash = (
  lockingScript: `0x${string}`,
  amount: bigint,
  feeOpts: BTCFeeOpts
): `0x${string}` => {
  return encodeAbiParameters(unstakingPayloadAbi, [
    lockingScript,
    amount,
    BTCFeeOpts.BytesString(feeOpts),
  ]);
};
