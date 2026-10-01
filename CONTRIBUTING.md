# Contributing

## The contract every client implements

Generators produce transport. They do not produce the behaviour below, and
without writing it down five implementations drift apart. Each client's test
suite must assert each point.

### Credentials
- Take an API key, falling back to `GRAPHSOLVE_API_KEY`.
- Fail at construction when it is missing. Only the network exchange is lazy; a
  missing key is a programming error and should surface as one.
- **Never log the key, or any part of it.** It must not appear in an error
  message, a log line, or a debug rendering of the client.

### Tokens
The API key is exchanged for a short-lived access token at the platform's token
endpoint, which is a different host from the engine API:

```
POST {token_url}
Authorization: Bearer <api_key>
Content-Type: application/x-www-form-urlencoded

grant_type=client_credentials
  → {"access_token": "…", "token_type": "Bearer", "expires_in": 3600}
```

- Default the token URL to the production platform and let it be overridden
  (`GRAPHSOLVE_TOKEN_URL`) independently of the base URL (`GRAPHSOLVE_BASE_URL`).
  Never derive it from `base_url`: tokens are issued centrally for every
  deployment.
- Cache the token and reuse it until `min(300 s, expires_in / 2)` of life
  remains. Refreshing early is what keeps a 401 off the retry path entirely;
  the cap on half the life is what stops that margin eating the whole token.
  The platform limits a token's life to the API key's own remaining life, so a
  key in its last five minutes issues tokens shorter than the flat margin —
  compared against 300 s alone, every one of them is stale on arrival and every
  call exchanges again, into a rate-limited endpoint.
- Refuse an `expires_in` of zero or less rather than caching it, and say that
  the key is at or past its expiry.
- Concurrent callers must trigger **exactly one** exchange. A client is shared
  across threads, tasks or clones, so this needs a lock, a shared in-flight
  promise, or the language's equivalent — not just a cached value.
- The exchange retries on `429`, `502`, `503` and `504` — wider than a tool
  call, because it runs no tool and is charged nothing, so repeating it cannot
  cost anything.
- A `5xx` during exchange is "the authentication service is unavailable"; a
  `4xx` is "this API key was rejected". They are not the same problem and the
  caller should not have to guess which happened.
- Expose the current token, so a caller wiring their own HTTP can reuse it.
- A `403` from the engine means the token lacks the scope a tool needs:
  `engine:read` for free lookups, `engine:solve` for anything that runs a
  calculation.

### Retries
- Retry `429` and `503` on any request, with exponential backoff and jitter,
  honouring `Retry-After` when present. Neither reached the engine, so
  repeating the request cannot duplicate a charge.
- Retry `502`, `504` and a transport failure **only on a `GET`**. Each of those
  can arrive after the engine has already run the tool, and a tool that ran is
  metered whether or not its response arrived. The engine does not read
  `Idempotency-Key`, so repeating a `POST` would be charged twice.
- **Never retry any other 4xx.** A `400` will fail identically every time, and
  retrying a `402` or `403` changes nothing either.
- When a `POST` is abandoned for that reason, raise **`OutcomeUnknown`**, not
  a generic server error. The caller cannot otherwise tell "nothing happened"
  from "it may have run and been charged", and those need different responses.
- Send an auto-generated `Idempotency-Key` on every call, one per logical call
  and reused across its retries. The engine does not honour it yet; send it so
  that a server that does needs no change here. **Do not describe it as what
  prevents a double charge** — the retry policy above is.

### Errors carry meaning, not just status
Raise typed errors: `InsufficientCredits`, `RateLimited(retry_after)`,
`ToolNotPermitted`, `InvalidPayload`, `SolverDidNotConverge`, `OutcomeUnknown`.

**The distinction that matters most:** a calculation that ran and did not
converge is a `200` carrying `failure_kind: "computation_failed"`, and it **is
charged**, because it consumed the compute. A malformed request is a `400`, and
it is **free**. A client that collapses both into "error" makes the API look
broken, gets its retry logic wrong, and makes billing questions unanswerable.

### Billing is visible
Surface `credits_charged` from every response. A caller should never have to
reconcile a bill to find out what a call cost. Balances are held by the
platform, not the engine, so `/v1/me` reports identity and scopes rather than a
balance.

### Long-running work
Tools in the `transient` domain run for minutes to hours and are submitted as
jobs. Provide:
- a quote (`dry_run`) that spends nothing, so a caller can size a run before
  submitting it;
- a blocking wrapper (`wait=True` or the idiomatic equivalent) that polls, so
  async is invisible unless the caller wants it.

### Escape hatch
Always expose `call(tool_name, arguments)`. A tool newer than the installed
client must still be reachable without waiting for a release.

## Regenerating

Clients are generated from `spec/graphsolve-v1.json` by the emitters in `emit/`.
CI regenerates and fails if the committed output differs, so a spec bump cannot
half-land.

```sh
python emit/emit_python.py
python emit/emit_typescript.py
python emit/emit_rust.py     # needs rustfmt: rustup component add rustfmt
python emit/emit_docs.py     # the tool reference pages under docs/reference/tools
```

`emit/_spec.py` holds what must not drift between languages — which operations
exist, how a `$ref` resolves, what counts as a fixed vocabulary, and how a price
is worded. Language-specific type mapping lives in each emitter.

## Per-language gates

| Language | Gate |
|---|---|
| Python | `ruff check src tests` and `pytest tests` |
| TypeScript | `npm run lint && npm run typecheck && npm test && npm run build` |
| Rust | `cargo fmt -- --check && cargo clippy --all-targets -- -D warnings && cargo test` |

The Rust client additionally follows: `thiserror` in the library and `anyhow` in
the CLI, no `.unwrap()` outside tests, enums rather than boolean parameters or
stringly-typed vocabularies, named constants rather than magic numbers, and
`# Errors` on everything public that returns a `Result`.

## Comments

Plain and factual. Say what the code does or why it is there, then stop.

- One or two short sentences. Often one clause is enough.
- Name the concrete failure if that is the point of the comment, e.g. "an older
  npm falls back to a classic publish and fails with ENEEDAUTH".
- Do not editorialise. "Without this the rule is a sentence in a document rather
  than something that holds" says nothing; "check every client claims the same
  major.minor as the spec" says it all.
- Do not reference design documents, section numbers, prototypes or past
  experiments. Readers of this repo do not have them. Referring to files that
  are in this repo is fine.
- Do not explain the same thing twice in one comment.

## Public repository

This repository is public and the engine is not. Do not reference internal
hostnames, AWS account ids, stack names, or private crate paths — in code,
comments, examples, or issue templates.

CI greps every file for the names in `.github/private-names.txt`. Add to that
list whenever a new private repository, crate or host exists; it is a backstop,
not a substitute for not writing them.

## Documentation

The site under `docs/` is built with MkDocs Material and published to GitHub
Pages from `main` by `.github/workflows/docs.yml`. The guides are written by
hand; the tool reference is generated by `emit/emit_docs.py`, the Python
reference is read from the docstrings by mkdocstrings, and each tool links to
its method in all three client references. The example pages include
the files in `examples/` directly, so an example is edited in one place.

The TypeScript reference is generated by TypeDoc (`typedoc-plugin-markdown`)
into `docs/reference/typescript/api/` on every build and is not committed.

```sh
pip install -r docs/requirements.txt
(cd typescript && npm ci)
scripts/build-docs.sh serve  # live preview at http://127.0.0.1:8000
scripts/build-docs.sh        # what CI runs
```

