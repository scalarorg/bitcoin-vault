
import Client from "bitcoin-core-ts";
import { globalParams } from "../params";
import { BtcUnspent } from "../types/btc";
import { AddressTxsUtxo } from "@mempool/mempool.js/lib/interfaces/bitcoin/addresses";

export const defaultClient = new Client({
    network: globalParams.networkName,
    host: globalParams.btcHost,
    port: globalParams.btcPort,
    wallet: globalParams.walletName,
    username: globalParams.btcRpcUser,
    password: globalParams.btcRpcPassword
});

export const getUnspentTransactionOutputs = async (address: string, btcClient?: Client): Promise<AddressTxsUtxo[]> => {
    const client = btcClient || defaultClient;
    const listUnspent: BtcUnspent[] = await client.command("listunspent", 1, 9999999, [address], true, {"minimumAmount": 1/100000});
    const mempoolUtxos: AddressTxsUtxo[] = listUnspent.map((utxo: BtcUnspent) => {
        return {
            txid: utxo.txid,
            vout: utxo.vout,
            value: Math.round(utxo.amount * 100000000), // convert to satoshis
            confirmations: utxo.confirmations,
            status: {
                confirmed: utxo.confirmations > 0,
                block_height: 0,
                block_hash: "",
                block_time: 0
            }
        };
    });
    return mempoolUtxos;
}

export const sendrawtransaction = async (hex: string, btcClient?: Client): Promise<string> => {
    const client = btcClient || defaultClient;
    const txid = await client.command("sendrawtransaction", hex);
    return txid;
}
const btcNodeClient = {
    rpcClient: defaultClient,
    sendrawtransaction,
    getUnspentTransactionOutputs
};

export default btcNodeClient;