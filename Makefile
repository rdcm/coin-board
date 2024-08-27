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

secret:
	./generate_secret.sh $(ARGS) > github-registry.yaml

up:
	openssl req \
	-x509 -newkey rsa:4096 -sha256 -nodes \
	-keyout tls.key -out tls.crt \
	-subj "/CN=ingress.local" -days 365

	kubectl create secret tls api-secret \
  	--cert=tls.crt \
  	--key=tls.key

	kubectl apply -f github-registry.yaml

	helm upgrade --install --atomic --timeout 300s --wait coin-board helm

down:
	kubectl delete secret github-registry --ignore-not-found
	kubectl delete secret api-secret --ignore-not-found
	helm delete coin-board