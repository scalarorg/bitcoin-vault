FROM rust:1.82-alpine3.20
RUN apk add --no-cache git libc-dev openssl-dev pkgconfig
# Build bitcoin-vault lib
# Todo: select a specific version
WORKDIR /bitcoin-vault

COPY ffi ./ffi
COPY cmd ./cmd
COPY macros ./macros
COPY vault ./vault
COPY wasm ./wasm

COPY Cargo.toml .

RUN cargo build -p vault -p macros -p ffi --release
