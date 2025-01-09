package tests

import (
	"encoding/hex"
	"os"
	"testing"

	"github.com/scalarorg/bitcoin-vault/ffi/go-vault"
)

var pubkeys = []string{
	"0215da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488",
	"02f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5",
	"03594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811",
	"03b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102",
	"03e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff",
}

var custodianPubKeys = []vault.PublicKey{}

func TestMain(m *testing.M) {
	for _, pubkey := range pubkeys {
		p := mustDecodeHex(pubkey)
		custodianPubKeys = append(custodianPubKeys, vault.PublicKey(p))
	}

	os.Exit(m.Run())
}

func mustDecodeHex(s string) []byte {
	decoded, err := hex.DecodeString(s)
	if err != nil {
		panic(err)
	}
	return decoded
}
