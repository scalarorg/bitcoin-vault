#!/bin/sh

test() {
    # Validate cargo is installed
    if ! command -v cargo >/dev/null 2>&1; then
        echo "Error: cargo is not installed" >&2
        exit 1
    fi

    test_file=${1:-'*'}

    echo "Running tests for: ${test_file}"

    if [ -z "$2" ]; then
        cargo test --package bitcoin-vault --test "$test_file" -- --exact --show-output || {
            echo "Tests failed with exit code $?" >&2
            exit 1
        }
    else
        cargo test --package bitcoin-vault --test "$test_file" -- "$test_file::$2" --exact --show-output || {
            echo "Tests failed with exit code $?" >&2
            exit 1
        }
    fi
}



test "$@"
