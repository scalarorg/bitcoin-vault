package tests

import (
	"bytes"
	"encoding/base64"
	"encoding/hex"
	"fmt"
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

func TestUnstaking(t *testing.T) {
	tag := []byte("tsclr")
	serviceTag := []byte("proto")
	version := uint8(1)
	network := vault.NetworkKindTestnet
	inputs := []vault.PreviousStakingUTXO{
		{
			OutPoint: vault.OutPoint{
				Txid: func() [32]byte {
					var txid [32]byte
					copy(txid[:], bytes.Repeat([]byte{0x01}, 32))
					return txid
				}(),
				Vout: 0,
			},
			Amount: 100000000,
			Script: []byte{0x01, 0x02, 0x03},
		},
	}
	outputs := []vault.UnstakingOutput{
		{
			LockingScript: []byte{0x01, 0x02, 0x03},
			Amount:        100000000,
		},
	}
	covenantPubKeys := []vault.PublicKey{}
	for _, pubkey := range pubkeys {
		pubkeyBytes, _ := hex.DecodeString(pubkey)
		covenantPubKeys = append(covenantPubKeys, vault.PublicKey(pubkeyBytes))
	}
	covenantQuorum := uint8(1)
	rbf := false
	feeRate := uint64(1)

	tx, err := vault.BuildCovenantOnlyUnstakingTx(tag, serviceTag, version, network, inputs, outputs, covenantPubKeys, covenantQuorum, rbf, feeRate)
	if err != nil {
		t.Fatal(err)
	}

	fmt.Println(hex.EncodeToString(tx))

	base64 := base64.StdEncoding.EncodeToString(tx)

	fmt.Println("base64: ", base64)
}
