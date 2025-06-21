#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
. sh/pid.sh

set -ex

name=$(basename $DIR)

onexit() {
  docker compose -p $name down
}

trap onexit EXIT
set -a
. env.sh
set +a

./docker/up.sh -d

exec mise exec -- bun x concurrently -r --names "kv,srv" \
  "docker-compose -p $name logs -f" \
  "./sh/wait-for-it.sh $R_NODE && ./srv/dev.sh"
