package tests

import (
	"testing"

	"github.com/scalarorg/bitcoin-vault/ffi/go-vault"
)

func TestLockingScript(t *testing.T) {
	scripts, err := vault.OnlyCovenantsLockingScript(custodianPubKeys, 3)
	if err != nil {
		t.Fatalf("Failed to build taproot script: %v", err)
	}

	t.Logf("Taproot script: %x", scripts)
}
