#!/usr/bin/env bash

testnet4() {
    export TEST_ENV="vault/.env.test.testnet4" && cargo run --package vault --bin collect_utxos
}

regtest() {
    export TEST_ENV="vault/.env.test.regtest" && cargo run --package vault --bin collect_utxos
}

"$@"
