package vault

/*
#include <stdint.h>
#include <stdlib.h>

typedef struct {
    uint8_t* data;
    size_t len;
} ByteBuffer;


ByteBuffer only_covenants_locking_script(
    const uint8_t (*covenant_pub_keys_ptr)[33],
    size_t covenant_pub_keys_len,
    uint8_t covenant_quorum
);

void free_byte_buffer(ByteBuffer buffer);
*/
import "C"
import (
	"unsafe"
)

func OnlyCovenantsLockingScript(covenantPubKeys []PublicKey, covenantQuorum uint8) ([]byte, error) {
	result := C.only_covenants_locking_script(
		(*[33]C.uint8_t)(unsafe.Pointer(&covenantPubKeys[0])),
		C.size_t(len(covenantPubKeys)),
		C.uint8_t(covenantQuorum),
	)

	defer C.free_byte_buffer(result)

	if result.data == nil || result.len == 0 {
		return nil, ErrFailedToBuildCovenantOnlyUnstakingTx
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}
