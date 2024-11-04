# bitcoin-vault

A rust lib for working with vault transaction

## Test

## Prerequisite:
- Add toolchain
```sh
rustup toolchain install stable --target wasm32-unknown-unknown --component clippy --component rustfmt
```

If Mac:
```sh 
brew install llvm
```

- Build wasm:

```
cd wasm && bun run build
```

- Link wasm:

```
cd wasm && bun run build && bun link
```

- Install dependencies:

```
cd binding && bun i
```

- Test:

```
bun test
```

## Troubleshooting:

- [Unable to build for wasm32-unknown-unknown on macOS with Apple Clang](https://github.com/briansmith/ring/issues/1824)
```sh
brew install llvm
```