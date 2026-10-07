# graphsolve

Python client for the [GraphSolve](https://graphsolve.ai) engine API —
production-network solving, PVT/EOS, flow assurance and transient multiphase
flow, as a hosted service.

```sh
pip install graphsolve
export GRAPHSOLVE_API_KEY=...
```

```python
from graphsolve import GraphSolve

gs = GraphSolve()          # or GraphSolve(api_key)

# Any of the 101 tools
result = gs.call("solve_network", {"network_json": model})
print(result["metadata"]["billing"]["credits_charged"])

# Price a long run before committing to it
quote = gs.quote("run_transient_wave", request)
print(quote["quote"]["estimated_credits"])

# Long-running work, without the async
answer = gs.run("run_transient_wave", request)

# Which key this is, and what it may do
print(gs.me()["scopes"])
```

## Units

SI throughout — pressure MPa, temperature K, rates kSm³/day, lengths m.
Permeability in millidarcy is the single exception.

## Errors

Failures are typed, because the right response differs in each case — and so
does the bill:

| Raised | Meaning | Charged |
|---|---|---|
| `InvalidPayload` | the request was wrong; nothing ran | no |
| `AuthenticationError` | the API key was rejected, or the token is invalid | no |
| `ToolNotPermitted` | the key lacks the scope this tool needs | no |
| `RateLimited` | slow down; carries `retry_after` | no |
| `SolverDidNotConverge` | it ran and produced no answer | **yes** |
| `ServerError` | our fault | no |

That distinction is the one worth knowing: a calculation that runs and does not
converge is a real result which consumed real compute, so it is charged. A
malformed request is free.

Keys carry scopes. `engine:read` covers the free catalogue and property lookups;
`engine:solve` covers anything that runs a calculation.

## Authentication

The API key is exchanged at the platform's token endpoint for a short-lived
access token on the first call. The token is cached and refreshed before it
expires, so nothing in your code has to think about it. Sharing one client
across threads is safe, and concurrent first calls produce a single exchange.

```python
token = gs.get_access_token()   # if you need the same bearer elsewhere
```

For a non-production platform, point both endpoints at it with `base_url` and
`token_url`, or `GRAPHSOLVE_BASE_URL` and `GRAPHSOLVE_TOKEN_URL`. The token URL is
never derived from the base URL.

## Retries

`429` and `503` are retried with backoff and jitter, honouring `Retry-After`.
Nothing else is — a `400` fails identically every time. Every call carries an
`Idempotency-Key`, so a retry after a network timeout is never billed twice.

## Licence

Apache-2.0.
