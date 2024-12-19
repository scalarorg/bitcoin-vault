package encode

import "github.com/ethereum/go-ethereum/accounts/abi"

var (
	bytes20Type, _ = abi.NewType("bytes20", "bytes20", nil)
	bytes32Type, _ = abi.NewType("bytes32", "bytes32", nil)
	bytesType, _   = abi.NewType("bytes", "bytes", nil)
	bytes1Type, _  = abi.NewType("bytes1", "bytes1", nil)
	uint64Type, _  = abi.NewType("uint64", "uint64", nil)

	stakingPayload = abi.Arguments{{Type: bytes20Type}, {Type: uint64Type}, {Type: bytes32Type}}
	unstakingPayload = abi.Arguments{{Type: bytesType}, {Type: uint64Type}, {Type: bytes1Type}}
)
