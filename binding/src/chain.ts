/**
 * Please follow go-utils/chain/chain.go for the chain type definition.
 */

export enum ChainType {
  Bitcoin = 0, // 0x00
  EVM = 1, // 0x01
  Solana = 2, // 0x02
  Cosmos = 3, // 0x03
}

export function chainTypeToString(chainType: ChainType): string {
  switch (chainType) {
    case ChainType.Bitcoin:
      return "Bitcoin";
    case ChainType.EVM:
      return "EVM";
    case ChainType.Solana:
      return "Solana";
    case ChainType.Cosmos:
      return "Cosmos";
    default:
      return "Unknown";
  }
}

export class DestinationChain {
  constructor(
    public readonly chainType: ChainType,
    public readonly chainId: bigint
  ) {}

  public toBytes(): Uint8Array {
    const bytes = new Uint8Array(8);
    bytes[0] = this.chainType;
    
    const buffer = new ArrayBuffer(8);
    const view = new DataView(buffer);
    view.setBigUint64(0, this.chainId, false); // false for big-endian
    bytes.set(new Uint8Array(buffer).slice(1), 1);
    
    return bytes;
  }

  static fromBytes(bytes: Uint8Array): DestinationChain | null {
    if (bytes.length !== 8) {
      return null;
    }

    const chainType = bytes[0];
    if (!validateChainType(chainType)) {
      return null;
    }

    // Create a copy of bytes and set first byte to 0
    const chainIdBytes = new Uint8Array(bytes);
    chainIdBytes[0] = 0;

    // Convert to BigInt using DataView for big-endian conversion
    const view = new DataView(chainIdBytes.buffer);
    const chainId = BigInt(view.getBigUint64(0));

    return new DestinationChain(chainType, chainId);
  }
}

export function validateChainType(chainType: number): boolean {
  return chainType <= ChainType.Cosmos;
}
