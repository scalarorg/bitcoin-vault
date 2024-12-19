package types

func (f BTCFeeOpts) Bytes() [1]byte {
	return [1]byte{byte(f)}
}

func (f BTCFeeOpts) Size() int {
	return 1
}
