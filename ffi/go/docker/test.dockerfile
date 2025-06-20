# syntax=docker/dockerfile:experimental

FROM rust:1.82-alpine3.20 as libbuilder
RUN apk add --no-cache git libc-dev
# Build bitcoin-vault lib
# Todo: select a specific feature, eg ffi
# WORKDIR /bitcoin-vault

# WORKDIR /scalar
# RUN git clone https://github.com/scalarorg/bitcoin-vault.git

WORKDIR /scalar/bitcoin-vault

COPY ffi ./ffi
COPY macros ./macros
COPY vault ./vault
COPY wasm ./wasm
COPY tools ./tools
COPY Cargo.toml .
COPY Cargo.lock .

RUN cd ffi && cargo build --release

FROM golang:1.23.3-alpine3.20 as build

RUN apk add --no-cache --update \
    ca-certificates \
    git \
    make \
    build-base \
    linux-headers

# Copy the bitcoin-vault lib
COPY --from=libbuilder /scalar/bitcoin-vault/target/release/libbitcoin_vault_ffi.* /usr/lib/

WORKDIR /scalar/bitcoin-vault/

COPY . .

WORKDIR /scalar/bitcoin-vault/ffi/go-vault

RUN go mod download

ARG TEST_PATTERN=.
ENV TEST_PATTERN=${TEST_PATTERN}

CMD CGO_LDFLAGS="-L/usr/lib -lbitcoin_vault_ffi" \
    CGO_CFLAGS="-I/usr/lib" \
    go test ./tests/... -v -cover -count=1 -run "${TEST_PATTERN}"
