#!/usr/bin/env bash
#
# Bring in a new API spec from the engine and regenerate every client.
#
#     scripts/sync-spec.sh [path/to/graphsolve-v1.json]
#
# Takes the path to the engine's generated spec, or reads it from
# GRAPHSOLVE_SPEC_SOURCE. There is no baked-in default: this repository is
# public and does not name the engine's layout. Run by hand from a machine that
# has both repos; nothing here holds a credential that can read the engine. See
# MAINTAINING.md.
#
# Does not commit. Leaves the working tree for you to review the diff.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOURCE="${1:-${GRAPHSOLVE_SPEC_SOURCE:-}}"
DEST="$ROOT/spec/graphsolve-v1.json"

if [ -z "$SOURCE" ]; then
  echo "usage: scripts/sync-spec.sh <path-to-generated-spec>" >&2
  echo "   or: GRAPHSOLVE_SPEC_SOURCE=<path> scripts/sync-spec.sh" >&2
  exit 2
fi

if [ ! -f "$SOURCE" ]; then
  echo "No spec at $SOURCE" >&2
  exit 1
fi

command -v rustfmt >/dev/null || {
  echo "rustfmt not found; the Rust emitter needs it. rustup component add rustfmt" >&2
  exit 1
}

version() { python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['info']['version'])" "$1"; }

echo "  from  $SOURCE  (v$(version "$SOURCE"))"
echo "  into  spec/graphsolve-v1.json  (v$(version "$DEST" 2>/dev/null || echo "none"))"
echo

# Runs before the copy, so an internal reference never reaches the working tree.
# The primary check belongs in the engine, before anything is pushed here.
if grep -nE '[a-z_]+::[a-z_]+|[0-9]{12}|\.amazonaws\.com|chatbot|\bcrates?\b' "$SOURCE"; then
  echo "::error:: internal reference in the incoming spec — do not sync this" >&2
  exit 1
fi

# Names that may not be published, customers included, are listed in a file
# this repository does not track. Without it the sync is unsafe, so it is
# required: copy it in from the private master copy (see MAINTAINING.md).
NAMES="$ROOT/.github/private-names.txt"
if [ ! -f "$NAMES" ]; then
  echo "No $NAMES; it is untracked, copy it in before syncing" >&2
  exit 1
fi
private=$(grep -vE '^\s*(#|$)' "$NAMES" | paste -sd '|' -)
if grep -niE "$private" "$SOURCE"; then
  echo "::error:: a private name is in the incoming spec — do not sync this" >&2
  exit 1
fi

cp "$SOURCE" "$DEST"

python3 "$ROOT/emit/emit_python.py"
python3 "$ROOT/emit/emit_typescript.py"
python3 "$ROOT/emit/emit_rust.py"
python3 "$ROOT/emit/emit_docs.py"

echo
git -C "$ROOT" --no-pager diff --stat -- spec python typescript rust docs/reference/tools || true
git -C "$ROOT" status --short -- docs/reference/tools
echo
echo "Now:"
echo "  1. review the diff — a removed tool or a newly required argument is a"
echo "     breaking change"
echo "  2. bump the three client versions to match the spec's major.minor"
echo "  3. python scripts/check_versions.py"
echo "  4. run each language's gate (see CONTRIBUTING.md)"
