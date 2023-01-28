FROM rust:buster

RUN apt update
RUN apt install -y libsasl2-dev libzstd-dev
RUN mkdir /app

COPY target/release/main /app/produce-clt
CMD ["/app/produce-clt"]
