#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

set -a
source ../conf/r.env
source ../conf/mariadb.env
set +a

echo -e "R_PASSWORD=${R_PASSWORD}\nR_PORT=${R_PORT}" >.env

docker compose -p $(basename $(dirname $DIR)) up $@
