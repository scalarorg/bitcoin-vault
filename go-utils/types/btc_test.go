package types_test

import (
	fmt "fmt"
	"testing"

	"github.com/scalarorg/bitcoin-vault/go-utils/types"
)

func TestBTCFeeOpts(t *testing.T) {
	fmt.Println(types.FastestFee.Bytes())
}
