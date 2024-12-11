# Bitcoin Vault FFI

This directory contains the Foreign Function Interface (FFI) implementation for the Bitcoin Vault library, providing language bindings for non-Rust applications.

## Structure

ffi/
├── src/ # Rust FFI implementation
├── go-vault/ # Go language bindings
└── Cargo.toml # Rust package configuration

## Features

- Parse vault embedded data from Bitcoin script pubkeys
- Sign PSBTs (Partially Signed Bitcoin Transactions) with single keys
- Cross-platform support (Linux, macOS)

## Go Bindings

The Go bindings provide a high-level interface to the Rust FFI library. To use the Go bindings:

1. Build the Rust library:

```bash
cargo build --release
```

2. Copy the library files to the appropriate location:

```bash
cd go-vault
make copy
```

3. Run the tests:

```bash
make test
```

For Linux environments, you can run tests in Docker:

```bash
make test-docker
```

## How to add new functions

1. Add the module to the `src/lib.rs` file
2. Add the type to the `src/types.rs` file
3. Add the memory management functions to the `src/memory.rs` file
4. Add the function to the `src/ffi.rs` file
5. Add the ffi function to `go-vault`
6. Build the library and copy the files to the `go-vault` directory
7. Add tests to the `go-vault` directory
