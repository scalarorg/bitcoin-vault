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

ByteBuffer create_staking_psbt(
    const uint8_t* tag,
    size_t tag_len,
    uint8_t version,
    uint64_t staking_amount,
    const uint8_t* staker_pubkey,
    size_t staker_pubkey_len,
    const uint8_t* protocol_pubkey,
    size_t protocol_pubkey_len,
    const uint8_t* custodial_pubkeys,
    size_t custodial_pubkeys_len,
    int32_t covenant_quorum,
    bool have_only_covenants,
    uint64_t destination_chain_id,
    const uint8_t* destination_smart_contract_address,
    size_t destination_smart_contract_address_len,
    const uint8_t* destination_recipient_address,
    size_t destination_recipient_address_len
);
*/
import "C"
import (
	"fmt"
	"unsafe"
)

type NetworkKind uint8

const (
	NetworkKindMainnet NetworkKind = iota
	NetworkKindTestnet
)

func (n NetworkKind) Valid() bool {
	return n == NetworkKindMainnet || n == NetworkKindTestnet
}

// CreateStakingPsbt creates a new Partially Signed Bitcoin Transaction (PSBT) for staking operations.
// Parameters:
//   - tag: byte slice containing the staking tag
//   - version: protocol version number
//   - stakingAmount: amount to stake in satoshis
//   - stakerPubkey: public key of the staker
//   - protocolPubkey: public key of the protocol
//   - custodialPubkeys: concatenated public keys of custodial participants
//   - covenantQuorum: minimum number of covenant signatures required
//   - haveOnlyCovenants: boolean indicating if only covenant signatures are needed
//   - destinationChainId: identifier of the destination chain
//   - destinationSmartContractAddress: address of the smart contract on destination chain
//   - destinationRecipientAddress: recipient's address on destination chain
//
// Returns:
//   - []byte: serialized PSBT
//   - error: error if any occurred during PSBT creation
func CreateStakingPsbt(
	tag []byte,
	version byte,
	stakingAmount uint64,
	stakerPubkey []byte,
	protocolPubkey []byte,
	custodialPubkeys []byte,
	covenantQuorum int,
	haveOnlyCovenants bool,
	destinationChainId uint64,
	destinationSmartContractAddress []byte,
	destinationRecipientAddress []byte) ([]byte, error) {
	if !network.Valid() {
		return nil, fmt.Errorf("invalid network kind")
	}

	result := C.create_staking_psbt(
		(*C.uint8_t)(unsafe.Pointer(&tag[0])),
		C.size_t(len(tag)),
		C.uint32_t(version),
		C.uint64_t(stakingAmount),
		(*C.uint8_t)(unsafe.Pointer(&stakerPubkey[0])),
		C.size_t(len(stakerPubkey)),
		(*C.uint8_t)(unsafe.Pointer(&protocolPubkey[0])),
		C.size_t(len(protocolPubkey)),
		(*C.uint8_t)(unsafe.Pointer(&custodialPubkeys[0])),
		C.size_t(len(custodialPubkeys)),
		C.int32_t(covenantQuorum),
		C.bool(haveOnlyCovenants),
		C.uint64_t(destinationChainId),
		(*C.uint8_t)(unsafe.Pointer(&destinationSmartContractAddress[0])),
		C.size_t(len(destinationSmartContractAddress)),
		(*C.uint8_t)(unsafe.Pointer(&destinationRecipientAddress[0])),
		C.size_t(len(destinationRecipientAddress)),
	)
	defer C.free_byte_buffer(result)

	if result.data == nil || result.len == 0 {
		return nil, fmt.Errorf("failed to sign PSBT: result is nil or empty")
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}
