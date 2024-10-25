import { VaultWasm } from "wasm/bitcoin-vault-web";
import { BTC_DUST_SAT, BTC_PUBKEY_SIZE } from "./constants";
import { PsbtOutputExtended } from "../types/psbt";
import { Network, Psbt } from "bitcoinjs-lib";
import { UTXO } from "../types/btc";

export const createVaultWasm = (tag: string, version: number) => {
    return VaultWasm.new(new Uint8Array(Buffer.from(tag)), version);
}
export const buildStakingOutput = (vault: VaultWasm,
    stakingAmount: bigint,
    stakerPubkey: String,
    protocolPubkey: String,
    custodialPubkeys: String[],
    covenantQuorum: number,
    haveOnlyCovenants: boolean,
    destinationChainId: string,
    destinationSmartContractAddress: String,
    destinationRecipientAddress: String) => {
    const pubkeys = new Uint8Array(custodialPubkeys.length * BTC_PUBKEY_SIZE);
    for (let i = 0; i < custodialPubkeys.length; i++) {
        pubkeys.set(new Uint8Array(Buffer.from(custodialPubkeys[i])), i * BTC_PUBKEY_SIZE);
    }
    const output_buffer = vault.build_staking_output(
        stakingAmount,  
        new Uint8Array(Buffer.from(stakerPubkey)),
        new Uint8Array(Buffer.from(protocolPubkey)),
        pubkeys,
        covenantQuorum,
        haveOnlyCovenants,
        BigInt(destinationChainId),
        new Uint8Array(Buffer.from(destinationSmartContractAddress)),
        new Uint8Array(Buffer.from(destinationRecipientAddress)),
    );
    // Decode the output buffer to a PsbtOutputExtended list    
    return decodeStakingOutput(output_buffer);
}
export const buildUnstakingOutput = (vault: VaultWasm,
    stakerPubkey: String,
    protocolPubkey: String,
    custodialPubkeys: String[],
    covenantQuorum: number,
    haveOnlyCovenants: boolean,
    txHex: string) => {
    const pubkeys = new Uint8Array(custodialPubkeys.length * BTC_PUBKEY_SIZE);
    for (let i = 0; i < custodialPubkeys.length; i++) {
        pubkeys.set(new Uint8Array(Buffer.from(custodialPubkeys[i])), i * BTC_PUBKEY_SIZE);
    }
    const output_buffer = vault.build_unstaking_output(
        new Uint8Array(Buffer.from(stakerPubkey)),
        new Uint8Array(Buffer.from(protocolPubkey)),
        pubkeys,
        covenantQuorum,
        haveOnlyCovenants,
        new Uint8Array(Buffer.from(txHex))
    );
    return decodeUnStakingOutput(output_buffer);
}
export const createStakingPsbt = (network: Network,
    publicKeyNoCoord: Buffer,
    selectedUTXOs: UTXO[],
    //ScriptPubkey of all UTXOs
    scriptPubKey: Uint8Array,
    psbtOutputs: PsbtOutputExtended[],
    amount: number, fee: number,
    changeAddress: string,
    //lockHeight: number,
    ) => { 
    // Create a partially signed transaction
    const psbt = new Psbt({ network });
    // Add the UTXOs provided as inputs to the transaction
    for (let i = 0; i < selectedUTXOs.length; ++i) {
        const input = selectedUTXOs[i];
        psbt.addInput({
            hash: input.txid,
            index: input.vout,
            witnessUtxo: {
                script: scriptPubKey,
                value: BigInt(input.value),
            },
            // this is needed only if the wallet is in taproot mode
            ...(publicKeyNoCoord && { tapInternalKey: publicKeyNoCoord }),
            sequence: 0xfffffffd, // Enable locktime by setting the sequence value to (RBF-able)
        });
    }

    // Add the staking output to the transaction
    psbt.addOutputs(psbtOutputs);

    // Add a change output only if there's any amount leftover from the inputs
    const inputsSum = inputValueSum(selectedUTXOs);
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
}
export const createUnStakingPsbt = (network: Network,
    publicKeyNoCoord: Buffer,
    scriptPubKey: Uint8Array,
    unstakingOutput: PsbtOutputExtended[],
    feeRate: number,
    rbf: boolean) => {
    // Create a partially signed transaction
    const psbt = new Psbt({ network });
    psbt.addOutputs(unstakingOutput);
    return {
        psbt
    }
}
export const inputValueSum = (inputUTXOs: UTXO[]): number => {
  return inputUTXOs.reduce((acc, utxo) => acc + utxo.value, 0);
};
const decodeStakingOutput = (output_buffer: Uint8Array) => {
    let len = output_buffer.length;
    let offset = 0;
    let psbt_outputs: PsbtOutputExtended[] = [];
    while (offset < len) {
        //Read first 2 bytes to get the length of the psbt output, this length does not include the 2 bytes for the length itself
        const psbt_output_length = new DataView(output_buffer.buffer, offset, 2).getUint16(0); 
        offset += 2;
        const value = new DataView(output_buffer.buffer, offset, 8).getBigUint64(0); //Default is big endian
        offset += 8;
        const script = output_buffer.subarray(offset, offset + psbt_output_length - 8);
        offset += psbt_output_length - 8;
        psbt_outputs.push({ value, script: Buffer.from(script) });
    }
    return psbt_outputs;
}

const decodeUnStakingOutput = (output_buffer: Uint8Array) => {
    let len = output_buffer.length;
    let offset = 0;
    let psbt_outputs: PsbtOutputExtended[] = [];
    
    return psbt_outputs;
}