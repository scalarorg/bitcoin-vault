package tests

import (
	"encoding/hex"
	"testing"

	"github.com/scalarorg/bitcoin-vault/ffi/go-vault"
)

var EXPECTED_TAP_SCRIPT_SIGS []vault.TapScriptSig

func Init() {
	tapScriptSig1Key, _ := hex.DecodeString("b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102")
	tapScriptSig1LeafHash, _ := hex.DecodeString("5a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb")
	tapScriptSig1Signature, _ := hex.DecodeString("ace560e1711c76f8df381f8a3ba2f5b9591ef7da5598e099f2e06a8ad3e8a79ac42666d9d3a2b8a212cc06b19a9a6b6871cec691c529ebf50c8368d695d5727a")

	tapScriptSig2Key, _ := hex.DecodeString("b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102")
	tapScriptSig2LeafHash, _ := hex.DecodeString("5a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb")
	tapScriptSig2Signature, _ := hex.DecodeString("593a39b8149fbfa87ab2c40b04f07db4de5c1f1023ecc8f76edb160b60c84df3c457b53988ba20151f2dec0e0c41108c7434219cf9d3376bbadb798df5bbcc49")

	EXPECTED_TAP_SCRIPT_SIGS = []vault.TapScriptSig{
		{
			KeyXOnly:  *(*[32]byte)(tapScriptSig1Key),
			LeafHash:  *(*[32]byte)(tapScriptSig1LeafHash),
			Signature: *(*[64]byte)(tapScriptSig1Signature),
		},
		{
			KeyXOnly:  *(*[32]byte)(tapScriptSig2Key),
			LeafHash:  *(*[32]byte)(tapScriptSig2LeafHash),
			Signature: *(*[64]byte)(tapScriptSig2Signature),
		},
	}
}

// Test constants moved to package level
const (
	PSBT_HEX    = "70736274ff0100a6020000000287ca13fc0a9424c6a0b372ac69d48b0df1ef690ada0a54148c912016b7e3aaaa0000000000fdffffff86b6764fd56f990f628958577fe3799e98696e41ecbaa78e7cc8ea70575ff2e80000000000fdffffff02a11900000000000016001450dceca158a9c872eb405d52293d351110572c9ee8f10200000000002251207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d5000000000001012ba0860100000000002251207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d5010304000000002215c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0ad2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a2c0211615da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e164148825015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb781125015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc610225015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff25015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb525015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac00118205a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb0001012ba0860100000000002251207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d5010304000000002215c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0ad2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a2c0211615da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e164148825015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb781125015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc610225015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff25015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb525015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac00118205a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000"
	PRIVKEY_HEX = "f92d44713b18ec56bf387201b0439d8e8ef0731235d487f81c5f3d5f18a52af3"
)

// Helper function to decode hex string and handle error
func mustDecodeHex(t *testing.T, s string) []byte {
	decoded, err := hex.DecodeString(s)
	if err != nil {
		t.Fatalf("Failed to decode hex: %v", err)
	}
	return decoded
}

func TestSignPsbtAndCollectSigs(t *testing.T) {
	Init()

	// Setup test inputs
	psbtBytes := mustDecodeHex(t, PSBT_HEX)
	privkeyBytes := mustDecodeHex(t, PRIVKEY_HEX)

	// Execute the function being tested
	tapScriptSigs, err := vault.SignPsbtAndCollectSigs(
		psbtBytes,
		privkeyBytes,
		vault.NetworkKindTestnet,
	)
	if err != nil {
		t.Fatalf("SignPsbtAndCollectSigs failed: %v", err)
	}

	// Verify results
	if len(tapScriptSigs) != len(EXPECTED_TAP_SCRIPT_SIGS) {
		t.Errorf("Expected %d signatures, got %d", len(EXPECTED_TAP_SCRIPT_SIGS), len(tapScriptSigs))
	}

	for i, actual := range tapScriptSigs {
		expected := EXPECTED_TAP_SCRIPT_SIGS[i]
		t.Logf("Verifying signature %d", i)
		t.Logf("KeyXOnly: %x", actual.KeyXOnly)

		if actual.KeyXOnly != expected.KeyXOnly {
			t.Errorf("TapScriptSig KeyXOnly mismatch at index %d\nexpected: %x\ngot: %x",
				i, expected.KeyXOnly, actual.KeyXOnly)
		}

		if actual.LeafHash != expected.LeafHash {
			t.Errorf("TapScriptSig LeafHash mismatch at index %d\nexpected: %x\ngot: %x",
				i, expected.LeafHash, actual.LeafHash)
		}
		if actual.Signature != expected.Signature {
			t.Errorf("TapScriptSig Signature mismatch at index %d\nexpected: %x\ngot: %x",
				i, expected.Signature, actual.Signature)
		}
	}

	t.Logf("SignPsbtAndCollectSigs passed")
}
