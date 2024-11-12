import { AddressTxsUtxo } from "@mempool/mempool.js/lib/interfaces/bitcoin/addresses";
import Client from "bitcoin-core-ts";
import { getUnspentTransactionOutputs } from "./bitcoin";
import { default as BtcMempool, defaultMempoolClient } from "./mempool";

export { sendrawtransaction, testmempoolaccept } from "./bitcoin";

export { BtcMempool, defaultMempoolClient };

export async function getAddressUtxos(config: {
  address: string;
  btcClient?: Client;
  mempoolClient?: BtcMempool;
}): Promise<AddressTxsUtxo[]> {
  const { address, btcClient, mempoolClient } = config;

  if (!address) {
    throw new Error("Address is required");
  }

  if (!btcClient && !mempoolClient) {
    throw new Error("Either btcClient or mempoolClient is required");
  }

  console.log(
    `getUnspentTransactionOutputs of the address ${address} from the bitcoin node`
  );
  let utxos: AddressTxsUtxo[] = [];

  if (btcClient) {
    try {
      utxos = await getUnspentTransactionOutputs(address, btcClient);

      if (utxos.length == 0) {
        throw new Error(
          `Cannot find utxos for address ${address} from the bitcoin node`
        );
      }
    } catch (error) {
      console.error(`Error getting utxos for address ${address}: ${error}`);
    }
  }

  try {
    utxos = await (
      mempoolClient || defaultMempoolClient
    ).addresses.getAddressTxsUtxo({
      address,
    });
  } catch (error) {
    console.error(`Error getting utxos for address ${address}: ${error}`);
  }

  return utxos;
}

export * from "./bitcoin";
export * from "./mempool";
