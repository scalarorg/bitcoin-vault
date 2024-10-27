import { AddressTxsUtxo } from '@mempool/mempool.js/lib/interfaces/bitcoin/addresses';
import { default as BtcMempool, defaultMempoolClient } from './mempool';
import { defaultClient as defaultBtcClient, getUnspentTransactionOutputs } from './bitcoin';
import Client from 'bitcoin-core-ts';

export { sendrawtransaction } from './bitcoin';

export { BtcMempool, defaultMempoolClient };

export async function getAddressUtxos(address: string, btcClient?: Client): Promise<AddressTxsUtxo[]> {
  //First try get utxo from bitcoin node
  console.log(`getUnspentTransactionOutputs of the address ${address} from the bitcoin node`);
  let utxos: AddressTxsUtxo[] = await getUnspentTransactionOutputs(address, btcClient);
  //Then try get utxo from mempool
  if (utxos.length == 0) {
     console.log(`getAddressTxsUtxo of the address ${address} from the mempool`);
    utxos = await defaultMempoolClient.addresses.getAddressTxsUtxo({ address });
  }
  return utxos;
}