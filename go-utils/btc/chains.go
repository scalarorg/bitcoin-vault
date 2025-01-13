package btc

import (
	"github.com/btcsuite/btcd/chaincfg"
	"github.com/scalarorg/bitcoin-vault/go-utils/chain"
)

var ChaincfgTestnet4ParamsName = "testnet4"

const (
	BTCChainIDMainnet uint64 = iota
	BTCChainIDTestnet
	BTCChainIDRegtest
	BTCChainIDSignet
	BTCChainIDTestnet4
)

// Chain represents a Bitcoin network configuration
type Chain struct {
	*chain.BaseChain
	Params *chaincfg.Params
}

// Define all chains in a single source of truth
var chains = []Chain{
	{
		BaseChain: &chain.BaseChain{
			ID:   BTCChainIDMainnet,
			Name: chaincfg.MainNetParams.Name,
		},
		Params: &chaincfg.MainNetParams,
	},
	{
		BaseChain: &chain.BaseChain{
			ID:   BTCChainIDTestnet,
			Name: chaincfg.TestNet3Params.Name,
		},
		Params: &chaincfg.TestNet3Params,
	},
	{
		BaseChain: &chain.BaseChain{
			ID:   BTCChainIDRegtest,
			Name: chaincfg.RegressionNetParams.Name,
		},
		Params: &chaincfg.RegressionNetParams,
	},
	{
		BaseChain: &chain.BaseChain{
			ID:   BTCChainIDSignet,
			Name: chaincfg.SigNetParams.Name,
		},
		Params: &chaincfg.SigNetParams,
	},
	{
		BaseChain: &chain.BaseChain{
			ID:   BTCChainIDTestnet4,
			Name: ChaincfgTestnet4ParamsName,
		},
		Params: &chaincfg.TestNet3Params, // TODO: Update this to TestNet4Params when btcd supports itƒ
	},
}

var (
	chainIDByName map[string]uint64
	chainNameByID map[uint64]string
	paramsByName  map[string]*chaincfg.Params
)

func init() {
	chainIDByName = make(map[string]uint64)
	chainNameByID = make(map[uint64]string)
	paramsByName = make(map[string]*chaincfg.Params)

	for _, chain := range chains {
		chainIDByName[chain.Name] = chain.ID
		chainNameByID[chain.ID] = chain.Name
		paramsByName[chain.Name] = chain.Params
	}
}

// BTCChainRecords provides access to Bitcoin chain configuration and parameters
type Records struct{}

var _ chain.ChainRecords = &Records{}

var records = &Records{}

func BtcChainsRecords() *Records {
	return records
}

// GetChainNameByID returns the chain name for a given chain ID.
// Returns empty string if the chain ID is not found.
func (b *Records) GetChainNameByID(chainID uint64) string {
	name, ok := chainNameByID[chainID]
	if !ok {
		return ""
	}
	return name
}

// GetChainIDByName returns the chain ID for a given chain name.
// Returns 0 if the chain name is not found.
func (b *Records) GetChainIDByName(chainName string) uint64 {
	id, ok := chainIDByName[chainName]
	if !ok {
		return 0
	}
	return id
}

// GetChainParams returns the chain parameters for a given chain ID.
// Returns nil if the chain ID is not found.
func (b *Records) GetChainParams(chainID uint64) *chaincfg.Params {
	name, ok := chainNameByID[chainID]
	if !ok {
		return nil
	}
	return paramsByName[name]
}

// GetChainParamsByName returns the chain parameters for a given chain name.
// Returns nil if the chain name is not found.
func (b *Records) GetChainParamsByName(chainName string) *chaincfg.Params {
	params, ok := paramsByName[chainName]
	if !ok {
		return nil
	}
	return params
}
