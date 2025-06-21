#!/usr/bin/env bash

cd gen

if ! command -v pbc &>/dev/null; then
  cargo install pbc
fi

pbc || (rm -rf pb && exit 1)
