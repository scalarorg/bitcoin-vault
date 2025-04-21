module github.com/scalarorg/bitcoin-vault/ffi/go-vault

go 1.23

require (
	github.com/scalarorg/bitcoin-vault/go-utils v0.0.0-20250122183134-1966c1835a07
	github.com/stretchr/testify v1.10.0
	golang.org/x/crypto v0.31.0
)

require (
	github.com/davecgh/go-spew v1.1.1 // indirect
	github.com/gogo/protobuf v1.3.2 // indirect
	github.com/pmezard/go-difflib v1.0.0 // indirect
	golang.org/x/sys v0.28.0 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
)

replace github.com/scalarorg/bitcoin-vault/go-utils => ../../go-utils
