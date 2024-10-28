#!/bin/bash
bitcoin() {
    NAME=${1:-bitcoin-regtest}
    docker run --rm -d \
        --name ${NAME} \
        -v $(pwd)/.bitcoin:/root/.bitcoin \
        -v $(pwd)/bitcoin.conf:/root/.bitcoin/bitcoin.conf \
        -v $(pwd)/bitcoin.sh:/root/bitcoin.sh \
        -e DATADIR=/root/.bitcoin \
        -u root \
        -w /root/.bitcoin \
        --entrypoint /bin/sh \
        lncm/bitcoind:v25.0 /root/bitcoin.sh entrypoint
}
build() {
    bun run --cwd ./wasm build
}
test() {
    bun test --cwd ./binding
}
build_test() {
    build run --cwd ./wasm build
    bun test --cwd ./binding
}
$@
