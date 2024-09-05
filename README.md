## Up & Running

`make help` - for getting full list of commands  

## Local::Docker

1. `make docker-build` - build docker images
2. `make docker-up`  - up & running docker containers 
3. `make docker-down` - down docker containers

## Local::Docker::Endpoints

- `http://localhost:7373/rates` - api
- `http://localhost:3000/` - ui


## Local::Kubernetes

1. `make create-secrets username={github_username} token={github_token}` - generate secrets for github registry
2. `make deploy-secrets`  - deploying `github-registry.yaml` for pulling private images
3. `make deploy-coin-board-dev` - deploying applications
4. (optional)`make deploy-monitoring-dev` - deploying infrastructure for monitoring

## Local::Kubernetes::Endpoints

Applications: 
- `kubectl port-forward <api_pod_name> 7373:7373` - for access to api `http://localhost:7373/rates`.
- `kubectl port-forward <ui_pod_name> 3000:3000`  - for access to ui  `http://localhost:3000/`.

Monitoring tools: 
- `kubectl port-forward prometheus-stateful-set-0 9090:9090 -n monitoring` - for access to prometheus `http://localhost:9090/`
- `kubectl port-forward grafana-stateful-set-0 3000:3000 -n monitoring` - for access to grafana `http://localhost:3000/`

## Digitalocean::Kubernetes

1. `make create-secrets username={github_username} token={github_token}` - generate secrets for github registry
2. `make deploy-secrets`  - deploying `github-registry.yaml` for pulling private images
3. `make deploy-ingress-controller` - deploying ingress-controller 
4. `make deploy-cert-manager` - deploying cert-manager 
5. `make deploy-cluster-issuer` - deploying cluster-issuer
6. `make deploy-coin-board-prod` - deploying applications
7. `make deploy-monitoring-prod` - deploying infrastructure for monitoring
8. `make trigger-export` - for exporting currency rates immediately
9. Enjoy!

## Digitalocean::Kubernetes::Endpoints

Applications:
 
- `https://api.coin-board.io/rates` - api
- `https://coin-board.io/` - ui 

Monitoring tools:
- `kubectl port-forward prometheus-stateful-set-0 9090:9090 -n monitoring` - for access to prometheus `http://localhost:9090/`
- `kubectl port-forward grafana-stateful-set-0 3000:3000 -n monitoring` - for access to grafana `http://localhost:3000/`