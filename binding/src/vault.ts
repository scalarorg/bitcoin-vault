import {
  UnstakingInput,
  UnstakingOutput,
  VaultWasm,
} from "@scalar-lab/bitcoin-wasm";
import ECPairFactory from "ecpair";

import * as bitcoinLib from "bitcoinjs-lib";
import * as ecc from "tiny-secp256k1";

import {
  TBuildUnsignedStakingPsbt,
  TBuildUnsignedStakingWithOnlyCovenantsPsbt,
  TBuildUnsignedUnstakingUserProtocolPsbt,
  TBuildUnsignedUnstakingWithOnlyCovenantsPsbt,
  TNetwork,
} from "./types";

import {
  createStakingPsbt,
  decodeStakingOutput,
  getNetwork,
  getStakingInputsAndFee,
  hexToBytes,
  prepareExtraInputByAddress,
} from "./utils";

export const NetworkKind: Record<TNetwork, 0 | 1> = {
  bitcoin: 0,
  testnet: 1,
  testnet4: 1,
  regtest: 1,
};

export const isTestnet = (network: TNetwork) => NetworkKind[network] === 1;

export class VaultUtils {
  private wasm: VaultWasm | null = null;
  private network: bitcoinLib.Network | null = null;
  // private tag: string | null = null;
  // private serviceTag: string | null = null;
  // private version: number | null = null;

  public static NETWORK_KIND: Record<TNetwork, number> = {
    bitcoin: 0,
    testnet: 1,
    testnet4: 1,
    regtest: 1,
  };

  public static ECPair = ECPairFactory(ecc);

  private static instance: VaultUtils | null = null;

  constructor(
    network: TNetwork,
    tag: string,
    serviceTag: string,
    version: number
  ) {
    this.network = getNetwork(network);
    const tagBytes = Buffer.from(tag, "ascii");
    const serviceTagBytes = Buffer.from(serviceTag, "ascii");
    this.wasm = VaultWasm.new(
      tagBytes,
      serviceTagBytes,
      version,
      VaultUtils.NETWORK_KIND[network]
    );
  }

  static getInstance(
    tag: string,
    serviceTag: string,
    version: number,
    network: TNetwork
  ) {
    if (!VaultUtils.instance) {
      VaultUtils.instance = new VaultUtils(network, tag, serviceTag, version);
    }
    return VaultUtils.instance;
  }

  public buildStakingOutput = (params: TBuildUnsignedStakingPsbt) => {
    if (!this.wasm) {
      throw new Error("VaultWasm instance not initialized");
    }

    if (!this.network) {
      throw new Error("Network not initialized");
    }

    const outputBuf = this.wasm.build_staking_output(
      params.stakingAmount,
      params.stakerPubkey,
      params.protocolPubkey,
      params.custodialPubkeys,
      params.covenantQuorum,
      params.haveOnlyCovenants,
      params.destinationChain.toBytes(),
      params.destinationContractAddress,
      params.destinationRecipientAddress
    );

    const psbtOutputs = decodeStakingOutput(outputBuf);

    const result = getStakingInputsAndFee({
      availableUTXOs: params.availableUTXOs,
      stakingAmount: Number(params.stakingAmount),
      nOutputs: psbtOutputs.length,
      feeRate: params.feeRate,
    });

    const { selectedUTXOs, fee } = result;

    const inputByAddress = prepareExtraInputByAddress(
      params.stakerAddress,
      params.stakerPubkey,
      this.network
    );

    const psbtResult = createStakingPsbt(
      this.network,
      inputByAddress,
      selectedUTXOs,
      psbtOutputs,
      Number(params.stakingAmount),
      fee,
      params.stakerAddress,
      params.rbf
    );

    return {
      psbt: psbtResult.psbt,
      fee: psbtResult.fee,
    };
  };

