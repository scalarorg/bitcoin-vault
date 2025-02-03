.PHONY: build-docker

build-docker:
	docker build -t scalar/bitcoin-vault .
