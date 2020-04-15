# Alpine is way (!) lighter than debian based distros
FROM alpine:latest


ENV DUMB_INIT_VERSION=1.2.2 \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUSTUP_URL="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-musl/rustup-init" \
    RUSTFLAGS='-C target-feature=-crt-static'

# Dependencies
RUN apk add --update --no-cache ca-certificates gcc musl-dev mariadb-dev npm inotify-tools

# dumb-init
RUN set -eux \
    && wget -O /usr/local/bin/dumb-init https://github.com/Yelp/dumb-init/releases/download/v${DUMB_INIT_VERSION}/dumb-init_${DUMB_INIT_VERSION}_amd64 \
    && chmod +x /usr/local/bin/dumb-init

# cargo && rust
RUN set -eux \
    && wget "$RUSTUP_URL" \
    && chmod +x rustup-init \
    && ./rustup-init -y --no-modify-path --default-toolchain nightly \
    && rm rustup-init \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME

# tz
RUN set -eux \
 && apk add --update --no-cache tzdata \
 && cp /usr/share/zoneinfo/Europe/Brussels /etc/localtime \
 && echo "Europe/Brussels" > /etc/timezone \
 && apk del tzdata

VOLUME /usr/src/app
VOLUME /usr/local/cargo/registry
EXPOSE 8000

ADD scripts/entrypoint.sh /
ADD scripts/reload.sh /usr/local/bin/reload
RUN chmod +x /entrypoint.sh /usr/local/bin/reload

ENTRYPOINT ["/usr/local/bin/dumb-init", "--"]
CMD ["/entrypoint.sh"]
