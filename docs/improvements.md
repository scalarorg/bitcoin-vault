### Security Parameters

```text
MINIMUM_QUORUM = 3           // Minimum number of required covenant signatures
MAX_CUSTODIANS = 15         // Maximum number of covenant signers
TIMELOCK_BLOCKS = 144       // 24-hour timelock for covenant-only path
```

## Tree Structure

### Enhanced Tree Structure (Recommended Configuration)

```text
                    Root
                     │
            ┌────────┴────────┐
            │                 │
    Emergency Branch     Standard Branch
            │           ┌─────┴─────┐
     Cov+Timelock       │           │
                    User+Protocol  Covenant Branch
                                ┌─────┴─────┐
                                │           │
                           Cov+Protocol   Cov+User
```

## Spending Paths

### 4. Emergency Recovery Path

```asm
<absolute_timelock> CHECKLOCKTIMEVERIFY
<custodian_pubkey_1> CHECKSIG
<custodian_pubkey_2> CHECKSIGADD
...
<custodian_pubkey_n> CHECKSIGADD
<quorum> GREATERTHANOREQUAL
```

To spend via this path:

- Must wait for timelock period to expire
- Requires higher quorum threshold (e.g., 75% of custodians)
- Must reveal script and Merkle proof

### Additional Security Measures

1. **Timelocks**:

   - Emergency recovery path requires absolute timelock
   - Optional relative timelocks between spending attempts

2. **Key Rotation**:

   - Periodic covenant key rotation procedure
   - MuSig2 key aggregation for reduced witness size
   - Deterministic key derivation paths (BIP32)

3. **Monitoring**:

   - Covenant signature tracking
   - Spending attempt notifications
   - Timelock monitoring

4. **Best Practices**:
   - Hardware security module (HSM) support
   - Geographically distributed custodians
   - Regular security audits
   - Disaster recovery procedures

### Future Improvements

- Cross-input signature aggregation (when available)
- Point Time Lock Contracts (PTLCs) integration
- Adaptor signatures for atomic swaps
- Recursive covenants (with future soft fork)
