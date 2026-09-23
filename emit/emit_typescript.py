#!/usr/bin/env python3
"""Emit the typed method surface for the TypeScript client from the OpenAPI spec.

    python emit/emit_typescript.py

Writes typescript/src/generated.ts. CI regenerates and fails if the committed
file differs, so the client cannot silently fall behind the spec.

Two naming decisions:

* Methods and parameters stay snake_case, matching the wire format. Renaming to
  camelCase would need a translation layer on every call. Options the client
  itself defines (baseUrl, maxRetries) are camelCase.
* Parameters are emitted as `type`, not `interface`. A type alias gets an
  implicit index signature so it is assignable to Record<string, unknown>; an
  interface is not, and every call site would need a cast.

Named enums in the spec become named exported unions rather than being inlined,
so a caller can import FlowCorrelationName and hold one in a variable.
"""

from __future__ import annotations

import json
import textwrap
from typing import Any

from _spec import ROOT, body_schema, credits_line, enum_values, load, resolve

OUT = ROOT / "typescript" / "src" / "generated.ts"

#: Type names that types.ts already declares. Re-declaring one here would
#: collide on the `export *` in index.ts.
RESERVED = {"FailureKind", "ToolResponse", "ToolArguments", "ToolMetadata"}

SCALARS = {
    "number": "number",
    "integer": "number",
    "string": "string",
    "boolean": "boolean",
    "object": "Record<string, unknown>",
}


def ts_type(
    schema: dict[str, Any], comps: dict[str, Any], used: set[str] | None = None
) -> tuple[str, bool]:
    """Map a JSON Schema node to a TypeScript annotation.

    Returns (annotation, nullable). `nullable` means the schema admits null.
    Optionality is decided separately by the caller.

    Named enums seen along the way are recorded in `used`, so only enums a
    request body can reach get declared.
    """
    if not schema:
        return "unknown", False

    if "$ref" in schema:
        name = schema["$ref"].rsplit("/", 1)[-1]
        target = resolve(schema, comps)
        # Fixed vocabularies become a named string union so the editor offers
        # the valid names, and the type can be imported and reused.
        values = enum_values(target)
        if values and name not in RESERVED:
            if used is not None:
                used.add(name)
            return name, False
        if values:
            return union(values), False
        return "Record<string, unknown>", False

    if "anyOf" in schema:
        branches = [b for b in schema["anyOf"] if b.get("type") != "null"]
        nullable = len(branches) != len(schema["anyOf"])
        if not branches:
            return "unknown", True
        parts, seen = [], set()
        for b in branches:
            t, _ = ts_type(b, comps, used)
            if t not in seen:
                seen.add(t)
                parts.append(t)
        return " | ".join(parts), nullable

    values = enum_values(schema)
    if values:
        return union(values), False

    kind = schema.get("type")
    if isinstance(kind, list):
        nullable = "null" in kind
        real = [k for k in kind if k != "null"]
        if not real:
            return "unknown", True
        inner, _ = ts_type({**schema, "type": real[0]}, comps, used)
        return inner, nullable

    if kind == "array":
        item, _ = ts_type(schema.get("items") or {}, comps, used)
        # `A | B[]` would parse as `A | (B[])`, so unions use Array<...>.
        return f"Array<{item}>" if "|" in item else f"{item}[]", False
    if kind in SCALARS:
        return SCALARS[kind], False
    return "unknown", False


def union(values: list[Any]) -> str:
    return " | ".join(json.dumps(v) for v in values)


def pascal(tool: str) -> str:
    return "".join(part.capitalize() for part in tool.split("_"))


def jsdoc(op: dict[str, Any], indent: str) -> list[str]:
    summary = textwrap.wrap(op["summary"].strip(), width=72) or [""]
    lines = [f"{indent}/**"]
    lines += [f"{indent} * {line}" for line in summary]
    lines += [f"{indent} *", f"{indent} * {credits_line(op['x-graphsolve-credits'])}"]
    lines += [f"{indent} */"]
    return lines


def emit_params(
    tool: str, op: dict[str, Any], comps: dict[str, Any], used: set[str]
) -> str | None:
    """The parameter type for one tool, or None when the tool takes nothing."""
    schema = body_schema(op)
    props: dict[str, Any] = schema.get("properties") or {}
    if not props:
        return None
    required = set(schema.get("required") or [])

    lines = [f"export type {pascal(tool)}Params = {{"]
    for name in sorted(props, key=lambda n: (n not in required, n)):
        annotation, nullable = ts_type(props[name], comps, used)
        if name in required and not nullable:
            lines.append(f"  {name}: {annotation};")
        else:
            # An omitted property is absent from the JSON body. An explicit
            # null is sent through, which is what nullable fields are for.
            suffix = " | null" if nullable else ""
            lines.append(f"  {name}?: {annotation}{suffix};")
    lines.append("};")
    return "\n".join(lines)


def emit_method(tool: str, op: dict[str, Any], has_params: bool) -> str:
    doc = "\n".join(jsdoc(op, "  "))
    if has_params:
        signature = f"  {tool}(params: {pascal(tool)}Params): Promise<ToolResponse> {{"
        if len(signature) > 80:
            signature = (
                f"  {tool}(\n"
                f"    params: {pascal(tool)}Params,\n"
                f"  ): Promise<ToolResponse> {{"
            )
        return (
            f"{doc}\n"
            f"{signature}\n"
            f'    return this.call("{tool}", params);\n'
            f"  }}"
        )
    return (
        f"{doc}\n"
        f"  {tool}(): Promise<ToolResponse> {{\n"
        f'    return this.call("{tool}", {{}});\n'
        f"  }}"
    )


def main() -> None:
    ops, comps, version = load()
    names = sorted(ops)

    header = textwrap.dedent(f"""\
        /**
         * Typed methods for every tool in the GraphSolve API.
         *
         * GENERATED FROM spec/graphsolve-v1.json — DO NOT EDIT BY HAND.
         * Run `python emit/emit_typescript.py` after a spec change; CI fails if
         * this file and the spec disagree.
         *
         * Engine API version: {version}
         * Tools: {len(ops)}
         */

        import type {{ ToolArguments, ToolResponse }} from "./types.js";

        /** Every tool this client knows about. */
        export type ToolName =
        """)
    header += "\n".join(f'  | "{name}"' for name in names) + ";\n"

    used: set[str] = set()
    params = [emit_params(name, ops[name], comps, used) for name in names]
    param_types = "\n\n".join(p for p in params if p)

    # Only the enums a request body can actually reach; the response-side ones
    # are hand-written in types.ts.
    enums = "\n\n".join(
        f"export type {name} =\n"
        + "\n".join(f"  | {json.dumps(v)}" for v in enum_values(comps[name]) or [])
        + ";"
        for name in sorted(used)
    )

    klass = textwrap.dedent('''\
        /**
         * One method per tool. Extended by {@link GraphSolve}.
         *
         * Each method wraps `call()`, which handles retries, idempotency and
         * error mapping. Use `call()` directly for a tool newer than this file.
         */
        export abstract class GeneratedMethods {
          abstract call<T = unknown>(
            tool: string,
            args?: ToolArguments,
          ): Promise<ToolResponse<T>>;
        ''')

    methods = "\n\n".join(
        emit_method(name, ops[name], params[i] is not None) for i, name in enumerate(names)
    )

    OUT.write_text(f"{header}\n{enums}\n\n{param_types}\n\n{klass}\n{methods}\n}}\n")
    print(f"wrote {OUT.relative_to(ROOT)} — {len(ops)} methods")


if __name__ == "__main__":
    main()
