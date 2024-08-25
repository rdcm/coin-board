# build image
FROM rust:1.79.0-slim-buster as build
WORKDIR /app
COPY . .

# installing deps
RUN apt-get update && apt-get install -y build-essential \
     openssl libssl-dev \
     pkg-config

# caching application build deps
RUN --mount=type=cache,target=/app/target cargo build --release && cp target/release/rates-api /rates-api


# app image
FROM debian:buster-slim as api

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates

COPY --from=build /rates-api /rates-api
CMD ["/rates-api"]