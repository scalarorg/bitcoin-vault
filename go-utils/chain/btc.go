package chain

import "github.com/btcsuite/btcd/chaincfg"

var ChaincfgTestnet4ParamsName = "testnet4"

var BtcChainConfigValueInt = map[string]uint64{
	chaincfg.MainNetParams.Name:       0,
	chaincfg.TestNet3Params.Name:      1,
	chaincfg.SigNetParams.Name:        2,
	chaincfg.RegressionNetParams.Name: 3,
	ChaincfgTestnet4ParamsName:        4,
}

var BtcChainConfigValueParams = map[string]*chaincfg.Params{
	chaincfg.MainNetParams.Name:       &chaincfg.MainNetParams,
	chaincfg.TestNet3Params.Name:      &chaincfg.TestNet3Params,
	chaincfg.SigNetParams.Name:        &chaincfg.SigNetParams,
	chaincfg.RegressionNetParams.Name: &chaincfg.RegressionNetParams,
	ChaincfgTestnet4ParamsName:        &chaincfg.TestNet3Params,
}
