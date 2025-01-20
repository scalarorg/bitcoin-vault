package types

import "encoding/json"

// signing

type PublicKey [33]byte

type TapScriptSig struct {
	KeyXOnly  [32]byte
	LeafHash  [32]byte
	Signature [64]byte
}

type NetworkKind uint8

const (
	NetworkKindMainnet NetworkKind = iota
	NetworkKindTestnet
)

func (n NetworkKind) Valid() bool {
	return n == NetworkKindMainnet || n == NetworkKindTestnet
}

type OutPoint struct {
	Txid [32]byte
	Vout uint32
}

type ScriptBuf = []byte

type PreviousStakingUTXO struct {
	OutPoint OutPoint
	Amount   uint64
	Script   ScriptBuf
}

type UnstakingOutput struct {
	LockingScript ScriptBuf
	Amount        uint64
}

// parsing

type TransactionType string

const (
	TransactionTypeUnstaking TransactionType = "Unstaking"
	TransactionTypeStaking   TransactionType = "Staking"
)

func (t TransactionType) String() string {
	return string(t)
}

func (t TransactionType) MarshalJSON() ([]byte, error) {
	return json.Marshal(string(t))
}

func (t *TransactionType) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}
	*t = TransactionType(s)
	return nil
}

// VaultReturnTxOutput represents the parsed vault return transaction output
type VaultReturnTxOutput struct {
	Tag                         []byte          `json:"tag"`
	Version                     uint8           `json:"version"`
	NetworkID                   uint8           `json:"network_id"`
	Flags                       uint8           `json:"flags"`
	ServiceTag                  []byte          `json:"service_tag"`
	TransactionType             TransactionType `json:"transaction_type"`
	CustodianQuorum             uint8           `json:"custodian_quorum"`
	DestinationChain            []byte          `json:"destination_chain"`
	DestinationTokenAddress     []byte          `json:"destination_token_address"`
	DestinationRecipientAddress []byte          `json:"destination_recipient_address"`
}
