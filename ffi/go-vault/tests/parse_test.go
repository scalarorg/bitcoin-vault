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
		{scriptHex: "6a3f5343414c41520001806c6967687403a736aa00000000001f98c06d8734d5a9ff0b53e3294626e62e4d232c130c4810d57140e1e62967cbf742caeae91b6ece", name: "Protocol Version 0"},
		{scriptHex: "6a3a5343414c4152000140030100000000aa36a77b58e797655aa9569aa0bdf3aa842d176b44eb3324a1db57fa3ecafcbad91d6ef068439aceeae090", name: "Only Covenants Version 0"},
		{scriptHex: "6a3a5343414c4152010140030100000000aa36a71f98c06d8734d5a9ff0b53e3294626e62e4d232c130c4810d57140e1e62967cbf742caeae91b6ece", name: "Only Covenants Version 1"},
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

			t.Logf("Tag: %x", output.Tag)
			t.Logf("Version: %d", output.Version)
			t.Logf("Flags: %+v", output.Flags)
			t.Logf("HaveOnlyCovenants: %+v", output.HaveOnlyCovenants)
			t.Logf("Covenants Quorum: %+v", output.CovenantQuorum)
			t.Logf("Destination: %+x", output.DestinationChain)
			t.Logf("Destination Contract Address: %+x", output.DestinationContractAddress)
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
