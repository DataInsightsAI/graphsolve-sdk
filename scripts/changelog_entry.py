#!/usr/bin/env python3
"""Open a CHANGELOG.md section for the spec's version.

Moves whatever is under `## Unreleased` into a new `## X.Y.Z — YYYY-MM-DD`
section and adds the tools the new spec gained or lost since the previous one.
The tool list is a starting point: changed arguments, prices and behaviour are
not detected, so the section still needs an edit before the release is tagged.

    python scripts/changelog_entry.py <previous-spec.json>

Does nothing when a section for the version already exists.
"""

from __future__ import annotations

import datetime
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
CHANGELOG = ROOT / "CHANGELOG.md"
UNRELEASED = "## Unreleased\n"


def tools(spec: dict) -> dict[str, str]:
    """Tool name -> one-line summary, for the concrete tool routes."""
    return {
        item["post"]["operationId"]: item["post"].get("summary", "").strip()
        for path, item in spec["paths"].items()
        if path.startswith("/v1/tools/") and "{" not in path and "post" in item
    }


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: scripts/changelog_entry.py <previous-spec.json>", file=sys.stderr)
        return 2
    before = tools(json.loads(pathlib.Path(sys.argv[1]).read_text()))
    spec = json.loads((ROOT / "spec" / "graphsolve-v1.json").read_text())
    after = tools(spec)
    version = spec["info"]["version"]

    text = CHANGELOG.read_text()
    if f"\n## {version} " in text:
        print(f"CHANGELOG.md already has a section for {version}")
        return 0
    if UNRELEASED not in text:
        print("CHANGELOG.md has no '## Unreleased' heading", file=sys.stderr)
        return 1

    head, rest = text.split(UNRELEASED, 1)
    cut = rest.find("\n## ")
    pending, older = (rest, "") if cut < 0 else (rest[:cut], rest[cut + 1 :])
    pending = pending.strip("\n")

    added = sorted(set(after) - set(before))
    removed = sorted(set(before) - set(after))
    lines = []
    if pending:
        lines += [pending, ""]
    if added:
        lines += ["### Added", ""]
        lines += [f"- `{t}`: {after[t]}" if after[t] else f"- `{t}`" for t in added]
        lines += [""]
    if removed:
        lines += ["### Breaking", ""]
        lines += [f"- `{t}` is removed." for t in removed]
        lines += [""]
    if not lines:
        lines += [f"Built from engine {version}. No tools added or removed.", ""]

    today = datetime.date.today().isoformat()
    section = f"## {version} — {today}\n\n" + "\n".join(lines)
    CHANGELOG.write_text(f"{head}{UNRELEASED}\n{section}\n{older}")
    print(f"CHANGELOG.md: {version}, {len(added)} tool(s) added, {len(removed)} removed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
