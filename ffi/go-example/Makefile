LIBRARY_PATH := $(shell pwd)/../../target/release
ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: build
build: copy
	go build -o bin/main main.go

.PHONY: copy
copy:
	mkdir -p lib
	cp $(LIBRARY_PATH)/libbitcoin_vault_ffi.* ./lib

.PHONY: run
run: copy
	go run main.go

.PHONY: run-binary
run-binary: build
	DYLD_FALLBACK_LIBRARY_PATH=$(ROOT_DIR)lib ./bin/main

.PHONY: debug
debug:
	@echo "Library path: $(LIBRARY_PATH)"
	@ls -l $(LIBRARY_PATH)/libbitcoin_vault_ffi.*`