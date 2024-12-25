package encode

import (
	"github.com/scalarorg/bitcoin-vault/go-utils/crypto"
)

/**
With bitcoin transactions, the tx id can be formated in different ways.
- Natural bytes order: Little endian (internal representation)
- Reversed bytes order: Big endian (when displayed)
- We use the reversed bytes order when hashing the payload
**/

/*
CalculateStakingPayloadHash hashes the staking payload

- sender: [20]byte (address)

- amount: int64 (amount in satoshis)

- sourceTxHash: [32]byte (tx id of the bitcoin transaction in reverse bytes order)
*/
func CalculateStakingPayloadHash(
	destRecipient [20]byte, //Address on the the destination chain
	amount uint64,
	sourceTxHash [32]byte,
) ([]byte, []byte, error) {
	// Manual ABI encoding:
	// 1. address: left-pad to 32 bytes
	// 2. uint256: left-pad to 32 bytes
	// 3. bytes32: already 32 bytes

	encodedPayload, err := executePayload.Pack(destRecipient, amount, sourceTxHash)
	if err != nil {
		return nil, nil, err
	}

	// Calculate keccak256 hash
	payloadHash := crypto.Keccak256(encodedPayload)

	return encodedPayload, payloadHash, nil
}
