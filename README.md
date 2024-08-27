## Setup 

`/etc/hosts`:
- 127.0.0.1 dev-wep-api.com
- 127.0.0.1 dev-coin-board-ui.com

## Up & Running

`make secret ARGS="{github_username} {github_token}"` or `./generate_secret.sh {github_username} {github_token}` - generate k8s secret for private github registry.   
`make up` - up services at k8s.   
`make down` - down services. 

## Endpoints

`http://dev-coin-board-ui.com` - ui  
`https://dev-wep-api.com/rates` - endpoint with currency rates