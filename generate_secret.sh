#!/bin/bash

username=""
token=""

function show_help {
    echo "Usage: $0 --username USERNAME --token TOKEN"
    echo
    echo "Options:"
    echo "  --username   Specify the username (required)"
    echo "  --token      Specify the token (required)"
    echo "  --help       Display this help message"
    echo
    exit 0
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --username)
            if [[ -n "$2" && "$2" != "--"* ]]; then
                username="$2"
                shift 2
            else
                echo "Error: '--username' requires a non-empty value."
                show_help
            fi
            ;;
        --token)
            if [[ -n "$2" && "$2" != "--"* ]]; then
                token="$2"
                shift 2
            else
                echo "Error: '--token' requires a non-empty value."
                show_help
            fi
            ;;
        --help)
            show_help
            ;;
        *)
            echo "Invalid option: $1"
            show_help
            ;;
    esac
done

if [[ -z "$username" || -z "$token" ]]; then
    echo "Error: Both 'username' and 'token' parameters are required."
    exit 1
fi

token_base_64=$(echo -n "${username}":"${token}" | base64)
auths_json=$(echo -n "{\"auths\":{\"ghcr.io\":{\"auth\":\"$token_base_64\"}}}" | base64)

echo "kind: Secret
type: kubernetes.io/dockerconfigjson
apiVersion: v1
metadata:
  name: github-registry
  labels:
    app: coin-board
data:
  .dockerconfigjson: ${auths_json}"