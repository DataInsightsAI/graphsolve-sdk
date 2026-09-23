"""Shared spec reading for the emitters.

Holds the parts that must be identical across languages: which operations exist,
how a $ref resolves, what counts as a fixed vocabulary, and the wording of the
price line. Type mapping is per-language and lives in each emitter.
"""

from __future__ import annotations

import json
import pathlib
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parent.parent
SPEC = ROOT / "spec" / "graphsolve-v1.json"


def load() -> tuple[dict[str, Any], dict[str, Any], str]:
    """Return (operations by tool name, component schemas, engine version)."""
    spec = json.loads(SPEC.read_text())
    # Concrete tool paths only. The spec also documents templated routes such
    # as /v1/tools/{tool_name}, which are not tools and must not become methods.
    ops = {
        item["post"]["operationId"]: item["post"]
        for path, item in spec["paths"].items()
        if path.startswith("/v1/tools/") and "{" not in path and "post" in item
    }
    return ops, spec["components"]["schemas"], spec["info"]["version"]


def resolve(schema: dict[str, Any], comps: dict[str, Any]) -> dict[str, Any]:
    """Follow a local $ref one hop. The spec hoists every $defs into components."""
    ref = schema.get("$ref")
    if not ref:
        return schema
    return comps.get(ref.rsplit("/", 1)[-1], {})


def enum_values(schema: dict[str, Any]) -> list[Any] | None:
    """Permitted values of a fixed-vocabulary schema, or None if it is not one.

    The schema generator writes a plain `enum` for a bare variant list, but a
    `oneOf` of `const`s once the variants have doc comments. Both forms mean the
    same thing. Handling only the first types the `model` field on corrosion,
    hydrate and wax screening as a free-form object.
    """
    if "enum" in schema:
        return schema["enum"]
    branches = schema.get("oneOf")
    if branches and all("const" in b for b in branches):
        return [b["const"] for b in branches]
    return None


def body_schema(op: dict[str, Any]) -> dict[str, Any]:
    return op["requestBody"]["content"]["application/json"]["schema"]


def credits_line(cost: dict[str, Any]) -> str:
    """The tool's price as one sentence, worded the same in every client."""
    base, free, rate = cost["base_credits"], cost["free_ms"], cost["ms_per_credit"]
    if base == 0:
        return "Free."
    unit = "credit" if base == 1 else "credits"
    if rate == 0:
        return f"Costs {base} {unit}."
    return (
        f"Costs {base} {unit}, plus 1 per {rate} ms beyond the first "
        f"{free / 1000:g} s of compute."
    )
