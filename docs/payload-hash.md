## Payload Hash

After receiving a bitcoin transaction, the relayer will parse the transaction and extract the payload hash.

The payload hash is used to verify the authenticity of the transaction to Scalar network.

```
payload = abi.encode(
    ['address', 'uint256', 'bytes32'],
    [destination_recipient_address, staking_amount, bitcoin_tx_hash]
);

payload_hash = keccak256(payload);
```

## Encoded payload of transferRemote of Scalar contract

```
encoded_metadata = abi.encode(
    ['uint64', 'bytes', 'bytes'],
    [amount, recipient_chain_identifier, metadata]
);

encoded_payload = abi.encode(
    ['address', 'address', 'symbol', 'bytes'],
    [sender_address, source_contract_address, symbol, encoded_metadata]
);
```

## Payload Encoding Structure

The payload encoding consists of two parts: the metadata encoding and the final payload encoding.

### Metadata Encoding

The metadata is encoded with the following parameters:

```solidity
encoded_metadata = abi.encode(
    ['uint64', 'bytes', 'bytes'],
    [amount, recipient_chain_identifier, metadata]
);
```

- `amount`: The transaction amount (uint64)
- `recipient_chain_identifier`: Identifier for the recipient on the destination chain (bytes)
- `metadata`: Additional transaction metadata (bytes)

### Final Payload Encoding

The final payload combines the transaction context with the encoded metadata:

```solidity
encoded_payload = abi.encode(
    ['address', 'address', 'symbol', 'bytes'],
    [sender_address, source_contract_address, symbol, encoded_metadata]
);
```

- `sender_address`: The address initiating the transaction (address)
- `source_contract_address`: The address of the contract handling the transaction (address)
- `symbol`: The token symbol or identifier (string)
- `encoded_metadata`: The previously encoded metadata (bytes)

This two-step encoding process ensures all necessary transaction information is properly structured and verifiable on-chain.
