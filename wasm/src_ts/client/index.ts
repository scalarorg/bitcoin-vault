import { AddressTxsUtxo } from '@mempool/mempool.js/lib/interfaces/bitcoin/addresses';
import { default as BtcMempool, defaultMempoolClient } from './mempool';
import { default as btcClient } from './bitcoin';;
export { BtcMempool, btcClient, defaultMempoolClient };

export async function getAddressUtxos(address: string): Promise<AddressTxsUtxo[]> {
  //First try get utxo from bitcoin node
  console.log(`getUnspentTransactionOutputs of the address ${address} from the bitcoin node`);
  let utxos: AddressTxsUtxo[] = await btcClient.getUnspentTransactionOutputs(address);
  //Then try get utxo from mempool
  if (utxos.length == 0) {
     console.log(`getAddressTxsUtxo of the address ${address} from the mempool`);
    utxos = await defaultMempoolClient.addresses.getAddressTxsUtxo({ address });
  }
  return utxos;
}