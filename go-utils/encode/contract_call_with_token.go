package encode

import (
	"fmt"

	"github.com/scalarorg/bitcoin-vault/go-utils/crypto"
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

type CustodianOnly struct {
	FeeOptions               types.BTCFeeOpts
	RBF                      bool
	RecipientChainIdentifier []byte
}

type UPC struct {
	Psbt []byte
}

type ContractCallWithTokenPayload struct {
	PayloadType ContractCallWithTokenPayloadType
	*CustodianOnly
	*UPC
}

func CalculateContractCallWithTokenPayload(
	payloadArgs ContractCallWithTokenPayload,
) ([]byte, []byte, error) {
	var encodedPayload []byte
	var err error

	switch payloadArgs.PayloadType {
	case ContractCallWithTokenPayloadType_CustodianOnly:
		if payloadArgs.RecipientChainIdentifier == nil {
			return nil, nil, fmt.Errorf("recipient chain identifier is required")
		}
		encodedPayload, err = contractCallWithTokenCustodianOnly.Pack(uint8(payloadArgs.FeeOptions), payloadArgs.RBF, payloadArgs.RecipientChainIdentifier)
	case ContractCallWithTokenPayloadType_UPC:
		if payloadArgs.Psbt == nil {
			return nil, nil, fmt.Errorf("psbt is required")
		}
		encodedPayload, err = contractCallWithTokenUPC.Pack(payloadArgs.Psbt)
	default:
		return nil, nil, fmt.Errorf("invalid payload type")
	}

	if err != nil {
		return nil, nil, err
	}

	finalPayload := AppendPayload(payloadArgs.PayloadType, encodedPayload)
	hash := crypto.Keccak256(finalPayload)
	return finalPayload, hash, nil
}

func DecodeContractCallWithTokenPayload(payload []byte) (*ContractCallWithTokenPayload, error) {
	payloadType, err := FromBytes(payload[:1])
	if err != nil {
		return nil, err
	}
	encodedPayload := payload[1:]
	switch payloadType {
	case ContractCallWithTokenPayloadType_CustodianOnly:
		return DecodeCustodianOnly(encodedPayload)
	case ContractCallWithTokenPayloadType_UPC:
		return DecodeUPC(encodedPayload)
	default:
		return nil, fmt.Errorf("invalid payload type")
	}
}

func DecodeCustodianOnly(payload []byte) (*ContractCallWithTokenPayload, error) {
	decoded, err := contractCallWithTokenCustodianOnly.Unpack(payload)
	if err != nil {
		return nil, err
	}

	feeOptions := types.BTCFeeOpts(decoded[0].(uint8))
	rbf := decoded[1].(bool)
	recipientChainIdentifier := decoded[2].([]byte)

	return &ContractCallWithTokenPayload{
		PayloadType: ContractCallWithTokenPayloadType_CustodianOnly,
		CustodianOnly: &CustodianOnly{
			FeeOptions:               feeOptions,
			RBF:                      rbf,
			RecipientChainIdentifier: recipientChainIdentifier,
		},
	}, nil
}

func DecodeUPC(payload []byte) (*ContractCallWithTokenPayload, error) {
	decoded, err := contractCallWithTokenUPC.Unpack(payload)
	if err != nil {
		return nil, err
	}
	psbt := decoded[0].([]byte)
	return &ContractCallWithTokenPayload{
		PayloadType: ContractCallWithTokenPayloadType_UPC,
		UPC:         &UPC{Psbt: psbt},
	}, nil
}

func AppendPayload(payloadType ContractCallWithTokenPayloadType, encodedPayload []byte) []byte {
	var finalPayload []byte
	finalPayload = append(finalPayload, payloadType.Bytes()...)
	finalPayload = append(finalPayload, encodedPayload...)
	return finalPayload
}
