LIBRARY_PATH := $(shell pwd)/../../target/release
ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: test copy clean debug test-linux

test-linux:
	@docker build -f docker/test.dockerfile -t bitcoin-vault-ffi $(ROOT_DIR)/../../
	@docker run --rm -it bitcoin-vault-ffi
	
test:
	@CGO_LDFLAGS="-L./lib/darwin -lbitcoin_vault_ffi" CGO_CFLAGS="-I./lib/darwin" go test ./tests/... -v -cover -count=1

copy:
	@mkdir -p lib/darwin
	@cp $(LIBRARY_PATH)/libbitcoin_vault_ffi.* ./lib/darwin

clean:
	@rm -rf lib

debug:
	@otool -L bin/main
