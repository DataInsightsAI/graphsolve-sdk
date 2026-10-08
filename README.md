# GraphSolve client SDKs

Client libraries for the [GraphSolve](https://graphsolve.ai) engine API —
physics-based production-network solving, PVT/EOS, flow assurance and transient
multiphase flow, as a hosted API.

**Documentation: [datainsightsai.github.io/graphsolve-sdk](https://datainsightsai.github.io/graphsolve-sdk/)**
— getting started, guides, a worked example, every tool with its arguments and
price, and the API reference for each language.

| Language | Package | Install | API reference |
|---|---|---|---|
| Python | [`graphsolve`](https://pypi.org/project/graphsolve/) | `pip install graphsolve` | [Python](https://datainsightsai.github.io/graphsolve-sdk/reference/python/) |
| TypeScript | [`@graphsolve/sdk`](https://www.npmjs.com/package/@graphsolve/sdk) | `npm install @graphsolve/sdk` | [TypeScript](https://datainsightsai.github.io/graphsolve-sdk/reference/typescript/) |
| Rust | [`graphsolve`](https://docs.rs/graphsolve) | `cargo add graphsolve` | [docs.rs](https://docs.rs/graphsolve) |

Julia and R clients are planned.

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
language. It takes a tool's arguments as JSON from a file or stdin and prints
the result:

```sh
cargo install graphsolve
graphsolve me
echo '{"value": 1, "from_unit": "MPa", "to_unit": "psi"}' | graphsolve convert_units
```

## What this repo is

Every client here is generated or emitted from **one contract**:
[`spec/graphsolve-v1.json`](spec/graphsolve-v1.json), the OpenAPI 3.1 document
published by the engine. That is why they live together: an API change touches
every language at once, and CI proves they all still build against the same spec.
The tool reference in the documentation is generated from the same spec.

Method and argument names are the API's own — `solve_network`, `network_json` —
in every language, so the documentation and the code always agree.

Releases are per-language and independent — tags look like `python-v1.2.0`,
`ts-v1.2.0` and `rust-v1.2.0`. [`CHANGELOG.md`](CHANGELOG.md) lists what each
release adds, changes or breaks.

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
lookups, `engine:solve` for anything that runs a calculation. See
[Authentication](https://datainsightsai.github.io/graphsolve-sdk/getting-started/authentication/).

To check a new key end to end, run [`examples/smoke.py`](examples/smoke.py). It
exchanges the key, reads its identity and scopes, and runs one free tool; see
[`examples/README.md`](examples/README.md) for which hosts to point it at.

## Units

SI throughout — pressure MPa, temperature K, rates kSm³/day, lengths m.
Permeability in millidarcy is the single exception. See
[Units](https://datainsightsai.github.io/graphsolve-sdk/guides/units/) for the
conventions that are easy to get backwards.

## Contributing

Every client implements the same behavioural contract regardless of language.
See [CONTRIBUTING.md](CONTRIBUTING.md) before adding or changing one, and
[MAINTAINING.md](MAINTAINING.md) for how the spec gets here, how releases work,
and why no workflow in this repository holds a secret. The documentation site
is built with `scripts/build-docs.sh`; see CONTRIBUTING.md.
