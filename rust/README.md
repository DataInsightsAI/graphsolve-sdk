# graphsolve

Rust client and CLI for the [GraphSolve](https://graphsolve.ai) engine API —
production-network solving, PVT/EOS, flow assurance and transient multiphase
flow, as a hosted service.

## The CLI

```sh
cargo install graphsolve
export GRAPHSOLVE_API_KEY=...
```

```sh
graphsolve tools                              # every tool, its domain and price
graphsolve solve_network model.json           # run one
graphsolve quote run_transient_wave big.json  # what would that cost? spends nothing
graphsolve run run_transient_wave big.json    # submit as a job and wait
graphsolve me                                 # which key this is, and its scopes
```

Results go to stdout as JSON, so `graphsolve solve_network model.json | jq
.node_results` works. What the call cost goes to stderr, so it is visible without
polluting the data.

**This is a client, not the engine.** It contains no physics. It reads
`GRAPHSOLVE_API_KEY`, makes the same authenticated HTTPS calls as any SDK in this
repository, and every one of them is metered and billed identically. Without a
key it prints help and does nothing.

## The library

```toml
[dependencies]
graphsolve = { version = "1.0", default-features = false }
```

`default-features = false` drops `clap` and `anyhow`, which only the CLI needs.

```rust
use graphsolve::{CalculatePressureDropParams, FlowCorrelationName, GraphSolve};

let gs = GraphSolve::new()?;        // reads GRAPHSOLVE_API_KEY

let response = gs
    .calculate_pressure_drop(CalculatePressureDropParams {
        angle: 90.0,
        correlation: FlowCorrelationName::BeggsBrill,
        density: vec![800.0, 20.0],
        diameter: 0.15,
        ift: 0.02,
        pressure: 10.0,
        roughness: 4.5e-5,
        velocity: vec![1.2, 3.4],
        viscosity: vec![1.0e-3, 1.8e-5],
    })
    .await?;

println!("{:?} credits", response.credits_charged());
```

Async on `tokio`, `rustls` rather than OpenSSL, and one params struct per tool.

## What the types are for

`correlation` is not a `String`. `FlowCorrelationName::BeggsBrilll` does not
compile, and the editor lists all 19 names — as it does for the EOS models, the
four hydrate models, and the rest. A struct literal names every field at the call
site, which is what keeps `ift` and `pressure` from being swapped:

```rust
// Optional fields have a Default, so only the tail needs skipping.
gs.calculate_compressor(CalculateCompressorParams {
    inlet_pressure: 5.0,
    inlet_temperature: 350.0,
    pressure_ratio: 2.0,
    fluid: serde_json::json!({ "gas_rate": 500_000, "gas_mw": 18.5 }),
    ..Default::default()
})
.await?;
```

An unset optional is left out of the payload entirely rather than sent as null:
several tools distinguish "not given" from an explicit null.

## Units

SI throughout — pressure MPa, temperature K, rates kSm³/day, lengths m.
Permeability in millidarcy is the single exception.

## Errors

One `Error` enum, because the right response differs in each case — and so does
the bill:

| Variant | Meaning | Charged |
|---|---|---|
| `InvalidPayload` | the request was wrong; nothing ran | no |
| `CredentialsRejected` | the API key was rejected at the token endpoint | no |
| `ToolNotPermitted` | the key lacks the scope this tool needs | no |
| `RateLimited { retry_after }` | slow down | no |
| `SolverDidNotConverge { credits_charged }` | it ran and produced no answer | **yes** |
| `Server` | our fault | no |

That distinction is the one worth knowing: a calculation that runs and does not
converge is a real result which consumed real compute, so it is charged. A
malformed request is free. `Error::is_charged()` answers it directly.

Keys carry scopes. `engine:read` covers the free catalogue and property lookups;
`engine:solve` covers anything that runs a calculation.

## Authentication

The API key is exchanged at the platform's token endpoint for a short-lived
access token on the first call. The token is cached and refreshed before it
expires, and clones of a client share one token, so concurrent callers cause a
single exchange.

```rust
let token = gs.access_token().await?;  // if you need the bearer elsewhere
```

`Error::CredentialsRejected` means the key is wrong; `Error::TokenExchange` means
the authentication service could not be reached and the key may well be fine.

For a non-production platform, set both `base_url` and `token_url` on the
builder, or `GRAPHSOLVE_BASE_URL` and `GRAPHSOLVE_TOKEN_URL`. The token URL is
never derived from the base URL.

## Retries

`429` and `503` are retried with backoff and jitter, honouring `Retry-After`.
Nothing else is — a `400` fails identically every time. Every call carries an
`Idempotency-Key`, so a retry after a network timeout is never billed twice.

## Escape hatch

A tool newer than the installed crate is still reachable:

```rust
gs.call("a_tool_released_yesterday", &serde_json::json!({ /* ... */ })).await?;
```

## Licence

Apache-2.0.
