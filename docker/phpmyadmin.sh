#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

PORT=9232
docker run -d --name my-phpmyadmin -v "$(dirname DIR)/conf/phpmyadmin.config.php:/etc/phpmyadmin/config.user.inc.php" -p $PORT:80 phpmyadmin/phpmyadmin &&
  sleep 1 && open http://localhost:${PORT}
