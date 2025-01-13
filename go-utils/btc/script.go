package btc

import (
	"fmt"

	"github.com/btcsuite/btcd/btcutil"
	"github.com/btcsuite/btcd/txscript"
)

func ScriptPubKeyToAddress(scriptPubKey []byte, paramsName string) (btcutil.Address, error) {

	chainParams := BtcChainsRecords().GetChainParamsByName(paramsName)
	if chainParams == nil {
		return nil, fmt.Errorf("chain params not found")
	}

	// Extract the type of script
	_, addresses, _, err := txscript.ExtractPkScriptAddrs(scriptPubKey, chainParams)
	if err != nil {
		return nil, err
	}

	if len(addresses) == 0 {
		return nil, fmt.Errorf("no addresses found")
	}

	return addresses[0], nil
}
