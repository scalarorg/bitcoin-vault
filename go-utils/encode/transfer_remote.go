package encode

import (
	"github.com/ethereum/go-ethereum/common"
	"github.com/scalarorg/bitcoin-vault/go-utils/crypto"
)

func CalculateTransferRemoteMetadataPayloadHash(
	amount uint64,
	recipientChainIdentifier []byte,
	metadata []byte,
) ([]byte, []byte, error) {

	encodedPayload, err := transferRemoteMetadata.Pack(amount, recipientChainIdentifier, metadata)
	if err != nil {
		return nil, nil, err
	}

	// Calculate hash
	hash := crypto.Keccak256(encodedPayload)
	return encodedPayload, hash, nil
}

// ```solidity
// encoded_payload = abi.encode(
//     ['address', 'address', 'symbol', 'bytes'],
//     [sender_address, source_contract_address, symbol, encoded_metadata]
// );
// ```

func CalculateTransferRemotePayloadHash(
	address common.Address,
	sourceContractAddress common.Address,
	symbol string,
	metadata []byte,
) ([]byte, []byte, error) {

	encodedPayload, err := transferRemotePayload.Pack(address, sourceContractAddress, symbol, metadata)
	if err != nil {
		return nil, nil, err
	}

	hash := crypto.Keccak256(encodedPayload)
	return encodedPayload, hash, nil
}

func DecodeTransferRemotePayload(payload []byte) (sender common.Address, sourceContract common.Address, symbol string, metadata []byte, err error) {
	values, err := transferRemotePayload.Unpack(payload)
	if err != nil {
		return common.Address{}, common.Address{}, "", nil, err
	}
	return values[0].(common.Address), values[1].(common.Address), values[2].(string), values[3].([]byte), nil
}

func DecodeTransferRemoteMetadataPayload(payload []byte) (amount uint64, recipientChainIdentifier []byte, metadata []byte, err error) {
	values, err := transferRemoteMetadata.Unpack(payload)
	if err != nil {
		return 0, nil, nil, err
	}
	return values[0].(uint64), values[1].([]byte), values[2].([]byte), nil
}
