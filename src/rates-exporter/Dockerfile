# build image
FROM rust:1.73.0-slim-buster as build
WORKDIR /app
COPY . .

# caching application build deps
RUN --mount=type=cache,target=/app/target cargo build --release && cp target/release/rates-exporter /rates-exporter

# acl image
FROM debian:buster-slim as rates-exporter
COPY --from=build /rates-exporter /rates-exporter
CMD ["/rates-exporter"]