#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

PORT=9232
docker run -d --rm --name myadmin \
  --add-host=host.docker.internal:host-gateway \
  -v ${DIR%/*}/conf/phpmyadmin.config.php:/etc/phpmyadmin/config.user.inc.php \
  -v ./mnt/cache/phpmyadmin/session/:/sessions/ \
  -p $PORT:80 phpmyadmin/phpmyadmin &&
  sleep 1 && open http://localhost:$PORT
