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
	# https://stackoverflow.com/questions/63135361/how-to-create-kubernetes-namespace-if-it-does-not-exist
	kubectl create namespace coin-board --dry-run=client -o yaml | kubectl apply -f -
	kubectl apply -f github-registry.yaml -n coin-board

deploy-coin-board-dev:
	helm upgrade --install --atomic --timeout 300s --wait coin-board helm/coin-board -f ./helm/coin-board/values/dev.yaml --create-namespace --namespace coin-board

deploy-coin-board-prod:
	helm upgrade --install --atomic --timeout 300s --wait coin-board helm/coin-board -f ./helm/coin-board/values/prod.yaml --create-namespace --namespace coin-board

deploy-ingress-controller:
	helm upgrade --install --atomic --timeout 300s --wait ingress-controller helm/ingress-controller --create-namespace --namespace ingress-controller

deploy-cert-manager:
	helm upgrade --install --atomic --timeout 300s --wait cert-manager helm/cert-manager --create-namespace --namespace cert-manager

deploy-cluster-issuer:
	helm upgrade --install --atomic --timeout 300s --wait cluster-issuer helm/cluster-issuer --create-namespace --namespace cert-manager

deploy-monitoring-dev:
	helm upgrade --install --atomic --timeout 300s --wait monitoring helm/monitoring -f ./helm/monitoring/values/dev.yaml --create-namespace --namespace monitoring

deploy-monitoring-prod:
	helm upgrade --install --atomic --timeout 300s --wait monitoring helm/monitoring -f ./helm/monitoring/values/prod.yaml --create-namespace --namespace monitoring

delete-ingress-controller:
	helm delete ingress-controller --namespace ingress-controller

delete-cert-manager:
	helm delete cert-manager --namespace cert-manager

delete-cluster-issuer:
	helm delete cluster-issuer --namespace cert-manager

delete-coin-board:
	helm delete coin-board --namespace coin-board

delete-monitoring:
	helm delete monitoring --namespace monitoring

render-coin-board-prod:
	helm template -f ./helm/coin-board/values/prod.yaml helm/coin-board > template-render-prod.yaml

render-coin-board-dev:
	helm template -f ./helm/coin-board/values/dev.yaml helm/coin-board > template-render-dev.yaml

trigger-export:
	kubectl create job --from=cronjob/rates-exporter-cronjob rates-exporter-job --namespace coin-board

help:
	@echo "Local:"
	@echo "  ${GREEN}make lint${NC}                     ${YELLOW}(no arguments)${NC}                      run code linter"
	@echo "  ${GREEN}make format${NC}                   ${YELLOW}(no arguments)${NC}                      run code formatter"
	@echo "  ${GREEN}make build-dev${NC}                ${YELLOW}(no arguments)${NC}                      make application dev builds"
	@echo "  ${GREEN}make run-api-dev${NC}              ${YELLOW}(no arguments)${NC}                      run api dev build locally"
	@echo "  ${GREEN}make run-exporter-dev${NC}         ${YELLOW}(no arguments)${NC}                      run exporter dev build locally"
	@echo "Docker:"
	@echo "  ${BLUE}make docker-build${NC}             ${YELLOW}(no arguments)${NC}                      build docker images with docker-compose"
	@echo "  ${BLUE}make docker-up${NC}                ${YELLOW}(no arguments)${NC}                      up docker images with docker-compose"
	@echo "  ${BLUE}make docker-down${NC}              ${YELLOW}(no arguments)${NC}                      down docker-compose services "
	@echo "Helm:"
	@echo "  ${PURPLE}make create-secrets${NC}           ${YELLOW}username=<username> token=<token>${NC}   generate k8s secret for pulling images from private github registry "
	@echo "  ${PURPLE}make deploy-coin-board-dev${NC}    ${YELLOW}(no arguments)${NC}                      deploy applications helm chart with dev values"
	@echo "  ${PURPLE}make deploy-coin-board-prod${NC}   ${YELLOW}(no arguments)${NC}                      deploy applications helm chart with prod values"
	@echo "  ${PURPLE}make deploy-cert-manager${NC}      ${YELLOW}(no arguments)${NC}                      deploy cert-manger ${RED}(only for prod env)${NC}"
	@echo "  ${PURPLE}make deploy-cluster-issuer${NC}    ${YELLOW}(no arguments)${NC}                      deploy cluster-issuer ${RED}(only for prod env)${NC}"
	@echo "  ${PURPLE}make deploy-ingress-controller${NC}${YELLOW}(no arguments)${NC}                      deploy ingress-controller ${RED}(only for prod env)${NC}"
	@echo "  ${PURPLE}make deploy-secrets${NC}           ${YELLOW}(no arguments)${NC}                      deploy github registry secrets"
	@echo "  ${PURPLE}make deploy-monitoring-dev${NC}    ${YELLOW}(no arguments)${NC}                      deploy monitoring infrastructure with dev values"
	@echo "  ${PURPLE}make deploy-monitoring-prod${NC}   ${YELLOW}(no arguments)${NC}                      deploy monitoring infrastructure with prod values"
	@echo "  ${PURPLE}make trigger-export${NC}           ${YELLOW}(no arguments)${NC}                      trigger cronjob for export currency rates "
	@echo "  ${PURPLE}make render-coin-board-prod${NC}   ${YELLOW}(no arguments)${NC}                      render helm chart with prod values"
	@echo "  ${PURPLE}make render-coin-board-dev${NC}    ${YELLOW}(no arguments)${NC}                      render helm chart with dev values"
	@echo "  ${RED}make delete-coin-board${NC}        ${YELLOW}(no arguments)${NC}                      delete helm chart with applications"
	@echo "  ${RED}make delete-cert-manager${NC}      ${YELLOW}(no arguments)${NC}                      delete helm chart with cert-manager"
	@echo "  ${RED}make delete-cluster-issuer${NC}    ${YELLOW}(no arguments)${NC}                      delete helm chart with cluster-issuer"
	@echo "  ${RED}make delete-ingress-controller${NC}${YELLOW}(no arguments)${NC}                      delete helm chart with ingress-controller"
	@echo "  ${RED}make delete-monitoring${NC}        ${YELLOW}(no arguments)${NC}                      delete helm chart with monitoring"

