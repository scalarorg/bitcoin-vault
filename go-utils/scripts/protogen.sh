#!/usr/bin/env bash

set -eo pipefail

proto_dirs=$(find ./proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)
for dir in $proto_dirs; do
    # shellcheck disable=SC2046
    buf protoc \
        -I "proto" \
        --gocosmos_out=. \
        $(find "${dir}" -maxdepth 1 -name '*.proto')
done

# move proto files to the right places
cp -r github.com/scalarorg/bitcoin-vault/go-utils/* ./
rm -rf github.com
