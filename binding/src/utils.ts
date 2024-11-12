import * as bitcoinLib from "bitcoinjs-lib";
import {
  BTC_DUST_SAT,
  DEFAULT_INPUT_SIZE,
  P2TR_INPUT_SIZE,
  P2WPKH_INPUT_SIZE,
} from "./constants";
import {
  AddressType,
  InputByAddress,
  PsbtOutputExtended,
  TNetwork,
  UTXO,
} from "./types";

export const hexToBytes = (hex: string) => {
  return new Uint8Array(Buffer.from(hex, "hex"));
};

export const bytesToHex = (buffer: Uint8Array) => {
  return Buffer.from(buffer).toString("hex");
};

export const getStakingInputsAndFee = (params: {
  availableUTXOs: UTXO[];
  stakingAmount: number;
  nOutputs: number;
  feeRate: number;
}): {
  selectedUTXOs: UTXO[];
  fee: number;
} => {
  if (params.availableUTXOs.length === 0) {
    throw new Error("Insufficient funds");
  }

  // Sort UTXOs by value (highest to lowest)
  const sortedUTXOs = [...params.availableUTXOs].sort(
    (a, b) => b.value - a.value
  );

  const selectedUTXOs: UTXO[] = [];
  let totalValue = 0;

  for (const utxo of sortedUTXOs) {
    selectedUTXOs.push(utxo);
    totalValue += utxo.value;

    // Calculate fee with current number of inputs: (148 * n_inputs + 34 * n_outputs + 10) * fee_rate
    const currentFee = getEstimatedFee(
      params.feeRate,
      selectedUTXOs.length,
      params.nOutputs
    );

    // Check if we have enough to cover both staking amount and fees
    if (totalValue >= params.stakingAmount + currentFee) {
      return {
        selectedUTXOs,
        fee: currentFee,
      };
    }
  }

  throw new Error(
    "Insufficient funds: unable to gather enough UTXOs to cover the staking amount and fees"
  );
};

export const getEstimatedFee = (
  feeRate: number,
  nInputs: number,
  nOutputs: number
) => {
  return (148 * nInputs + 34 * nOutputs + 11) * feeRate;
};

export const createStakingPsbt = (
  network: bitcoinLib.Network,
  inputByAddress: InputByAddress,
  selectedUTXOs: UTXO[],
  psbtOutputs: PsbtOutputExtended[],
  amount: number,
  fee: number,
  changeAddress: string,
  rbf: boolean = true
  //lockHeight: number,
) => {
  // Create a partially signed transaction
  const psbt = new bitcoinLib.Psbt({ network });
  // Add the UTXOs provided as inputs to the transaction
  for (let i = 0; i < selectedUTXOs.length; ++i) {
    const input = utxoToInput(selectedUTXOs[i], inputByAddress, rbf);
    psbt.addInput(input);
  }

  // Add the staking output to the transaction
  psbt.addOutputs(psbtOutputs);

  // Add a change output only if there's any amount leftover from the inputs
  const inputsSum = selectedUTXOs.reduce((acc, utxo) => acc + utxo.value, 0);
  // Check if the change amount is above the dust limit, and if so, add it as a change output
  if (inputsSum - (amount + fee) > BTC_DUST_SAT) {
    psbt.addOutput({
      address: changeAddress,
      value: BigInt(inputsSum - (amount + fee)),
    });
  }

  // Set the locktime field if provided. If not provided, the locktime will be set to 0 by default
  // Only height based locktime is supported
  // if (lockHeight) {
  //     if (lockHeight >= BTC_LOCKTIME_HEIGHT_TIME_CUTOFF) {
  //         throw new Error("Invalid lock height");
  //     }
  //     psbt.setLocktime(lockHeight);
  // }

  return {
    psbt,
    fee,
  };
};

export const utxoToInput = (
  utxo: UTXO,
  inputByAddress: InputByAddress,
  rbf: boolean = true
) => {
  let baseInput = {
    hash: utxo.txid,
    index: utxo.vout,
    witnessUtxo: {
      script: inputByAddress.outputScript,
      value: BigInt(utxo.value),
    },
  };
  let input = inputByAddress.tapInternalKey
    ? { ...baseInput, tapInternalKey: inputByAddress.tapInternalKey }
    : inputByAddress.redeemScript
    ? { ...baseInput, redeemScript: inputByAddress.redeemScript }
    : baseInput;
  return rbf ? { ...input, sequence: 0xfffffffd } : input;
};

