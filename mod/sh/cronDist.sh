#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*/*}
set -ex

. $DIR/conf/cron/projectPrefix.sh

deployctl deploy \
  --save-config=false \
  --force --prod \
  --project=${PROJECT_PREFIX}$1 \
  --env-file=/tmp/srv_cron.env
