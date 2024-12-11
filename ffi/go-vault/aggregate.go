package vault

/*
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
    uint8_t key_x_only[32];
    uint8_t leaf_hash[32];
    uint8_t signature[64];
} TapScriptSigFFI;

typedef struct {
    uint8_t* data;
    size_t len;
} ByteBuffer;

ByteBuffer aggregate_tap_script_sigs(
    const uint8_t* psbt_bytes,
    size_t psbt_len,
    const TapScriptSigFFI* tap_script_sigs,
    size_t tap_script_sigs_len
);

void free_byte_buffer(ByteBuffer buffer);
*/
import "C"
import (
	"unsafe"
)

func AggregateTapScriptSigs(psbtBytes []byte, tapScriptSigs []TapScriptSig) ([]byte, error) {
	if len(psbtBytes) == 0 {
		return nil, ErrInvalidPsbt
	}

	if len(tapScriptSigs) == 0 {
		return nil, ErrNoTapScriptSigs
	}

	result := C.aggregate_tap_script_sigs(
		(*C.uchar)(unsafe.Pointer(&psbtBytes[0])),
		C.size_t(len(psbtBytes)),
		(*C.TapScriptSigFFI)(unsafe.Pointer(&tapScriptSigs[0])),
		C.size_t(len(tapScriptSigs)),
	)
	defer C.free_byte_buffer(result)

	if result.data == nil || result.len == 0 {
		return nil, ErrFailedToAggregateTapScriptSigs
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}
