package main

/*
#cgo LDFLAGS: -L./lib -lbitcoin_vault_ffi
#cgo CFLAGS: -I./lib
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
	"encoding/hex"
	"fmt"
	"log"
	"unsafe"
)

const PSBT_HEX = "70736274ff0100520200000001e0a68346c9118f584c22c9afa89b641e06127d1b1fa661788ea922261dee37600000000000fdffffff012823000000000000160014acd07b22adf2299c56909c9ca537fd2c58127ecc000000000001012b102700000000000022512054bfa5690019d09073d75d1094d6eb9a551a5d61b0fcfc1fd474da6bfea88627010304000000004215c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac007e94635a4727997d13497f6529f00a9ca291c2e6e10253eb995eecd130a9eeb4520f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256fad20992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbbacc02116992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbb25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a000000002116f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0011820867e83e93516ecde27680f5af69af0bd633f9918874b975c7e65c0b2419047ee0000"

const EXPECTED_HEX = "70736274ff0100520200000001e0a68346c9118f584c22c9afa89b641e06127d1b1fa661788ea922261dee37600000000000fdffffff012823000000000000160014acd07b22adf2299c56909c9ca537fd2c58127ecc000000000001012b102700000000000022512054bfa5690019d09073d75d1094d6eb9a551a5d61b0fcfc1fd474da6bfea88627010304000000004114f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f9e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a40b21c79a3f1196e8d8d309eff56b4ca2f39cb2957c0a540f66aed88d1ca33bdcaea2434cc02c71c30bb2ceaa629dcdf2fd2b6a5efef019cd07bde292edeb2230d4215c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac007e94635a4727997d13497f6529f00a9ca291c2e6e10253eb995eecd130a9eeb4520f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256fad20992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbbacc02116992b50ef84354a4c0b5831bc90b36b5da98f7fc8969df5f4c88f5ec270b0dfbb25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a000000002116f02e0d96250daf3ed999f12a2a7c3c198e7d26f6bef5add3ef764831004d256f25019e450b1a6179e18dd5ab6aeff0e5172728cb84fc236261768579eb5252cd574a0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0011820867e83e93516ecde27680f5af69af0bd633f9918874b975c7e65c0b2419047ee0000"

const PRIVKEY_HEX = "7ddd6c59e93689262760f9258bb205e92d353d1d6a97c7d9d986c247fcffce1e"

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

func main() {
	psbtBytes, err := hex.DecodeString(PSBT_HEX)
	if err != nil {
		log.Fatal(err)
	}

	privkeyBytes, err := hex.DecodeString(PRIVKEY_HEX)
	if err != nil {
		log.Fatal(err)
	}

	signedPsbt, err := SignPsbtBySingleKey(
		psbtBytes,    // []byte containing PSBT
		privkeyBytes, // []byte containing private key
		1,            // TestNet
		false,        // finalize
	)
	if err != nil {
		log.Fatal(err)
	}

	if hex.EncodeToString(signedPsbt) != EXPECTED_HEX {
		log.Fatal("Signed PSBT does not match expected value")
	}

	fmt.Println("Signed PSBT: ", hex.EncodeToString(signedPsbt))
}
