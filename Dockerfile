FROM rustlang/rust:nightly

ADD . /usr/src/app
ADD entrypoint.sh /
RUN chmod +x /entrypoint.sh

EXPOSE 8000
VOLUME ["/usr/src/app"]

ENTRYPOINT /entrypoint.sh
