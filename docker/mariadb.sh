#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -e

set -a
source ../conf/mariadb.env
set +a

set -x

mariadb --port=$MYSQL_PORT --user=$MYSQL_USER $MYSQL_DB
