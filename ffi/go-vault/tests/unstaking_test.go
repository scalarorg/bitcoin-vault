package tests

import (
	"bytes"
	"encoding/base64"
	"encoding/hex"
	"fmt"
	"testing"

	"github.com/scalarorg/bitcoin-vault/ffi/go-vault"
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

func TestUnstaking(t *testing.T) {
	tag := []byte("tsclr")
	serviceTag := []byte("proto")
	version := uint8(1)
	network := types.NetworkKindTestnet
	inputs := []types.PreviousStakingUTXO{
		{
			OutPoint: types.OutPoint{
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
	outputs := []types.UnstakingOutput{
		{
			LockingScript: []byte{0x01, 0x02, 0x03},
			Amount:        10_000,
		},
	}
	custodianPubKeys := []types.PublicKey{}
	for _, pubkey := range pubkeys {
		pubkeyBytes, _ := hex.DecodeString(pubkey)
		custodianPubKeys = append(custodianPubKeys, types.PublicKey(pubkeyBytes))
	}
	custodianQuorum := uint8(1)
	rbf := false
	feeRate := uint64(1)

	tx, err := vault.BuildCustodianOnlyUnstakingTx(tag, serviceTag, version, network, inputs, outputs, custodianPubKeys, custodianQuorum, rbf, feeRate)
	if err != nil {
		t.Fatal(err)
	}

	fmt.Println(hex.EncodeToString(tx))

	base64 := base64.StdEncoding.EncodeToString(tx)

	fmt.Println("base64: ", base64)
}
