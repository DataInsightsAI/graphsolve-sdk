#!/usr/bin/env python3
"""Check every client claims the same major.minor as the spec.

A client's major.minor tracks the engine API it speaks; the patch is the
client's own. So any 1.2.x client works against engine 1.2.

Also checks the Python package's `__version__` matches its pyproject.toml,
since the literal is easy to miss in a bump.

    python scripts/check_versions.py
"""

from __future__ import annotations

import json
import pathlib
import re
import sys
import tomllib

ROOT = pathlib.Path(__file__).resolve().parent.parent


def minor(version: str) -> str:
    """`1.2.7` -> `1.2`."""
    parts = version.split(".")
    if len(parts) < 2:
        raise SystemExit(f"not a version: {version!r}")
    return ".".join(parts[:2])


def read_versions() -> dict[str, str]:
    spec = json.loads((ROOT / "spec" / "graphsolve-v1.json").read_text())
    pyproject = tomllib.loads((ROOT / "python" / "pyproject.toml").read_text())
    package = json.loads((ROOT / "typescript" / "package.json").read_text())
    cargo = tomllib.loads((ROOT / "rust" / "Cargo.toml").read_text())
    init = (ROOT / "python" / "src" / "graphsolve" / "__init__.py").read_text()
    dunder = re.search(r'^__version__ = "([^"]*)"$', init, re.MULTILINE)
    return {
        "spec": spec["info"]["version"],
        "python": pyproject["project"]["version"],
        "python __version__": dunder.group(1) if dunder else "missing",
        "typescript": package["version"],
        "rust": cargo["package"]["version"],
    }


def main() -> int:
    versions = read_versions()
    for name, version in versions.items():
        if not re.fullmatch(r"\d+\.\d+\.\d+", version):
            print(f"::error::{name} version {version!r} is not major.minor.patch")
            return 1

    if versions["python __version__"] != versions["python"]:
        print(
            f"::error::graphsolve.__version__ is {versions['python __version__']!r} "
            f"but pyproject.toml says {versions['python']!r}. Set both."
        )
        return 1

    target = minor(versions["spec"])
    wrong = {
        name: version
        for name, version in versions.items()
        if name != "spec" and minor(version) != target
    }

    for name, version in versions.items():
        print(f"  {name:<20} {version}")

    if wrong:
        print()
        print(f"::error::the spec speaks {target}, so every client must be {target}.x")
        for name, version in wrong.items():
            print(f"::error::  {name} claims {version}")
        print(
            "::error::Bump the client versions to match. See MAINTAINING.md."
        )
        return 1

    print(f"\nall clients speak engine {target}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
