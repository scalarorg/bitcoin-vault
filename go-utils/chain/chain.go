package chain

import (
	"encoding/binary"
	"fmt"
	"strconv"
	"strings"
)

type ChainType uint8

const (
	ChainTypeBitcoin ChainType = iota // 0x00
	ChainTypeEVM                      // 0x01
	ChainTypeSolana                   // 0x02
	ChainTypeCosmos                   // 0x03
)

const (
	ChainTypeBitcoinStr = "bitcoin"
	ChainTypeEVMStr     = "evm"
	ChainTypeSolanaStr  = "solana"
	ChainTypeCosmosStr  = "cosmos"
)

var ChainTypeString = map[ChainType]string{
	ChainTypeBitcoin: ChainTypeBitcoinStr,
	ChainTypeEVM:     ChainTypeEVMStr,
	ChainTypeSolana:  ChainTypeSolanaStr,
	ChainTypeCosmos:  ChainTypeCosmosStr,
}

var ChainTypeFromString = map[string]ChainType{
	ChainTypeBitcoinStr: ChainTypeBitcoin,
	ChainTypeEVMStr:     ChainTypeEVM,
	ChainTypeSolanaStr:  ChainTypeSolana,
	ChainTypeCosmosStr:  ChainTypeCosmos,
}

func (ct ChainType) String() string {
	return ChainTypeString[ct]
}

func (ct *ChainType) FromString(s string) error {
	chainType, ok := ChainTypeFromString[s]
	if !ok {
		return fmt.Errorf("invalid chain type")
	}
	*ct = chainType
	return nil
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

func (c ChainInfoBytes) ChainType() ChainType {
	return ChainType(c[0])
}

func (c ChainInfoBytes) ChainID() uint64 {
	chainInfoBytes := make([]byte, 8)
	copy(chainInfoBytes, c.Bytes())
	chainInfoBytes[0] = 0
	return binary.BigEndian.Uint64(chainInfoBytes)
}

const SEPARATOR = "|"

func (c ChainInfoBytes) String() string {
	return fmt.Sprintf("%s%s%d", c.ChainType(), SEPARATOR, c.ChainID())
}

func (c *ChainInfoBytes) FromString(s string) error {
	parts := strings.Split(s, SEPARATOR)
	if len(parts) != 2 {
		return fmt.Errorf("invalid format")
	}

	var chainType ChainType
	err := chainType.FromString(parts[0])
	if err != nil {
		return fmt.Errorf("invalid chain type")
	}

	chainID, err := strconv.ParseUint(parts[1], 10, 64)
	if err != nil {
		return fmt.Errorf("invalid chain id")
	}

	c[0] = byte(chainType)

	chainIDBytes := make([]byte, 8)
	binary.BigEndian.PutUint64(chainIDBytes, chainID)
	copy(c[1:], chainIDBytes[1:])
	return nil
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
