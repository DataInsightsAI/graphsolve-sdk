#!/usr/bin/env python3
"""Set every client's version to the spec's version.

The clients carry the engine's version number, so a release of the engine is a
release of all three. Rewrites the version in each package manifest, in the
lockfiles that repeat it, and in the Python package's `__version__`.

    python scripts/set_version.py            # the version in spec/graphsolve-v1.json
    python scripts/set_version.py 1.2.3      # an explicit version
"""

from __future__ import annotations

import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent

# Each entry: file, pattern with the version as group 2, how many matches to expect.
TARGETS = [
    ("python/pyproject.toml", r'(?m)^(version = ")([^"]*)(")$', 1),
    ("python/src/graphsolve/__init__.py", r'(?m)^(__version__ = ")([^"]*)(")$', 1),
    ("rust/Cargo.toml", r'(?m)^(version = ")([^"]*)(")$', 1),
    (
        "rust/Cargo.lock",
        r'(\[\[package\]\]\nname = "graphsolve"\nversion = ")([^"]*)(")',
        1,
    ),
    ("typescript/package.json", r'(?m)^(  "version": ")([^"]*)(",)$', 1),
    # The lockfile repeats the package's own version at the top and under
    # packages[""]. Both come before any dependency's entry.
    (
        "typescript/package-lock.json",
        r'(  "name": "@graphsolve/sdk",\n\s*"version": ")([^"]*)(")',
        2,
    ),
]


def main() -> int:
    if len(sys.argv) > 2:
        print("usage: scripts/set_version.py [version]", file=sys.stderr)
        return 2
    if len(sys.argv) == 2:
        version = sys.argv[1]
    else:
        spec = json.loads((ROOT / "spec" / "graphsolve-v1.json").read_text())
        version = spec["info"]["version"]
    if not re.fullmatch(r"\d+\.\d+\.\d+", version):
        print(f"not a major.minor.patch version: {version!r}", file=sys.stderr)
        return 1

    for name, pattern, expected in TARGETS:
        path = ROOT / name
        text, count = re.subn(pattern, rf"\g<1>{version}\g<3>", path.read_text())
        if count != expected:
            print(
                f"{name}: expected {expected} version line(s), found {count}",
                file=sys.stderr,
            )
            return 1
        path.write_text(text)
        print(f"  {name:<36} {version}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
