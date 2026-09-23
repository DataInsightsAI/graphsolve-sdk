# GraphSolve client SDKs

Client libraries for the [GraphSolve](https://graphsolve.ai) engine API —
physics-based production-network solving, PVT/EOS, flow assurance and transient
multiphase flow, as a hosted API.

| Language | Package | Registry | Status |
|---|---|---|---|
| Python | `graphsolve` | PyPI | in progress |
| TypeScript | `@graphsolve/sdk` | npm | in progress |
| Rust | `graphsolve` | crates.io | in progress |
| Julia | `GraphSolve.jl` | General | planned |
| R | `graphsolve` | r-universe | planned |

```python
from graphsolve import GraphSolve

gs = GraphSolve()         # reads GRAPHSOLVE_API_KEY
result = gs.solve_network(network_json=model)
print(result["metadata"]["billing"]["credits_charged"])
```

```ts
import { GraphSolve } from "@graphsolve/sdk";

const gs = new GraphSolve();  // reads GRAPHSOLVE_API_KEY
const result = await gs.solve_network({ network_json: model });
console.log(result.metadata?.billing?.credits_charged);
```

The Rust crate also installs a CLI, for engineers who do not write code in any
language:

```sh
cargo install graphsolve
graphsolve solve_network model.json | jq .node_results
```

## What this repo is

Every client here is generated or emitted from **one contract**:
[`spec/graphsolve-v1.json`](spec/graphsolve-v1.json), the OpenAPI 3.1 document
published by the engine. That is why they live together: an API change touches
every language at once, and CI proves they all still build against the same spec.

Method and argument names are the API's own — `solve_network`, `network_json` —
in every language, so the operator guide and the code always agree.

Releases are per-language and independent — tags look like `python-v1.2.0`,
`ts-v1.2.0` and `rust-v1.2.0`.

## Versioning

A client's **major.minor matches the engine API it speaks**; the patch number is
the client's own. Any `1.2.x` client works against engine `1.2`.

## Getting an API key

The API is commercial. API keys are issued per customer and every call is
metered. Contact us to get one, then:

```sh
export GRAPHSOLVE_API_KEY=...
```

The clients exchange the key at the platform's token endpoint for a short-lived
access token and refresh it before it expires; nothing in your code has to
handle that. Keys carry scopes: `engine:read` for the free catalogue and property
lookups, `engine:solve` for anything that runs a calculation.

To check a new key end to end, run [`examples/smoke.py`](examples/smoke.py). It
exchanges the key, reads its identity and scopes, and runs one free tool; see
[`examples/README.md`](examples/README.md) for which hosts to point it at.

## Units

SI throughout — pressure MPa, temperature K, rates kSm³/day, lengths m.
Permeability in millidarcy is the single exception.

## Contributing

Every client implements the same behavioural contract regardless of language.
See [CONTRIBUTING.md](CONTRIBUTING.md) before adding or changing one, and
[MAINTAINING.md](MAINTAINING.md) for how the spec gets here, how releases work,
and why no workflow in this repository holds a secret.
