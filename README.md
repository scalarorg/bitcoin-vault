# Bitcoin Vault Documentation

> 2025-01-14 @0xdavid

## Project Overview

Bitcoin Vault is a comprehensive solution for managing Bitcoin vault transactions, providing functionality for **staking**, **unstaking**, and **custodian management** on bitcoin network; it provides a **rust** library, **wasm** bindings, **go** ffi bindings, and a **typescript** sdk.

## Runtime Requirements

- Runs natively on Rust
- Other languages require bindings:
  - Go: via FFI bindings
  - Browser/Node.js: via WebAssembly
  - TypeScript: via WebAssembly

## Core Components

### 🌟 Vault Library (vault)

> Details in [vault/README.md](./vault/README.md)
> The foundational Rust library implementing core Bitcoin vault functionality.

#### Key Features

1. Staking transaction creation:

   - Create a staking output with supporting multiple taproot spend paths:

     - [x] P2TR with multi-signatures of custodian key holders
     - [x] P2TR with user and protocol signatures
     - [x] P2TR with user and multi-signature of custodian key holders
     - [x] P2TR with protocol and multi-signature of custodian key holders

   - Create a staking output just only custodian key holders spend path:
     - [x] P2TR with multi-signature of custodian key holders

2. Unstaking transaction management for:

   - [x] P2TR with multi-signatures of custodian key holders
   - [x] P2TR with user and protocol signatures
   - [x] P2TR with user and multi-signature of custodian key holders
   - [x] P2TR with protocol and multi-signature of custodian key holders

3. Signing:

   - [x] Signing with ECDSA
   - [x] Signing with Schnorr

4. PSBT:

   - [x] PSBT parsing
   - [x] PSBT signing
   - [x] PSBT dynamic finalization

5. Details:

   - Supports RBF (Replace-By-Fee) for staking and unstaking transactions
   - Supports musig and OP_CHECKSIGADD for custodian script with quorum-based approval system
   - Supports segwit and p2tr addresses

6. Future development:
   - Apply musig2 for custodians
   - Apply OP_CAT

#### Directory Structure

```
vault/
├── src/
│   ├── core/           # Core vault functionality
│   ├── parser/         # Transaction parsing
│   ├── types/          # Type definitions
│   └── lib.rs          # Library entry point
└── tests/              # Integration tests
```

#### Testing

- At the root of the project, run `./bitcoin.sh run` to start a bitcoin node
- It will auto import the wif and generate the addresses for bond holder in `.bitcoin` directory
- Please fill into `vault/.env.test` file, check `vault/.env.example` for more details
- Navigate to `test` directory and run each test case only with commented script at the beginning of the file

Eg: `cargo test --package vault --test mod -- test_e2e::test_staking --exact --show-output`

### 🌟 WebAssembly Module (wasm)

Browser and Node.js compatible WebAssembly bindings for the vault library.

#### Key Features

- Cross-platform compatibility
- Browser support
- Node.js support
- TypeScript definitions
- Binary Data Handling
- Efficient serialization
- Memory management
- Buffer conversions

#### Directory Structure

```
wasm/
├── src/               # Rust WASM source
├── scripts/           # Build scripts
└── dist/             # Output directory
    ├── bitcoin-vault-node_bg.wasm         # Node.js specific output
    └── bitcoin-vault-web_bg.wasm          # Browser specific output
    ... other files
```

#### Build System

##### Prerequisite

1. Rust toolchain with wasm32 target:

```bash
rustup toolchain install stable --target wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown

```

2. Wasm pack:

```bash
cargo install wasm-pack
```

3. Bun:

```bash
curl -fsSL https://bun.sh/install | bash
```

##### Build

```bash
cd wasm && bun run build
```

##### Troubleshooting

- If you encounter the error like `error: linking with`cc`failed: exit status 1`, you can try to install clang:

```bash
brew install llvm

echo 'export PATH="/opt/homebrew/opt/llvm/bin:$PATH"' >> ~/.zshrc
```

- [Reference](https://github.com/briansmith/ring/issues/1824#issuecomment-2059955073)

### 🌟 TypeScript SDK (binding)

High-level JavaScript/TypeScript SDK for application integration.

#### Key Features

- Transaction Management
  - Staking transaction builder
  - Unstaking workflow
  - PSBT handling
  - Fee estimation
- Network Support
  - Mainnet support
  - Testnet support
  - Regtest capabilities
- Utility Functions
  - Address validation
  - Key management

#### Usage Example

```typescript
// Initialize vault utils
const vault = VaultUtils.getInstance({
  network: "testnet4",
  tag: "SCALAR",
  serviceTag: "pools",
  version: 1,
});

// Create staking transaction
const { psbt: unsignedVaultPsbt, fee: estimatedFee } =
  TestSuite.vaultUtils.buildStakingOutput({
    stakingAmount: BigInt(100000000),
    stakerPubkey: Buffer.from("02...", "hex"),
    stakerAddress: Buffer.from("bc1...", "hex"),
    protocolPubkey: Buffer.from("02...", "hex"),
    custodialPubkeys: [Buffer.from("02...", "hex"), ...].concat(),
    custodianQuorum: 1,
    haveOnlycustodians: false,
    destinationChain: new DestinationChain(
      ChainType.EVM,
      11155111
    ),
    destinationContractAddress: hexToBytes("0x..."),
    destinationRecipientAddress: hexToBytes("0x..."),
    availableUTXOs: addressUtxos,
    feeRate,
    rbf: true,
  });

// Create unstaking transaction

const params: TBuildUnsignedUnstakingUserProtocolPsbt = {
  input,
  output,
  stakerPubkey: Buffer.from("02...", "hex"),
  protocolPubkey: Buffer.from("02...", "hex"),
  custodianPubkeys: [Buffer.from("02...", "hex"), ...].concat(),
  custodianQuorum: 1,
  haveOnlycustodians: false,
  feeRate: BigInt(feeRate),
  rbf: true,
};

      // Build the unsigned psbt
const psbtHex =
  TestSuite.vaultUtils.buildUnsignedUnstakingUserProtocolPsbt(params);
```

- More details can be found in `binding/test`

### 🌟 FFI Bindings (ffi)

Foreign Function Interface for language interoperability.

#### Supported Languages

- Go

This module is used for **vault** library to interact with **bitcoin** network.

- [x] PSBT signing
- [x] Staking transaction parser

##### Testing

- Build the **vault** rust library
- Run `cargo build --release` in `ffi/` directory
- Copy the output library to `ffi/go-vault/lib` directory
- Run `cd ffi/go-vault && make test`

#### Installation

- `go get github.com/scalarorg/bitcoin-vault/ffi`
- `Add CGO_LDFLAGS="path/to/libbitcoin_vault_ffi.dylib" to your build command`

#### License

MIT License - See LICENSE file for details
This documentation provides a more structured overview of the Bitcoin Vault project without excessive code examples. Let me know if you'd like me to expand on any particular section!
