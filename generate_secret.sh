#!/bin/bash

username=$1
token=$2
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