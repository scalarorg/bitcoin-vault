package encode_test

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"os"
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

func TestDecodeUPC(t *testing.T) {

	payload, _ := hex.DecodeString("010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000088f70736274ff0100bf020000000206b86ca03ea05c2fd037dc25da665331f10ab4530fcd5912cadc7f47b700dd520100000000fdffffff4ae2a5b67be3077eff310ff4185f90f0eca22128529262919a75012072f825fa0100000000fdffffff030000000000000000106a0e5343414c41520201819a5b6f5381280200000000000016001463dc22751d9a7778aa4450ceeb0b5c3ee214401cca050000000000002251203174c17382ff8b9cb77d41c21b7d5b4d9beeef04a88f26653e1fe69bea7b915f000000000001012bcd060000000000002251203174c17382ff8b9cb77d41c21b7d5b4d9beeef04a88f26653e1fe69bea7b915f0103040000000041140d1acabc9af43d39d064effd67fcec44125d7b5cf20ff23e34406fae475fc7aa04893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d532929883624076099d68b92c9a2705dc5dff3e03eee3e78b392a310e3b4fcfb1e06bde69c7392b58095d8a6d09c77e2f36d5669dc974aa5b6003a95675c4422541df7b4bff156215c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac09c739c91af9ac0e06a9e0e463c7be2848d15c31b74938543337bcdd714507801fad3a0fc920e39001c0d33c20cd193e546d1da88339806e511498762bf88e0cbcf200d1acabc9af43d39d064effd67fcec44125d7b5cf20ff23e34406fae475fc7aaad2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a2c021160d1acabc9af43d39d064effd67fcec44125d7b5cf20ff23e34406fae475fc7aa250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d5329298836200000000211615da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d53292988362000000002116594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d53292988362000000002116b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d53292988362000000002116e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d53292988362000000002116f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d532929883620000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0011820f234611d513929a2aa58d0b0b9ef341fbb9fe25c9ee6ebc2ab7ecd1d7fdbc4040001012be5020000000000002251203174c17382ff8b9cb77d41c21b7d5b4d9beeef04a88f26653e1fe69bea7b915f0103040000000041140d1acabc9af43d39d064effd67fcec44125d7b5cf20ff23e34406fae475fc7aa04893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d5329298836240d4e93e662d5f0122cc42e7569993b8cd14a58c3dfaff541de0caa37d6ab4dc93ffd577774861dd95989bfd85886f3571a44866f139a91dcd98a6075b091d0e296215c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac09c739c91af9ac0e06a9e0e463c7be2848d15c31b74938543337bcdd714507801fad3a0fc920e39001c0d33c20cd193e546d1da88339806e511498762bf88e0cbcf200d1acabc9af43d39d064effd67fcec44125d7b5cf20ff23e34406fae475fc7aaad2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a2c021160d1acabc9af43d39d064effd67fcec44125d7b5cf20ff23e34406fae475fc7aa250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d5329298836200000000211615da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d53292988362000000002116594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d53292988362000000002116b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d53292988362000000002116e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d53292988362000000002116f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5250104893e4adf1422fbb16f84e70b5af013b7c48376118fe89faa38d532929883620000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0011820f234611d513929a2aa58d0b0b9ef341fbb9fe25c9ee6ebc2ab7ecd1d7fdbc404000000000000000000000000000000000000000000")

	fmt.Println("length", len(payload))

	decoded, err := encode.DecodeContractCallWithTokenPayload(payload)
	if err != nil {
		t.Fatal(err)
	}

	// write decoded UPC to file
	os.WriteFile("upc.txt", []byte(hex.EncodeToString(decoded.Psbt)), 0644)

	fmt.Printf("UPC: %x\n\n\n\n", decoded.UPC.Psbt)
}
