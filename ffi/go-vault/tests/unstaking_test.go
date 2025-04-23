package tests

import (
	"bytes"
	"encoding/base64"
	"encoding/hex"
	"fmt"
	"testing"

	"github.com/scalarorg/bitcoin-vault/ffi/go-vault"
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
	"github.com/stretchr/testify/require"
	"golang.org/x/crypto/sha3"
)

func TestUnstaking(t *testing.T) {
	tag := []byte("tsclr")
	serviceTag := []byte("proto")
	version := uint8(1)
	network := types.NetworkKindTestnet
	inputs := []types.PreviousOutpoint{
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
	outputs := []types.UnlockingOutput{
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
	inputs := []types.PreviousOutpoint{
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
	outputs := []types.UnlockingOutput{
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

// CGO_LDFLAGS="-L./lib -lbitcoin_vault_ffi" CGO_CFLAGS="-I./lib" go test -timeout 30s -run ^TestEncodePoolingRedeemParams$ github.com/scalarorg/bitcoin-vault/ffi/go-vault/tests
func TestEncodePoolingRedeemParams(t *testing.T) {

	custodianGroupUID := sha3.Sum256([]byte("scalarv32"))
	tag := []byte("SCALAR")
	serviceTag := []byte("pools")
	version := uint8(3)
	network := types.NetworkKindTestnet
	inputs := []types.PreviousOutpoint{
		{
			OutPoint: types.OutPoint{
				Txid: [32]byte{82, 192, 23, 61, 98, 192, 198, 167, 154, 178, 218, 24, 63, 5, 149, 128, 253, 153, 108, 36, 114, 120, 148, 211, 224, 246, 207, 54, 163, 203, 119, 115},
				Vout: 0,
			},
			Amount: 1000,
			Script: []byte{81, 32, 168, 252, 80, 216, 127, 22, 216, 146, 180, 212, 208, 135, 210, 89, 192, 171, 65, 126, 16, 107, 4, 75, 41, 26, 119, 40, 210, 174, 19, 67, 222, 127},
		},
	}
	outputs := []types.UnlockingOutput{
		{
			LockingScript: []byte{0, 20, 99, 220, 34, 117, 29, 154, 119, 120, 170, 68, 80, 206, 235, 11, 92, 62, 226, 20, 64, 28},
			Amount:        1000,
		},
		{
			LockingScript: []byte{0, 20, 99, 220, 34, 117, 29, 154, 119, 120, 170, 68, 80, 206, 235, 11, 92, 62, 226, 20, 64, 28},
			Amount:        1000,
		},
	}
	custodianPubKeys := []types.PublicKey{}
	for _, pubkey := range pubkeys {
		pubkeyBytes, _ := hex.DecodeString(pubkey)
		custodianPubKeys = append(custodianPubKeys, types.PublicKey(pubkeyBytes))
	}
	custodianQuorum := uint8(3)
	rbf := false
	feeRate := uint64(1)
	sessionSequence := uint64(1)
	data := vault.EncodePoolingRedeemParams(
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
		custodianGroupUID[:])
	require.Equal(t, "065343414c415205706f6f6c730301000000010000004e52c0173d62c0c6a79ab2da183f059580fd996c24727894d3e0f6cf36a3cb77730000000000000000000003e85120a8fc50d87f16d892b4d4d087d259c0ab417e106b044b291a7728d2ae1343de7f000000020000001e00000000000003e8001463dc22751d9a7778aa4450ceeb0b5c3ee214401c0000001e00000000000003e8001463dc22751d9a7778aa4450ceeb0b5c3ee214401c000000050215da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e164148802f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb503594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb781103b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc610203e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff0300000000000000000100000000000000013e79326a9493896e13af62194e694dff4c9300700407449363564b0eaeaf07e8", hex.EncodeToString(data))
	fmt.Println(hex.EncodeToString(data))

}

// CGO_LDFLAGS="-L./lib -lbitcoin_vault_ffi" CGO_CFLAGS="-I./lib" go test -timeout 30s -run ^TestEncodePoolingRedeemParamsV33$ github.com/scalarorg/bitcoin-vault/ffi/go-vault/tests
func TestEncodePoolingRedeemParamsV33(t *testing.T) {

	custodianGroupUID := sha3.Sum256([]byte("scalarv33"))
	tag := []byte("SCALAR")
	serviceTag := []byte("pools")
	version := uint8(3)
	network := types.NetworkKindTestnet
	inputs := []types.PreviousOutpoint{
		{
			OutPoint: types.OutPoint{
				Txid: [32]byte{82, 192, 23, 61, 98, 192, 198, 167, 154, 178, 218, 24, 63, 5, 149, 128, 253, 153, 108, 36, 114, 120, 148, 211, 224, 246, 207, 54, 163, 203, 119, 115},
				Vout: 0,
			},
			Amount: 1000,
			Script: []byte{81, 32, 168, 252, 80, 216, 127, 22, 216, 146, 180, 212, 208, 135, 210, 89, 192, 171, 65, 126, 16, 107, 4, 75, 41, 26, 119, 40, 210, 174, 19, 67, 222, 127},
		},
	}
	outputs := []types.UnlockingOutput{
		{
			LockingScript: []byte{0, 20, 99, 220, 34, 117, 29, 154, 119, 120, 170, 68, 80, 206, 235, 11, 92, 62, 226, 20, 64, 28},
			Amount:        1000,
		},
	}
	custodianPubKeys := []types.PublicKey{}
	for _, pubkey := range pubkeys {
		pubkeyBytes, _ := hex.DecodeString(pubkey)
		custodianPubKeys = append(custodianPubKeys, types.PublicKey(pubkeyBytes))
	}
	custodianQuorum := uint8(3)
	rbf := false
	feeRate := uint64(1)
	sessionSequence := uint64(1)
	data := vault.EncodePoolingRedeemParams(
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
		custodianGroupUID[:])
	fmt.Println(hex.EncodeToString(data))
	require.Equal(t, "065343414c415205706f6f6c730301000000010000004e52c0173d62c0c6a79ab2da183f059580fd996c24727894d3e0f6cf36a3cb77730000000000000000000003e85120a8fc50d87f16d892b4d4d087d259c0ab417e106b044b291a7728d2ae1343de7f000000010000001e00000000000003e8001463dc22751d9a7778aa4450ceeb0b5c3ee214401c000000050215da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e164148802f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb503594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb781103b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc610203e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff030000000000000000010000000000000001bffb71bf819ae4cb65188905ac54763a09144bc3a0629808d7142dd5dbd98693", hex.EncodeToString(data))
	psbt, err := vault.BuildPoolingRedeemTx(tag,
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
		custodianGroupUID[:])
	require.NoError(t, err)
	fmt.Println("psbt: ", hex.EncodeToString(psbt))
	base64 := base64.StdEncoding.EncodeToString(psbt)
	fmt.Println("base64: ", base64)
}
