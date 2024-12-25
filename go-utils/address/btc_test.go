package address_test

import (
	"encoding/hex"
	"testing"

	"github.com/scalarorg/bitcoin-vault/go-utils/address"
)

func TestScriptPubKeyToAddress(t *testing.T) {
	scriptPubKey, err := hex.DecodeString("51200f94f9d9c4c6e39cbef6c708b632173d8007b827936907176e19495c3e355c12")
	if err != nil {
		t.Fatal(err)
	}

	add, err := address.ScriptPubKeyToAddress(scriptPubKey, "testnet3")
	if err != nil {
		t.Fatal(err)
	}

	t.Logf("address: %s", add.String())

	scriptPubKey, err = hex.DecodeString("001450dceca158a9c872eb405d52293d351110572c9e")
	if err != nil {
		t.Fatal(err)
	}

	add, err = address.ScriptPubKeyToAddress(scriptPubKey, "testnet4")
	if err != nil {
		t.Fatal(err)
	}
	t.Logf("address: %s", add.String())
}
