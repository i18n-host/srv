#!/usr/bin/env bash

DIR="$(dirname $(realpath ${BASH_SOURCE[0]}))"

. $DIR/sh/rustflag.sh

set -a
RUST_LOG=debug,html5ever=warn,supervisor=warn,hyper=warn,rustls=warn,h2=warn,tower=warn,h3=warn,quinn_udp=warn,quinn_proto=warn,watchexec=warn,globset=warn,hickory_proto=warn,hickory_resolver=warn,aws_smithy_runtime=warn,aws_sdk_s3=warn,process_wrap=warn,tokio_postgres=warn,swc_ecma_transforms_base=warn,swc_timer=warn,swc_ecma_minifier=warn,swc_ecma_transforms_optimization=warn,fjall=warn,lsm_tree=warn,reqwest=warn,grep_regex=warn,cargo_machete=warn,fred=warn,ignored=warn

RUST_BACKTRACE=short

NODE_TLS_REJECT_UNAUTHORIZED=0
set +a

env_sh() {
  local nowdir=$(pwd)
  cd $DIR/conf
  local i
  for i in $@; do
    set -a
    . "$i.env"
    set +a
  done

  cd $nowdir
  unset -f env_sh
}

env_sh r mariadb
#env_sh host kv gt redis mq pg mail clip nchan api qdrant
