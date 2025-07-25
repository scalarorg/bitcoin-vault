# Feature Development Guide

This guide helps developers create new features in the Bitcoin Vault project, specifically in the `/vault/src/core/feat/` directory.

## Table of Contents

1. [Overview](#overview)
2. [Feature Architecture](#feature-architecture)
3. [Taproot Trees](#taproot-trees)
4. [Bitcoin Scripts](#bitcoin-scripts)
5. [Creating a New Feature](#creating-a-new-feature)
6. [Implementation Patterns](#implementation-patterns)
7. [Testing Guide](#testing-guide)
8. [Best Practices](#best-practices)
9. [Examples](#examples)

## Overview

The Bitcoin Vault project uses a modular feature system where each feature represents a different type of Bitcoin transaction pattern. Features are implemented as traits and provide functionality for:

- **Locking**: Creating Bitcoin outputs that lock funds according to specific rules
- **Unlocking**: Creating PSBTs (Partially Signed Bitcoin Transactions) to spend locked funds
- **Script Generation**: Creating the necessary Bitcoin scripts for the feature

### Current Features

- **CustodianOnly**: Simple multi-signature with custodians only
- **UPC**: User-Protocol-Custodian with three different unlock paths
- **TimeGated**: Time-locked transactions with custodian fallback

## Feature Architecture

### Core Components

Each feature consists of several key components:

```
vault/src/core/
├── feat/
│   ├── mod.rs              # Feature module declarations
│   ├── your_feature.rs     # Your feature implementation
│   └── ...
├── traits.rs               # Feature trait definitions
├── params.rs               # Parameter structures
├── taproot.rs              # Taproot tree definitions
├── scripts.rs              # Script generation utilities
├── branches.rs             # Branch definitions
└── types.rs                # Type definitions
```

### Trait Structure

All features implement a common trait pattern:

```rust
pub trait YourFeature {
    type Error;
    
    // Required methods
    fn build_locking_output(
        &self,
        params: &YourFeatureLockingParams,
    ) -> Result<LockingOutput, Self::Error>;
    
    fn build_unlocking_psbt(
        &self,
        params: &YourFeatureUnlockingParams,
    ) -> Result<Psbt, Self::Error>;
    
    fn locking_script(
        // feature-specific parameters
    ) -> Result<LockingScript, Self::Error>;
    
    // Optional: for features that need data scripts
    fn data_script(
        &self,
        // feature-specific parameters
    ) -> Result<DataScript, Self::Error>;
}
```

## Taproot Trees

### Understanding Taproot

Taproot is a Bitcoin upgrade that enables more complex spending conditions while maintaining privacy and efficiency. In the Bitcoin Vault project, Taproot trees define the different ways funds can be unlocked.

### Tree Structure

Each feature defines its own Taproot tree structure in `vault/src/core/taproot.rs`:

```rust
// Example: CustodianOnly tree (simplest case)
pub struct CustodianOnlyTree {
    pub custodian_only_branch: CustodianOnlyBranch,
}

// Example: UPC tree (complex multi-path)
pub struct UPCTaprootTree {
    pub user_protocol_branch: UserProtocolBranch,
    pub custodian_protocol_branch: CustodianProtocolBranch, 
    pub custodian_user_branch: CustodianUserBranch,
}

// Example: TimeGated tree (time-based conditions)
pub struct TimeGatedTree {
    pub custodian_only_branch: CustodianOnlyBranch,
    pub csv_party_branch: CSVPartyBranch,
}
```

### Branch Types

Branches represent different unlock conditions:

```rust
// Multi-signature branch
pub struct CustodianOnlyBranch {
    pub script: Script,
    pub control_block: ControlBlock,
}

// User + Protocol signature branch
pub struct UserProtocolBranch {
    pub script: Script,
    pub control_block: ControlBlock,
}

// Time-locked branch (CSV = CheckSequenceVerify)
pub struct CSVPartyBranch {
    pub script: Script,
    pub control_block: ControlBlock,
    pub sequence: u16,
}
```

### Tree Construction

Trees are constructed using the `TaprootTree` generic:

```rust
use crate::{TaprootTree, get_global_secp};

// Create a tree for your feature
let secp = get_global_secp();
let tree = TaprootTree::<YourFeatureTree>::new(
    secp,
    // Parameters specific to your tree type
    user_key,
    protocol_key, 
    custodian_keys,
    quorum,
)?;

// Generate the locking script
let locking_script = tree.into_script(secp);
```

### Tree Design Principles

1. **Efficiency**: More common unlock paths should be cheaper (fewer branches)
2. **Security**: Critical paths should require multiple signatures
3. **Flexibility**: Different unlock conditions for different scenarios
4. **Privacy**: All branches look identical on-chain until spent

### Common Tree Patterns

#### Single Branch (CustodianOnly)
```
      Root
       │
   Custodians
   (3-of-5)
```

#### Multi-Path (UPC)
```
        Root
         │
    ┌────┴────┐
    │         │
 User+Prot   │
         ┌───┴───┐
         │       │
    Cust+Prot  Cust+User
```

#### Time-Gated
```
      Root
       │
   ┌───┴───┐
   │       │
Custodians Party
           (CSV)
```

## Bitcoin Scripts

### Script Fundamentals

Bitcoin scripts define the conditions for spending Bitcoin. In Taproot, scripts are organized into trees where each leaf represents a different spending condition.

### Script Types in Bitcoin Vault

#### 1. Multi-Signature Scripts

```rust
// Generate a multi-sig script for custodians
pub fn create_multisig_script(
    pubkeys: &[XOnlyPublicKey],
    quorum: u8,
) -> Script {
    let mut builder = Builder::new();
    
    // Add public keys
    for pubkey in pubkeys {
        builder = builder.push_x_only_key(pubkey);
    }
    
    // Add quorum check
    builder = builder
        .push_int(quorum as i64)
        .push_opcode(OP_CHECKMULTISIG);
    
    builder.into_script()
}
```

#### 2. Time-Locked Scripts (CSV)

```rust
// Create a time-locked script using CheckSequenceVerify
pub fn create_csv_script(
    pubkey: &XOnlyPublicKey,
    sequence: u16,
) -> Script {
    Builder::new()
        .push_int(sequence as i64)
        .push_opcode(OP_CSV)  // CheckSequenceVerify
        .push_opcode(OP_DROP)
        .push_x_only_key(pubkey)
        .push_opcode(OP_CHECKSIG)
        .into_script()
}
```

#### 3. Data Scripts (OP_RETURN)

```rust
// Create data script for cross-chain information
pub fn create_data_script(data: &[u8]) -> Script {
    Builder::new()
        .push_opcode(OP_RETURN)
        .push_slice(data)
        .into_script()
}
```

### Script Generation Utilities

The project provides utilities in `vault/src/core/scripts.rs`:

```rust
use crate::scripts::{
    CustodianOnlyLockingScriptParams,
    create_custodian_only_locking_script,
};

// Use existing script generators
let params = CustodianOnlyLockingScriptParams {
    custodian_pub_keys: &x_only_custodian_keys,
    custodian_quorum: 3,
};

let script = create_custodian_only_locking_script(&params)?;
```

### Script Validation

Always validate your scripts:

```rust
pub fn validate_script(script: &Script) -> Result<(), CoreError> {
    // Check script size limits
    if script.len() > 10000 {
        return Err(CoreError::ScriptTooLarge);
    }
    
    // Check for invalid opcodes
    for instruction in script.instructions() {
        match instruction {
            Ok(Instruction::Op(opcode)) => {
                if opcode == OP_RETURN && script.len() > 83 {
                    return Err(CoreError::DataScriptTooLarge);
                }
            }
            Err(_) => return Err(CoreError::InvalidScript),
            _ => {}
        }
    }
    
    Ok(())
}
```

### Script Security Considerations

1. **Opcode Limits**: Bitcoin has limits on script size and complexity
2. **Standardness**: Non-standard scripts may not be relayed
3. **Malleability**: Use only canonical signatures and opcodes
4. **Resource Usage**: Complex scripts cost more in fees

### Common Script Patterns

#### Pattern 1: Simple Multi-Sig
```rust
// 3-of-5 multisig
OP_3 <pubkey1> <pubkey2> <pubkey3> <pubkey4> <pubkey5> OP_5 OP_CHECKMULTISIG
```

#### Pattern 2: Time-Locked Single Sig
```rust
// Must wait 144 blocks, then single signature
<144> OP_CSV OP_DROP <pubkey> OP_CHECKSIG
```

#### Pattern 3: Either/Or Conditions
```rust
// Either immediate multisig OR time-locked single sig
// (This would be in separate Taproot branches)
Branch 1: OP_2 <pubkey1> <pubkey2> <pubkey3> OP_3 OP_CHECKMULTISIG
Branch 2: <144> OP_CSV OP_DROP <pubkey4> OP_CHECKSIG
```

### Debugging Scripts

```rust
pub fn debug_script(script: &Script, name: &str) {
    println!("=== Script Debug: {} ===", name);
    println!("Length: {} bytes", script.len());
    println!("Hex: {}", script.to_hex_string());
    
    for (i, instruction) in script.instructions().enumerate() {
        match instruction {
            Ok(Instruction::Op(opcode)) => {
                println!("  {}: {:?}", i, opcode);
            }
            Ok(Instruction::PushBytes(bytes)) => {
                println!("  {}: PUSH {} bytes", i, bytes.len());
            }
            Err(e) => {
                println!("  {}: ERROR {:?}", i, e);
            }
        }
    }
    println!("=== End Script Debug ===");
}
```

## Creating a New Feature

### Step 1: Define the Trait

Add your trait definition to `vault/src/core/traits.rs`:

```rust
pub trait YourFeature {
    type Error;
    
    fn build_locking_output(
        &self,
        params: &YourFeatureLockingParams,
    ) -> Result<LockingOutput, Self::Error>;
    
    fn build_unlocking_psbt(
        &self,
        params: &YourFeatureUnlockingParams,
    ) -> Result<Psbt, Self::Error>;
    
    fn locking_script(
        // Define parameters specific to your feature
        param1: &Type1,
        param2: &Type2,
    ) -> Result<LockingScript, Self::Error>;
}
```

### Step 2: Define Parameter Structures

Add parameter structures to `vault/src/core/params.rs`:

```rust
pub struct YourFeatureLockingParams {
    pub locking_amount: u64,
    // Add feature-specific fields
    pub your_field: YourType,
}

pub struct YourFeatureUnlockingParams {
    pub inputs: Vec<PreviousOutpoint>,
    pub outputs: Vec<TxOut>,
    // Add feature-specific fields
    pub your_field: YourType,
    pub rbf: bool,
    pub fee_rate: u64,
}
```

### Step 3: Define Taproot Tree Structure

Add your tree structure to `vault/src/core/taproot.rs`:

```rust
pub struct YourFeatureTree {
    pub branch_one: BranchType,
    pub branch_two: BranchType,
    // Add more branches as needed
}
```

### Step 4: Create the Feature Implementation

Create `vault/src/core/feat/your_feature.rs`:

```rust
use bitcoin::{Psbt, PublicKey};
use crate::{
    get_global_secp, CoreError, LockingOutput, LockingScript,
    TaprootTree, VaultManager, YourFeature, YourFeatureLockingParams,
    YourFeatureUnlockingParams, YourFeatureTree,
};

impl YourFeature for VaultManager {
    type Error = CoreError;
    
    fn build_locking_output(
        &self,
        params: &YourFeatureLockingParams,
    ) -> Result<LockingOutput, Self::Error> {
        let locking_script = <Self as YourFeature>::locking_script(
            // Pass required parameters
        )?;
        
        // Optional: create data script if needed
        let data_script = if self.needs_data_script() {
            Some(<Self as YourFeature>::data_script(self, /* params */)?) 
        } else { 
            None 
        };
        
        Ok(LockingOutput::new(
            params.locking_amount,
            locking_script,
            data_script,
        ))
    }
    
    fn locking_script(
        // Your parameters
    ) -> Result<LockingScript, Self::Error> {
        let secp = get_global_secp();
        
        // Convert public keys to x-only if needed
        // Create your taproot tree
        let tree = TaprootTree::<YourFeatureTree>::new(
            secp,
            // Your tree parameters
        )?;
        
        Ok(LockingScript(tree.into_script(secp)))
    }
    
    fn build_unlocking_psbt(
        &self,
        params: &YourFeatureUnlockingParams,
    ) -> Result<Psbt, Self::Error> {
        // Validate parameters
        let (total_input_value, total_output_value) = params.validate()?;
        
        let secp = get_global_secp();
        
        // Recreate the taproot tree
        let tree = TaprootTree::<YourFeatureTree>::new(
            secp,
            // Same parameters as locking_script
        )?;
        
        // Build the unsigned transaction
        let unsigned_tx = self.build_unlocking_transaction(&UnlockingParams {
            total_input_value,
            total_output_value,
            inputs: &params.inputs,
            outputs: &params.outputs,
            tree_type: UnlockingTaprootTreeType::YourFeature,
            script: &tree.clone().into_script(secp),
            rbf: params.rbf,
            fee_rate: params.fee_rate,
            // Add other required fields
        })?;
        
        // Create PSBT from unsigned transaction
        let mut psbt = Psbt::from_unsigned_tx(unsigned_tx)
            .map_err(|_| CoreError::FailedToCreatePSBT)?;
        
        // Determine which branch and keys to use based on unlock type
        let (branch, keys) = match params.unlock_type {
            YourFeatureUnlockType::PathOne => {
                (&tree.raw.branch_one, vec![key1, key2])
            }
            YourFeatureUnlockType::PathTwo => {
                (&tree.raw.branch_two, vec![key3, key4])
            }
        };
        
        // Prepare PSBT inputs with the selected branch and keys
        psbt.inputs = self.prepare_psbt_inputs(
            &params.inputs, 
            &tree.root, 
            branch, 
            &keys
        );
        
        Ok(psbt)
    }
}
```

### Step 5: Update Module Declaration

Add your feature to `vault/src/core/feat/mod.rs`:

```rust
mod custodian_only;
mod upc;
mod time_gated;
mod your_feature;  // Add this line
```

### Step 6: Export Types and Traits

Update `vault/src/core/mod.rs` to export your new types:

```rust
pub use traits::YourFeature;
pub use params::{YourFeatureLockingParams, YourFeatureUnlockingParams};
```

## Implementation Patterns

### Pattern 1: Simple Multi-Signature (like CustodianOnly)

```rust
// Single branch with multiple signers
pub struct SimpleMultiSigTree {
    pub multisig_branch: MultisigBranch,
}

// Implementation focuses on quorum validation
fn locking_script(
    pubkeys: &[PublicKey],
    quorum: u8,
) -> Result<LockingScript, Self::Error> {
    // Convert to x-only keys
    // Create simple tree with one branch
    // Return script
}
```

### Pattern 2: Multi-Path Unlocking (like UPC)

```rust
// Multiple branches for different unlock conditions
pub struct MultiPathTree {
    pub path_one_branch: BranchType,
    pub path_two_branch: BranchType,
    pub path_three_branch: BranchType,
}

// Implementation handles different unlock types
fn build_unlocking_psbt(
    &self,
    params: &MultiPathUnlockingParams,
) -> Result<Psbt, Self::Error> {
    // Select branch based on unlock type
    let (branch, keys) = match params.unlock_type {
        UnlockType::PathOne => (/* branch and keys */),
        UnlockType::PathTwo => (/* branch and keys */),
        UnlockType::PathThree => (/* branch and keys */),
    };
    // Continue with PSBT creation
}
```

### Pattern 3: Time-Locked Features (like TimeGated)

```rust
// Include time constraints
pub struct TimeLockedTree {
    pub immediate_branch: ImmediateBranch,
    pub time_locked_branch: TimeLockedBranch,
}

// Implementation includes sequence numbers
fn build_unlocking_psbt(
    &self,
    params: &TimeLockedUnlockingParams,
) -> Result<Psbt, Self::Error> {
    // Add inputs with specific sequence numbers
    tx_builder.add_input_with_sequence(
        params.input.outpoint,
        Sequence::from_height(params.sequence),
    );
    // Continue with transaction building
}
```

## Testing Guide

### Test Structure

Create comprehensive tests in `vault/tests/test_your_feature.rs`:

```rust
#[cfg(test)]
mod test_your_feature {
    use vault::{
        YourFeature, YourFeatureLockingParams, YourFeatureUnlockingParams,
        VaultManager, TestSuite, // other imports
    };
    use lazy_static::lazy_static;
    
    lazy_static! {
        static ref TEST_SUITE: TestSuite = TestSuite::new_with_loaded_env("TEST_ENV");
    }
    
    #[test]
    fn test_basic_locking() {
        // Test basic locking functionality
        let params = YourFeatureLockingParams {
            locking_amount: 10000,
            // your feature-specific params
        };
        
        let vault_manager = VaultManager::new(/* params */);
        let result = vault_manager.build_locking_output(&params);
        
        assert!(result.is_ok());
        // Add more assertions
    }
    
    #[test]
    fn test_unlocking_path_one() {
        // Test first unlock path
        // 1. Create staking transaction
        // 2. Build unlocking PSBT
        // 3. Sign PSBT
        // 4. Finalize and broadcast
    }
    
    #[test]
    fn test_unlocking_path_two() {
        // Test second unlock path
    }
    
    #[test]
    fn test_edge_cases() {
        // Test edge cases and error conditions
    }
}
```

### Test Categories

1. **Unit Tests**: Test individual functions
2. **Integration Tests**: Test complete workflows
3. **Edge Case Tests**: Test boundary conditions
4. **Error Handling Tests**: Test error scenarios

### Testing Workflow

```rust
#[test]
fn test_complete_workflow() {
    // 1. Setup test environment
    let test_suite = &*TEST_SUITE;
    let test_account = SuiteAccount::new(/* params */);
    
    // 2. Create staking transaction
    let utxos = get_approvable_utxos(/* params */).unwrap();
    let staking_tx = test_suite.prepare_staking_tx(
        10000,
        TaprootTreeType::YourFeature,
        test_account.clone(),
        /* other params */
    ).unwrap();
    
    // 3. Build unlocking PSBT
    let mut unlocking_psbt = test_suite.build_your_feature_unlocking_tx(
        &[staking_tx],
        vec![/* outputs */],
    );
    
    // 4. Sign PSBT
    let signing_keys = test_suite.get_required_keys();
    for key in signing_keys {
        VaultManager::sign_psbt_by_single_key(
            &mut unlocking_psbt,
            key.as_slice(),
            test_suite.network_id(),
            false,
        ).unwrap();
    }
    
    // 5. Finalize PSBT
    unlocking_psbt.finalize();
    
    // 6. Broadcast transaction
    let result = test_suite.send_psbt_by_rpc(unlocking_psbt);
    assert!(result.is_ok());
}
```

## Best Practices

### Security Considerations

1. **Validate all inputs** in parameter structures
2. **Use proper key conversion** (secp256k1 to x-only when needed)
3. **Implement proper error handling** for all operations
4. **Test edge cases** thoroughly
5. **Follow Bitcoin script best practices**

### Code Quality

1. **Use descriptive names** for functions and variables
2. **Add comprehensive documentation** with examples
3. **Follow Rust conventions** and use clippy
4. **Keep functions focused** on single responsibilities
5. **Use proper error types** and propagation

### Performance

1. **Reuse secp256k1 context** via `get_global_secp()`
2. **Minimize allocations** where possible
3. **Cache expensive computations** when appropriate
4. **Use efficient data structures**

## Examples

### Example: Simple Two-Party Feature

```rust
// In traits.rs
pub trait TwoParty {
    type Error;
    
    fn build_locking_output(
        &self,
        params: &TwoPartyLockingParams,
    ) -> Result<LockingOutput, Self::Error>;
    
    fn build_unlocking_psbt(
        &self,
        params: &TwoPartyUnlockingParams,
    ) -> Result<Psbt, Self::Error>;
    
    fn locking_script(
        party_a: &PublicKey,
        party_b: &PublicKey,
    ) -> Result<LockingScript, Self::Error>;
}

// In params.rs
pub struct TwoPartyLockingParams {
    pub locking_amount: u64,
    pub party_a_pubkey: PublicKey,
    pub party_b_pubkey: PublicKey,
}

pub struct TwoPartyUnlockingParams {
    pub inputs: Vec<PreviousOutpoint>,
    pub outputs: Vec<TxOut>,
    pub party_a_pubkey: PublicKey,
    pub party_b_pubkey: PublicKey,
    pub rbf: bool,
    pub fee_rate: u64,
}

// In taproot.rs
pub struct TwoPartyTree {
    pub both_parties_branch: BothPartiesBranch,
}

// In feat/two_party.rs
impl TwoParty for VaultManager {
    type Error = CoreError;
    
    fn build_locking_output(
        &self,
        params: &TwoPartyLockingParams,
    ) -> Result<LockingOutput, Self::Error> {
        let locking_script = <Self as TwoParty>::locking_script(
            &params.party_a_pubkey,
            &params.party_b_pubkey,
        )?;
        
        Ok(LockingOutput::new(
            params.locking_amount,
            locking_script,
            None, // No data script needed
        ))
    }
    
    fn locking_script(
        party_a: &PublicKey,
        party_b: &PublicKey,
    ) -> Result<LockingScript, Self::Error> {
        let secp = get_global_secp();
        let party_a_x_only = convert_pubkey_to_x_only_key(party_a);
        let party_b_x_only = convert_pubkey_to_x_only_key(party_b);
        
        let tree = TaprootTree::<TwoPartyTree>::new(
            secp,
            party_a_x_only,
            party_b_x_only,
        )?;
        
        Ok(LockingScript(tree.into_script(secp)))
    }
    
    fn build_unlocking_psbt(
        &self,
        params: &TwoPartyUnlockingParams,
    ) -> Result<Psbt, Self::Error> {
        // Implementation similar to other features
        // but simpler since only one unlock path
        todo!("Implement based on patterns above")
    }
}
```

This guide provides a comprehensive foundation for developing new features in the Bitcoin Vault project. Follow these patterns and best practices to ensure your feature integrates seamlessly with the existing codebase.