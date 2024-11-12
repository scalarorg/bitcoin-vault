import Client from "bitcoin-core-ts";
import * as bitcoin from "bitcoinjs-lib";
import { z } from "zod";
import {
  TBuildUnsignedStakingPsbt,
  getAddressUtxos,
  sendrawtransaction,
  getMempoolClient,
  bytesToHex,
  hexToBytes,
  VaultUtils,
} from "../src";

export const readEnv = async () => {
  const envText = await Bun.file(StaticEnv.BTC_ENV_PATH).text();
  const envMap = new Map(
    envText
      .split("\n")
      .filter((line) => line.trim() && !line.startsWith("#"))
      .map((line) => {
        const [key, value] = line.split("=").map((part) => part.trim());
        return [key, value];
      })
  );
  return envMap;
};

const StaticEnvSchema = z.object({
  TAG: z.string().optional().default("SCALAR"),
  SERVICE_TAG: z.string().optional().default("light"),
  VERSION: z.number().optional().default(0),
  NETWORK: z
    .enum(["bitcoin", "testnet", "regtest", "testnet4"])
    .optional()
    .default("regtest"),
  HOST: z.string().optional().default("localhost"),
  PORT: z.string().optional().default("18332"),
  USERNAME: z.string().optional().default("user"),
  PASSWORD: z.string().optional().default("password"),
  WALLET_NAME: z.string().optional().default("legacy"),
  STAKING_AMOUNT: z.bigint().optional().default(BigInt(10_000)),
  HAVE_ONLY_CUSTODIAL: z.boolean().optional().default(false),
  CUSTODIAL_QUORUM: z.number().optional().default(3),
  CUSTODIAL_NUMBER: z.number().optional().default(5),
  DEST_CHAIN_ID: z.bigint().min(BigInt(1)).default(BigInt(11155111)),
  DEST_USER_ADDRESS: z
    .string()
    .length(40)
    .default("24a1dB57Fa3ecAFcbaD91d6Ef068439acEeAe090"),
  DEST_SMART_CONTRACT_ADDRESS: z
    .string()
    .length(40)
    .default("B91e3A8Ef862567026d6F376c9F3d6b814Ca4337"),
  BTC_ENV_PATH: z.string().optional().default(".bitcoin/.env.btc"),
  BOND_HOLDER_ADDRESS: z.string().optional(),
  BOND_HOLDER_PRIVATE_KEY: z.string().optional(),
});

export const StaticEnv = StaticEnvSchema.parse({
  TAG: process.env.TAG,
  VERSION: process.env.VERSION,
  NETWORK: process.env.NETWORK,
  HOST: process.env.HOST,
  PORT: process.env.PORT,
  USERNAME: process.env.USERNAME,
  PASSWORD: process.env.PASSWORD,
  WALLET_NAME: process.env.WALLET_NAME,
  STAKING_AMOUNT: process.env.STAKING_AMOUNT,
  HAVE_ONLY_CUSTODIAL: process.env.HAVE_ONLY_CUSTODIAL,
  CUSTODIAL_QUORUM: process.env.CUSTODIAL_QUORUM,
  CUSTODIAL_NUMBER: process.env.CUSTODIAL_NUMBER,
  DEST_CHAIN_ID: process.env.DEST_CHAIN_ID,
  DEST_USER_ADDRESS: process.env.DEST_USER_ADDRESS,
  DEST_SMART_CONTRACT_ADDRESS: process.env.DEST_SMART_CONTRACT_ADDRESS,
  BTC_ENV_PATH: process.env.BTC_ENV_PATH,
  BOND_HOLDER_ADDRESS: process.env.BOND_HOLDER_ADDRESS,
  BOND_HOLDER_PRIVATE_KEY: process.env.BOND_HOLDER_PRIVATE_KEY,
});

