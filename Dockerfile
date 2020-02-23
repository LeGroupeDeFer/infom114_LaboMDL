# Alpine is way (!) lighter than debian based distros
FROM alpine:latest


ENV DUMB_INIT_VERSION=1.2.2 \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUSTUP_URL="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-musl/rustup-init" \
    RUSTFLAGS='-C target-feature=-crt-static'

# dumb-init
RUN set -eux \
    && apk add --update --no-cache ca-certificates \
    && wget -O /usr/local/bin/dumb-init https://github.com/Yelp/dumb-init/releases/download/v${DUMB_INIT_VERSION}/dumb-init_${DUMB_INIT_VERSION}_amd64 \
    && chmod +x /usr/local/bin/dumb-init

# cargo && rust
RUN set -eux \
    && apk add --no-cache gcc musl-dev mariadb-dev \
    && wget "$RUSTUP_URL" \
    && chmod +x rustup-init \
    && ./rustup-init -y --no-modify-path --default-toolchain nightly \
    && rm rustup-init \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME

# npm
RUN set -eux \
    && apk add --update --no-cache npm

VOLUME /usr/src/app
VOLUME /usr/local/cargo/registry
EXPOSE 8000

ADD entrypoint.sh /
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/usr/local/bin/dumb-init", "--"]
CMD ["/entrypoint.sh"]

