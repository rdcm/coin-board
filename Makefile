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

deploy-dev:
	helm upgrade --install --atomic --timeout 300s --wait coin-board helm -f ./helm/values/dev.yml

deploy-prod:
	helm upgrade --install --atomic --timeout 300s --wait coin-board helm -f ./helm/values/prod.yml

down:
	helm delete coin-board

render-prod:
	helm template -f ./helm/values/prod.yml helm > template-render-prod.yml

render-dev:
	helm template -f ./helm/values/dev.yml helm > template-render-dev.yml

export:
	kubectl create job --from=cronjob/rates-exporter rates-exporter

