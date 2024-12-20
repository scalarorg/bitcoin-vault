package encode_test

import (
	"encoding/hex"
	"testing"

	"github.com/scalarorg/bitcoin-vault/go-utils/encode"
)

func TestCalculateStakingPayloadHash(t *testing.T) {

	senderBytes, _ := hex.DecodeString("24a1dB57Fa3ecAFcbaD91d6Ef068439acEeAe090")
	amount := uint64(1000000)
	sourceTxHash, _ := hex.DecodeString("6490dde5442923eff18224d7da7fb2a5645373f70a64f69fd4305dd3455a0f7f")

	var sender [20]byte
	copy(sender[:], senderBytes)

	var sourceTx [32]byte
	copy(sourceTx[:], sourceTxHash)

	payloadBytes, payloadHash, _ := encode.CalculateStakingPayloadHash(sender, amount, sourceTx)

	t.Logf("Payload bytes: %x", payloadBytes)
	t.Logf("Payload hash: %x", payloadHash)

	// 00000000000000000000000024a1db57fa3ecafcbad91d6ef068439aceeae09000000000000000000000000000000000000000000000000000000000000f42406490dde5442923eff18224d7da7fb2a5645373f70a64f69fd4305dd3455a0f7f
}
