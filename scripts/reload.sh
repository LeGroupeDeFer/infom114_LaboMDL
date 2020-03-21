#!/usr/bin/env sh

# ------------------------------------------------------------------- Functions

usage() {
    echo "$0: Start (or reload) the given development server." >&2
    echo "Usage:" >&2
    echo "    $0 [BACKEND|FRONTEND]" >&2
}

safe_kill() {
    PID=$1
    ps -o pid | grep "$PID" >/dev/null
    if [ ! $? -eq 0 ]; then
        echo "WARNING: Process was not shutdown properly" >&2
    else
        kill -9 $PID
    fi
}

# ------------------------------------------------------------------------ Main

SERVER=$1
cd /usr/src/app

if [ "$SERVER" == "BACKEND" ]; then

    if [ -f .backend.pid ]; then
        echo "Terminating backend..." >&2
        PID=$(cat .backend.pid)
        safe_kill $PID
        echo "OK." >&2
    fi

    echo "Starting backend..." >&2
    cargo run > backend.log 2>&1 &
    echo $! > .backend.pid

    echo "OK." >&2

elif [ "$SERVER" == "FRONTEND" ]; then

    if [ -f .backend.pid ]; then
        echo "Terminating frontend..." >&2
        PID=$(cat .frontend.pid)
        safe_kill $PID
        echo "OK." >&2
    fi

    echo "Starting frontend..." >&2
    npm run watch > frontend.log 2>&1 &
    echo $! > .frontend.pid
    echo "OK." >&2

else

    usage
    exit 1

fi
