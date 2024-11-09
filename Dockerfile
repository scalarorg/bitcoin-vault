FROM rust:1.82-alpine3.20 as libbuilder
RUN apk add --no-cache git libc-dev
# Build bitcoin-vault lib
# Todo: select a specific version
WORKDIR /bitcoin-vault
COPY . .
RUN cargo build --release