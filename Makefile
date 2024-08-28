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

create-secrets:
	./generate_secret.sh $(ARGS) > github-registry.yaml

deploy-secrets:
	kubectl apply -f github-registry.yaml

up:
	helm upgrade --install --atomic --timeout 300s --wait coin-board helm

down:
	helm delete coin-board

deploy:
	helm upgrade --install --atomic --timeout 300s --wait coin-board helm \
      --set ui.backendUri="https://api.coin-board.io" \
      --set ui.ingress.host="coin-board.io" \
      --set api.cors.origin="https://coin-board.io" \
      --set api.ingress.host="api.coin-board.io"