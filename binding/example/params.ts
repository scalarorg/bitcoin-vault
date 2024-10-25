import * as bitcoin from "bitcoinjs-lib";
import { z } from "zod";
import { extractPublicKeyFromWIF } from "../src";

const getNetwork = () => {
  let network = bitcoin.networks.regtest;
  if (process.env.NETWORK === "testnet") {
    network = bitcoin.networks.testnet;
  } else if (process.env.NETWORK === "mainnet") {
    network = bitcoin.networks.bitcoin;
  }
  return network;
};

const getChainID = () => {
  let destChainId = Number(process.env.DEST_CHAIN_ID || "").toString(16);
  if (destChainId.length % 2) {
    destChainId = "0" + destChainId;
  }
  return destChainId;
};

const ConfigParamsSchema = z.object({
  networkName: z.string().optional().default("regtest"),
  network: z.any(),
  mempoolUrl: z.string().optional().default("http://localhost:8999"),
  btcClientMode: z.enum(["mempool", "bitcoin"]).default("mempool"),
  btcHost: z.string().optional().default("localhost"),
  btcPort: z.string().optional().default("18332"),
  btcRpcUser: z.string().optional().default("user"),
  btcRpcPassword: z.string().optional().default("password"),
  walletName: z.string().optional().default("legacy"),
  bondHolderAddress: z.string().min(40),
  bondHolderPrivKey: z.string().length(52),
  bondHolderPublicKey: z.string().length(66),
  protocolPrivKey: z.string().length(52),
  protocolPublicKey: z.string().length(66),
  covenantPrivKeys: z.array(z.string()).length(5),
  covenantPublicKeys: z.array(z.string().length(66)).length(5),
  covenantQuorum: z.number().optional().default(1),
  tag: z.string().optional().default("01020304"),
  version: z.number().optional().default(0),
  destChainId: z.string().min(1).default("11111111111"),
  destUserAddress: z
    .string()
    .length(40)
    .default("0000000000000000000000000000000000000000"),
  destSmartContractAddress: z
    .string()
    .length(40)
    .default("0000000000000000000000000000000000000000"),
});

const network = getNetwork();

export const globalParams = ConfigParamsSchema.parse({
  networkName: process.env.NETWORK,
  network: network,
  btcClientMode: process.env.BTC_CLIENT_MODE,
  btcHost: process.env.BITCOIN_HOST,
  btcPort: process.env.BITCOIN_PORT,
  btcRpcUser: process.env.BITCOIN_RPC_USER,
  btcRpcPassword: process.env.BITCOIN_RPC_PASSWORD,
  walletName: process.env.WALLET_NAME,
  mempoolUrl: process.env.MEMPOOL_URL,
  bondHolderAddress: process.env.BOND_HOLDER_ADDRESS,
  bondHolderPrivKey: process.env.BOND_HOLDER_PRIVATE_KEY,
  bondHolderPublicKey:
    process.env.BOND_HOLDER_PRIVATE_KEY &&
    extractPublicKeyFromWIF(process.env.BOND_HOLDER_PRIVATE_KEY, network),
  protocolPrivKey: process.env.PROTOCOL_PRIVATE_KEY,
  protocolPublicKey:
    process.env.PROTOCOL_PRIVATE_KEY &&
    extractPublicKeyFromWIF(process.env.PROTOCOL_PRIVATE_KEY, network),
  covenantPrivKeys:
    process.env.COVENANT_PRIVKEYS && process.env.COVENANT_PRIVKEYS.split(","),
  covenantPublicKeys:
    process.env.COVENANT_PRIVKEYS &&
    process.env.COVENANT_PRIVKEYS.split(",")
      .map((key) => key.trim())
      .map((key) => extractPublicKeyFromWIF(key, network)),
  covenantQuorum: Number(process.env.COVENANT_QUORUM || 1),
  tag: process.env.TAG,
  version: Number(process.env.VERSION || 0),
  destChainId: getChainID(),
  destUserAddress: process.env.DEST_USER_ADDRESS,
  destSmartContractAddress: process.env.DEST_SMART_CONTRACT_ADDRESS,
});
