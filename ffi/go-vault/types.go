package vault

type NetworkKind uint8

const (
	NetworkKindMainnet NetworkKind = iota
	NetworkKindTestnet
)

func (n NetworkKind) Valid() bool {
	return n == NetworkKindMainnet || n == NetworkKindTestnet
}
