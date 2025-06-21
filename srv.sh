#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

set -e
. env.sh
set -x

cargo run -p hpc_srv
