package encode

import (
	"github.com/scalarorg/bitcoin-vault/go-utils/crypto"
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

func CalculateContractCallWithTokenPayload(
	feeOptions types.BTCFeeOpts,
	rbf bool,
	recipientChainIdentifier []byte,
) ([]byte, []byte, error) {

	encodedPayload, err := contractCallWithToken.Pack(uint8(feeOptions), rbf, recipientChainIdentifier)
	if err != nil {
		return nil, nil, err
	}

	// Calculate hash
	hash := crypto.Keccak256(encodedPayload)
	return encodedPayload, hash, nil
}

func DecodeContractCallWithTokenPayload(payload []byte) (feeOpts types.BTCFeeOpts, rbf bool, recipientChainIdentifier []byte, err error) {
	decoded, err := contractCallWithToken.Unpack(payload)
	if err != nil {
		return 0, false, nil, err
	}

	return types.BTCFeeOpts(decoded[0].(uint8)), decoded[1].(bool), decoded[2].([]byte), nil
}
