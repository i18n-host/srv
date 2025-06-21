#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

set -ex
sed -i 's/includedir \/etc\/mysql\/conf.d\//includedir \/etc\/mysql\/conf\//g' /etc/mysql/mariadb.cnf

if [ -z "${MYSQL_DATABASE}" ]; then
  echo "错误: 环境变量 MYSQL_DATABASE 未设置" >&2
  exit 1
fi

if [ -z "${MYSQL_USER}" ]; then
  echo "错误: 环境变量 MYSQL_USER 未设置" >&2
  exit 1
fi

if [ -z "${MYSQL_PASSWORD}" ]; then
  echo "错误: 环境变量 MYSQL_PASSWORD 未设置" >&2
  exit 1
fi

cat >init.sql <<EOF
CREATE DATABASE \`${MYSQL_DATABASE}\` CHARACTER SET binary;
CREATE USER '${MYSQL_USER}'@'%' IDENTIFIED BY '${MYSQL_PASSWORD}';
GRANT ALL PRIVILEGES ON \`${MYSQL_DATABASE}\`.* TO '${MYSQL_USER}'@'%';
EOF