  public buildUnsignedUnstakingUserProtocolPsbt = (
    params: TBuildUnsignedUnstakingUserProtocolPsbt
  ): Uint8Array => {
    if (!this.wasm) {
      throw new Error("VaultWasm instance not initialized");
    }

    const input = new UnstakingInput(
      params.input.script_pubkey,
      hexToBytes(params.input.txid),
      params.input.vout,
      params.input.value
    );

    const inputs = [input];

    const output = new UnstakingOutput(
      params.output.script,
      params.output.value
    );

    return this.wasm.build_user_protocol_spend(
      inputs,
      output,
      params.stakerPubkey,
      params.protocolPubkey,
      params.covenantPubkeys,
      params.covenantQuorum,
      params.haveOnlyCovenants,
      params.rbf
    );
  };

  public buildStakingOutputWithOnlyCovenants = (
    params: TBuildUnsignedStakingWithOnlyCovenantsPsbt
  ) => {
    if (!this.wasm) {
      throw new Error("VaultWasm instance not initialized");
    }

    if (!this.network) {
      throw new Error("Network not initialized");
    }

    const outputBuf = this.wasm.build_staking_output_with_only_covenants(
      params.stakingAmount,
      params.custodialPubkeys,
      params.covenantQuorum,
      params.destinationChain.toBytes(),
      params.destinationContractAddress,
      params.destinationRecipientAddress
    );

    const psbtOutputs = decodeStakingOutput(outputBuf);

    const result = getStakingInputsAndFee({
      availableUTXOs: params.availableUTXOs,
      stakingAmount: Number(params.stakingAmount),
      nOutputs: psbtOutputs.length,
      feeRate: params.feeRate,
    });

    const { selectedUTXOs, fee } = result;

    const inputByAddress = prepareExtraInputByAddress(
      params.stakerAddress,
      params.stakerPubkey,
      this.network
    );

    const psbtResult = createStakingPsbt(
      this.network,
      inputByAddress,
      selectedUTXOs,
      psbtOutputs,
      Number(params.stakingAmount),
      fee,
      params.stakerAddress,
      params.rbf
    );

    return {
      psbt: psbtResult.psbt,
      fee: psbtResult.fee,
    };
  };

  public buildUnsignedUnstakingWithOnlyCovenantsPsbt = (
    params: TBuildUnsignedUnstakingWithOnlyCovenantsPsbt
  ) => {
    if (!this.wasm) {
      throw new Error("VaultWasm instance not initialized");
    }

    const inputs = params.inputs.map((input) => {
      return new UnstakingInput(
        input.script_pubkey,
        hexToBytes(input.txid),
        input.vout,
        input.value
      );
    });

    const output = new UnstakingOutput(
      params.output.script,
      params.output.value
    );

    return this.wasm.build_unstaking_with_only_covenants(
      inputs,
      output,
      params.covenantPubkeys,
      params.covenantQuorum,
      params.rbf
    );
  };

  public getNetwork() {
    if (!this.network) {
      throw new Error("Network not initialized");
    }
    return this.network;
  }

  public signPsbt(params: {
    psbt: bitcoinLib.Psbt;
    wif: string;
    finalize: boolean;
  }) {
    if (!this.wasm) {
      throw new Error("VaultWasm instance not initialized");
    }

    if (!this.network) {
      throw new Error("Network not initialized");
    }

    const keyPair = VaultUtils.ECPair.fromWIF(params.wif, this.network);

    const signedPsbt = params.psbt.signAllInputs(keyPair);

    if (params.finalize) {
      signedPsbt.finalizeAllInputs();
    }

    return signedPsbt;
  }

  public onlyCovenantsLockingScript = (params: {
    covenantPubkeys: Uint8Array;
    covenantQuorum: number;
  }) => {
    if (!this.wasm) {
      throw new Error("VaultWasm instance not initialized");
    }
    return this.wasm.only_covenants_locking_script(
      params.covenantPubkeys,
      params.covenantQuorum
    );
  };
}
