FROM rustlang/rust:nightly
MAINTAINER marete kent<maretekent@gmail.com>

RUN mkdir -p /usr/src/cloudenvapp
WORKDIR /usr/src/cloudenvapp
COPY . /usr/src/cloudenvapp
COPY init-user-db.sh /docker-entrypoint-initdb.d/

RUN rustc --version
RUN cargo install
RUN cargo install diesel_cli
