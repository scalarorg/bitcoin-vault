package psbt

/*
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
    uint8_t* data;
    size_t len;
} ByteBuffer;

ByteBuffer sign_psbt_by_single_key(
    const uint8_t* psbt_bytes,
    size_t psbt_len,
    const uint8_t* privkey_bytes,
    size_t privkey_len,
    uint8_t network,
    bool finalize
);

void free_byte_buffer(ByteBuffer buffer);
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func SignPsbtBySingleKey(psbt []byte, privkey []byte, network uint8, finalize bool) ([]byte, error) {
	result := C.sign_psbt_by_single_key(
		(*C.uint8_t)(unsafe.Pointer(&psbt[0])),
		C.size_t(len(psbt)),
		(*C.uint8_t)(unsafe.Pointer(&privkey[0])),
		C.size_t(len(privkey)),
		C.uint8_t(network),
		C.bool(finalize),
	)
	defer C.free_byte_buffer(result)

	if result.data == nil || result.len == 0 {
		return nil, fmt.Errorf("failed to sign PSBT: result is nil or empty")
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}
