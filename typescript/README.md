# @graphsolve/sdk

TypeScript client for the [GraphSolve](https://graphsolve.ai) engine API —
production-network solving, PVT/EOS, flow assurance and transient multiphase
flow, as a hosted service.

```sh
npm install @graphsolve/sdk
export GRAPHSOLVE_API_KEY=...
```

```ts
import { GraphSolve } from "@graphsolve/sdk";

const gs = new GraphSolve();

// Any of the 98 tools, typed
const result = await gs.solve_network({ network_json: model });
console.log(result.metadata?.billing?.credits_charged);

// Price a long run before committing to it
const quote = await gs.quote("run_transient_wave", request);
console.log(quote.quote.estimated_credits);

// Long-running work, without the async
const answer = await gs.run("run_transient_wave", request);

// Which key this is, and what it may do
console.log((await gs.me()).scopes);
```

No runtime dependencies — it uses the platform `fetch`. Node 18 or newer, or any
runtime with a global `fetch`; pass your own with `new GraphSolve({ fetch })`.
ESM and CommonJS builds are both published.

## Why the names are snake_case

`gs.run_transient_wave({ network_json })`, not `runTransientWave({ networkJson })`.
Tool and argument names are the wire contract: they are what the API is
documented with and what appears in an error message. Renaming them would mean a
translation layer on every call and documentation that never matches the code.
Options the client defines itself — `baseUrl`, `maxRetries`, `creditsCharged` —
are camelCase.

## What the types are for

The `correlation` argument is not a `string`:

```ts
await gs.calculate_pressure_drop({ correlation: "Beggs Brill", ... });
//                                               ^ Type '"Beggs Brill"' is not
//                                                 assignable to type
//                                                 'FlowCorrelationName'.
//                                                 Did you mean '"Beggs-Brill"'?
```

All 19 correlation names, the three EOS models, the four hydrate models and the
rest are string unions emitted from the API spec, so the editor offers them and
a typo fails before the call is made. They are exported too, if you want to hold
one in a variable of your own:

```ts
import type { FlowCorrelationName } from "@graphsolve/sdk";
```

## Units

SI throughout — pressure MPa, temperature K, rates kSm³/day, lengths m.
Permeability in millidarcy is the single exception.

## Errors

Failures are typed, because the right response differs in each case — and so
does the bill:

| Thrown | Meaning | Charged |
|---|---|---|
| `InvalidPayload` | the request was wrong; nothing ran | no |
| `AuthenticationError` | the API key was rejected, or the token is invalid | no |
| `ToolNotPermitted` | the key lacks the scope this tool needs | no |
| `RateLimited` | slow down; carries `retryAfter` | no |
| `SolverDidNotConverge` | it ran and produced no answer | **yes** |
| `ServerError` | our fault | no |

That distinction is the one worth knowing: a calculation that runs and does not
converge is a real result which consumed real compute, so it is charged. A
malformed request is free. `SolverDidNotConverge` carries `creditsCharged` so
the cost is visible where it is noticed.

Keys carry scopes. `engine:read` covers the free catalogue and property lookups;
`engine:solve` covers anything that runs a calculation.

## Authentication

The API key is exchanged at the platform's token endpoint for a short-lived
access token on the first call. The token is cached and refreshed before it
expires, so nothing in your code has to think about it. Concurrent calls share a
single exchange.

```ts
const token = await gs.getAccessToken();  // if you need the bearer elsewhere
```

For a non-production platform, pass `baseUrl` and `tokenUrl`, or set
`GRAPHSOLVE_BASE_URL` and `GRAPHSOLVE_TOKEN_URL`. The token URL is never derived
from the base URL.

## Retries

`429` and `503` are retried with backoff and jitter, honouring `Retry-After`.
Nothing else is — a `400` fails identically every time. Every call carries an
`Idempotency-Key`, so a retry after a network timeout is never billed twice.

## Escape hatch

A tool newer than the installed client is still reachable:

```ts
await gs.call("a_tool_released_yesterday", { ... });
```

## Licence

Apache-2.0.
