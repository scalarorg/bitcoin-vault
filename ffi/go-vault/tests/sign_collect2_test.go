package tests

import (
	"testing"

	"github.com/scalarorg/bitcoin-vault/ffi/go-vault"
	go_utils "github.com/scalarorg/bitcoin-vault/go-utils/types"
)

var MockTapScriptSigs map[string][]go_utils.TapScriptSig = map[string][]go_utils.TapScriptSig{
	"1": {
		{
			KeyXOnly:  [32]byte{181, 158, 87, 92, 239, 135, 62, 169, 82, 115, 175, 213, 89, 86, 200, 69, 144, 80, 114, 0, 212, 16, 230, 147, 228, 176, 121, 164, 38, 204, 97, 2},
			Signature: [64]byte{221, 237, 58, 54, 12, 123, 139, 142, 9, 128, 96, 95, 105, 253, 21, 121, 199, 253, 66, 172, 81, 184, 32, 236, 154, 30, 69, 184, 44, 106, 98, 139, 208, 242, 122, 43, 190, 200, 158, 149, 8, 10, 190, 76, 96, 25, 230, 220, 30, 40, 160, 38, 163, 21, 121, 39, 211, 237, 255, 185, 176, 127, 157, 66},
			LeafHash:  [32]byte{90, 16, 165, 236, 114, 150, 41, 198, 221, 134, 61, 194, 139, 113, 98, 225, 143, 150, 176, 13, 237, 216, 127, 21, 139, 34, 132, 40, 162, 152, 188, 203},
		},
		{
			KeyXOnly:  [32]byte{181, 158, 87, 92, 239, 135, 62, 169, 82, 115, 175, 213, 89, 86, 200, 69, 144, 80, 114, 0, 212, 16, 230, 147, 228, 176, 121, 164, 38, 204, 97, 2},
			Signature: [64]byte{244, 44, 248, 81, 51, 16, 107, 113, 210, 183, 125, 94, 125, 196, 231, 67, 8, 53, 247, 144, 218, 133, 238, 59, 172, 52, 66, 240, 248, 9, 216, 117, 53, 38, 238, 197, 170, 48, 223, 15, 65, 49, 75, 242, 203, 223, 248, 52, 165, 251, 40, 195, 120, 156, 244, 69, 124, 108, 127, 246, 170, 185, 45, 224},
			LeafHash:  [32]byte{90, 16, 165, 236, 114, 150, 41, 198, 221, 134, 61, 194, 139, 113, 98, 225, 143, 150, 176, 13, 237, 216, 127, 21, 139, 34, 132, 40, 162, 152, 188, 203},
		},
	},

	"2": {
		{
			KeyXOnly: [32]byte{240, 243, 217, 190, 175, 122, 57, 69, 188, 170, 20, 126, 4, 26, 225, 213, 202, 2, 155, 222, 126, 64, 216, 37, 31, 7, 131, 214, 236, 190, 143, 181},
			Signature: [64]byte{
				248, 115, 180, 134, 206, 189, 23, 228, 18, 99, 78, 37, 78, 101, 124, 3, 107, 116, 191, 139, 212, 86, 162, 56, 237, 171, 36, 83, 217, 139, 18, 170, 118, 52, 71, 168, 205, 136, 76, 207, 72, 139, 7, 89, 218, 176, 211, 112, 88, 177, 191, 83, 159, 30, 184, 73, 37, 121, 2, 50, 94, 172, 113, 118,
			},
			LeafHash: [32]byte{90, 16, 165, 236, 114, 150, 41, 198, 221, 134, 61, 194, 139, 113, 98, 225, 143, 150, 176, 13, 237, 216, 127, 21, 139, 34, 132, 40, 162, 152, 188, 203},
		},
		{
			KeyXOnly: [32]byte{240, 243, 217, 190, 175, 122, 57, 69, 188, 170, 20, 126, 4, 26, 225, 213, 202, 2, 155, 222, 126, 64, 216, 37, 31, 7, 131, 214, 236, 190, 143, 181},
			Signature: [64]byte{
				230, 97, 46, 117, 159, 251, 236, 58, 117, 172, 28, 26, 161, 59, 126, 10, 212, 60, 227, 159, 46, 207, 1, 235, 31, 93, 130, 55, 45, 178, 124, 217, 37, 216, 172, 166, 252, 62, 30, 126, 211, 222, 34, 179, 106, 248, 55, 100, 101, 174, 99, 82, 183, 236, 130, 230, 5, 176, 181, 245, 35, 5, 126, 69,
			},
			LeafHash: [32]byte{90, 16, 165, 236, 114, 150, 41, 198, 221, 134, 61, 194, 139, 113, 98, 225, 143, 150, 176, 13, 237, 216, 127, 21, 139, 34, 132, 40, 162, 152, 188, 203},
		},
	},
	"3": {
		{
			KeyXOnly:  [32]byte{89, 78, 120, 192, 162, 150, 130, 16, 217, 193, 85, 13, 74, 211, 27, 3, 213, 228, 185, 101, 156, 242, 246, 120, 66, 72, 59, 179, 194, 187, 120, 17},
			Signature: [64]byte{80, 39, 197, 216, 233, 38, 36, 133, 172, 15, 178, 147, 33, 29, 253, 142, 2, 55, 237, 243, 255, 60, 64, 73, 165, 226, 126, 123, 179, 43, 185, 49, 222, 15, 5, 200, 48, 8, 242, 190, 156, 104, 104, 156, 195, 152, 55, 3, 54, 14, 251, 137, 15, 56, 109, 6, 228, 218, 12, 117, 182, 19, 2, 48},
			LeafHash:  [32]byte{90, 16, 165, 236, 114, 150, 41, 198, 221, 134, 61, 194, 139, 113, 98, 225, 143, 150, 176, 13, 237, 216, 127, 21, 139, 34, 132, 40, 162, 152, 188, 203},
		},
		{
			KeyXOnly:  [32]byte{89, 78, 120, 192, 162, 150, 130, 16, 217, 193, 85, 13, 74, 211, 27, 3, 213, 228, 185, 101, 156, 242, 246, 120, 66, 72, 59, 179, 194, 187, 120, 17},
			Signature: [64]byte{123, 69, 188, 144, 137, 75, 220, 80, 176, 117, 18, 232, 244, 32, 73, 63, 94, 196, 123, 200, 215, 116, 187, 150, 28, 227, 124, 1, 150, 60, 65, 34, 111, 48, 11, 108, 60, 79, 37, 25, 70, 227, 247, 89, 58, 175, 86, 182, 6, 188, 72, 205, 189, 144, 209, 178, 67, 116, 127, 31, 189, 42, 193, 96},
			LeafHash:  [32]byte{90, 16, 165, 236, 114, 150, 41, 198, 221, 134, 61, 194, 139, 113, 98, 225, 143, 150, 176, 13, 237, 216, 127, 21, 139, 34, 132, 40, 162, 152, 188, 203},
		},
	},
	"4": {
		{
			KeyXOnly:  [32]byte{21, 218, 145, 59, 62, 135, 180, 147, 43, 30, 27, 135, 217, 102, 124, 40, 231, 37, 10, 160, 237, 96, 179, 163, 16, 149, 245, 65, 225, 100, 20, 136},
			Signature: [64]byte{48, 220, 139, 250, 50, 174, 220, 196, 61, 237, 98, 160, 99, 16, 49, 250, 191, 60, 121, 82, 213, 227, 235, 242, 230, 141, 145, 183, 221, 171, 18, 16, 210, 6, 218, 211, 229, 63, 39, 255, 1, 197, 159, 234, 83, 93, 227, 200, 48, 96, 28, 184, 0, 23, 254, 212, 208, 19, 169, 7, 130, 39, 43, 75},
			LeafHash:  [32]byte{90, 16, 165, 236, 114, 150, 41, 198, 221, 134, 61, 194, 139, 113, 98, 225, 143, 150, 176, 13, 237, 216, 127, 21, 139, 34, 132, 40, 162, 152, 188, 203},
		}, {
			KeyXOnly: [32]byte{
				21, 218, 145, 59, 62, 135, 180, 147, 43, 30, 27, 135, 217, 102, 124, 40, 231, 37, 10, 160, 237, 96, 179, 163, 16, 149, 245, 65, 225, 100, 20, 136,
			},
			Signature: [64]byte{246, 195, 19, 170, 208, 155, 143, 251, 103, 59, 160, 117, 2, 0, 54, 45, 108, 50, 128, 152, 128, 61, 110, 30, 153, 71, 70, 107, 163, 35, 132, 85, 56, 29, 202, 46, 140, 199, 14, 131, 29, 8, 19, 182, 211, 130, 152, 138, 108, 39, 34, 106, 145, 190, 108, 247, 0, 213, 134, 158, 220, 145, 253, 167},
			LeafHash:  [32]byte{90, 16, 165, 236, 114, 150, 41, 198, 221, 134, 61, 194, 139, 113, 98, 225, 143, 150, 176, 13, 237, 216, 127, 21, 139, 34, 132, 40, 162, 152, 188, 203},
		},
	},
}

