RED=\033[0;31m
GREEN=\033[0;32m
YELLOW=\033[1;33m
BLUE=\033[0;34m
PURPLE=\033[0;35m
NC=\033[0m # No Color

lint:
	cargo clippy

format:
	cargo fmt

build-dev:
	cargo build

run-api-dev:
	./target/debug/rates-api

run-exporter-dev:
	./target/debug/rates-exporter

docker-build:
	DOCKER_BUILDKIT=1 docker compose build --progress=plain

docker-up:
	docker compose up -d

docker-down:
	docker compose down

create-secrets:
	./generate_secret.sh --username $(username) --token $(token) > github-registry.yaml

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

trigger-export:
	kubectl create job --from=cronjob/rates-exporter rates-exporter

help:
	@echo "Usage:"
	@echo "  ${GREEN}make create-secrets${NC}     ${YELLOW}username=<username> token=<token>${NC}   generate k8s secret for pulling images from private github registry "
	@echo "  ${GREEN}make lint${NC}               ${YELLOW}(no arguments)${NC}                      run code linter"
	@echo "  ${GREEN}make format${NC}             ${YELLOW}(no arguments)${NC}                      run code formatter"
	@echo "  ${GREEN}make build-dev${NC}          ${YELLOW}(no arguments)${NC}                      make application dev builds"
	@echo "  ${GREEN}make run-api-dev${NC}        ${YELLOW}(no arguments)${NC}                      run api dev build locally"
	@echo "  ${GREEN}make run-exporter-dev${NC}   ${YELLOW}(no arguments)${NC}                      run exporter dev build locally"
	@echo "  ${BLUE}make docker-build${NC}       ${YELLOW}(no arguments)${NC}                      build docker images with docker-compose"
	@echo "  ${BLUE}make docker-up${NC}          ${YELLOW}(no arguments)${NC}                      up docker images with docker-compose"
	@echo "  ${BLUE}make docker-down${NC}        ${YELLOW}(no arguments)${NC}                      down docker-compose services "
	@echo "  ${PURPLE}make deploy-dev${NC}         ${YELLOW}(no arguments)${NC}                      deploy applications helm chart with dev values "
	@echo "  ${PURPLE}make deploy-prod${NC}        ${YELLOW}(no arguments)${NC}                      deploy applications helm chart with prod values "
	@echo "  ${PURPLE}make deploy-secrets${NC}     ${YELLOW}(no arguments)${NC}                      deploy github registry secrets "
	@echo "  ${PURPLE}make trigger-export${NC}     ${YELLOW}(no arguments)${NC}                      trigger cronjob for export currency rates "
	@echo "  ${RED}make down${NC}               ${YELLOW}(no arguments)${NC}                      delete helm chart with applications"
	@echo "  ${GREEN}make render-prod${NC}        ${YELLOW}(no arguments)${NC}                      render helm chart with prod values"
	@echo "  ${GREEN}make render-dev${NC}         ${YELLOW}(no arguments)${NC}                      render helm chart with dev values"
