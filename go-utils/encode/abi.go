package encode

import "github.com/ethereum/go-ethereum/accounts/abi"

var (
	bytes32Type, _ = abi.NewType("bytes32", "bytes32", nil)
	bytesType, _   = abi.NewType("bytes", "bytes", nil)
	uint64Type, _  = abi.NewType("uint64", "uint64", nil)
	uint8Type, _   = abi.NewType("uint8", "uint8", nil)
	stringType, _  = abi.NewType("string", "string", nil)
	addressType, _ = abi.NewType("address", "address", nil)
	boolType, _    = abi.NewType("bool", "bool", nil)
	// amount, sourceTxHash, recipientChainIdentifier
	executePayload = abi.Arguments{{Type: uint64Type}, {Type: bytes32Type}, {Type: bytesType}}

	transferRemoteMetadata = abi.Arguments{{Type: uint64Type}, {Type: bytesType}, {Type: bytesType}}

	transferRemotePayload = abi.Arguments{{Type: addressType}, {Type: addressType}, {Type: stringType}, {Type: bytesType}}

	contractCallWithToken = abi.Arguments{{Type: uint8Type}, {Type: boolType}, {Type: bytesType}}
)
