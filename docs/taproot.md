# Bitcoin Vault Taproot Implementation

## Overview

The Bitcoin Vault implements a Taproot-based script structure that enables multiple spending conditions through different script paths. The implementation focuses on Taproot's script-path spending capabilities to create a secure and flexible vault system.

## Technical Implementation

### Locking Structure

The UTXO is locked using a Taproot output that consists of:

1. A Taproot internal key (derived using BIP341 tweaking process)

2. A Merkle tree of alternative spending scripts

The internal key uses a NUMS (Nothing Up My Sleeve - [BIP341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)) point as the base internal key for security. The resulting Taproot output key commits to both the internal key and all script paths.

## Tree Structure

### Standard Tree (Default Configuration)

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

### Covenant-Only Tree

```text
      Root
       │
       1
       │
   Covenants
```

## Spending Paths

### 1. User + Protocol Path

This is the primary spending path that requires signatures from both the user and the protocol.

```asm
<user_pubkey> CHECKSIGVERIFY
<protocol_pubkey> CHECKSIG
```

To spend via this path:

- User must provide a valid signature for `user_pubkey`
- Protocol must provide a valid signature for `protocol_pubkey`
- Both signatures must be valid for the specific transaction being signed
- Must reveal the script and Merkle proof if using script path
- Example of witness:

```text
<user_signature> <protocol_signature> <tap_script> <control_block>
```

### 2. Covenant + Protocol Path

Requires a quorum of covenant signatures plus the protocol signature.

```asm
<protocol_pubkey> CHECKSIGVERIFY
<custodian_pubkey_1> CHECKSIG
<custodian_pubkey_2> CHECKSIGADD
...
<custodian_pubkey_n> CHECKSIGADD
<quorum> GREATERTHANOREQUAL
```

To spend via this path:

- Protocol must provide a valid signature
- Must collect signatures from covenant signers
- Number of valid covenant signatures must meet or exceed the quorum
- Uses CHECKSIGADD for efficient multi-signature validation
- Must reveal script and Merkle proof

- Example of witness:

```text
<protocol_signature> <custodian_signature_1> <custodian_signature_2> <custodian_signature_3> ... <tap_script> <control_block>
```

- If custodians are not present, the signature will be empty

### 3. Covenant + User Path

Requires a quorum of covenant signatures plus the user signature.

```asm
<user_pubkey> CHECKSIGVERIFY
<custodian_pubkey_1> CHECKSIG
<custodian_pubkey_2> CHECKSIGADD
...
<custodian_pubkey_n> CHECKSIGADD
<quorum> GREATERTHANOREQUAL
```

To spend via this path:

- User must provide a valid signature
- Must collect required covenant signatures meeting quorum
- Must reveal script and Merkle proof

- Example of witness:

```text
<user_signature> <custodian_signature_1> <custodian_signature_2> <custodian_signature_3> ... <tap_script> <control_block>
```

### 4. Covenants Only Path (Optional)

Only requires a quorum of covenant signatures. Useful for recovery or emergency situations.

```asm
<custodian_pubkey_1> CHECKSIG
<custodian_pubkey_2> CHECKSIGADD
...
<custodian_pubkey_n> CHECKSIGADD
<quorum> GREATERTHANOREQUAL
```

To spend via this path:

- Only requires covenant signatures meeting quorum
- Must reveal script and Merkle proof

- Example of witness:

```text
<custodian_signature_1> <custodian_signature_2> <custodian_signature_3> ...<tap_script> <control_block>
```

## Implementation Details

1. **Sorted Public Keys**: Covenant public keys are sorted to ensure deterministic script generation
2. **Duplicate Prevention**: Checks for duplicate covenant keys to prevent potential vulnerabilities
3. **Flexible Quorum**: Configurable number of required covenant signatures
4. **BIP341 Compliance**: Uses standardized **NUMS** point for Taproot internal key
5. **Security Considerations**:
   - `CHECKSIGVERIFY` ensures critical signatures cannot be skipped
   - Quorum requirement prevents single point of failure
   - Duplicate key checks prevent signature reuse attacks

### Improvements
- Can use MuSig2 for key path spending

