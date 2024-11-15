package goutils_test

import (
	"encoding/hex"
	"testing"

	goutils "github.com/scalarorg/bitcoin-vault/go-utils"
)

func TestParseVaultEmbeddedData(t *testing.T) {

	scriptHex := "6a3f5343414c41526c69676874000100030000000000aa36a7b91e3a8ef862567026d6f376c9f3d6b814ca433724a1db57fa3ecafcbad91d6ef068439aceeae090"

	scriptBytes, err := hex.DecodeString(scriptHex)
	if err != nil {
		t.Fatalf("Failed to decode hex: %v", err)
	}

	data, err := goutils.ParseVaultEmbeddedData(scriptBytes)
	if err != nil {
		t.Fatalf("Failed to parse vault embedded data: %v", err)
	}

	t.Logf("%+v", data)

	// &{Tag:[83 67 65 76 65 82] ServiceTag:[108 105 103 104 116] Version:0 NetworkID:1 HaveOnlyCovenants:false CovenantQuorum:3 DestinationChainID:[0 0 0 0 0 170 54 167] DestinationContractAddress:[185 30 58 142 248 98 86 112 38 214 243 118 201 243 214 184 20 202 67 55] DestinationRecipientAddress:[36 161 219 87 250 62 202 252 186 217 29 110 240 104 67 154 206 234 224 144]}
}
