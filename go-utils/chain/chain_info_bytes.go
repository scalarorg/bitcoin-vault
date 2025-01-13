package chain

import (
	"encoding/binary"
	"fmt"
	"strconv"
	"strings"
)

type ChainInfoBytes [ChainInfoBytesSize]byte

func (ChainInfoBytes) Size() int {
	return ChainInfoBytesSize
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
	chainInfoBytes := make([]byte, ChainInfoBytesSize)
	copy(chainInfoBytes, c.Bytes())
	chainInfoBytes[0] = 0
	return binary.BigEndian.Uint64(chainInfoBytes)
}

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

	chainIDBytes := make([]byte, ChainInfoBytesSize)
	binary.BigEndian.PutUint64(chainIDBytes, chainID)
	copy(c[1:], chainIDBytes[1:])
	return nil
}
