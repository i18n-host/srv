#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*}
cd $DIR
set -ex
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=git

ncu -u && bun i

cargo update
cargo upgrade -i --recursive --verbose

cd mod/gen/js
ncu -u
bun i
