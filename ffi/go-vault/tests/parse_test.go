package tests

import (
	"encoding/hex"
	"testing"

	"github.com/scalarorg/bitcoin-vault/ffi/go-vault"
)

func TestParseVaultEmbeddedData(t *testing.T) {
	// Example script pubkey hex
	scriptHex := "6a3f5343414c41520001806c6967687403a736aa00000000001f98c06d8734d5a9ff0b53e3294626e62e4d232c130c4810d57140e1e62967cbf742caeae91b6ece"

	scriptHexWithOnlyCovenants := "6a3a5343414c4152000140030100000000aa36a77b58e797655aa9569aa0bdf3aa842d176b44eb3324a1db57fa3ecafcbad91d6ef068439aceeae090"

	scriptHexes := []string{scriptHex, scriptHexWithOnlyCovenants}

	for _, scriptHex := range scriptHexes {
		scriptBytes, err := hex.DecodeString(scriptHex)
		if err != nil {
			t.Fatalf("Failed to decode hex: %v", err)
		}

		output, err := vault.ParseVaultEmbeddedData(scriptBytes)
		if err != nil {
			t.Fatalf("Failed to parse vault data: %v", err)
		}

		// Add assertions for the expected values
		if output.Version != 0 {
			t.Errorf("Expected version 0, got %d", output.Version)
		}
		// Add more assertions as needed

		t.Logf("Tag: %x", output.Tag)
		t.Logf("Version: %d", output.Version)
		t.Logf("Flags: %+v", output.Flags)
		t.Logf("HaveOnlyCovenants: %+v", output.HaveOnlyCovenants)
		t.Logf("Covenants Quorum: %+v", output.CovenantQuorum)
		t.Logf("Destination: %+x", output.DestinationChain)
		t.Logf("Destination Contract Address: %+x", output.DestinationContractAddress)
		t.Logf("Destination Recipient: %+x", output.DestinationRecipientAddress)
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
