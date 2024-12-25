package encode

import "github.com/ethereum/go-ethereum/accounts/abi"

var (
	bytes20Type, _ = abi.NewType("bytes20", "bytes20", nil)
	bytes32Type, _ = abi.NewType("bytes32", "bytes32", nil)
	bytesType, _   = abi.NewType("bytes", "bytes", nil)
	uint64Type, _  = abi.NewType("uint64", "uint64", nil)
	stringType, _  = abi.NewType("string", "string", nil)
	addressType, _ = abi.NewType("address", "address", nil)

	executePayload = abi.Arguments{{Type: bytes20Type}, {Type: uint64Type}, {Type: bytes32Type}}

	transferRemoteMetadata = abi.Arguments{{Type: uint64Type}, {Type: bytesType}, {Type: bytesType}}

	transferRemotePayload = abi.Arguments{{Type: addressType}, {Type: addressType}, {Type: stringType}, {Type: bytesType}}
)
