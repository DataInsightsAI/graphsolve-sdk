#!/usr/bin/env python3
"""Check every client carries the spec's version.

The clients are released with the engine and take its version number, so a
1.2.7 client was generated from the engine 1.2.7 spec.

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

    target = versions["spec"]
    wrong = {
        name: version
        for name, version in versions.items()
        if name != "spec" and version != target
    }

    for name, version in versions.items():
        print(f"  {name:<20} {version}")

    if wrong:
        print()
        print(f"::error::the spec is {target}, so every client must be {target}")
        for name, version in wrong.items():
            print(f"::error::  {name} claims {version}")
        print(
            "::error::Run scripts/set_version.py. See MAINTAINING.md."
        )
        return 1

    print(f"\nall clients are engine {target}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
