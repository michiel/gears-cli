package httpapi.authz

import input as http_api

# io.jwt.decode takes one argument (the encoded token) and has three outputs:
# the decoded header, payload and signature, in that order. Our policy only
# cares about the payload, so we ignore the others.
token = {"payload": payload} {
  io.jwt.decode(http_api.token, [header, payload, signature])
}

user_owns_token { http_api.user = token.payload.azp }

default allow = false

allow {
  http_api.method = "GET"
  http_api.path = ["jsonapi", "model", _]
  username = token.payload.user
  # user_owns_token
}

allow {
  http_api.method = "PUT"
  http_api.path = ["jsonapi", "model", _]
  username = token.payload.user
  # user_owns_token
}

allow {
  http_api.method = "GET"
  http_api.path = ["jsonapi", "model"]
  username = token.payload.user
  # user_owns_token
}

allow {
  http_api.method = "POST"
  http_api.path = ["jsonapi", "model"]
  username = token.payload.user
  # user_owns_token
}

