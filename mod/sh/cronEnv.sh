#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*/*}
cd $DIR
set -ex

li=$(ls conf/cron/*.env | sort)

bun x envexpand $li >/tmp/srv_cron.env
