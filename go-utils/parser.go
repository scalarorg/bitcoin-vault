package goutils

import "fmt"

// Please follow the immplementation of rust lib at: https://github.com/scalarorg/bitcoin-vault/blob/main/vault/src/types/transaction.rs#L37

// Constants for field sizes

const (
	TagHashSize           = 6 // adjust size as needed
	ServiceTagHashSize    = 5 // adjust size as needed
	VersionSize           = 1
	NetworkIdSize         = 1
	HaveOnlyCovenantsSize = 1
	CovenantQuorumSize    = 1
	ChainIdSize           = 8  // adjust size as needed
	AddressSize           = 20 // adjust size as needed
)

const EMBEDDED_DATA_SCRIPT_SIZE = TagHashSize + ServiceTagHashSize + VersionSize + NetworkIdSize + HaveOnlyCovenantsSize + CovenantQuorumSize + ChainIdSize + AddressSize + AddressSize

const SCRIPT_SIZE = 1 + 1 + EMBEDDED_DATA_SCRIPT_SIZE // OP_RETURN OP_PUSHBYTES + embedded data size

// ParserError represents various parsing errors
type ParserError struct {
	msg string
}

func (e *ParserError) Error() string {
	return e.msg
}

// VaultReturnTxOutput represents the parsed vault return transaction output
type VaultReturnTxOutput struct {
	Tag                         []byte `json:"tag"`
	ServiceTag                  []byte `json:"service_tag"`
	Version                     uint8  `json:"version"`
	NetworkID                   uint8  `json:"network_id"`
	HaveOnlyCovenants           bool   `json:"have_only_covenants"`
	CovenantQuorum              uint8  `json:"covenant_quorum"`
	DestinationChainID          []byte `json:"destination_chain_id"`
	DestinationContractAddress  []byte `json:"destination_contract_address"`
	DestinationRecipientAddress []byte `json:"destination_recipient_address"`
}

// ParseVaultEmbeddedData parses the script pubkey and returns the vault return transaction output
func ParseVaultEmbeddedData(scriptPubkey []byte) (*VaultReturnTxOutput, error) {

	if len(scriptPubkey) != SCRIPT_SIZE {
		fmt.Println("Invalid script pubkey size", len(scriptPubkey), SCRIPT_SIZE)
		return nil, &ParserError{msg: "Invalid script pubkey size"}
	}

	if scriptPubkey[0] != 0x6a { // OP_RETURN
		return nil, &ParserError{msg: "Invalid script pubkey"}
	}

	data := scriptPubkey[2:]

	cursor := 0

	tag := data[cursor : cursor+TagHashSize]
	cursor += TagHashSize

	serviceTag := data[cursor : cursor+ServiceTagHashSize]
	cursor += ServiceTagHashSize

	version := data[cursor]
	cursor += 1

	networkID := data[cursor]
	cursor += 1

	haveOnlyCovenants := data[cursor] == 1
	cursor += 1

	covenantQuorum := data[cursor]
	cursor += 1

	destinationChainID := data[cursor : cursor+ChainIdSize]
	cursor += ChainIdSize

	destinationContractAddress := data[cursor : cursor+AddressSize]
	cursor += AddressSize

	destinationRecipientAddress := data[cursor : cursor+AddressSize]
	cursor += AddressSize

	return &VaultReturnTxOutput{
		Tag:                         tag,
		ServiceTag:                  serviceTag,
		Version:                     version,
		NetworkID:                   networkID,
		HaveOnlyCovenants:           haveOnlyCovenants,
		CovenantQuorum:              covenantQuorum,
		DestinationChainID:          destinationChainID,
		DestinationContractAddress:  destinationContractAddress,
		DestinationRecipientAddress: destinationRecipientAddress,
	}, nil
}
