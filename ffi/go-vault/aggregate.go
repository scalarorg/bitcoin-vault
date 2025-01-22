package vault

/*
#include <stdint.h>
#include <stdlib.h>

typedef struct {
    uint8_t* data;
    size_t len;
} ByteBuffer;

void free_byte_buffer(ByteBuffer buffer);

ByteBuffer aggregate_tap_script_sigs(
    const uint8_t* psbt_bytes,
    size_t psbt_len,
    const uint8_t* tap_script_sigs_map_bytes,
    size_t tap_script_sigs_map_len
);

*/
import "C"
import (
	"encoding/json"
	"unsafe"

	go_utils "github.com/scalarorg/bitcoin-vault/go-utils/types"
)

func AggregateTapScriptSigs(psbtBytes []byte, tapScriptSigsMap go_utils.TapScriptSigsMap) ([]byte, error) {
	if len(psbtBytes) == 0 {
		return nil, ErrInvalidPsbt
	}

	if len(tapScriptSigsMap) == 0 {
		return nil, ErrNoTapScriptSigs
	}

	jsonOutput, err := json.Marshal(tapScriptSigsMap)
	if err != nil {
		return nil, err
	}

	result := C.aggregate_tap_script_sigs(
		(*C.uint8_t)(unsafe.Pointer(&psbtBytes[0])),
		C.size_t(len(psbtBytes)),
		(*C.uint8_t)(unsafe.Pointer(&jsonOutput[0])),
		C.size_t(len(jsonOutput)),
	)
	defer C.free_byte_buffer(result)

	if result.data == nil || result.len == 0 {
		return nil, ErrFailedToAggregateTapScriptSigs
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}
