package encode_test

import (
	"encoding/hex"
	"fmt"
	"testing"

	"github.com/scalarorg/bitcoin-vault/go-utils/encode"
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

/*
CalculateUnstakingPayloadHash hashes the unstaking payload
It applies the dynamic abi encoding rules

- locking_script: [20]byte (address)

- amount: int64 (amount in satoshis)

- fee_opts: types.BTCFeeOpts (fee options)
*/
func TestCalculateUnstakingPayloadHash(
	t *testing.T,
) {
	lockingScript, _ := hex.DecodeString("1234567890123456789012345678901234567890123456789012345678901234567890")
	amount := uint64(7)
	feeOpts := types.FastestFee

	payload, hash, err := encode.CalculateUnstakingPayloadHash(lockingScript, amount, feeOpts)
	if err != nil {
		t.Fatalf("Error calculating unstaking payload hash: %v", err)
	}

	payloadString := hex.EncodeToString(payload)

	fmt.Println("payloadString: ", payloadString)

	if payloadString != "000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000070400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002312345678901234567890123456789012345678901234567890123456789012345678900000000000000000000000000000000000000000000000000000000000" {
		t.Fatalf("Payload does not match expected value")
	}

	t.Logf("Payload: %x", payload)
	t.Logf("Hash: %x", hash)
}
