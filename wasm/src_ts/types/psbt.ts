import { PsbtOutput } from "bip174";

export type PsbtOutputExtended =
  | PsbtOutputExtendedAddress
  | PsbtOutputExtendedScript;

interface PsbtOutputExtendedAddress extends PsbtOutput {
  address: string;
  value: bigint;
}

interface PsbtOutputExtendedScript extends PsbtOutput {
  script: Buffer;
  value: bigint;
}

export const isPsbtOutputExtendedAddress = (
  output: PsbtOutputExtended,
): output is PsbtOutputExtendedAddress => {
  return (output as PsbtOutputExtendedAddress).address !== undefined;
};
