#!/usr/bin/env bash
#
# Everything that must not be in a public repository, checked in one go.
#
#     scripts/audit-public.sh
#
# Runs the structural checks CI runs, plus the one CI cannot: the private
# names in .github/private-names.txt, which is untracked because the list
# itself names customers. Covers untracked files too, because the first public
# commit will contain whatever is on disk.
#
# Exit status is the number of checks that found something.

set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

failures=0

files() {
  # Tracked and untracked, minus generated trees and lock files.
  { git ls-files; git ls-files --others --exclude-standard; } \
    | grep -vE '^(\.github/private-names\.txt|scripts/audit-public\.sh|.*\.lock|.*package-lock\.json)$' \
    | grep -vE '^(python/\.venv|typescript/node_modules|typescript/dist|rust/target)/'
}

check() {
  local title="$1" pattern="$2" scope="$3"
  echo "== $title"
  local hits
  if [ "$scope" = spec ]; then
    hits=$(grep -nE "$pattern" spec/graphsolve-v1.json | cut -c1-160)
  else
    hits=$(files | xargs grep -nEI "$pattern" 2>/dev/null | cut -c1-160)
  fi
  if [ -n "$hits" ]; then
    echo "$hits"
    failures=$((failures + 1))
  else
    echo "   clean"
  fi
}

check "spec: internal references" \
  '[a-z_]+::[a-z_]+|[0-9]{12}|\.amazonaws\.com|chatbot|\bcrates?\b' spec

NAMES=.github/private-names.txt
echo "== private names (.github/private-names.txt), spec and tree"
if [ ! -f "$NAMES" ]; then
  echo "   MISSING: $NAMES is untracked; copy it in from the private master copy"
  failures=$((failures + 1))
else
  private=$(grep -vE '^\s*(#|$)' "$NAMES" | paste -sd '|' -)
  hits=$( { grep -niE "$private" spec/graphsolve-v1.json; files | xargs grep -niEI "$private" 2>/dev/null; } | cut -c1-160)
  if [ -n "$hits" ]; then echo "$hits"; failures=$((failures + 1)); else echo "   clean"; fi
fi

check "tree: cloud and account identifiers" \
  '\b[0-9]{12}\b|\.amazonaws\.com|execute-api|arn:aws|AKIA[0-9A-Z]{12}' tree

check "tree: credentials" \
  'BEGIN (RSA|OPENSSH|EC) PRIVATE|aws_secret_access_key|gsk?_live_[0-9a-f]{8}' tree

check "tree: personal paths and addresses" \
  '/Users/[a-z]|/home/[a-z]|[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[a-z]{2,}' tree

echo
if [ "$failures" -eq 0 ]; then
  echo "clean"
else
  echo "$failures check(s) found something"
fi
exit "$failures"
