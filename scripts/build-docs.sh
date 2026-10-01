#!/usr/bin/env bash
#
# Build the documentation site into site/.
#
#     scripts/build-docs.sh            # build
#     scripts/build-docs.sh serve      # live preview at http://127.0.0.1:8000
#
# Needs the TypeScript dev dependencies (npm ci in typescript/) and the docs
# requirements (pip install -r docs/requirements.txt).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

python emit/emit_docs.py
(cd typescript && npm run --silent docs)

export DISABLE_MKDOCS_2_WARNING=true
if [ "${1:-build}" = "serve" ]; then
  mkdocs serve
else
  mkdocs build --strict
fi
