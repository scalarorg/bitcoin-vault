package vault

/*
#include <stdint.h>
#include <stdlib.h>

typedef struct {
    uint8_t* data;
    size_t len;
} ByteBuffer;


ByteBuffer custodians_only_locking_script(
    const uint8_t (*custodian_pub_keys_ptr)[33],
    size_t custodian_pub_keys_len,
    uint8_t custodian_quorum
);

void free_byte_buffer(ByteBuffer buffer);
*/
import "C"
import (
	"unsafe"
)

func CustodiansOnlyLockingScript(custodianPubKeys []PublicKey, custodianQuorum uint8) ([]byte, error) {
	result := C.custodians_only_locking_script(
		(*[33]C.uint8_t)(unsafe.Pointer(&custodianPubKeys[0])),
		C.size_t(len(custodianPubKeys)),
		C.uint8_t(custodianQuorum),
	)

	defer C.free_byte_buffer(result)

	if result.data == nil || result.len == 0 {
		return nil, ErrFailedToBuildCustodianOnlyUnstakingTx
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}
