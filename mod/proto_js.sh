#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cd gen/proto
# rsync -avz ../../ProtoScript/packages/protoscript/dist ./node_modules/@3-/protoscript/
bun x protoscript
# ./bundle.coffee
