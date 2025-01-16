LIBRARY_PATH := $(shell pwd)/../../target/release
ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
LIB_DIR := $(ROOT_DIR)/lib

.PHONY: test copy clean debug test-linux

test-linux:
	@docker build -f docker/test.dockerfile -t bitcoin-vault-ffi $(ROOT_DIR)/../../
	@docker run --rm -it bitcoin-vault-ffi

# Usage: `make test TestSignPsbtAndCollectSigs`
# Or: `make test` to run all tests
test:
	test_file=$(t)
	@if [ -n "$(t)" ]; then \
		CGO_LDFLAGS="-L$(LIB_DIR) -lbitcoin_vault_ffi" CGO_CFLAGS="-I$(LIB_DIR)" go test ./tests/... -v -cover -count=1 -run "^$(t)$$"; \
	else \
		CGO_LDFLAGS="-L$(LIB_DIR) -lbitcoin_vault_ffi" CGO_CFLAGS="-I$(LIB_DIR)" go test ./tests/... -v -cover -count=1; \
	fi


copy:
	@mkdir -p $(LIB_DIR)
	@rm -f $(LIB_DIR)/*
	@cp $(LIBRARY_PATH)/libbitcoin_vault_ffi.* $(LIB_DIR)

clean:
	@rm -rf $(LIB_DIR)

debug:
	@otool -L bin/main
