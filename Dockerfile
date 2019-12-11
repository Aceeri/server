FROM rust:1.39-buster AS build

COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release

RUN mkdir -p /build/
RUN cp -r ./target/release/server /build/

FROM debian:buster AS release

RUN apt-get update \
    apt-get install -y \
        curl \
        vim

COPY --from=build /build /
RUN touch /etc/server_config.yaml

CMD ["/server"]
