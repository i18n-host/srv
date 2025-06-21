#!/usr/bin/env bash

if [ -n "$1" ]; then
  export PROJECT=$1
else
  echo "USAGE : $0 project_name"
  exit 1
fi

set -ex
cargo new --lib $1 # this will add lib to workspace
rm -rf $1

cp -R _tmpl $1

cd $1

rpl _tmpl $1

git add .
