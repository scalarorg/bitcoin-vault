
import * as bitcoin from "bitcoinjs-lib";
import {
    buildStakingOutput,
    createStakingPsbt,
    createVaultWasm,
    decodeStakingOutput,
    ECPair,
    getStakingTxInputUTXOsAndFees,
    logToJSON,
    prepareExtraInputByAddress,
    signPsbt
} from "@/utils";
import { bytesToHex, hexToBytes } from "@/utils/encode";
import { defaultMempoolClient, getAddressUtxos, sendrawtransaction } from "@/client";
import { UTXO } from "@/types";
import { PsbtOutputExtended } from "bip174";
import Client from "bitcoin-core-ts";
import { AddressTxsUtxo } from "@mempool/mempool.js/lib/interfaces/bitcoin/addresses";

import { describe, it, beforeEach, expect } from "bun:test";
import { buildUnsignedStakingPsbt } from "@/staking";

//Start local regtest bitcoin node before running the test
describe("Vault-Staking", () => {
    const tag = "01020304";
    const version = 1;
    const network = bitcoin.networks.regtest;
    const btcRegtestClient = new Client({
        network: "regtest",
        host: "localhost",
        port: "18332",
        wallet: "legacy",
        username: "user",
        password: "password"
    });
    const stakingAmount = BigInt(10000100);
    //Todo: update this parameters for each test
    //P2WPKH on regtest
    const stakerAddress = "bcrt1q27ply66u77athpuw6xtwy7nj40wmjfjwrwts07";
    const stakerPubkey = "035893a120ab06b95fd5ee353e72fe7d5f93dd89a4c37cb834696e5171f768b449";
    const stakerPrivKey = "cTWEoXkDzhga8fSJFGjU7Y8XqqfDhK1J6mABRF9r2ff1sMsk65Ho";
    //P2TR
    const protocolPubkey = "03fefc11781d13c9dc2956b2902aa540350c2db05baa1fd923dddbe432abb3f211";
    const custodialPubkeys = [
        "02e9981ca48ed3b47a1b88b64fb94dc51981c8ff83f5ab62cf2f92d0dacfccf2dc",
        "034672c96c8f8e0b3c64188b78246732904b4a764de86c85c2bf716e4766ca07ab",
        "038f9289604ebb27df2902877d1c1b75693884c62ca806cac84db83be06869a7b2",
        "0349fe0362607f44803926fd2e8733e40e9612b9c8b949a4a703c96386ced14586",
        "034b5a8758481e939e48fea9371f677e29bb962b26eb792456e839cf40f55390f1"
    ];
    const custodialQuorum = 3;
    const custodialNumber = 5;
    const custodialPubkeysBuffer = new Uint8Array(33 * custodialNumber);
    const vaultWasm = createVaultWasm(tag, version);

    // Destination info
    const dstChainId = BigInt(11155111);
    const dstUserAddress =  "130C4810D57140e1E62967cBF742CaEaE91b6ecE";
    const dstSmartContractAddress = "1F98C06D8734D5A9FF0b53e3294626E62e4d232C";
    beforeEach(() => {
        for (let i = 0; i < custodialNumber; i++) {
            custodialPubkeysBuffer.set(hexToBytes(custodialPubkeys[i]), i * 33);
        }
    });
    it("shoult config with correct params", () => { 
        const keyPair = ECPair.fromWIF(stakerPrivKey, network);
        expect(bytesToHex(keyPair.publicKey)).toBe(stakerPubkey);
    })
    it("should create staking output", () => {
        let stakingOutputBuffer = vaultWasm.build_staking_output(stakingAmount,
            hexToBytes(stakerPubkey),
            hexToBytes(protocolPubkey),
            custodialPubkeysBuffer, 1, false, dstChainId,
            hexToBytes(dstSmartContractAddress),
            hexToBytes(dstUserAddress));
        let stakingOutputs: PsbtOutputExtended[] = decodeStakingOutput(stakingOutputBuffer);
        logToJSON(stakingOutputs);
        expect(stakingOutputs.length).toBe(2);
        expect(stakingOutputs[0].value).toBe(stakingAmount);
        const psbt = new bitcoin.Psbt({ network });
    });
    it("should create then sign staking psbt", async () => {
        return;
        const addressUtxos = await getAddressUtxos(stakerAddress, btcRegtestClient);
        const regularUTXOs: UTXO[] = addressUtxos.map(
            ({ txid, vout, value }: AddressTxsUtxo) => ({
                txid,
                vout,
                value
            })
        );
        const { fees } = defaultMempoolClient;
        const { fastestFee: feeRate } = await fees.getFeesRecommended(); // Get this from Mempool API
        const rbf = true; // Replace by fee, need to be true if we want to replace the transaction when the fee is low
        const outputs = buildStakingOutput(
            tag,
            version,
            stakingAmount,
            stakerPubkey,
            protocolPubkey,
            custodialPubkeys,
            custodialQuorum,
            false,
            dstChainId,
            dstSmartContractAddress,
            dstUserAddress
        );

        //Create pay to taproot script pubkey
        // const scriptPubKey = publicKeyToP2trScript(
        //     stakerPubkey,
        //     network
        // );
        
        // console.log("scriptPubKey", scriptPubKey);
        const inputByAddress = prepareExtraInputByAddress(stakerAddress, stakerPubkey, network);
        const { selectedUTXOs, fee } = getStakingTxInputUTXOsAndFees(
            network,
            regularUTXOs,
            inputByAddress.outputScriptSize,
            Number(stakingAmount),
            feeRate,
            outputs
        );
        console.log("selectedUTXOs:", selectedUTXOs);
        console.log("fee", fee)
        // const publicKeyNoCoord = getPublicKeyNoCoord(
        //     stakerPubkey
        // );
        
        const { psbt: unsignedVaultPsbt, fee: estimatedFee } = createStakingPsbt(
            network,
            inputByAddress,
            selectedUTXOs,
            //scriptPubKey,
            outputs,
            Number(stakingAmount),
            fee,
            stakerAddress,
            rbf
        );
        console.log("TxInputs", unsignedVaultPsbt.txInputs);
        console.log("TxOutputs", unsignedVaultPsbt.txOutputs);
        // logToJSON(unsignedVaultPsbt);
        // Simulate signing
        const signedPsbt = signPsbt(
            network,
            stakerPrivKey,
            unsignedVaultPsbt
        );
    });
    it("should create, signed and broadcast staking psbt", async() => {
        const addressUtxos = await getAddressUtxos(stakerAddress, btcRegtestClient);
        const { fees } = defaultMempoolClient;
        const { fastestFee: feeRate } = await fees.getFeesRecommended(); // Get this from Mempool API
        //1. Build the unsigned psbt
        const { psbt: unsignedVaultPsbt, fee: estimatedFee } = buildUnsignedStakingPsbt(
            tag,
            version,
            network,
            stakerAddress,
            stakerPubkey,
            protocolPubkey,
            custodialPubkeys,
            custodialQuorum,
            dstChainId,
            dstSmartContractAddress,
            dstUserAddress,
            addressUtxos,
            feeRate,
            stakingAmount
        );
        //2. Sign the psbt
        const { signedPsbt, isValid } = signPsbt(
            network,
            stakerPrivKey,
            unsignedVaultPsbt
        );
        expect(isValid).toBe(true);
        //3. Extract the transaction and broadcast
        let transaction = signedPsbt.extractTransaction(false);
        //console.log("inputs", signedPsbt.data.inputs);
        //console.log("transaction", transaction);
        const txHexfromPsbt = transaction.toHex();
        logToJSON({ txHexfromPsbt, fee: estimatedFee });
        //4. Broadcast the transaction
        const txid = await sendrawtransaction(txHexfromPsbt, btcRegtestClient);
        console.log("Successfully broadcasted txid", txid);
    });
});
