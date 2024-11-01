LIBRARY_PATH := $(shell pwd)/../../target/release
ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: test-ubuntu test copy clean debug build-linux

test-linux:
	@docker build -f docker/test_ffi.dockerfile -t bitcoin-vault-ffi $(ROOT_DIR)/../../
	@docker run --rm -it bitcoin-vault-ffi
test: copy
	@CGO_LDFLAGS="-L./lib/darwin -lbitcoin_vault_ffi" CGO_CFLAGS="-I./lib/darwin" go test ./tests/... -v -cover -count=1

build-linux:
	@mkdir -p $(ROOT_DIR)/lib/linux
	@docker build \
		-f docker/build_linux.dockerfile \
		-t bitcoin-vault-ffi \
		$(ROOT_DIR)/../../
	@docker create --name temp-container bitcoin-vault-ffi
	@docker cp temp-container:/app/ffi/go/lib/linux/. $(ROOT_DIR)/lib/linux/
	@docker rm temp-container

copy:
	@mkdir -p lib/darwin
	@cp $(LIBRARY_PATH)/libbitcoin_vault_ffi.* ./lib/darwin

clean:
	@rm -rf lib

debug:
	@otool -L bin/main
