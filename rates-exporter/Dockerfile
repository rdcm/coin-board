# build image
FROM rust:1.73.0-slim-buster as build
WORKDIR /app
COPY . .

# installing deps
RUN apt-get update && apt-get install -y build-essential \
     openssl libssl-dev \
     pkg-config

# caching application build deps
RUN --mount=type=cache,target=/app/target cargo build --release && cp target/release/rates-exporter /rates-exporter


# acl image
FROM debian:buster-slim as rates-exporter

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates

COPY --from=build /rates-exporter /rates-exporter
CMD ["/rates-exporter"]