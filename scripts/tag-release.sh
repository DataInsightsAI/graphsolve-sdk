#!/usr/bin/env bash
#
# Tag the three clients at the committed version and push the tags, which
# starts the three publish workflows.
#
#     scripts/tag-release.sh
#
# Run on an up-to-date main after the release pull request is merged. Each
# publish still waits for its environment's reviewer.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

python3 scripts/check_versions.py >/dev/null
VERSION="$(python3 -c "import json; print(json.load(open('spec/graphsolve-v1.json'))['info']['version'])")"

git fetch --quiet origin main
if [ "$(git rev-parse HEAD)" != "$(git rev-parse origin/main)" ]; then
  echo "HEAD is not origin/main; check out an up-to-date main first" >&2
  exit 1
fi
if [ -n "$(git status --porcelain)" ]; then
  echo "the working tree has changes; tag a clean main" >&2
  exit 1
fi
if ! grep -q "^## ${VERSION} " CHANGELOG.md; then
  echo "CHANGELOG.md has no section for ${VERSION}" >&2
  exit 1
fi

TAGS=("python-v${VERSION}" "ts-v${VERSION}" "rust-v${VERSION}")
for tag in "${TAGS[@]}"; do
  git tag "$tag"
done
git push origin "${TAGS[@]}"
echo "pushed ${TAGS[*]}"
