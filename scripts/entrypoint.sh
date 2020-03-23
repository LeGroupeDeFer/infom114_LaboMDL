#!/usr/bin/env sh

set -eu

cd /usr/src/app

# Migrations
echo "Starting migrations..." >&2
cargo install diesel_cli --no-default-features --features mysql
if [ -f .redo ]; then
    diesel migration redo
else
    diesel migration run
fi
echo "OK." >&2

# Rust
echo "Starting Rust server..." >&2
cargo build
cargo run > backend.log 2>&1 &
echo $! > .backend.pid
echo "OK." >&2

# JS
echo "Starting webpack..." >&2
npm ci
npm run watch > frontend.log 2>&1 &
echo $! > .frontend.pid
echo "OK." >&2

# Kill the servers on exit

cleanup() {
    kill -9 $(cat .backend.pid)
    kill -9 $(cat .frontend.pid)
    rm .backend.pid .frontend.pid
    exit 0
}

trap cleanup SIGINT
trap cleanup SIGQUIT
trap cleanup SIGTERM

# Backend reloading
echo "Starting reloader..." >&2
inotifywait -m -r -e create -e modify -e move -e delete --format "%w%f %e" src | while read FILE EVENT; do
    echo "Change (${EVENT}) to ${FILE} detected, reloading..." >&2
    reload BACKEND
    echo "OK." >&2
done
