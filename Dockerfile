FROM rust:1.82-alpine3.20
RUN apk add --no-cache git libc-dev
# Build bitcoin-vault lib
# Todo: select a specific version
WORKDIR /bitcoin-vault

COPY ffi ./ffi
COPY macros ./macros
COPY vault ./vault
COPY wasm ./wasm
COPY tools ./tools
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release
