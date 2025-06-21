#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

. sh/rust2proto.before.sh
bun x rust2proto
. sh/rust2proto.after.sh
./proto_js.sh
