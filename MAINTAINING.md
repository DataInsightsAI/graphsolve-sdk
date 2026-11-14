# Maintaining this repository

This repository is **public**. The engine it speaks to is **private**. Most of
what follows is a consequence of that.

## What may hold a credential, and what may not

**CI holds nothing.** `ci.yml` builds and tests; no job needs a secret, so none
has one. A pull request from a fork runs code its author wrote, on our runners.

Two guardrails:

- **`pull_request`, never `pull_request_target` or `workflow_run`.** Those two
  run with the base repository's secrets while checking out the fork's code.
  GitHub also withholds secrets from fork `pull_request` runs and gives them a
  read-only token, but that is the backstop rather than the design.
- **`permissions: contents: read` at the top of every workflow**, raised only on
  the job that needs more. A compromised build step then cannot push a commit,
  cut a release, or move a tag.

**Publishing holds nothing either.** All three registries authenticate by OIDC:
PyPI, npm and crates.io each exchange a short-lived GitHub identity token for a
registry token that lives minutes. There is no `NPM_TOKEN`, `CARGO_REGISTRY_TOKEN`
or PyPI token stored in this repository. Each publish job declares
`id-token: write` and names an `environment:`.

### The first publish of each package is the exception

Trusted publishing is configured *on the registry*, against a package it already
knows about — so it cannot be used for a package that does not exist yet:

| Registry | First publish |
|---|---|
| PyPI | OIDC works from the start — configure a **pending publisher** before the project exists. |
| npm | Publish once from a laptop, then add the trusted publisher on the package settings page. |
| crates.io | Publish once from a laptop, then link the repository. The crate must exist to be configured. |

So npm and crates.io need one manual publish from a machine with a token. Do it
from a clean checkout of the tag and revoke the token afterwards.

**The gate on publishing is the environment, not the workflow file.** `pypi`,
`npm` and `crates-io` are GitHub environments; give each one **required
reviewers**, so a human approves each release. Without that, anyone with write
access can publish by pushing a tag. This is a repository setting and cannot be
committed here.

**Third-party actions are pinned to a commit SHA**, with the version in a
comment beside it, because a tag can be moved. This matters most in the publish
workflows, the only runs that can mint a registry token. `@redocly/cli` is
pinned to a version rather than `@latest` for the same reason: `npx` would
download and run whatever was published today, in a job holding our checkout.

To update a pin, resolve the tag yourself and change both the SHA and the
comment:

```sh
gh api repos/actions/checkout/tags --jq '.[0] | "\(.name) \(.commit.sha)"'
```

## Releasing

The three clients carry the engine's version and are released together, once
per engine release. Each still has its own tag and publish workflow:

| Tag | Publishes | Environment |
|---|---|---|
| `python-v1.2.0` | `graphsolve` on PyPI | `pypi` |
| `ts-v1.2.0` | `@graphsolve/sdk` on npm | `npm` |
| `rust-v1.2.0` | `graphsolve` on crates.io | `crates-io` |

The release pull request (see below) opens the `## X.Y.Z — YYYY-MM-DD` section
in [`CHANGELOG.md`](CHANGELOG.md), with the `## Unreleased` entries and the
tools the spec added or removed. Edit it before merging. Once the pull request
is merged, `scripts/tag-release.sh` on an up-to-date `main` pushes the three
tags.
Write for users: name every new tool, and put anything that can break a
working call under **Breaking**: a newly required argument, a removed tool, a
type change in one client. Put changed prices under **Changed**. Mark a change
that affects only one client with the language. The docs site includes the
file as its Changelog page, and PyPI links to it.

Each publish job **re-runs that language's tests and asserts the tag matches the
committed version** before uploading, rather than assuming CI passed on the
commit. A crates.io version cannot be unpublished at all.

## Bringing in a new spec

The spec is the contract all three clients are generated from. The engine
generates it from its own tool registry, and it is **vendored** here at
`spec/graphsolve-v1.json`.

### Push, never pull

The spec is **pushed** from the private engine into this public repository. A
credential that lives in the engine repo and can write here gives away nothing
about the engine if this repository is compromised. A credential here that could
read the private engine would sit in a public repository exposed to fork pull
requests. Do not build that.

