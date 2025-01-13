package chain

import (
	"github.com/scalarorg/bitcoin-vault/go-utils/btc"
	"github.com/scalarorg/bitcoin-vault/go-utils/evm"
	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

var chainRecords = map[types.ChainType]types.ChainRecordsType{
	types.ChainTypeBitcoin: btc.BtcChainsRecords(),
	types.ChainTypeEVM:     evm.EvmChainsRecords(),
	// ChainTypeSolana:  solana.SolanaChainsRecords(),
	// ChainTypeCosmos:  cosmos.CosmosChainsRecords(),
}

func GetChainRecords(chainType types.ChainType) types.ChainRecordsType {
	return chainRecords[chainType]
}

func GetDisplayedName(chainInfo ChainInfo) string {
	return chainRecords[chainInfo.ChainType].GetDisplayedName(chainInfo.ChainID)
}
