export type TNetwork = "bitcoin" | "testnet" | "testnet4" | "regtest";

export const NetworkKind: Record<TNetwork, 0 | 1> = {
  bitcoin: 0,
  testnet: 1,
  testnet4: 1,
  regtest: 1,
};

export const isTestnet = (network: TNetwork) => NetworkKind[network] === 1;
