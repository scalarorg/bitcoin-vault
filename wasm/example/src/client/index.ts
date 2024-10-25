import { AddressTxsUtxo } from '@mempool/mempool.js/lib/interfaces/bitcoin/addresses';
import { globalParams } from '../params';
import { default as BtcMempool } from './mempool';
import { default as btcClient } from './bitcoin';;
export { BtcMempool, btcClient };
    
export const mempoolClient = new BtcMempool(globalParams.mempoolUrl || 'https://mempool.space/api');

export async function getAddressUtxos(address: string): Promise<AddressTxsUtxo[]> {
  //First try get utxo from bitcoin node
  console.log(`getUnspentTransactionOutputs of the address ${address} from the bitcoin node`);
  let utxos: AddressTxsUtxo[] = await btcClient.getUnspentTransactionOutputs(address);
  //Then try get utxo from mempool
  if (utxos.length == 0) {
     console.log(`getAddressTxsUtxo of the address ${address} from the mempool`);
    utxos = await mempoolClient.addresses.getAddressTxsUtxo({ address });
  }
  return utxos;
}