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
	"unsafe"
)

// VaultReturnTxOutput represents the parsed vault return transaction output
type VaultReturnTxOutput struct {
	Tag                         []byte `json:"tag"`
	Version                     uint8  `json:"version"`
	NetworkID                   uint8  `json:"network_id"`
	Flags                       uint8  `json:"flags"`
	ServiceTag                  []byte `json:"service_tag"`
	HaveOnlyCovenants           bool   `json:"have_only_covenants"`
	CovenantQuorum              uint8  `json:"covenant_quorum"`
	DestinationChain            []byte `json:"destination_chain"`
	DestinationContractAddress  []byte `json:"destination_contract_address"`
	DestinationRecipientAddress []byte `json:"destination_recipient_address"`
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

	// Parse JSON into VaultReturnTxOutput
	var output VaultReturnTxOutput
	if err := json.Unmarshal(goBytes, &output); err != nil {
		return nil, err
	}

	return &output, nil
}
