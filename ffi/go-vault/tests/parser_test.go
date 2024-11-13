package tests

import (
	"encoding/hex"
	"testing"

	"github.com/scalarorg/bitcoin-vault/ffi/go-vault"
)

func TestParseVaultEmbeddedData(t *testing.T) {
	// Example script pubkey hex
	scriptHex := "6a3f5343414c41526c69676874000100030000000000aa36a7b91e3a8ef862567026d6f376c9f3d6b814ca433724a1db57fa3ecafcbad91d6ef068439aceeae090b6d17f"

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

	t.Logf("%+v", output)
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
