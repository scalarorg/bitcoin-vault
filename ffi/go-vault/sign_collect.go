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
    TapScriptSigFFI* data;
    size_t len;
} TapScriptSigArray;

// Function declarations
TapScriptSigArray sign_psbt_and_collect_sigs(
    const uint8_t* psbt_bytes,
    size_t psbt_len,
    const uint8_t* privkey_bytes,
    size_t privkey_len,
    uint8_t network
);

void free_tap_script_sig_array(TapScriptSigArray array);
*/
import "C"
import (
	"unsafe"
)

type TapScriptSig struct {
	KeyXOnly  [32]byte
	LeafHash  [32]byte
	Signature [64]byte
}

func SignPsbtAndCollectSigs(psbt []byte, privkey []byte, network NetworkKind) ([]TapScriptSig, error) {
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
	defer C.free_tap_script_sig_array(result)

	if result.data == nil || result.len == 0 {
		return nil, ErrFailedToSignAndCollectSigs
	}

	length := int(result.len)
	tapScriptSigs := make([]TapScriptSig, length)

	cSigs := unsafe.Slice(result.data, length)

	// Copy data from C array to Go slice
	for i := 0; i < length; i++ {
		tapScriptSigs[i] = TapScriptSig{
			KeyXOnly:  *(*[32]byte)(unsafe.Pointer(&cSigs[i].key_x_only)),
			LeafHash:  *(*[32]byte)(unsafe.Pointer(&cSigs[i].leaf_hash)),
			Signature: *(*[64]byte)(unsafe.Pointer(&cSigs[i].signature)),
		}
	}

	return tapScriptSigs, nil
}
