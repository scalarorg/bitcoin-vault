FROM rust:1.82-alpine3.20 AS libbuilder
RUN apk add --no-cache git libc-dev

WORKDIR /scalar/bitcoin-vault

COPY ffi ./ffi
COPY macros ./macros
COPY vault ./vault
COPY wasm ./wasm
COPY tools ./tools
COPY Cargo.toml .
COPY Cargo.lock .

RUN cd ffi && cargo build --release

FROM golang:1.23.3-alpine3.20 AS build

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

RUN CGO_LDFLAGS="-L/usr/lib -lbitcoin_vault_ffi" \
    CGO_CFLAGS="-I/usr/lib" \
    go build -o /usr/local/bin/test-alpine ./tests/cmd/main.go

# Copy the test binary to the alpine image
FROM alpine:3.20

RUN apk add --no-cache \
    libgcc \
    libstdc++

COPY --from=build /usr/local/bin/test-alpine /usr/local/bin/test-alpine
# COPY --from=build /usr/lib/libbitcoin_vault_ffi.* /usr/lib/

RUN chmod +x /usr/local/bin/test-alpine

CMD ["/usr/local/bin/test-alpine"]
