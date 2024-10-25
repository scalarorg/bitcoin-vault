#! /bin/bash
rustup toolchain install stable --target wasm32-unknown-unknown --component clippy --component rustfmt
rustup target add wasm32-unknown-unknown

# generate bindings

set -e
OUT_DIR=pkg
which wasm-pack || cargo install wasm-pack

# pack for bundler
wasm-pack build --release --target=bundler --out-name=bitcoin-vault-bundler --out-dir=${OUT_DIR}

# pack for browser
wasm-pack build --release --target=web --out-name=bitcoin-vault-web --out-dir=${OUT_DIR}

# pack for node.js
wasm-pack build --release --target=nodejs --out-name=bitcoin-vault-node --out-dir=${OUT_DIR}

rm ${OUT_DIR}/package.json ${OUT_DIR}/.gitignore
