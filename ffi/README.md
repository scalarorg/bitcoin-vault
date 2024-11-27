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
