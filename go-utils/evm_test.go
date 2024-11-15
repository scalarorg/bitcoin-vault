package goutils_test

import (
	"encoding/hex"
	"testing"

	goutils "github.com/scalarorg/bitcoin-vault/go-utils"
)

func TestCalculateStakingPayloadHash(t *testing.T) {

	senderBytes, _ := hex.DecodeString("24a1dB57Fa3ecAFcbaD91d6Ef068439acEeAe090")
	amount := int64(1000000)
	sourceTxHash, _ := hex.DecodeString("6490dde5442923eff18224d7da7fb2a5645373f70a64f69fd4305dd3455a0f7f")

	var sender [20]byte
	copy(sender[:], senderBytes)

	var sourceTx [32]byte
	copy(sourceTx[:], sourceTxHash)

	payloadBytes, payloadHash, _ := goutils.CalculateStakingPayloadHash(sender, amount, sourceTx)

	t.Logf("Payload bytes: %x", payloadBytes)
	t.Logf("Payload hash: %x", payloadHash)
}