export const decodeStakingOutput = (output_buffer: Uint8Array) => {
  let len = output_buffer.length;
  let offset = 0;
  let psbt_outputs: PsbtOutputExtended[] = [];
  while (offset < len) {
    //Read first 2 bytes to get the length of the psbt output, this length does not include the 2 bytes for the length itself
    const psbt_output_length = new DataView(
      output_buffer.buffer,
      offset,
      2
    ).getUint16(0);
    offset += 2;
    const value = new DataView(output_buffer.buffer, offset, 8).getBigUint64(0); //Default is big endian
    offset += 8;
    const script = output_buffer.subarray(
      offset,
      offset + psbt_output_length - 8
    );
    offset += psbt_output_length - 8;
    psbt_outputs.push({ value, script: Buffer.from(script) });
  }
  return psbt_outputs;
};

export const prepareExtraInputByAddress = (
  address: string,
  publicKey: Uint8Array,
  network: bitcoinLib.Network
): InputByAddress => {
  let decodeBase58;
  let decodeBech32;
  try {
    decodeBase58 = bitcoinLib.address.fromBase58Check(address);
  } catch (e) {
    try {
      decodeBech32 = bitcoinLib.address.fromBech32(address);
    } catch (e) {}
  }
  let addressType = defineAddressType(address, network);
  let outputScript;
  let outputScriptSize = DEFAULT_INPUT_SIZE;
  //Address type

  if (decodeBase58 && addressType === AddressType.P2PKH) {
    // This code get from Ba Hoang
    // Todo: check if this is correct
    const addressDecodedSub = Buffer.from(decodeBase58.hash).toString("hex");
    outputScript = new Uint8Array(
      Buffer.from(`76a914${addressDecodedSub}88ac`, "hex")
    );
  } else {
    outputScript = bitcoinLib.address.toOutputScript(address, network);
  }

  //For P2TR address, we need to get tapInternalKey
  let tapInternalKey;
  if (addressType === AddressType.P2TR) {
    // xonly public key
    tapInternalKey = publicKey.subarray(1, 33);
    outputScriptSize = P2TR_INPUT_SIZE;
  }

  let redeemScript;
  if (addressType == AddressType.P2WSH) {
    redeemScript = bitcoinLib.payments.p2wpkh({
      pubkey: publicKey,
    }).output;
    outputScriptSize = P2WPKH_INPUT_SIZE;
  }
  return {
    addressType,
    outputScript,
    outputScriptSize,
    tapInternalKey,
    redeemScript,
  };
};

export const defineAddressType = (
  address: string,
  network: bitcoinLib.Network
): AddressType => {
  try {
    let decodeBase58 = bitcoinLib.address.fromBase58Check(address);
    if (decodeBase58.version === network.pubKeyHash) {
      return AddressType.P2PKH;
    } else if (decodeBase58.version === network.scriptHash) {
      return AddressType.P2SH;
    }
  } catch (e) {
    try {
      let decodeBech32 = bitcoinLib.address.fromBech32(address);
      if (decodeBech32.version === 0) {
        if (decodeBech32.data.length === 20) {
          return AddressType.P2WPKH;
        } else if (decodeBech32.data.length === 32) {
          return AddressType.P2WSH;
        }
      } else if (decodeBech32.version === 1) {
        if (decodeBech32.data.length === 32) {
          return AddressType.P2TR;
        }
      }
    } catch (e) {}
  }
  return AddressType.P2WPKH;
};

export const getNetwork = (name: TNetwork) => {
  switch (name) {
    case "bitcoin":
      return bitcoinLib.networks.bitcoin;
    case "testnet":
      return bitcoinLib.networks.testnet;
    case "testnet4":
      return bitcoinLib.networks.testnet;
    case "regtest":
      return bitcoinLib.networks.regtest;
    default:
      throw new Error(`Unknown network: ${name}`);
  }
};
