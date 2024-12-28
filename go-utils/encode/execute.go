package encode

import (
	"encoding/hex"
	"errors"
	"strings"

	"github.com/scalarorg/bitcoin-vault/go-utils/crypto"
)

/*
SafeCalculateDestPayload calculates the staking payload hash
and returns the encoded payload and the payload hash
It also checks if the sourceTxHash is valid
sourceTxHash is the tx id of the bitcoin transaction in reverse bytes order aka RPC Byte Order, Network Byte Order, Mempool Byte Order
*/
func SafeCalculateDestPayload(amount uint64, sourceTxHash string, recipientChainIdentifier []byte) ([]byte, []byte, error) {
	if sourceTxHash == "" {
		return nil, nil, errors.New("sourceTxHash is required")
	}

	sourceTxHash = strings.TrimPrefix(sourceTxHash, "0x")
	if len(sourceTxHash) != 64 {
		return nil, nil, errors.New("sourceTxHash must be 64 characters long")
	}

	sourceTxHashBytes, err := hex.DecodeString(sourceTxHash)
	if err != nil {
		return nil, nil, err
	}

	var sourceTx [32]byte
	copy(sourceTx[:], sourceTxHashBytes)

	encodedPayload, err := executePayload.Pack(amount, sourceTx, recipientChainIdentifier)
	if err != nil {
		return nil, nil, err
	}

	payloadHash := crypto.Keccak256(encodedPayload)

	return encodedPayload, payloadHash, nil
}

func DecodeDestPayload(payload []byte) (amount uint64, sourceTx [32]byte, recipientChainIdentifier []byte, err error) {
	values, err := executePayload.Unpack(payload)
	if err != nil {
		return 0, [32]byte{}, nil, err
	}
	return values[0].(uint64), values[1].([32]byte), values[2].([]byte), nil
}
