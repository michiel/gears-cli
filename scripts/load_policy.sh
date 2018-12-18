#!/bin/bash

# curl -X PUT --data-binary @policy.rego \
#   localhost:8181/v1/policies/example

# curl -X PUT --data-binary @policy-token.rego \
#   localhost:8181/v1/policies/example-token

curl -X PUT --data-binary @./resource/policy-token.rego \
  localhost:8181/v1/policies/jsonapi-token


