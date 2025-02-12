.PHONY: build-docker

build-docker:
	docker build -t scalarorg/bitcoin-vault .
build-tools:
	cargo build --bin tvl_maker --release
