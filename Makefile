lint:
	cargo clippy

format:
	cargo fmt

build:
	cargo build

run-api:
	./target/debug/rates-api

run-exporter:
	./target/debug/rates-exporter

compose-build:
	DOCKER_BUILDKIT=1 docker compose build --progress=plain

compose-up:
	docker compose up -d

compose-down:
	docker compose down