package vault

/*
#include <stdint.h>
#include <stdlib.h>

typedef struct {
    uint8_t* data;
    size_t len;
} ByteBuffer;

void free_byte_buffer(ByteBuffer buffer);

// Function declarations
ByteBuffer sign_psbt_and_collect_sigs(
    const uint8_t* psbt_bytes,
    size_t psbt_len,
    const uint8_t* privkey_bytes,
    size_t privkey_len,
    uint8_t network
);

*/
import "C"
import (
	"encoding/json"
	"fmt"
	"unsafe"

	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

func SignPsbtAndCollectSigs(psbt []byte, privkey []byte, network types.NetworkKind) (types.TapScriptSigsMapType, error) {
	if !network.Valid() {
		return nil, ErrInvalidNetwork
	}

	result := C.sign_psbt_and_collect_sigs(
		(*C.uint8_t)(unsafe.Pointer(&psbt[0])),
		C.size_t(len(psbt)),
		(*C.uint8_t)(unsafe.Pointer(&privkey[0])),
		C.size_t(len(privkey)),
		C.uint8_t(network),
	)
	defer C.free_byte_buffer(result)

	if result.data == nil || result.len == 0 {
		return nil, ErrFailedToSignAndCollectSigs
	}

	goBytes := C.GoBytes(unsafe.Pointer(result.data), C.int(result.len))

	var output types.TapScriptSigsMapType
	if err := json.Unmarshal(goBytes, &output); err != nil {
		return nil, fmt.Errorf("failed to unmarshal tap script sigs: %w, got bytes: %s, raw bytes: %v", err, string(goBytes), goBytes)
	}

	if len(output) == 0 {
		return nil, ErrNoTapScriptSigs
	}

	return output, nil
}
