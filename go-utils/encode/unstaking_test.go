package encode_test

import (
	"bytes"
	"encoding/hex"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/scalarorg/bitcoin-vault/go-utils/encode"
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

var (
	lockingScript, _ = hex.DecodeString("001450dceca158a9c872eb405d52293d351110572c9e")
	amount           = uint64(10_000_000)
	feeOpts          = types.MinimumFee.Bytes()

	expectedEncodedMetadata, _ = hex.DecodeString("0000000000000000000000000000000000000000000000000000000000989680000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000016001450dceca158a9c872eb405d52293d351110572c9e0000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000")

	senderAddress         = common.HexToAddress("0x1234567890123456789012345678901234567890")
	sourceContractAddress = common.HexToAddress("0x9876543210987654321098765432109876543210")
	symbol                = "BTC"

	expectedPayload, _ = hex.DecodeString("00000000000000000000000012345678901234567890123456789012345678900000000000000000000000009876543210987654321098765432109876543210000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000003425443000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000989680000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000016001450dceca158a9c872eb405d52293d351110572c9e0000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000")
)

func TestCalculateUnstakingPayloadHash(
	t *testing.T,
) {
	payload, hash, err := encode.CalculateTransferRemoteMetadataPayloadHash(amount, lockingScript, feeOpts[:])
	if err != nil {
		t.Fatalf("Error calculating unstaking payload hash: %v", err)
	}

	if !bytes.Equal(payload, expectedEncodedMetadata) {
		t.Fatalf("Payload does not match expected value")
	}

	t.Logf("Payload: %x", payload)
	t.Logf("Hash: %x", hash)

	payload, hash, err = encode.CalculateTransferRemotePayloadHash(senderAddress, sourceContractAddress, symbol, expectedEncodedMetadata)
	if err != nil {
		t.Fatalf("Error calculating unstaking payload hash: %v", err)
	}

	t.Logf("Payload: %x", payload)

	if !bytes.Equal(payload, expectedPayload) {
		t.Logf("Payload: %x", payload)
		t.Fatal("Failed to encode payload")
	}

	t.Logf("Payload: %x", payload)
	t.Logf("Hash: %x", hash)
}

func TestDecodeTransferRemotePayload(t *testing.T) {
	sender, contract, symbol, metadata, err := encode.DecodeTransferRemotePayload(expectedPayload)
	if err != nil {
		t.Fatalf("Error decoding transfer remote payload: %v", err)
	}

	t.Logf("Sender Address: %x", sender)
	t.Logf("Source Contract Address: %x", contract)
	t.Logf("Symbol: %s", symbol)
	t.Logf("Metadata: %x", metadata)

	if !bytes.Equal(metadata, expectedEncodedMetadata) {
		t.Fatalf("Metadata does not match expected value")
	}

	if sender != senderAddress {
		t.Fatalf("Sender address does not match expected value")
	}

	if contract != sourceContractAddress {
		t.Fatalf("Source contract address does not match expected value")
	}

	if symbol != "BTC" {
		t.Fatalf("Symbol does not match expected value")
	}

	amount, recipientChainIdentifier, metadata, err := encode.DecodeTransferRemoteMetadataPayload(expectedEncodedMetadata)
	if err != nil {
		t.Fatalf("Error decoding transfer remote metadata payload: %v", err)
	}

	t.Logf("Amount: %d", amount)
	t.Logf("Recipient Chain Identifier: %x", recipientChainIdentifier)
	t.Logf("Metadata: %x", metadata)

	if amount != 7 {
		t.Fatalf("Amount does not match expected value")
	}

	if !bytes.Equal(recipientChainIdentifier, lockingScript) {
		t.Fatalf("Recipient chain identifier does not match expected value")
	}

	if !bytes.Equal(metadata, feeOpts[:]) {
		t.Fatalf("Metadata does not match expected value")
	}
}
