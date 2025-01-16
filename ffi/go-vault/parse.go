package vault

/*
#include <stdint.h>
#include <stdlib.h>

typedef struct {
    uint8_t* data;
    size_t len;
} ByteBuffer;

ByteBuffer parse_vault_embedded_data(
    const uint8_t* script_pubkey,
    size_t script_len
);

void free_byte_buffer(ByteBuffer buffer);
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"unsafe"
)

type TransactionType string

const (
	TransactionTypeUnstaking TransactionType = "Unstaking"
	TransactionTypeStaking   TransactionType = "Staking"
)

func (t TransactionType) String() string {
	return string(t)
}

func (t TransactionType) MarshalJSON() ([]byte, error) {
	return json.Marshal(string(t))
}

func (t *TransactionType) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}
	*t = TransactionType(s)
	return nil
}

// VaultReturnTxOutput represents the parsed vault return transaction output
type VaultReturnTxOutput struct {
	Tag                         []byte          `json:"tag"`
	Version                     uint8           `json:"version"`
	NetworkID                   uint8           `json:"network_id"`
	Flags                       uint8           `json:"flags"`
	ServiceTag                  []byte          `json:"service_tag"`
	TransactionType             TransactionType `json:"transaction_type"`
	CustodianQuorum             uint8           `json:"custodian_quorum"`
	DestinationChain            []byte          `json:"destination_chain"`
	DestinationTokenAddress     []byte          `json:"destination_token_address"`
	DestinationRecipientAddress []byte          `json:"destination_recipient_address"`
}

// ParseVaultEmbeddedData parses the script pubkey and returns the vault return transaction output
func ParseVaultEmbeddedData(scriptPubkey []byte) (*VaultReturnTxOutput, error) {
	if len(scriptPubkey) == 0 {
		return nil, ErrInvalidScript
	}

	result := C.parse_vault_embedded_data(
		(*C.uint8_t)(unsafe.Pointer(&scriptPubkey[0])),
		C.size_t(len(scriptPubkey)),
	)
	defer C.free_byte_buffer(result)

	if result.len == 0 || result.data == nil {
		return nil, ErrParsingFailed
	}

	// Convert the C buffer to Go slice
	goBytes := C.GoBytes(unsafe.Pointer(result.data), C.int(result.len))

	fmt.Printf("goBytes: %s\n", goBytes)

	// Parse JSON into VaultReturnTxOutput
	var output VaultReturnTxOutput
	if err := json.Unmarshal(goBytes, &output); err != nil {
		return nil, err
	}

	return &output, nil
}
