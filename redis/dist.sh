#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

mise exec -- ./init.coffee
cd r_
bun x mdt .
cargo build
touch Cargo.lock
cargo v patch -y
cargo publish --registry crates-io
