LIBRARY_PATH := $(shell pwd)/../../target/release
ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
LIB_DIR := $(ROOT_DIR)/lib
$(info ROOT_DIR: $(ROOT_DIR))

.PHONY: test copy clean debug test-linux

test-linux:
	@docker build --build-arg TEST_PATTERN=$(t) -f docker/test.dockerfile -t bitcoin-vault-ffi ../../
	@docker run --rm -it bitcoin-vault-ffi

.PHONY: test-alpine
test-alpine:
	@docker build -f docker/test.alpine.dockerfile -t bitcoin-vault-ffi-alpine ../../
	@docker run --rm -it bitcoin-vault-ffi-alpine

# Usage: `make test TestSignPsbtAndCollectSigs`
# Or: `make test` to run all tests
test:
	test_file=$(t)
	@if [ -n "$(t)" ]; then \
		CGO_LDFLAGS="-L$(LIB_DIR) -lbitcoin_vault_ffi" CGO_CFLAGS="-I$(LIB_DIR)" go test ./tests/... -v -cover -count=1 -run "^$(t)$$"; \
	else \
		CGO_LDFLAGS="-L$(LIB_DIR) -lbitcoin_vault_ffi" CGO_CFLAGS="-I$(LIB_DIR)" go test ./tests/... -v -cover -count=1 -run "."; \
	fi


copy:
	@mkdir -p $(LIB_DIR)
	@rm -f $(LIB_DIR)/*
	@cp $(LIBRARY_PATH)/libbitcoin_vault_ffi.* $(LIB_DIR)

clean:
	@rm -rf $(LIB_DIR)

debug:
	@otool -L bin/main
