FROM rust:1.39-buster

COPY . .
RUN cargo build --release

RUN mkdir -p /build/
RUN cp -r ./target/release/server /build/

FROM debian:buster

COPY --from=build /build /

CMD ["/server"]
