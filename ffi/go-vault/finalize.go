package vault

/*
#include <stdint.h>
#include <stdlib.h>

typedef struct {
    uint8_t* data;
    size_t len;
} ByteBuffer;

ByteBuffer finalize_psbt_and_extract_tx(
    const uint8_t* psbt_bytes,
    size_t psbt_len
);

void free_byte_buffer(ByteBuffer buffer);
*/
import "C"
import (
	"unsafe"
)

func FinalizePsbtAndExtractTx(psbtBytes []byte) ([]byte, error) {
	if len(psbtBytes) == 0 {
		return nil, ErrInvalidPsbt
	}

	result := C.finalize_psbt_and_extract_tx(
		(*C.uchar)(unsafe.Pointer(&psbtBytes[0])),
		C.size_t(len(psbtBytes)),
	)
	defer C.free_byte_buffer(result)

	if result.data == nil || result.len == 0 {
		return nil, ErrFailedToFinalizePsbtAndExtractTx
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}
