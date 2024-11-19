package chain

import "encoding/binary"

type ChainType uint8

const (
	ChainTypeBitcoin ChainType = iota // 0x00
	ChainTypeEVM                      // 0x01
	ChainTypeSolana                   // 0x02
	ChainTypeCosmos                   // 0x03
)

func (ct ChainType) String() string {
	switch ct {
	case ChainTypeBitcoin:
		return "Bitcoin"
	case ChainTypeEVM:
		return "EVM"
	case ChainTypeSolana:
		return "Solana"
	case ChainTypeCosmos:
		return "Cosmos"
	default:
		return "Unknown"
	}
}

type DestinationChain struct {
	ChainType ChainType
	ChainID   uint64
}

func NewDestinationChainFromBytes(bytes []byte) *DestinationChain {
	if len(bytes) != 8 {
		return nil
	}

	if !ValidateChainType(ChainType(bytes[0])) {
		return nil
	}

	chainID := binary.BigEndian.Uint64(bytes[1:])

	return &DestinationChain{
		ChainType: ChainType(bytes[0]),
		ChainID:   chainID,
	}
}

func ValidateChainType(chainType ChainType) bool {
	return chainType <= ChainTypeCosmos
}
