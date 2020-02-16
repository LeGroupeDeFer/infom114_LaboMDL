#!/usr/bin/env bash

set -ex

cd /usr/src/app

cargo install diesel_cli --no-default-features --features mysql
cargo build
./wait-for-it.sh db:3306 -t 100 -q -- diesel migration run

cargo run
