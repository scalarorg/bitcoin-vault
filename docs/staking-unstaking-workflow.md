# Bitcoin Vault Staking and Unstaking Workflow

This document provides a comprehensive guide to the staking and unstaking process in the Bitcoin Vault system, designed for team members who may not be familiar with Bitcoin, PSBT (Partially Signed Bitcoin Transactions), or Taproot.

## Table of Contents

1. [Overview](#overview)
2. [Key Concepts](#key-concepts)
3. [Staking Workflow](#staking-workflow)
4. [Unstaking Workflow](#unstaking-workflow)
5. [Code Implementation Details](#code-implementation-details)
6. [Error Handling](#error-handling)
7. [Security Considerations](#security-considerations)

## Overview

The Bitcoin Vault system implements a custodial staking mechanism where users can lock their Bitcoin in a special vault transaction that requires multiple custodian signatures to unlock. This provides enhanced security through multi-signature requirements while enabling cross-chain functionality.

## Key Concepts

### Bitcoin Basics
- **UTXO (Unspent Transaction Output)**: Bitcoin uses a UTXO model where each transaction consumes previous outputs and creates new ones
- **Transaction**: A data structure that transfers value from inputs (previous UTXOs) to outputs (new UTXOs)
- **Script**: Bitcoin's programming language that defines spending conditions

### Advanced Bitcoin Features
- **PSBT (Partially Signed Bitcoin Transaction)**: A standard format for transactions that need multiple signatures
- **Taproot**: Bitcoin's latest upgrade enabling more complex smart contracts with better privacy
- **Multi-signature**: Requiring multiple private keys to authorize a transaction

### Vault-Specific Concepts
- **Staking Transaction**: Locks Bitcoin in a vault with specific unlock conditions
- **Custodian-Only Mode**: Vault type requiring only custodian signatures (no user signature needed for unlock)
- **Custodian Quorum**: Minimum number of custodian signatures required (e.g., 3 out of 5)

## Staking Workflow

The staking process locks user Bitcoin into a vault transaction with specific unlock conditions.

### Step 1: Prepare Staking Transaction

**Function**: `prepare_staking_tx()`
**Location**: `vault/src/utils/suite.rs:130-268`

```rust
pub fn prepare_staking_tx(
    &self,
    amount: u64,                    // Amount to stake in satoshis
    taproot_tree_type: TaprootTreeType, // Vault type (CustodianOnly/UPC)
    account: SuiteAccount,          // User's account info
    dest: DestinationInfo,          // Cross-chain destination
    utxos: Vec<NeededUtxo>,        // Available UTXOs to spend
) -> Result<Transaction, anyhow::Error>
```

**What it does**:
1. **Build Locking Output**: Creates a special Bitcoin output using `build_locking_output()` that defines:
   - Custodian public keys and quorum requirements
   - Cross-chain destination information
   - Locking amount

2. **Create Transaction Structure**: Builds an unsigned transaction with:
   - Inputs: User's UTXOs to be spent
   - Outputs: Vault locking output + change output (if needed)
   - Fee calculation based on transaction size

3. **Create PSBT**: Converts the unsigned transaction to PSBT format with:
   - Witness UTXO information for each input
   - Taproot key origins for signing
   - Internal key information

4. **Sign Transaction**: Uses `sign_psbt_by_single_key()` to sign with user's private key

5. **Broadcast**: Sends the signed transaction to the Bitcoin network via `send_psbt()`

### Step 2: Transaction Broadcasting

**Function**: `send_psbt()`
**Location**: `vault/src/utils/suite.rs:380-449`

**What it does**:
1. **Extract Transaction**: Converts finalized PSBT to raw transaction
2. **Serialize**: Converts transaction to hex format for network transmission
3. **Submit to Network**: Uses Bitcoin RPC to broadcast transaction
4. **Retry Logic**: Handles mempool congestion with exponential backoff
5. **Confirmation**: Waits for transaction to appear in mempool/blockchain

## Unstaking Workflow

The unstaking process unlocks Bitcoin from vault transactions, requiring multiple custodian signatures.

### Step 1: Build Unstaking PSBT

**Function**: `build_batch_custodian_only_unstaking_tx()`
**Location**: `vault/src/utils/suite.rs:302-328`

```rust
pub fn build_batch_custodian_only_unstaking_tx(
    &self,
    staking_txs: &[Transaction],    // Previous staking transactions
    outputs: Vec<TxOut>,            // Where to send unlocked Bitcoin
) -> Psbt
```

**What it does**:
1. **Identify Inputs**: Maps staking transactions to spendable inputs:
   - Transaction ID and output index (VOUT)
   - Amount and script from the vault output

2. **Build Unlocking PSBT**: Creates unsigned PSBT using `build_unlocking_psbt()` with:
   - Custodian public keys and quorum requirements
   - Fee rate and RBF (Replace-By-Fee) settings
   - Session sequence for batch processing

### Step 2: Multi-Signature Signing

**Function**: `sign_psbt_by_single_key()`
**Location**: `vault/src/core/signing.rs:11-38`

```rust
fn sign_psbt_by_single_key(
    psbt: &mut Psbt,
    privkey: &[u8],                 // Custodian private key
    network_kind: NetworkKind,      // Bitcoin network (mainnet/testnet)
    finalize: bool,                 // Whether to finalize after signing
) -> Result<(Self::PsbtHex, SigningKeyMap), CoreError>
```

**What it does**:
1. **Key Preparation**: Converts private key bytes to proper key format
2. **Signature Generation**: Creates cryptographic signatures for each input
3. **PSBT Update**: Adds signatures to the PSBT structure
4. **Key Mapping**: Tracks which keys were used for signing

**Multi-Custodian Process**:
```rust
// Each custodian signs the PSBT
for privkey in signing_privkeys {
    <VaultManager as Signing>::sign_psbt_by_single_key(
        &mut unstaked_psbt,
        privkey.as_slice(),
        TEST_SUITE.network_id(),
        false,  // Don't finalize until all signatures collected
    ).unwrap();
}
```

### Step 3: PSBT Finalization

**Function**: `finalize()`
**Location**: `vault/src/core/psbt.rs:142-151`

**What it does**:
1. **Signature Aggregation**: Combines all custodian signatures
2. **Script Witness Creation**: Builds the final unlock script
3. **Transaction Completion**: Creates a fully valid Bitcoin transaction

### Step 4: Transaction Broadcasting

Same as staking workflow - uses `send_psbt_by_rpc()` to broadcast the finalized transaction.

## Code Implementation Details

### Core Traits and Implementations

1. **CustodianOnly Trait**: Handles custodian-only vault operations
   - `build_locking_output()`: Creates vault locking conditions
   - `build_unlocking_psbt()`: Creates unlocking transaction template

2. **Signing Trait**: Manages cryptographic signing
   - `sign_psbt_by_single_key()`: Signs PSBT with single private key
   - `sign_psbt_and_collect_tap_script_sigs()`: Collects Taproot signatures

3. **SignByKeyMap Trait**: Batch signing operations
   - `sign_by_key_map()`: Signs with multiple keys simultaneously
   - `finalize()`: Completes PSBT to valid transaction

### Key Data Structures

```rust
// Represents a previous transaction output to spend
struct PreviousOutpoint {
    outpoint: OutPoint,        // Transaction ID + output index
    amount_in_sats: Amount,    // Value of the output
    script_pubkey: ScriptBuf,  // Locking script
}

// Parameters for custodian-only unlocking
struct CustodianOnlyUnlockingParams {
    inputs: Vec<PreviousOutpoint>,     // Vault outputs to unlock
    outputs: Vec<TxOut>,               // Destination outputs
    custodian_pubkeys: Vec<PublicKey>, // Custodian public keys
    custodian_quorum: u8,              // Required signature count
    fee_rate: u64,                     // Transaction fee rate
    rbf: bool,                         // Replace-by-fee enabled
    session_sequence: u64,             // Batch processing sequence
    custodian_group_uid: [u8; 32],     // Custodian group identifier
}
```

## Error Handling

The system implements comprehensive error handling:

### Transaction Broadcasting Errors
- **Mempool Chain Too Long**: Automatic retry with exponential backoff
- **Network Connectivity**: Retry logic with timeout
- **Invalid Transaction**: Immediate failure with detailed error

### Signing Errors
- **Invalid Private Key**: Key format validation
- **Insufficient Signatures**: Quorum requirement checking
- **Wrong Network**: Network mismatch detection

### Example Error Handling
```rust
match TEST_SUITE.send_psbt_by_rpc(unstaked_psbt) {
    Ok(Some(result)) => {
        log_tx_result(&result);  // Success - log transaction details
    }
    Ok(None) => {
        panic!("tx not found");  // Transaction not in mempool
    }
    Err(e) => {
        panic!("tx not found with error: {}", e);  // Network/RPC error
    }
}
```

## Security Considerations

### Multi-Signature Security
- **Quorum Requirements**: Prevents single point of failure
- **Key Distribution**: Custodian keys stored separately
- **Signature Verification**: Each signature cryptographically verified

### Transaction Security
- **PSBT Validation**: All inputs/outputs validated before signing
- **Fee Verification**: Prevents excessive fee attacks
- **RBF Protection**: Replace-by-fee settings controlled

### Network Security
- **Retry Logic**: Prevents transaction loss due to network issues
- **Confirmation Waiting**: Ensures transaction propagation
- **Error Logging**: Comprehensive audit trail

## Testing and Validation

The test suite (`test_custodians.rs`) provides comprehensive examples:

- `test_partial_unstaking()`: Single vault unlock
- `test_partial_unstaking_multiple_utxos()`: Batch vault unlock
- `test_parallel_signing_multiple_utxos()`: Concurrent signing
