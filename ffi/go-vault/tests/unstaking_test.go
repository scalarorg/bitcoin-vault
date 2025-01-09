package tests

import (
	"bytes"
	"encoding/base64"
	"encoding/hex"
	"fmt"
	"testing"

	"github.com/scalarorg/bitcoin-vault/ffi/go-vault"
)



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
