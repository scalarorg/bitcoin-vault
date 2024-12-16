package chain

import (
	"encoding/json"
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
			want:      "Bitcoin",
		},
		{
			name:      "EVM chain type",
			chainType: ChainTypeEVM,
			want:      "EVM",
		},
		{
			name:      "Solana chain type",
			chainType: ChainTypeSolana,
			want:      "Solana",
		},
		{
			name:      "Cosmos chain type",
			chainType: ChainTypeCosmos,
			want:      "Cosmos",
		},
		{
			name:      "Unknown chain type",
			chainType: ChainType(99),
			want:      "Unknown",
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
			dc := &DestinationChain{
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
			newDC := NewDestinationChainFromBytes(bytes)
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
	assert.Nil(t, NewDestinationChainFromBytes(invalidBytes))
}

func TestDestinationChain_JSON(t *testing.T) {
	dc := &DestinationChain{
		ChainType: ChainTypeBitcoin,
		ChainID:   1,
	}

	// Test MarshalJSON
	jsonBytes, err := json.Marshal(dc)
	t.Log("jsonBytes", string(jsonBytes))
	assert.NoError(t, err)
	assert.NotNil(t, jsonBytes)

	// Test UnmarshalJSON
	var newDC DestinationChain
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
