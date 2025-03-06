import {
  UnstakingInput,
  UnstakingOutput,
  VaultWasm,
} from "@scalar-lab/bitcoin-wasm";
import ECPairFactory from "ecpair";

import type * as bitcoinLib from "bitcoinjs-lib";
import * as ecc from "tiny-secp256k1";

import type {
  TBuildUPCStakingPsbt,
  TBuildCustodianOnlyStakingPsbt,
  TBuildUPCUnstakingPsbt,
  TNetwork,
  AddressType,
} from "./types";

import {
  createStakingPsbt,
  decodeStakingOutput,
  defineAddressType,
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
  private wasm: VaultWasm;
  private network: bitcoinLib.Network | null = null;
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

  public buildUPCStakingPsbt = (params: TBuildUPCStakingPsbt) => {
    if (!this.network) {
      throw new Error("Network not initialized");
    }

    const outputBuf = this.wasm.build_upc_staking_output(
      params.stakingAmount,
      params.stakerPubkey,
      params.protocolPubkey,
      params.custodianPubkeys,
      params.custodianQuorum,
      params.destinationChain.toBytes(),
      params.destinationContractAddress,
      params.destinationRecipientAddress
    );

    const psbtOutputs = decodeStakingOutput(outputBuf);

    const addressType = defineAddressType(params.stakerAddress, this.network);

    const result = getStakingInputsAndFee({
      availableUTXOs: params.availableUTXOs,
      stakingAmount: Number(params.stakingAmount),
      nOutputs: psbtOutputs.length,
      feeRate: params.feeRate,
      addressType
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

  public buildUPCUnstakingPsbt = (
    params: TBuildUPCUnstakingPsbt
  ): Uint8Array => {
    if (params.type !== "user_custodian") {
      throw new Error(`Not supported unstaking type: ${params.type}`);
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

    return this.wasm.build_custodian_user_spend(
      inputs,
      output,
      params.stakerPubkey,
      params.protocolPubkey,
      params.custodianPubkeys,
      params.custodianQuorum,
      params.feeRate,
      params.rbf
    );
  };

  public buildCustodianOnlyStakingPsbt = (
    params: TBuildCustodianOnlyStakingPsbt
  ) => {
    if (!this.network) {
      throw new Error("Network not initialized");
    }

    const outputBuf = this.wasm.build_only_custodian_staking_output(
      params.stakingAmount,
      params.custodianPubkeys,
      params.custodianQuorum,
      params.destinationChain.toBytes(),
      params.destinationContractAddress,
      params.destinationRecipientAddress
    );

    const psbtOutputs = decodeStakingOutput(outputBuf);

    const addressType = defineAddressType(params.stakerAddress, this.network);

    const result = getStakingInputsAndFee({
      availableUTXOs: params.availableUTXOs,
      stakingAmount: Number(params.stakingAmount),
      nOutputs: psbtOutputs.length,
      feeRate: params.feeRate,
      addressType
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

  public custodianOnlyLockingScript = (params: {
    custodianPubkeys: Uint8Array;
    custodianQuorum: number;
  }) => {
    return this.wasm.custodian_only_locking_script(
      params.custodianPubkeys,
      params.custodianQuorum
    );
  };

  public upcLockingScript = (params: {
    userPubkey: Uint8Array;
    protocolPubkey: Uint8Array;
    custodianPubkeys: Uint8Array;
    custodianQuorum: number;
  }) => {
    return this.wasm.upc_locking_script(
      params.userPubkey,
      params.protocolPubkey,
      params.custodianPubkeys,
      params.custodianQuorum
    );
  };
}
