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

func TestPoolingRedeem(t *testing.T) {
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
	sessionSequence := uint64(1)
	custodianGroupUID := []byte{
		0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
		0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
	}

	tx, err := vault.BuildPoolingRedeemTx(
		tag,
		serviceTag,
		version,
		network,
		inputs,
		outputs,
		custodianPubKeys,
		custodianQuorum,
		rbf,
		feeRate,
		sessionSequence,
		custodianGroupUID)
	if err != nil {
		t.Fatal(err)
	}

	fmt.Println(hex.EncodeToString(tx))

	base64 := base64.StdEncoding.EncodeToString(tx)

	fmt.Println("base64: ", base64)
}
