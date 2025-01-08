package vault

/*
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
    uint8_t txid[32];
    uint32_t vout;
} OutPointFFI;

typedef struct {
    uint8_t* data;
    size_t len;
} ScriptBufFFI;

typedef uint64_t AmountFFI;

typedef struct {
    OutPointFFI outpoint;
    AmountFFI amount_in_sats;
    ScriptBufFFI script_pubkey;
} PreviousStakingUTXOFFI;


typedef struct {
    ScriptBufFFI locking_script;
    AmountFFI amount_in_sats;
} UnstakingOutputFFI;

typedef uint8_t PublicKeyFFI[33];

typedef struct {
    uint8_t* data;
    size_t len;
} ByteBuffer;

ByteBuffer build_with_only_covenants(
  const uint8_t* tag,
  size_t tag_len,
  const uint8_t* service_tag,
  size_t service_tag_len,
  uint8_t version,
  uint8_t network_kind,
  const PreviousStakingUTXOFFI* inputs_ptr,
  size_t inputs_len,
  const UnstakingOutputFFI* outputs_ptr,
  size_t outputs_len,
  const PublicKeyFFI* covenant_pub_keys_ptr,
  size_t covenant_pub_keys_len,
  uint8_t covenant_quorum,
  bool rbf,
  uint64_t fee_rate
);

void free_byte_buffer(ByteBuffer buffer);
*/
import "C"
import (
	"unsafe"
)

type OutPoint struct {
	Txid [32]byte
	Vout uint32
}

type ScriptBuf = []byte

type PreviousStakingUTXO struct {
	OutPoint OutPoint
	Amount   uint64
	Script   ScriptBuf
}

type UnstakingOutput struct {
	LockingScript ScriptBuf
	Amount        uint64
}

type PublicKey [33]byte

func convertInputsToFFI(inputs []PreviousStakingUTXO) ([]C.PreviousStakingUTXOFFI, []unsafe.Pointer) {
	inputsFFI := make([]C.PreviousStakingUTXOFFI, len(inputs))
	ptrs := make([]unsafe.Pointer, len(inputs))

	for i, input := range inputs {
		// Allocate C memory for script
		scriptPtr := C.CBytes(input.Script)
		ptrs[i] = scriptPtr

		inputsFFI[i] = C.PreviousStakingUTXOFFI{
			outpoint: C.OutPointFFI{
				txid: *(*[32]C.uint8_t)(unsafe.Pointer(&input.OutPoint.Txid[0])),
				vout: C.uint32_t(input.OutPoint.Vout),
			},
			amount_in_sats: C.uint64_t(input.Amount),
			script_pubkey: C.ScriptBufFFI{
				data: (*C.uint8_t)(scriptPtr),
				len:  C.size_t(len(input.Script)),
			},
		}
	}
	return inputsFFI, ptrs
}

func convertOutputsToFFI(outputs []UnstakingOutput) ([]C.UnstakingOutputFFI, []unsafe.Pointer) {
	outputsFFI := make([]C.UnstakingOutputFFI, len(outputs))
	ptrs := make([]unsafe.Pointer, len(outputs))

	for i, output := range outputs {
		scriptPtr := C.CBytes(output.LockingScript)
		ptrs[i] = scriptPtr

		outputsFFI[i] = C.UnstakingOutputFFI{
			locking_script: C.ScriptBufFFI{
				data: (*C.uint8_t)(scriptPtr),
				len:  C.size_t(len(output.LockingScript)),
			},
			amount_in_sats: C.uint64_t(output.Amount),
		}
	}
	return outputsFFI, ptrs
}

func BuildCovenantOnlyUnstakingTx(tag []byte, serviceTag []byte, version uint8, network NetworkKind, inputs []PreviousStakingUTXO, outputs []UnstakingOutput, covenantPubKeys []PublicKey, covenantQuorum uint8, rbf bool, feeRate uint64) ([]byte, error) {
	if !network.Valid() {
		return nil, ErrInvalidNetwork
	}

	inputsFFI, inputPtrs := convertInputsToFFI(inputs)
	outputsFFI, outputPtrs := convertOutputsToFFI(outputs)

	// Free C memory when done
	defer func() {
		for _, ptr := range inputPtrs {
			C.free(ptr)
		}
		for _, ptr := range outputPtrs {
			C.free(ptr)
		}
	}()

	result := C.build_with_only_covenants(
		(*C.uint8_t)(unsafe.Pointer(&tag[0])),
		C.size_t(len(tag)),
		(*C.uint8_t)(unsafe.Pointer(&serviceTag[0])),
		C.size_t(len(serviceTag)),
		C.uint8_t(version),
		C.uint8_t(network),
		(*C.PreviousStakingUTXOFFI)(unsafe.Pointer(&inputsFFI[0])),
		C.size_t(len(inputs)),
		(*C.UnstakingOutputFFI)(unsafe.Pointer(&outputsFFI[0])),
		C.size_t(len(outputs)),
		(*C.PublicKeyFFI)(unsafe.Pointer(&covenantPubKeys[0])),
		C.size_t(len(covenantPubKeys)),
		C.uint8_t(covenantQuorum),
		C.bool(rbf),
		C.uint64_t(feeRate),
	)
	defer C.free_byte_buffer(result)

	if result.data == nil || result.len == 0 {
		return nil, ErrFailedToBuildCovenantOnlyUnstakingTx
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}
