package vault

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

type PublicKey [33]byte
