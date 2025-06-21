#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

dc="mise exec -- docker-compose"
$dc -p $(basename $(dirname $DIR)) down $@
