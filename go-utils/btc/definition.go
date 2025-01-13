package btc

import (
	"github.com/btcsuite/btcd/chaincfg"
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

// Define all chains in a single source of truth
var chains = []chainType{
	{
		BaseChain: &types.BaseChain{
			ID:            0,
			DisplayedName: "Bitcoin",
		},
		Params:     &chaincfg.MainNetParams,
		ParamsName: chaincfg.MainNetParams.Name,
	},
	{
		BaseChain: &types.BaseChain{
			ID:            1,
			DisplayedName: "Bitcoin Testnet",
		},
		Params:     &chaincfg.TestNet3Params,
		ParamsName: chaincfg.TestNet3Params.Name,
	},
	{
		BaseChain: &types.BaseChain{
			ID:            2,
			DisplayedName: "Bitcoin Regression",
		},
		Params:     &chaincfg.RegressionNetParams,
		ParamsName: chaincfg.RegressionNetParams.Name,
	},
	{
		BaseChain: &types.BaseChain{
			ID:            3,
			DisplayedName: "Bitcoin Signet",
		},
		Params:     &chaincfg.SigNetParams,
		ParamsName: chaincfg.SigNetParams.Name,
	},
	{
		BaseChain: &types.BaseChain{
			ID:            4,
			DisplayedName: chaincfgTestnet4ParamsName,
		},
		Params:     &chaincfg.TestNet3Params, // TODO: Update this to TestNet4Params when btcd supports it
		ParamsName: chaincfgTestnet4ParamsName,
	},
}
