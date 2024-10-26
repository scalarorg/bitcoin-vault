#!/bin/bash
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
