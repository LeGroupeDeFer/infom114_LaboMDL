#!/usr/bin/env sh

docker-compose down
docker rm $(docker ps -aq)
docker image prune -a
docker volume rm $(docker volume ls -q)
sudo rm -rf target/

docker-compose build
docker-compose up -d

