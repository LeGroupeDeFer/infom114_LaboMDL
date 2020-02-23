#!/usr/bin/env sh

set -eux

cd /usr/src/app

# Cargo reloader
printf "#!/usr/bin/env sh\ncd /usr/src/app\nkill -9 \$(cat .backend.pid)\ncargo run > backend.log 2>&1 &\necho \$! > .backend.pid\n" > /usr/local/bin/reload
chmod +x /usr/local/bin/reload

# Rust
cargo install diesel_cli --no-default-features --features mysql
cargo build
diesel migration run
cargo run > backend.log 2>&1 &
echo $! > .backend.pid

# JS
npm ci
npm run watch > frontend.log 2>&1 &
echo $! > .frontend.pid

cleanup() {
    kill -9 $(cat .backend.pid)
    kill -9 $(cat .frontend.pid)
    rm .backend.pid .frontend.pid
    exit 0
}

trap cleanup SIGINT
trap cleanup SIGQUIT
trap cleanup SIGTERM

while true; do
    sleep 10
done
