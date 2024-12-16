package chain

import (
	"encoding/binary"
	"fmt"
)

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

type ChainInfo struct {
	ChainType ChainType `json:"chain_type"`
	ChainID   uint64    `json:"chain_id"`
}

type ChainInfoBytes [8]byte

func (ChainInfoBytes) Size() int {
	return 8
}

func (c ChainInfoBytes) MarshalTo(data []byte) (int, error) {
	copy(data, c.Bytes())
	return c.Size(), nil
}

func (c *ChainInfoBytes) Unmarshal(data []byte) error {
	if len(data) != c.Size() {
		return fmt.Errorf("invalid data length")
	}
	copy(c.Bytes(), data)
	return nil
}

func (c ChainInfoBytes) Bytes() []byte {
	return c[:]
}

func (c ChainInfoBytes) String() string {
	bytes := c.Bytes()
	return fmt.Sprintf("ChainType: %d, ChainID: %d", bytes[0], binary.BigEndian.Uint64(bytes[1:]))
}

func NewChainInfoFromBytes(bytes []byte) *ChainInfo {
	if len(bytes) != 8 {
		return nil
	}

	if !ValidateChainType(ChainType(bytes[0])) {
		return nil
	}
	chainType := ChainType(bytes[0])
	bytes[0] = 0
	chainID := binary.BigEndian.Uint64(bytes)

	return &ChainInfo{
		ChainType: chainType,
		ChainID:   chainID,
	}
}

func (dc *ChainInfo) ToBytes() ChainInfoBytes {
	return ChainInfoBytes(dc.Bytes())
}

func (dc *ChainInfo) Bytes() []byte {
	chainIDBytes := make([]byte, 8)
	binary.BigEndian.PutUint64(chainIDBytes, dc.ChainID)

	chainTypeBytes := byte(dc.ChainType)

	bytes := make([]byte, 8)
	bytes[0] = chainTypeBytes
	copy(bytes[1:], chainIDBytes[1:])

	return bytes
}

func (ChainInfo) Size() int {
	return 8
}

func (c ChainInfo) MarshalTo(data []byte) (int, error) {
	copy(data, c.Bytes())
	return c.Size(), nil
}

func (c *ChainInfo) Unmarshal(data []byte) error {
	if len(data) != c.Size() {
		return fmt.Errorf("invalid data length")
	}
	copy(c.Bytes(), data)
	return nil
}

func ValidateChainType(chainType ChainType) bool {
	return chainType <= ChainTypeCosmos
}
