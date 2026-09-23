#!/usr/bin/env python3
"""Emit the typed method surface for the Python client from the OpenAPI spec.

    python emit/emit_python.py

Writes python/src/graphsolve/_generated.py. CI regenerates and fails if the
committed file differs.

Every operation has the same shape: POST /v1/tools/{name}, one JSON body, one
envelope back. That regularity is why this is a small purpose-built emitter
rather than a general-purpose generator — it produces one named method per tool
with typed keyword arguments instead of a generic call_tool(name, body).
"""

from __future__ import annotations

import json
import textwrap
from typing import Any

from _spec import ROOT, body_schema, credits_line, enum_values, load, resolve

OUT = ROOT / "python" / "src" / "graphsolve" / "_generated.py"

SCALARS = {
    "number": "float",
    "integer": "int",
    "string": "str",
    "boolean": "bool",
    "object": "dict[str, Any]",
}


def py_type(schema: dict[str, Any], comps: dict[str, Any]) -> tuple[str, bool]:
    """Map a JSON Schema node to a Python annotation.

    Returns (annotation, optional). `optional` means the schema admits null.
    Whether the field is in `required` is decided separately by the caller.
    """
    if not schema:
        return "Any", False

    if "$ref" in schema:
        target = resolve(schema, comps)
        # Fixed vocabularies become a Literal so the editor offers the valid
        # names instead of accepting any string.
        values = enum_values(target)
        if values:
            return "Literal[" + ", ".join(json.dumps(v) for v in values) + "]", False
        return "dict[str, Any]", False

    if "anyOf" in schema:
        branches = [b for b in schema["anyOf"] if b.get("type") != "null"]
        nullable = len(branches) != len(schema["anyOf"])
        if not branches:
            return "Any", True
        if len(branches) == 1:
            inner, _ = py_type(branches[0], comps)
            return inner, nullable
        parts, seen = [], set()
        for b in branches:
            t, _ = py_type(b, comps)
            if t not in seen:
                seen.add(t)
                parts.append(t)
        return (" | ".join(parts) if len(parts) > 1 else parts[0]), nullable

    values = enum_values(schema)
    if values:
        return "Literal[" + ", ".join(json.dumps(v) for v in values) + "]", False

    kind = schema.get("type")
    if isinstance(kind, list):
        nullable = "null" in kind
        real = [k for k in kind if k != "null"]
        if not real:
            return "Any", True
        inner, _ = py_type({**schema, "type": real[0]}, comps)
        return inner, nullable

    if kind == "array":
        item, _ = py_type(schema.get("items") or {}, comps)
        return f"list[{item}]", False
    if kind in SCALARS:
        return SCALARS[kind], False
    return "Any", False


def emit_method(tool: str, op: dict[str, Any], comps: dict[str, Any]) -> str:
    schema = body_schema(op)
    props: dict[str, Any] = schema.get("properties") or {}
    required = set(schema.get("required") or [])

    # Required first: a parameter with a default cannot precede one without.
    names = sorted(props, key=lambda n: (n not in required, n))

    params, forwarded = [], []
    for name in names:
        annotation, nullable = py_type(props[name], comps)
        if name in required and not nullable:
            params.append(f"        {name}: {annotation},")
        else:
            params.append(f"        {name}: {annotation} | None = None,")
        forwarded.append(f"                {name}={name},")

    # Summary on the opening line, PEP 257 style.
    summary = textwrap.wrap(op["summary"].strip(), width=74) or [""]
    doc = ['        """' + summary[0]]
    doc += ["        " + line for line in summary[1:]]
    doc += ["", "        " + credits_line(op["x-graphsolve-credits"]), '        """']

    body = "\n".join(
        ['    def ' + tool + "(", "        self,"]
        + (["        *,"] if params else [])
        + params
        + ["    ) -> dict[str, Any]:"]
        + doc
        + ["        return self.call("]
        + [f'            "{tool}",']
        + (["            _present("] + forwarded + ["            ),"] if forwarded else ["            {},"])
        + ["        )"]
    )
    return body


def main() -> None:
    ops, comps, version = load()

    header = textwrap.dedent(f'''\
        """Typed methods for every tool in the GraphSolve API.

        GENERATED FROM spec/graphsolve-v1.json — DO NOT EDIT BY HAND.
        Run `python emit/emit_python.py` after a spec change; CI fails if this
        file and the spec disagree.

        Engine API version: {version}
        Tools: {len(ops)}
        """

        from __future__ import annotations

        from typing import Any, Literal


        def _present(**kwargs: Any) -> dict[str, Any]:
            """Drop unset arguments.

            An omitted optional argument must be absent from the payload
            rather than null. Several tools treat "not given" and null
            differently.
            """
            return {{k: v for k, v in kwargs.items() if v is not None}}


        class GeneratedMethods:
            """One method per tool. Mixed into :class:`~graphsolve.GraphSolve`.

            Each method wraps ``call()``, which handles retries, idempotency
            and error mapping. Use ``call()`` directly for a tool newer than
            this file.
            """

        ''')

    methods = "\n\n".join(emit_method(name, ops[name], comps) for name in sorted(ops))
    OUT.write_text(header + methods + "\n")
    print(f"wrote {OUT.relative_to(ROOT)} — {len(ops)} methods")


if __name__ == "__main__":
    main()
