package encode_test

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"testing"

	"github.com/scalarorg/bitcoin-vault/go-utils/encode"
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

func TestEncodeCustodianOnly(t *testing.T) {
	lockingScript, _ := hex.DecodeString("001450dceca158a9c872eb405d52293d351110572c9e")
	feeOptions := types.MinimumFee
	rbf := true

	payload, hash, err := encode.CalculateContractCallWithTokenPayload(
		encode.ContractCallWithTokenPayload{
			PayloadType: encode.ContractCallWithTokenPayloadType_CustodianOnly,
			CustodianOnly: &encode.CustodianOnly{
				FeeOptions:               feeOptions,
				RBF:                      rbf,
				RecipientChainIdentifier: lockingScript,
			},
		},
	)
	if err != nil {
		t.Fatal(err)
	}

	fmt.Println(hex.EncodeToString(payload))
	fmt.Println(hex.EncodeToString(hash))

	decoded, err := encode.DecodeContractCallWithTokenPayload(payload)
	if err != nil {
		t.Fatal(err)
	}

	fmt.Println("custodian only", decoded.CustodianOnly)

	if decoded.PayloadType != encode.ContractCallWithTokenPayloadType_CustodianOnly {
		t.Fatalf("expected %v, got %v", encode.ContractCallWithTokenPayloadType_CustodianOnly, decoded.PayloadType)
	}

	if decoded.CustodianOnly.FeeOptions != feeOptions {
		t.Fatalf("expected %v, got %v", feeOptions, decoded.CustodianOnly.FeeOptions)
	}

	if decoded.CustodianOnly.RBF != rbf {
		t.Fatalf("expected %v, got %v", rbf, decoded.CustodianOnly.RBF)
	}

	if !bytes.Equal(decoded.CustodianOnly.RecipientChainIdentifier, lockingScript) {
		t.Fatalf("expected %v, got %v", lockingScript, decoded.CustodianOnly.RecipientChainIdentifier)
	}
}

func TestEncodeUPC(t *testing.T) {
	psbt, _ := hex.DecodeString("0000123456")

	payload, hash, err := encode.CalculateContractCallWithTokenPayload(
		encode.ContractCallWithTokenPayload{
			PayloadType: encode.ContractCallWithTokenPayloadType_UPC,
			UPC:         &encode.UPC{Psbt: psbt},
		},
	)

	if err != nil {
		t.Fatal(err)
	}

	fmt.Println(hex.EncodeToString(payload))
	fmt.Println(hex.EncodeToString(hash))

	decoded, err := encode.DecodeContractCallWithTokenPayload(payload)
	if err != nil {
		t.Fatal(err)
	}

	fmt.Printf("upc: %x\n", decoded.UPC.Psbt)

	if decoded.PayloadType != encode.ContractCallWithTokenPayloadType_UPC {
		t.Fatalf("expected %v, got %v", encode.ContractCallWithTokenPayloadType_UPC, decoded.PayloadType)
	}

	if !bytes.Equal(decoded.UPC.Psbt, psbt) {
		t.Fatalf("expected %v, got %v", psbt, decoded.UPC.Psbt)
	}
}

func TestDecodeCustodianOnly(t *testing.T) {

	payload, _ := hex.DecodeString("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000016001450dceca158a9c872eb405d52293d351110572c9e00000000000000000000")

	fmt.Println("length", len(payload))

	decoded, err := encode.DecodeContractCallWithTokenPayload(payload)
	if err != nil {
		t.Fatal(err)
	}

	fmt.Println("custodian only", decoded.CustodianOnly)
}
