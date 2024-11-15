package goutils

import (
	"fmt"
	"math/big"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
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
	sender [20]byte,
	amount int64,
	sourceTxHash [32]byte,
) ([]byte, []byte, error) {

	// Create arguments array
	arguments := abi.Arguments{
		{Type: GetAddressType()},
		{Type: GetUint256Type()},
		{Type: GetBytes32Type()},
	}

	// Pack the values
	payloadBytes, err := arguments.Pack(
		common.BytesToAddress(sender[:]),
		new(big.Int).SetInt64(amount),
		sourceTxHash,
	)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to pack values: %w", err)
	}
	payloadHash := crypto.Keccak256(payloadBytes)
	return payloadBytes, payloadHash, nil
}

// Helper functions to create ABI types
func GetAddressType() abi.Type {
	t, _ := abi.NewType("address", "", nil)
	return t
}

func GetUint256Type() abi.Type {
	t, _ := abi.NewType("uint256", "", nil)
	return t
}

func GetBytes32Type() abi.Type {
	t, _ := abi.NewType("bytes32", "", nil)
	return t
}
