package tests

import (
	"testing"

	vault "github.com/scalarorg/bitcoin-vault/ffi/go"
)

func TestLockingScript(t *testing.T) {
	scripts, err := vault.CustodiansOnlyLockingScript(custodianPubKeys, 3)
	if err != nil {
		t.Fatalf("Failed to build taproot script: %v", err)
	}

	t.Logf("Taproot script: %x", scripts)
}
