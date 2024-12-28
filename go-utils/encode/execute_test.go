package encode_test

import (
	"encoding/hex"
	"testing"

	"github.com/scalarorg/bitcoin-vault/go-utils/encode"
)

func TestCalculateDestPayload(t *testing.T) {

	senderBytes, _ := hex.DecodeString("24a1dB57Fa3ecAFcbaD91d6Ef068439acEeAe090")
	amount := uint64(1000000)
	sourceTxHash := "6490dde5442923eff18224d7da7fb2a5645373f70a64f69fd4305dd3455a0f7f"

	payloadBytes, payloadHash, _ := encode.SafeCalculateDestPayload(amount, sourceTxHash, senderBytes)

	t.Logf("Payload bytes: %x", payloadBytes)
	t.Logf("Payload hash: %x", payloadHash)

	// 00000000000000000000000000000000000000000000000000000000000f42406490dde5442923eff18224d7da7fb2a5645373f70a64f69fd4305dd3455a0f7f0000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000001424a1db57fa3ecafcbad91d6ef068439aceeae090000000000000000000000000
}

func TestDecodeSourcePayload(t *testing.T) {
	payload, _ := hex.DecodeString("00000000000000000000000000000000000000000000000000000000000f42406490dde5442923eff18224d7da7fb2a5645373f70a64f69fd4305dd3455a0f7f0000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000001424a1db57fa3ecafcbad91d6ef068439aceeae090000000000000000000000000")
	amount, sourceTx, recipientChainIdentifier, err := encode.DecodeDestPayload(payload)
	if err != nil {
		t.Fatalf("failed to decode source payload: %v", err)
	}
	t.Logf("amount: %v", amount)
	t.Logf("sourceTx: %x", sourceTx)
	t.Logf("recipientChainIdentifier: %x", recipientChainIdentifier)
}


