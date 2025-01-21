package encode

import "fmt"

type ContractCallWithTokenPayloadType uint8

const (
	ContractCallWithTokenPayloadType_CustodianOnly ContractCallWithTokenPayloadType = iota
	ContractCallWithTokenPayloadType_UPC
)

func (c ContractCallWithTokenPayloadType) Bytes() []byte {
	return []byte{byte(c)}
}

func FromBytes(b []byte) (ContractCallWithTokenPayloadType, error) {
	if len(b) != 1 {
		return 0, fmt.Errorf("invalid bytes length")
	}
	return ContractCallWithTokenPayloadType(b[0]), nil
}
