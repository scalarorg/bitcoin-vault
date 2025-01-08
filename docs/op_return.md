# OP RETURN

## 2024-11-18

### The structure of OP_RETURN in 2nd TxOut of Staking TX:

| FIELD                  | SIZE     | DESCRIPTION                                            | EXAMPLE                     |
| ---------------------- | -------- | ------------------------------------------------------ | --------------------------- |
| Tag                    | 6 bytes  | Tag of the provider tx                                 | `0x5343414c4152`, b"SCALAR" |
| Version                | 1 byte   | Version of the protocol                                | `0x01`                      |
| Network                | 1 byte   | BTC Network Kind, `0` for mainnet, `1` for others      | `0x01`                      |
| Flags                  | 1 byte   | [See #Flags](#flags)                                   | `0x00`                      |
| Protocol tag           | 5 bytes  | For display purpose                                    | `0x6C69676874`, b"light"    |
| Covenant Quorum        | 1 byte   | Number of quorum keys                                  | `0x01`                      |
| Dest Chain             | 8 bytes  | Destination chain info, [See #Dest Chain](#dest-chain) | `0x01`                      |
| Token Contract Address | 20 bytes | ERC20 contract address on destination chain            | `0x01`                      |
| Dest Recipient Address | 20 bytes | Recipient address on destination chain                 | `0x01`                      |

#### Length of OP_RETURN

- In case of full fields, the length of OP_RETURN is 63 bytes.

#### Flags

- The flags is designed as feature flags.
- It contains 8 bits.
- Structure:

  - bit 7-6: type of the taproot tree (can be extended in the next bits)
    - `00`: one branch, only keys (not implemented yet, reserved for future)
    - `01`: one branch, only covenants
    - `10`: more than one branch, and dont have only-covenants feature
    - `11`: more than one branch, and have only-covenants feature
  - other bits: reserved for future features:
  - bit-0: for unstaking use, if set, the unstaking tx will be used
    - `-------1`: unstaking tx
    - `------0`: staking tx

- Example:

  ```rust
  pub enum TaprootTreeType {
    OneBranchOnlyKeys = 0b00000000,
    OneBranchOnlyCovenants = 0b01000000,
    ManyBranchNoCovenants = 0b10000000,
    ManyBranchWithCovenants = 0b11000000,

    OneBranchOnlyCovenants_Unstaking = 0b01000001,
  }
  ```

#### Dest Chain

- The destination chain is the chain that the user wants to stake to.
- It contains 8 bytes:
- Structure:

  - The first byte is the type of the networks:
    - `0x00`: Bitcoin, usually not used
    - `0x01`: EVM compatible chain
    - `0x02`: Solana
    - `0x03`: Cosmos
    - ...
  - The next 7 bytes is the chain id of the chain.
    - `0x01` is the chain id of Ethereum
    - `0xAA36A7` is the chain id of Sepolia

- Example:
  - `0x0100000000AA36A7` is the destination chain of Sepolia
