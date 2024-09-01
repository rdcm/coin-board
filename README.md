## Up & Running

`make help` - for getting full list of commands  

## Local

1. `make create-secrets username={github_username} token={github_token}`
2. `make deploy-secrets` - deploy `github-registry.yaml` to k8s for pulling private images
3. `make deploy-dev` - deploying applications to local context

## Local::Endpoints

`kubectl port-forward <api_pod_name> 7373:7373` - for access to api `http://localhost:7373/rates`.  

`kubectl port-forward <ui_pod_name> 3000:3000`  - for access to ui  `http://localhost:3000/`. 

For correct operation of UI it is necessary to do port forwarding of two pods simultaneously.

## Digitalocean

1. `kubectl apply -f github-registry.yaml` - yaml generated with command `make create-secrets`
2. `helm install nginx-ingress ingress-nginx/ingress-nginx --namespace ingress-nginx --create-namespace` - installing nginx ingress controller into separated namespace
3. `kubectl apply -f cluster-issuer.yml`:
```yml
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    server: https://acme-v02.api.letsencrypt.org/directory
    email: <example@mail.com> # actual email for receiving important notifications from letsencrypt
    privateKeySecretRef:
      name: letsencrypt-prod
    solvers:
      - http01:
          ingress:
            class: nginx
```
4. `make deploy-prod` deploying applications to production context

## Digitalocean::Endpoints

`https://coin-board.io/` - ui  
`https://api.coin-board.io/rates` - api
   
   
