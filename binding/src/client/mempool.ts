import { useAddresses } from "@mempool/mempool.js/lib/app/bitcoin/addresses";
import { useBlocks } from "@mempool/mempool.js/lib/app/bitcoin/blocks";
import { useDifficulty } from "@mempool/mempool.js/lib/app/bitcoin/difficulty";
import { useFees } from "@mempool/mempool.js/lib/app/bitcoin/fees";
import { useMempool } from "@mempool/mempool.js/lib/app/bitcoin/mempool";
import { useTransactions } from "@mempool/mempool.js/lib/app/bitcoin/transactions";
import axios, { AxiosRequestConfig } from "axios";
import { MempoolInstance } from "@mempool/mempool.js/lib/interfaces/bitcoin/mempool";
import { AddressInstance } from "@mempool/mempool.js/lib/interfaces/bitcoin/addresses";
import { BlockInstance } from "@mempool/mempool.js/lib/interfaces/bitcoin/blocks";
import { DifficultyInstance } from "@mempool/mempool.js/lib/interfaces/bitcoin/difficulty";
import { FeeInstance } from "@mempool/mempool.js/lib/interfaces/bitcoin/fees";
import { TxInstance } from "@mempool/mempool.js/lib/interfaces/bitcoin/transactions";
import { WsInstance } from "@mempool/mempool.js/lib/interfaces/bitcoin/websockets";
import { TNetwork } from "@/types";

class BtcMempool {
  config: AxiosRequestConfig;
  addresses: AddressInstance;
  blocks: BlockInstance;
  difficulty: DifficultyInstance;
  fees: FeeInstance;
  mempool: MempoolInstance;
  transactions: TxInstance;
  constructor(mempoolUrl: string) {
    this.config = {
      baseURL: mempoolUrl,
    };
    let api = axios.create(this.config);
    this.addresses = useAddresses(api);
    this.blocks = useBlocks(api);
    this.difficulty = useDifficulty(api);
    this.fees = useFees(api);
    this.mempool = useMempool(api);
    this.transactions = useTransactions(api);
  }
}

let mempoolClients: Record<TNetwork, BtcMempool | null> = {
  bitcoin: null,
  testnet: null,
  testnet4: null,
  regtest: null,
};

export const getMempoolClient = (network: TNetwork): BtcMempool => {
  if (!mempoolClients[network]) {
    if (network === "bitcoin" || network === "regtest") {
      mempoolClients[network] = new BtcMempool("https://mempool.space/api");
    } else {
      mempoolClients[network] = new BtcMempool(
        `https://mempool.space/${network}/api`
      );
    }
  }
  return mempoolClients[network];
};

export const defaultMempoolClient = getMempoolClient("bitcoin");
export default BtcMempool;

// export type BtcMempoolInstance = {
//     config: AxiosRequestConfig;
//     addresses: AddressInstance;
//     blocks: BlockInstance;
//     difficulty: DifficultyInstance;
//     fees: FeeInstance;
//     mempool: MempoolInstance;
//     transactions: TxInstance;
// }
// const config: AxiosRequestConfig = {
//     baseURL: globalParams.mempoolUrl || 'https://mempool.space/api',
// };

// let api = axios.create(config);
// export const btcMempoolInstance: BtcMempoolInstance = {
//     config,
//     addresses: useAddresses(api),
//     blocks: useBlocks(api),
//     difficulty: useDifficulty(api),
//     fees: useFees(api),
//     mempool: useMempool(api),
//     transactions: useTransactions(api),
// };
