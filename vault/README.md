# Bitcoin Vault Core

The core module provides the fundamental building blocks for Bitcoin Vault operations, implementing staking, unstaking, and transaction signing functionalities using Bitcoin's Taproot

## Core Features

1. Staking

- Supports two staking modes:

  - Standard staking with user, protocol, and custodian keys
  - Custodian-only staking

- Implements Taproot script trees for spending conditions
- Handles OP_RETURN data scripts for metadata

2. Unstaking

- Supports multiple unstaking paths:
  - User + Protocol
  - Custodians + Protocol
  - Custodians + User
  - Custodians Only
- Implements RBF (Replace-By-Fee) functionality
- Handles fee distribution and change outputs

3. Transaction Signing

- Supports multiple signing paths:
  - User + Protocol
  - Custodians + Protocol
  - Custodians + User
  - Custodians Only
- Handles transaction aggregation and signing
- Supports partial signing for multi-signature transactions

```rust
pub trait Signing {
    type PsbtHex;
    type TxHex;

    fn sign_psbt_by_single_key(...) -> Result<Self::PsbtHex, CoreError>;
    fn sign_psbt_and_collect_tap_script_sigs(...) -> Result<Vec<TapScriptSig>, CoreError>;
    fn aggregate_tap_script_sigs(...) -> Result<Self::PsbtHex, CoreError>;
    fn finalize_psbt_and_extract_tx(...) -> Result<Self::TxHex, CoreError>;
}
```

4. Script Types

- Locking Script:

  - Taproot-based scripts implementing spending conditions
  - Support for multiple spending paths
  - Custodian quorum enforcement

- Data Script:

  - `OP_RETURN` scripts for metadata
  - Embedded data format:
    - Tag hash (6 bytes)
    - Version (1 byte)
    - Network ID (1 byte)
    - Flags (1 byte)
    - Service tag hash (5 bytes)
    - Additional metadata like destination chain, recipient address, etc.

5. Taproot Tree Structure

> > Details: [taproot.md](../docs/taproot.md)

- Standard tree:

```text
                Root
                 │
        ┌────────┴────────┐
        │                 │
        1                 2
        │           ┌─────┴─────┐
        │           │           │
        │           3           4
        │           │           │
    User+Protocol  Cov+Protocol  Cov+User
```

- Custodian-only tree:

```text
      Root
       │
       1
       │
   Custodians
```

6. Usage Example

```rust

let vault_manager = VaultManager::new(
    tag,
    service_tag,
    version,
    network_id
);


/// Stake

let staking_output = vault_manager.build(&BuildStakingParams {
    user_pub_key,
    protocol_pub_key,
    custodian_pub_keys,
    custodian_quorum,
    staking_amount,
    destination_chain,
    destination_token_address,
    destination_recipient_address,
})?;


/// Unstake
let psbt = vault_manager.build(
    &BuildUnstakingParams {
        input,
        locking_script,
        user_pub_key,
        protocol_pub_key,
        custodian_pub_keys,
        custodian_quorum,
        rbf,
        fee_rate,
    },
    UnstakingType::UserProtocol
)?;

```

- More in tests: [tests](./tests/)

## Run Tests

- Setup `.env.test.regtest` for regtest
- Setup `.env.test.testnet4` for testnet4

### Locally

1. Start the bitcoin node regtest:

```sh
./bitcoin.sh run <bond_holder_wif>
```

- This will start a Bitcoin node in **regtest** mode.
- Then import the private key (wif) of the wallet into the Bitcoin node.
- Finally, it dumps an taproot address into the `.bitcoin/staker-p2tr.txt` file.
- Copy the address and replace in the `.env.test.regtest` file. Eg `BOND_HOLDER_ADDRESS=bcrt1p...`

2. Run the tests:

```sh
./scripts/test.sh <test_file> <test_name>
```

- `test_file`: The name of the test file to run.
- `test_name`: The name of the test to run.

### Testnet4

1. Add overriden env variables in `.env.test.testnet4`

```
# Override default network and wallet settings
NETWORK=testnet4
BTC_NODE_ADDRESS=
BTC_NODE_USER=
BTC_NODE_PASSWORD=
BOND_HOLDER_ADDRESS=
BOND_HOLDER_WALLET=
```

2. Run the tests:

```sh
TEST_ENV=testnet4 ./test.sh <test_file> <test_name>
```
