package chain

import (
	"encoding/json"
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestChainType_String(t *testing.T) {
	tests := []struct {
		name      string
		chainType ChainType
		want      string
	}{
		{
			name:      "Bitcoin chain type",
			chainType: ChainTypeBitcoin,
			want:      "bitcoin",
		},
		{
			name:      "EVM chain type",
			chainType: ChainTypeEVM,
			want:      "evm",
		},
		{
			name:      "Solana chain type",
			chainType: ChainTypeSolana,
			want:      "solana",
		},
		{
			name:      "Cosmos chain type",
			chainType: ChainTypeCosmos,
			want:      "cosmos",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equal(t, tt.want, tt.chainType.String())
		})
	}
}

func TestDestinationChain_Bytes_And_FromBytes(t *testing.T) {
	tests := []struct {
		name        string
		chainType   ChainType
		chainID     uint64
		shouldBeNil bool
	}{
		{
			name:        "Valid Bitcoin chain",
			chainType:   ChainTypeBitcoin,
			chainID:     1,
			shouldBeNil: false,
		},
		{
			name:        "Valid EVM chain",
			chainType:   ChainTypeEVM,
			chainID:     5,
			shouldBeNil: false,
		},
		{
			name:        "Invalid chain type",
			chainType:   ChainType(99),
			chainID:     1,
			shouldBeNil: true,
		},
	}

	t.Log("Test with valid chain")

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			dc := &ChainInfo{
				ChainType: tt.chainType,
				ChainID:   tt.chainID,
			}

			t.Log("Test Bytes()")

			// Test Bytes()
			bytes := dc.Bytes()

			t.Log("After Bytes()")
			assert.Equal(t, 8, len(bytes))
			assert.Equal(t, byte(tt.chainType), bytes[0])

			t.Log("Test NewDestinationChainFromBytes()")

			// Test NewDestinationChainFromBytes()
			newDC := NewChainInfoFromBytes(bytes)
			if tt.shouldBeNil {
				assert.Nil(t, newDC)
			} else {
				t.Logf("newDC: %+v", newDC)
				assert.NotNil(t, newDC)
				assert.Equal(t, tt.chainType, newDC.ChainType)
				assert.Equal(t, tt.chainID, newDC.ChainID)
			}
		})
	}

	t.Log("Test with invalid length")

	// Test with invalid length
	invalidBytes := make([]byte, 7)
	assert.Nil(t, NewChainInfoFromBytes(invalidBytes))
}

func TestDestinationChain_JSON(t *testing.T) {
	dc := &ChainInfo{
		ChainType: ChainTypeBitcoin,
		ChainID:   1,
	}

	// Test MarshalJSON
	jsonBytes, err := json.Marshal(dc)
	t.Log("jsonBytes", string(jsonBytes))
	assert.NoError(t, err)
	assert.NotNil(t, jsonBytes)

	// Test UnmarshalJSON
	var newDC ChainInfo
	err = json.Unmarshal(jsonBytes, &newDC)
	assert.NoError(t, err)
	assert.Equal(t, dc.ChainType, newDC.ChainType)
	assert.Equal(t, dc.ChainID, newDC.ChainID)
}

func TestValidateChainType(t *testing.T) {
	tests := []struct {
		name      string
		chainType ChainType
		want      bool
	}{
		{
			name:      "Valid Bitcoin chain",
			chainType: ChainTypeBitcoin,
			want:      true,
		},
		{
			name:      "Valid EVM chain",
			chainType: ChainTypeEVM,
			want:      true,
		},
		{
			name:      "Valid Solana chain",
			chainType: ChainTypeSolana,
			want:      true,
		},
		{
			name:      "Valid Cosmos chain",
			chainType: ChainTypeCosmos,
			want:      true,
		},
		{
			name:      "Invalid chain type",
			chainType: ChainType(99),
			want:      false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equal(t, tt.want, ValidateChainType(tt.chainType))
		})
	}
}

func TestChainInfoBytes_String(t *testing.T) {
	chainInfo := &ChainInfo{
		ChainType: ChainTypeBitcoin,
		ChainID:   1,
	}
	assert.Equal(t, "bitcoin|1", chainInfo.ToBytes().String())

	evmChainInfo := &ChainInfo{
		ChainType: ChainTypeEVM,
		ChainID:   11155111,
	}
	assert.Equal(t, "evm|11155111", evmChainInfo.ToBytes().String())
}

func TestChainInfoBytes_FromString(t *testing.T) {
	chainInfoBytes := ChainInfoBytes{}
	err := chainInfoBytes.FromString("bitcoin|2")
	assert.NoError(t, err)
	fmt.Printf("chainInfoBytes: %x\n", chainInfoBytes.Bytes())
	assert.Equal(t, ChainTypeBitcoin, chainInfoBytes.ChainType())
	assert.Equal(t, uint64(2), chainInfoBytes.ChainID())
}