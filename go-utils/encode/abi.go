package encode

import "github.com/ethereum/go-ethereum/accounts/abi"

var (
	uint8Type, _ = abi.NewType("uint8", "uint8", nil)
	boolType, _  = abi.NewType("bool", "bool", nil)
	bytesType, _ = abi.NewType("bytes", "bytes", nil)

	contractCallWithTokenCustodianOnly = abi.Arguments{{Type: uint8Type}, {Type: boolType}, {Type: bytesType}}
	contractCallWithTokenUPC           = abi.Arguments{{Type: bytesType}}
)
