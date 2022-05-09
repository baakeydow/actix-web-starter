#!/usr/bin/env bash

ROOT_DIR="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"

# delete old docker image if any
docker rmi -f actix_web_starter:latest &>/dev/null || true

# stop old container if any
docker stop running_core_api &>/dev/null || true

# build fresh image and run new container in detached mode
docker build --no-cache --pull -t actix_web_starter:latest . && \
  docker run --rm --name running_core_api -d \
  -p 1342:1342 \
  -v "$ROOT_DIR/runtime:/root/workspace/runtime" actix_web_starter

# display logs
docker logs -f running_core_api
