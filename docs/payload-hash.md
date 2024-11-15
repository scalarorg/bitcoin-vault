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
