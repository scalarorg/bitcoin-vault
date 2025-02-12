.PHONY: build-docker

build-docker:
	docker build -t scalar/bitcoin-vault .
build-tools:
	cargo build --bin tvl_maker
