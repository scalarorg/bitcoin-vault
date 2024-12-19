package encode

import (
	"github.com/scalarorg/bitcoin-vault/go-utils/crypto"
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

/*
CalculateUnstakingPayloadHash hashes the unstaking payload
It applies the dynamic abi encoding rules

- locking_script: [20]byte (address)

- amount: int64 (amount in satoshis)

- fee_opts: types.BTCFeeOpts (fee options)
*/
func CalculateUnstakingPayloadHash(
	lockingScript []byte,
	amount uint64,
	feeOpts types.BTCFeeOpts,
) ([]byte, []byte, error) {

	encodedPayload, err := unstakingPayload.Pack(lockingScript, amount, feeOpts.Bytes())
	if err != nil {
		return nil, nil, err
	}

	// Calculate hash
	hash := crypto.Keccak256(encodedPayload)
	return encodedPayload, hash, nil
}
