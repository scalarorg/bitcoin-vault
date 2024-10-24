# generate bindings

set -e

which wasm-pack || cargo install wasm-pack

# pack for bundler
wasm-pack build --release --target=bundler --out-name=bitcoin-vault-bundler --out-dir=dist

# pack for browser
wasm-pack build --release --target=web --out-name=bitcoin-vault-web --out-dir=dist

# pack for node.js
wasm-pack build --release --target=nodejs --out-name=bitcoin-vault-node --out-dir=dist

rm dist/package.json