### Hygiene runs upstream, not here

`ci.yml` greps the spec for crate paths, account ids and cloud hostnames. The
private-name check cannot run there: the list names customers and internal
systems, so `.github/private-names.txt` is untracked and exists only in local
checkouts. `scripts/audit-public.sh` runs it over the spec and everything a
human writes, and must pass before a push; `scripts/sync-spec.sh` refuses to
run without the file. All of this is a **backstop, not the gate**: by the time
CI runs the content is already in a public repository and in its history, and
reverting a commit does not unpublish it.

The primary check belongs in the engine's release job, **before** the spec is
pushed. `scripts/sync-spec.sh` runs it before the copy for the same reason.

### The engine's release opens a pull request here

The last job of the engine's release workflow checks this repository out, runs
`scripts/sync-spec.sh` on the released spec, and opens a pull request named
`Engine X.Y.Z` from a branch `engine-vX.Y.Z`. The pull request holds:

- the spec;
- the three clients and the tool reference pages regenerated from it;
- every client version set to the engine's (`scripts/set_version.py`);
- a new `CHANGELOG.md` section (`scripts/changelog_entry.py`).

CI runs on it like on any other pull request, including the documentation
build. Merging it publishes the documentation site, which `docs.yml` builds
from `main`.

Two rules the job follows:

1. **A pull request, never a push to `main`.** A spec-only push leaves the three
   generated clients stale and turns `main` red on the drift check.
2. **Everything is regenerated on the engine's side.** A pull request opened by
   this repository's own `GITHUB_TOKEN` does not trigger `pull_request`
   workflows, so CI would never run on it.

The job authenticates as a **GitHub App** installed on this repository only,
with `contents: write` and `pull_requests: write`. The app's id and private
key live in the engine repository, which mints a short-lived installation
token per run. The app never needs read access to anything else.

The job also writes `.github/private-names.txt` from a secret in the engine
repository before it runs the script. The file is ignored, so it is never
committed here.

### Doing it by hand

With both repositories checked out, point the script at the spec the engine
generated:

```sh
scripts/sync-spec.sh ../<engine-checkout>/<generated>/graphsolve-v1.json
```

Export `GRAPHSOLVE_SPEC_SOURCE` to skip the argument. The path is not written
down here: this repository is public and the engine's is not.

That copies the spec in, regenerates all three clients and the tool reference
pages, sets the client versions, opens the changelog section and prints the
diff.
Review it: a removed tool, a newly required argument or a changed price is a
breaking change. Record what changed in the new `CHANGELOG.md` section. Then
run each language's gate and open a PR.

This needs no credentials in either direction, but it does need
`.github/private-names.txt` in the checkout: the script refuses to run without
it.

## Versions

**A client's version is the engine's.** Client `1.2.7` was generated from the
engine `1.2.7` spec, and the three clients are released together with the
engine. `scripts/check_versions.py` checks this in CI against the spec's
`info.version`, and checks that the Python package's `__version__` in
`python/src/graphsolve/__init__.py` matches its `pyproject.toml`.
`scripts/set_version.py` sets all of them from the spec.

A fix to a client alone waits for the next engine release, or the engine cuts
a patch release to carry it.

The spec's version is the engine's own release version — the same number the
engine's release workflow asserts against its git tag.

## The gates, in one place

| Gate | Where | Catches |
|---|---|---|
| `redocly lint` | `ci.yml` → `spec` | a spec that is not valid OpenAPI |
| internal-reference grep | engine, then `ci.yml` → `spec` | a private path or account id reaching the spec |
| private-name grep | `scripts/audit-public.sh`, local only | a customer, private repo, crate or host named anywhere |
| emitter drift | `ci.yml` → `generated` | a spec bump that only half-landed |
| version agreement | `ci.yml` → `versions` | clients claiming an API they were not built from |
| per-language gates | `ci.yml` | see CONTRIBUTING.md |
| tag/version match | each publish workflow | `rust-v1.1.0` tagged on a `1.0.0` crate |
