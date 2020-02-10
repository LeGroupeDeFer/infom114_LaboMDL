FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features mysql

ADD . /usr/src/app

WORKDIR /usr/src/app

EXPOSE 8000

RUN cargo build

VOLUME ["/usr/local/cargo"]
