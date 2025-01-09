package tests

import (
	"encoding/hex"
	"testing"

	"github.com/scalarorg/bitcoin-vault/ffi/go-vault"
)

// make test t=TestParseVaultEmbeddedData

func TestParseVaultEmbeddedData(t *testing.T) {
	// Example script pubkey hex

	tests := []struct {
		scriptHex string
		name      string
	}{
		{scriptHex: "6a3f5343414c41520101406c69676874030100000000aa36a7abbeecbbfe4732b9da50ce6b298edf47e351fc058b73c6c3f60ac6f45bb6a7d2a0080af829c76e43", name: "Only Covenants Version 1"},
		{scriptHex: "6a3f5343414c41520101406c69676874030100000000aa36a7abbeecbbfe4732b9da50ce6b298edf47e351fc058b73c6c3f60ac6f45bb6a7d2a0080af829c76e43", name: "Only Covenants Version 1"},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			scriptBytes, err := hex.DecodeString(test.scriptHex)
			if err != nil {
				t.Fatalf("Failed to decode hex: %v", err)
			}

			output, err := vault.ParseVaultEmbeddedData(scriptBytes)
			if err != nil {
				t.Fatalf("Failed to parse vault data: %v", err)
			}

			t.Logf("Tag: %s", output.Tag)
			t.Logf("Version: %d", output.Version)
			t.Logf("Flags: %+v", output.Flags)
			t.Logf("Service Tag: %s", output.ServiceTag)
			t.Logf("Transaction Type: %+v", output.TransactionType)
			t.Logf("Covenants Quorum: %+v", output.CovenantQuorum)
			t.Logf("Destination: %+x", output.DestinationChain)
			t.Logf("Destination Token Address: %+x", output.DestinationTokenAddress)
			t.Logf("Destination Recipient: %+x", output.DestinationRecipientAddress)
		})
	}
}

func TestInvalidScriptPubkey(t *testing.T) {
	scriptHex := "6a3f5343414c41526c69676874000100030000000000aa36a7b91e3a8ef862567026d6f376c9f3d6b814ca433724a1db57fa3ecafcbad91d6ef068439aceeae0"

	scriptBytes, err := hex.DecodeString(scriptHex)
	if err != nil {
		t.Fatalf("Failed to decode hex: %v", err)
	}

	result, err := vault.ParseVaultEmbeddedData(scriptBytes)
	if err == nil {
		t.Fatalf("Expected error, got nil")
	}

	if result != nil {
		t.Fatalf("Expected nil, got %+v", result)
	}
}
