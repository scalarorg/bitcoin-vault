# Testing Guide for Bitcoin Vault Features

This guide provides comprehensive instructions for writing tests for Bitcoin Vault features, including unit tests, integration tests, and end-to-end testing workflows.

## Table of Contents

1. [Testing Overview](#testing-overview)
2. [Test Environment Setup](#test-environment-setup)
3. [Unit Testing](#unit-testing)
4. [Integration Testing](#integration-testing)
5. [End-to-End Testing](#end-to-end-testing)
6. [Test Utilities](#test-utilities)
7. [Common Testing Patterns](#common-testing-patterns)
8. [Debugging Tests](#debugging-tests)
9. [Best Practices](#best-practices)

## Testing Overview

### Test Categories

The Bitcoin Vault project uses several types of tests:

1. **Unit Tests**: Test individual functions and components
2. **Integration Tests**: Test feature workflows and component interactions
3. **End-to-End Tests**: Test complete staking/unstaking cycles
4. **Property Tests**: Test invariants and edge cases
5. **Performance Tests**: Test transaction building and signing performance

### Test Structure

```
vault/
├── tests/
│   ├── test_custodians.rs      # CustodianOnly feature tests
│   ├── test_upc.rs             # UPC feature tests
│   ├── test_time_gated.rs      # TimeGated feature tests
│   └── test_your_feature.rs    # Your feature tests
├── src/
│   └── core/
│       └── feat/
│           └── your_feature.rs # Feature implementation with unit tests
└── benches/                    # Performance benchmarks
```

## Test Environment Setup

### Required Dependencies

Add these dependencies to your `Cargo.toml`:

```toml
[dev-dependencies]
bitcoin = { version = "0.31", features = ["rand"] }
bitcoincore-rpc = "0.18"
rust_mempool = "1.0"
lazy_static = "1.4"
hex = "0.4"
base64 = "0.21"
rand = "0.8"
tokio = { version = "1.0", features = ["full"] }
```

### Test Suite Setup

Create a test suite instance for your tests:

```rust
use lazy_static::lazy_static;
use vault::{
    TestSuite, SuiteAccount, DestinationInfo, AccountEnv, DestinationInfoEnv
};

lazy_static! {
    static ref TEST_SUITE: TestSuite = TestSuite::new_with_loaded_env("TEST_ENV");
    static ref TEST_ACCOUNT: SuiteAccount = 
        SuiteAccount::new(AccountEnv::new(TEST_SUITE.env_path()).unwrap());
    static ref TEST_DESTINATION_INFO: DestinationInfo = 
        DestinationInfo::new(DestinationInfoEnv::new(TEST_SUITE.env_path()).unwrap());
}
```

### Environment Configuration

Create a `.env` file for test configuration:

```env
# Bitcoin RPC Configuration
BITCOIN_RPC_URL=http://localhost:18443
BITCOIN_RPC_USER=test
BITCOIN_RPC_PASSWORD=test

# Test Network
NETWORK=regtest

# Test Account
TEST_PRIVATE_KEY=your_test_private_key
TEST_ADDRESS=your_test_address

# Custodian Configuration
CUSTODIAN_PRIVATE_KEYS=key1,key2,key3,key4,key5
CUSTODIAN_QUORUM=3
```

## Unit Testing

### Testing Individual Functions

```rust
#[cfg(test)]
mod unit_tests {
    use super::*;
    use bitcoin::{PublicKey, secp256k1::Secp256k1};
    use vault::{VaultManager, YourFeature};
    
    #[test]
    fn test_locking_script_generation() {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        
        let result = VaultManager::locking_script(
            &PublicKey::new(public_key),
            // other parameters
        );
        
        assert!(result.is_ok());
        let script = result.unwrap();
        assert!(!script.0.is_empty());
        // Add more specific assertions
    }
    
    #[test]
    fn test_parameter_validation() {
        let invalid_params = YourFeatureLockingParams {
            locking_amount: 0, // Invalid: zero amount
            // other fields
        };
        
        let vault_manager = VaultManager::new(/* config */);
        let result = vault_manager.build_locking_output(&invalid_params);
        
        assert!(result.is_err());
        // Verify specific error type
        match result.unwrap_err() {
            CoreError::InvalidAmount => {}, // Expected error
            _ => panic!("Unexpected error type"),
        }
    }
    
    #[test]
    fn test_key_conversion() {
        let secp = Secp256k1::new();
        let (_, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        let bitcoin_pubkey = PublicKey::new(public_key);
        
        let x_only_key = convert_pubkey_to_x_only_key(&bitcoin_pubkey);
        
        // Verify conversion preserves key data
        assert_eq!(x_only_key.serialize(), public_key.x_only_public_key().0.serialize());
    }
}
```

### Testing Error Conditions

```rust
#[test]
fn test_insufficient_funds() {
    let params = YourFeatureUnlockingParams {
        inputs: vec![/* small input */],
        outputs: vec![/* large output */],
        // other fields
    };
    
    let vault_manager = VaultManager::new(/* config */);
    let result = vault_manager.build_unlocking_psbt(&params);
    
    assert!(matches!(result, Err(CoreError::InsufficientFunds)));
}

#[test]
fn test_invalid_quorum() {
    let params = YourFeatureLockingParams {
        custodian_quorum: 0, // Invalid
        custodian_pubkeys: vec![/* some keys */],
        // other fields
    };
    
    let result = VaultManager::locking_script(
        &params.custodian_pubkeys,
        params.custodian_quorum,
    );
    
    assert!(matches!(result, Err(CoreError::InvalidQuorum)));
}
```

## Integration Testing

### Testing Feature Workflows

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_locking_and_unlocking_workflow() {
        // 1. Setup
        let vault_manager = VaultManager::new(/* test config */);
        
        // 2. Create locking output
        let locking_params = YourFeatureLockingParams {
            locking_amount: 10000,
            // feature-specific parameters
        };
        
        let locking_output = vault_manager
            .build_locking_output(&locking_params)
            .expect("Failed to create locking output");
        
        // 3. Verify locking output
        assert_eq!(locking_output.amount, 10000);
        assert!(!locking_output.script.0.is_empty());
        
        // 4. Create unlocking PSBT
        let unlocking_params = YourFeatureUnlockingParams {
            inputs: vec![/* previous outpoint from locking */],
            outputs: vec![/* destination outputs */],
            // other parameters
        };
        
        let psbt = vault_manager
            .build_unlocking_psbt(&unlocking_params)
            .expect("Failed to create unlocking PSBT");
        
        // 5. Verify PSBT structure
        assert_eq!(psbt.inputs.len(), unlocking_params.inputs.len());
        assert_eq!(psbt.outputs.len(), unlocking_params.outputs.len());
        
        // 6. Verify PSBT can be signed (mock signing)
        // This would typically involve actual key signing in full tests
    }
    
    #[test]
    fn test_multiple_input_unlocking() {
        // Test unlocking with multiple inputs
        let unlocking_params = YourFeatureUnlockingParams {
            inputs: vec![
                /* multiple previous outpoints */
            ],
            outputs: vec![/* consolidated output */],
            // other parameters
        };
        
        let vault_manager = VaultManager::new(/* config */);
        let psbt = vault_manager
            .build_unlocking_psbt(&unlocking_params)
            .expect("Failed to create multi-input PSBT");
        
        // Verify all inputs are properly configured
        for (i, input) in psbt.inputs.iter().enumerate() {
            assert!(input.witness_utxo.is_some(), "Input {} missing witness UTXO", i);
            assert!(!input.tap_internal_key.is_none(), "Input {} missing tap internal key", i);
        }
    }
}
```

## End-to-End Testing

### Complete Transaction Lifecycle

```rust
#[test]
fn test_complete_staking_unstaking_cycle() {
    // This test requires a running Bitcoin regtest node
    
    // 1. Get UTXOs for staking
    let utxos = get_approvable_utxos(
        &TEST_SUITE.rpc,
        &TEST_ACCOUNT.address(),
        10000
    ).expect("Failed to get UTXOs");
    
    // 2. Create and broadcast staking transaction
    let staking_tx = TEST_SUITE.prepare_staking_tx(
        10000,
        TaprootTreeType::YourFeature,
        TEST_ACCOUNT.clone(),
        TEST_DESTINATION_INFO.clone(),
        utxos,
    ).expect("Failed to create staking transaction");
    
    println!("Staking TX ID: {}", staking_tx.compute_txid());
    
    // 3. Wait for confirmation (in real tests)
    // std::thread::sleep(std::time::Duration::from_secs(10));
    
    // 4. Create unstaking PSBT
    let mut unstaking_psbt = TEST_SUITE.build_your_feature_unstaking_tx(
        &[staking_tx],
        vec![TxOut {
            value: Amount::from_sat(8000), // Partial unstaking
            script_pubkey: TEST_ACCOUNT.address().script_pubkey(),
        }],
    );
    
    // 5. Sign PSBT with required keys
    let signing_keys = TEST_SUITE.get_required_signing_keys();
    for key in signing_keys {
        VaultManager::sign_psbt_by_single_key(
            &mut unstaking_psbt,
            key.as_slice(),
            TEST_SUITE.network_id(),
            false,
        ).expect("Failed to sign PSBT");
    }
    
    // 6. Finalize PSBT
    unstaking_psbt.finalize();
    
    // 7. Broadcast unstaking transaction
    let result = TEST_SUITE.send_psbt_by_rpc(unstaking_psbt)
        .expect("Failed to broadcast unstaking transaction");
    
    if let Some(tx_result) = result {
        println!("Unstaking TX ID: {}", tx_result.txid);
        assert!(!tx_result.txid.is_empty());
    } else {
        panic!("No transaction result returned");
    }
}
```

### Testing Different Unlock Paths

```rust
#[test]
fn test_all_unlock_paths() {
    for unlock_type in [
        YourFeatureUnlockType::PathOne,
        YourFeatureUnlockType::PathTwo,
        // Add all your unlock types
    ] {
        println!("Testing unlock path: {:?}", unlock_type);
        
        // Setup staking transaction
        let staking_tx = create_test_staking_tx();
        
        // Create unlocking PSBT for this path
        let unlocking_params = YourFeatureUnlockingParams {
            unlock_type,
            // other parameters
        };
        
        let psbt = TEST_SUITE.vault_manager
            .build_unlocking_psbt(&unlocking_params)
            .expect(&format!("Failed to create PSBT for {:?}", unlock_type));
        
        // Verify PSBT is valid for this unlock path
        verify_psbt_for_unlock_type(&psbt, unlock_type);
    }
}
```

## Test Utilities

### Helper Functions

```rust
// Test utility functions
pub fn create_test_keys(count: usize) -> Vec<PublicKey> {
    let secp = Secp256k1::new();
    (0..count)
        .map(|_| {
            let (_, public_key) = secp.generate_keypair(&mut rand::thread_rng());
            PublicKey::new(public_key)
        })
        .collect()
}

pub fn create_test_utxo(amount: u64) -> PreviousOutpoint {
    PreviousOutpoint {
        outpoint: OutPoint {
            txid: Txid::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            vout: 0,
        },
        amount_in_sats: amount,
        script_pubkey: ScriptBuf::new(),
    }
}

pub fn create_test_output(amount: u64, address: &Address) -> TxOut {
    TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: address.script_pubkey(),
    }
}

pub fn verify_psbt_structure(psbt: &Psbt, expected_inputs: usize, expected_outputs: usize) {
    assert_eq!(psbt.inputs.len(), expected_inputs, "Unexpected number of inputs");
    assert_eq!(psbt.outputs.len(), expected_outputs, "Unexpected number of outputs");
    
    // Verify all inputs have required fields
    for (i, input) in psbt.inputs.iter().enumerate() {
        assert!(input.witness_utxo.is_some(), "Input {} missing witness UTXO", i);
    }
}

pub fn mock_sign_psbt(psbt: &mut Psbt, key_count: usize) {
    // Mock signing for testing purposes
    // In real tests, use actual cryptographic signing
    for input in &mut psbt.inputs {
        // Add mock signatures
        // This is simplified - real implementation would use proper signing
    }
}
```

### Test Data Generators

```rust
pub struct TestDataGenerator {
    secp: Secp256k1<All>,
}

impl TestDataGenerator {
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }
    
    pub fn generate_custodian_keys(&self, count: usize) -> Vec<PublicKey> {
        (0..count)
            .map(|_| {
                let (_, public_key) = self.secp.generate_keypair(&mut rand::thread_rng());
                PublicKey::new(public_key)
            })
            .collect()
    }
    
    pub fn generate_locking_params(&self, amount: u64) -> YourFeatureLockingParams {
        YourFeatureLockingParams {
            locking_amount: amount,
            custodian_pubkeys: self.generate_custodian_keys(5),
            custodian_quorum: 3,
            // other feature-specific fields
        }
    }
    
    pub fn generate_unlocking_params(
        &self,
        inputs: Vec<PreviousOutpoint>,
        outputs: Vec<TxOut>,
    ) -> YourFeatureUnlockingParams {
        YourFeatureUnlockingParams {
            inputs,
            outputs,
            custodian_pubkeys: self.generate_custodian_keys(5),
            custodian_quorum: 3,
            rbf: true,
            fee_rate: 1000, // 1 sat/vbyte
            // other fields
        }
    }
}
```

## Common Testing Patterns

### Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_locking_amount_preservation(
        amount in 1000u64..1_000_000u64,
        quorum in 1u8..10u8,
    ) {
        let generator = TestDataGenerator::new();
        let custodian_count = (quorum as usize) + 2; // Ensure we have enough custodians
        
        let params = YourFeatureLockingParams {
            locking_amount: amount,
            custodian_pubkeys: generator.generate_custodian_keys(custodian_count),
            custodian_quorum: quorum,
            // other fields
        };
        
        let vault_manager = VaultManager::new(/* config */);
        let result = vault_manager.build_locking_output(&params);
        
        prop_assert!(result.is_ok());
        let output = result.unwrap();
        prop_assert_eq!(output.amount, amount);
    }
    
    #[test]
    fn test_fee_calculation_consistency(
        input_count in 1usize..10usize,
        output_count in 1usize..5usize,
        fee_rate in 100u64..10000u64,
    ) {
        let generator = TestDataGenerator::new();
        
        // Generate test inputs and outputs
        let inputs: Vec<_> = (0..input_count)
            .map(|_| create_test_utxo(10000))
            .collect();
        let outputs: Vec<_> = (0..output_count)
            .map(|_| create_test_output(1000, &TEST_ACCOUNT.address()))
            .collect();
        
        let params = generator.generate_unlocking_params(inputs, outputs);
        let vault_manager = VaultManager::new(/* config */);
        
        let result = vault_manager.build_unlocking_psbt(&params);
        prop_assert!(result.is_ok());
        
        // Verify fee calculation is consistent
        let psbt = result.unwrap();
        let calculated_fee = calculate_psbt_fee(&psbt);
        prop_assert!(calculated_fee > 0);
    }
}
```

### Parameterized Tests

```rust
#[test]
fn test_different_quorum_sizes() {
    let test_cases = vec![
        (3, 5),   // 3-of-5
        (2, 3),   // 2-of-3
        (5, 7),   // 5-of-7
        (1, 1),   // 1-of-1 (edge case)
    ];
    
    for (quorum, total_custodians) in test_cases {
        println!("Testing {}-of-{} configuration", quorum, total_custodians);
        
        let generator = TestDataGenerator::new();
        let params = YourFeatureLockingParams {
            locking_amount: 10000,
            custodian_pubkeys: generator.generate_custodian_keys(total_custodians),
            custodian_quorum: quorum,
            // other fields
        };
        
        let vault_manager = VaultManager::new(/* config */);
        let result = vault_manager.build_locking_output(&params);
        
        assert!(result.is_ok(), "Failed for {}-of-{} configuration", quorum, total_custodians);
    }
}
```

## Debugging Tests

### Logging and Debugging

```rust
use log::{debug, info, warn, error};

#[test]
fn test_with_detailed_logging() {
    env_logger::init();
    
    info!("Starting test with detailed logging");
    
    let params = YourFeatureLockingParams {
        // parameters
    };
    
    debug!("Created locking params: {:?}", params);
    
    let vault_manager = VaultManager::new(/* config */);
    let result = vault_manager.build_locking_output(&params);
    
    match result {
        Ok(output) => {
            info!("Successfully created locking output");
            debug!("Output script length: {}", output.script.0.len());
            debug!("Output amount: {}", output.amount);
        }
        Err(e) => {
            error!("Failed to create locking output: {:?}", e);
            panic!("Test failed");
        }
    }
}
```

### Test Debugging Utilities

```rust
pub fn debug_psbt(psbt: &Psbt, name: &str) {
    println!("=== PSBT Debug: {} ===", name);
    println!("Inputs: {}", psbt.inputs.len());
    println!("Outputs: {}", psbt.outputs.len());
    
    for (i, input) in psbt.inputs.iter().enumerate() {
        println!("Input {}: ", i);
        println!("  Witness UTXO: {:?}", input.witness_utxo.is_some());
        println!("  Tap Internal Key: {:?}", input.tap_internal_key.is_some());
        println!("  Signatures: {}", input.tap_script_sigs.len());
    }
    
    println!("PSBT Base64: {}", base64::encode(psbt.serialize()));
    println!("PSBT Hex: {}", hex::encode(psbt.serialize()));
    println!("=== End PSBT Debug ===");
}

pub fn verify_transaction_validity(tx: &Transaction) -> Result<(), String> {
    // Basic transaction validation
    if tx.input.is_empty() {
        return Err("Transaction has no inputs".to_string());
    }
    
    if tx.output.is_empty() {
        return Err("Transaction has no outputs".to_string());
    }
    
    let total_output_value: u64 = tx.output.iter().map(|o| o.value.to_sat()).sum();
    if total_output_value == 0 {
        return Err("Transaction outputs have zero value".to_string());
    }
    
    Ok(())
}
```

## Best Practices

### Test Organization

1. **Group related tests** in modules
2. **Use descriptive test names** that explain what is being tested
3. **Keep tests focused** on single functionality
4. **Use setup and teardown** functions for common test preparation
5. **Document complex test scenarios** with comments

### Test Data Management

1. **Use deterministic test data** when possible
2. **Generate random data** for property-based tests
3. **Clean up test data** after tests complete
4. **Use realistic test values** that match production scenarios
5. **Test edge cases** and boundary conditions

### Performance Considerations

1. **Mock expensive operations** in unit tests
2. **Use parallel test execution** where safe
3. **Minimize network calls** in tests
4. **Cache test setup** when possible
5. **Profile slow tests** and optimize

### Error Testing

1. **Test all error paths** explicitly
2. **Verify error messages** are helpful
3. **Test error recovery** scenarios
4. **Use proper error assertions** (not just `is_err()`)
5. **Test error propagation** through the call stack

### Continuous Integration

```yaml
# Example GitHub Actions workflow
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: |
          cargo test --all-features
          cargo test --release --all-features
      - name: Run integration tests
        run: cargo test --test '*' --all-features
```

This comprehensive testing guide provides the foundation for writing robust tests for Bitcoin Vault features. Follow these patterns and practices to ensure your features are thoroughly tested and reliable.