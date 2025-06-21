#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

. sh/rust2proto.before.sh

$DIR_HPC/build.sh
$DIR_HPC/lib/gen.js

. sh/rust2proto.after.sh
