# Contract Call With Token Payload

## 1. Pooling models

> > Detail in [contract_call_with_token.go](../go-utils/encode/contract_call_with_token.go)

The Contract Call With Token payload is specifically used for unstaking operations, where tokens need to be transferred to a destination chain (e.g., Bitcoin network). This payload is passed to the `callContractWithToken` function in the gateway contract.

### Payload Structure

The payload for contract calls with token transfers is encoded with the following parameters:

```solidity
encoded_payload = abi.encode(
    ['uint8', 'bool', 'bytes'],
    [feeOptions, rbf, recipientChainIdentifier]
);
```

Parameters:

- `feeOptions`: (uint8) Bitcoin transaction fee options
  - Determines the fee rate for the Bitcoin transaction
- `rbf`: (bool) Replace-By-Fee flag
  - When true, allows the transaction to be replaced with a higher fee
- `recipientChainIdentifier`: (bytes)
  - For Bitcoin destinations, this contains the recipient's BTC address
  - Used by Scalar Core to determine the final recipient on the destination chain

### Usage Example

When unstaking tokens to Bitcoin network:

```go
// Example values
feeOptions := types.BTCFeeOpts(2)      // Medium fee rate
rbf := true                            // Enable RBF
recipientAddr := []byte("0014...")     // Bitcoin recipient script

// Encode the payload
payload, hash, err := encode.CalculateContractCallWithTokenPayload(
    feeOptions,
    rbf,
    recipientAddr,
)
```

### Integration with Gateway Contract

The encoded payload is used as part of the `callContractWithToken` function:

```solidity
gateway.callContractWithToken(
    "bitcoin",                // destinationChain
    "0x...",                 // contractAddress
    payload,                 // encoded payload from above
    "sBtc",                 // token symbol
    amount                  // amount to unstake
);
```

The Scalar Core will decode this payload to extract the recipient's information and execute the unstaking operation on the destination chain according to the specified parameters.

## 2. Staking payload

> > Detail in [staking_payload.go](../go-utils/encode/staking_payload.go)