// Test constants moved to package level
const (
	PSBT_HEX2 = "70736274ff0100a602000000022aab2ff2a776da8dc894306e83562776a664e56ec64d346d61b79c819965394a0000000000fdffffff6bdd8c7e85c6a5599ca62f758ecac1369ebc14fec9569c84678a7aa12a371bcd0000000000fdffffff02a11900000000000016001450dceca158a9c872eb405d52293d351110572c9ee8f10200000000002251207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d5000000000001012ba0860100000000002251207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d5010304000000002215c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0ad2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a2c0211615da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e164148825015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb781125015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc610225015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff25015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb525015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac00118205a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb0001012ba0860100000000002251207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d5010304000000002215c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0ad2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a2c0211615da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e164148825015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb781125015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc610225015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfff25015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000002116f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb525015a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb0000000001172050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac00118205a10a5ec729629c6dd863dc28b7162e18f96b00dedd87f158b228428a298bccb000000"
)

// Helper function to decode hex string and handle error

func TestSignCollect2(t *testing.T) {
	Init()

	// Setup test inputs
	psbtBytes := mustDecodeHex(PSBT_HEX2)
	var err error

	for _, tapScriptSigs := range MockTapScriptSigs {
		psbtBytes, err = vault.AggregateTapScriptSigs(
			psbtBytes,
			tapScriptSigs,
		)
		if err != nil {
			t.Fatalf("AggregateTapScriptSigs failed: %v", err)
		}
	}

	tx, err := vault.FinalizePsbtAndExtractTx(psbtBytes)
	if err != nil {
		t.Fatalf("FinalizePSBT failed: %v", err)
	}

	t.Logf("Tx: %x", tx)

}