export const setUpTest = async () => {
  const envMap = await readEnv();
  console.log("envMap", envMap);
  console.log("StaticEnv", StaticEnv);

  const vaultUtils = VaultUtils.getInstance(
    StaticEnv.TAG,
    StaticEnv.SERVICE_TAG,
    StaticEnv.VERSION,
    StaticEnv.NETWORK
  );

  const network = vaultUtils.getNetwork();

  const btcClient = new Client({
    network: StaticEnv.NETWORK === "testnet4" ? "testnet" : StaticEnv.NETWORK,
    host: StaticEnv.HOST,
    port: StaticEnv.PORT,
    wallet: StaticEnv.WALLET_NAME,
    username: StaticEnv.USERNAME,
    password: StaticEnv.PASSWORD,
  });

  const custodialPubkeys = envMap.get("COVENANT_PUBKEYS")?.split(",");
  if (!custodialPubkeys) {
    throw new Error("COVENANT_PUBKEYS is not set");
  }

  const custodialPubkeysBuffer = new Uint8Array(
    33 * StaticEnv.CUSTODIAL_NUMBER
  );

  for (let i = 0; i < StaticEnv.CUSTODIAL_NUMBER; i++) {
    custodialPubkeysBuffer.set(hexToBytes(custodialPubkeys[i]), i * 33);
  }

  const bondHolderAddress =
    StaticEnv.BOND_HOLDER_ADDRESS || envMap.get("BOND_HOLDER_ADDRESS");
  if (!bondHolderAddress) {
    throw new Error("BOND_HOLDER_ADDRESS is not set");
  }

  const bondHolderWif =
    StaticEnv.BOND_HOLDER_PRIVATE_KEY || envMap.get("BOND_HOLDER_PRIVATE_KEY");
  if (!bondHolderWif) {
    throw new Error("BOND_HOLDER_PRIVATE_KEY is not set");
  }

  const keyPair = VaultUtils.ECPair.fromWIF(bondHolderWif, network);

  const protocolPubkey = envMap.get("PROTOCOL_PUBLIC_KEY");
  if (!protocolPubkey) {
    throw new Error("PROTOCOL_PUBLIC_KEY is not set");
  }

  const protocolPrivkey = envMap.get("PROTOCOL_PRIVATE_KEY");
  if (!protocolPrivkey) {
    throw new Error("PROTOCOL_PRIVATE_KEY is not set");
  }

  console.log("STAKER_PUBKEY", bytesToHex(keyPair.publicKey));

  return {
    network: vaultUtils.getNetwork(),
    btcClient,
    vaultUtils,
    mempoolClient: getMempoolClient(StaticEnv.NETWORK),
    custodialPubkeys: custodialPubkeysBuffer,
    stakerAddress: bondHolderAddress,
    stakerWif: bondHolderWif,
    stakerPubKey: keyPair.publicKey,
    stakerKeyPair: keyPair,
    protocolPubkey: hexToBytes(protocolPubkey),
    protocolKeyPair: VaultUtils.ECPair.fromWIF(protocolPrivkey, network),
  };
};

export const setupStakingTx = async () => {
  const TestSuite = await setUpTest();
  console.log("TestSuite.stakerAddress", TestSuite.stakerAddress);
  const addressUtxos = await getAddressUtxos({
    address: TestSuite.stakerAddress,
    btcClient: TestSuite.btcClient,
  });
  const { fastestFee: feeRate } =
    await TestSuite.mempoolClient.fees.getFeesRecommended(); // Get this from Mempool API
  //1. Build the unsigned psbt

  const params: TBuildUnsignedStakingPsbt = {
    stakingAmount: StaticEnv.STAKING_AMOUNT,
    stakerPubkey: TestSuite.stakerPubKey,
    stakerAddress: TestSuite.stakerAddress,
    protocolPubkey: TestSuite.protocolPubkey,
    custodialPubkeys: TestSuite.custodialPubkeys,
    covenantQuorum: StaticEnv.CUSTODIAL_QUORUM,
    haveOnlyCovenants: StaticEnv.HAVE_ONLY_CUSTODIAL,
    destinationChainId: StaticEnv.DEST_CHAIN_ID,
    destinationSmartContractAddress: hexToBytes(
      StaticEnv.DEST_SMART_CONTRACT_ADDRESS
    ),
    destinationRecipientAddress: hexToBytes(StaticEnv.DEST_USER_ADDRESS),
    availableUTXOs: addressUtxos,
    feeRate,
    rbf: true,
  };

  const { psbt: unsignedVaultPsbt, fee: estimatedFee } =
    TestSuite.vaultUtils.buildStakingOutput(params);
  //2. Sign the psbt
  const signedPsbt = TestSuite.vaultUtils.signPsbt({
    psbt: unsignedVaultPsbt,
    wif: TestSuite.stakerWif,
    finalize: true,
  });

  //3. Extract the transaction and broadcast
  let transaction = signedPsbt.extractTransaction(false);
  const txHexfromPsbt = transaction.toHex();
  logToJSON({ txHexfromPsbt, fee: estimatedFee });
  const txid = await sendrawtransaction(txHexfromPsbt, TestSuite.btcClient);
  console.log("Successfully broadcasted txid", txid);
  const scriptPubkeyOfLocking = transaction.outs[0].script;
  return {
    txid,
    txHexfromPsbt,
    TestSuite,
    scriptPubkeyOfLocking,
  };
};

export function logToJSON(any: any) {
  console.log(
    JSON.stringify(
      any,
      (k, v) => {
        if (v.type === "Buffer") {
          return Buffer.from(v.data).toString("hex");
        }
        if (k === "network") {
          switch (v) {
            case bitcoin.networks.bitcoin:
              return "bitcoin";
            case bitcoin.networks.testnet:
              return "testnet";
            case bitcoin.networks.regtest:
              return "regtest";
          }
        }
        if (typeof v == "bigint") {
          return v.toString(10);
        }
        return v;
      },
      2
    )
  );
}
