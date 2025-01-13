package evm

import (
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

type chainType struct {
	*types.BaseChain
}

var (
	displayedNameByID map[uint64]string
)

func init() {
	displayedNameByID = make(map[uint64]string)

	for _, chain := range chains {
		displayedNameByID[chain.ID] = chain.DisplayedName
	}
}

// EvmChainRecords provides access to EVM chain configuration and parameters
type Records struct{}

var _ types.ChainRecordsType = &Records{}

var records = &Records{}

func EvmChainsRecords() *Records {
	return records
}

// GetDisplayedName returns the displayed name for a given chain ID.
// Returns empty string if the chain ID is not found.
func (b *Records) GetDisplayedName(chainID uint64) string {
	name, ok := displayedNameByID[chainID]
	if !ok {
		return ""
	}
	return name
}
