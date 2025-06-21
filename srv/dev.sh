#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

. ../sh/pid.sh

set -ex

exec watchexec \
  --shell=none \
  --project-origin . \
  -w . \
  -w ../../mod \
  --exts rs,toml \
  -r \
  -- "../srv.sh"