// 020000000001022aab2ff2a776da8dc894306e83562776a664e56ec64d346d61b79c819965394a0000000000fdffffff6bdd8c7e85c6a5599ca62f758ecac1369ebc14fec9569c84678a7aa12a371bcd0000000000fdffffff02a11900000000000016001450dceca158a9c872eb405d52293d351110572c9ee8f10200000000002251207f815abf6dfd78423a708aa8db1c2c906eecac910c035132d342e4988a37b8d50740f873b486cebd17e412634e254e657c036b74bf8bd456a238edab2453d98b12aa763447a8cd884ccf488b0759dab0d37058b1bf539f1eb849257902325eac71760040dded3a360c7b8b8e0980605f69fd1579c7fd42ac51b820ec9a1e45b82c6a628bd0f27a2bbec89e95080abe4c6019e6dc1e28a026a3157927d3edffb9b07f9d42405027c5d8e9262485ac0fb293211dfd8e0237edf3ff3c4049a5e27e7bb32bb931de0f05c83008f2be9c68689cc3983703360efb890f386d06e4da0c75b61302304030dc8bfa32aedcc43ded62a0631031fabf3c7952d5e3ebf2e68d91b7ddab1210d206dad3e53f27ff01c59fea535de3c830601cb80017fed4d013a90782272b4bac2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a221c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac00740e6612e759ffbec3a75ac1c1aa13b7e0ad43ce39f2ecf01eb1f5d82372db27cd925d8aca6fc3e1e7ed3de22b36af8376465ae6352b7ec82e605b0b5f523057e450040f42cf85133106b71d2b77d5e7dc4e7430835f790da85ee3bac3442f0f809d8753526eec5aa30df0f41314bf2cbdff834a5fb28c3789cf4457c6c7ff6aab92de0407b45bc90894bdc50b07512e8f420493f5ec47bc8d774bb961ce37c01963c41226f300b6c3c4f251946e3f7593aaf56b606bc48cdbd90d1b243747f1fbd2ac16040f6c313aad09b8ffb673ba0750200362d6c328098803d6e1e9947466ba3238455381dca2e8cc70e831d0813b6d382988a6c27226a91be6cf700d5869edc91fda7ac2015da913b3e87b4932b1e1b87d9667c28e7250aa0ed60b3a31095f541e1641488ac20594e78c0a2968210d9c1550d4ad31b03d5e4b9659cf2f67842483bb3c2bb7811ba20b59e575cef873ea95273afd55956c84590507200d410e693e4b079a426cc6102ba20e2d226cfdaec93903c3f3b81a01a81b19137627cb26e621a0afb7bcd6efbcfffba20f0f3d9beaf7a3945bcaa147e041ae1d5ca029bde7e40d8251f0783d6ecbe8fb5ba53a221c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac000000000
