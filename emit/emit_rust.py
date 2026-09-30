#!/usr/bin/env python3
"""Emit the typed method surface for the Rust client from the OpenAPI spec.

    python emit/emit_rust.py

Writes rust/src/generated.rs. CI regenerates and fails if the committed file
differs, so the client cannot silently fall behind the spec.

Design notes:

* One params struct per tool, so the struct literal names every field at the
  call site. A new(a, b, c, ...) constructor would be shorter but error-prone:
  calculate_pressure_drop takes nine required f64 and Vec<f64> arguments.
* Required fields are plain; optional fields are Option<T> and are skipped when
  None, so an omitted optional is absent from the payload rather than null.
* Default is derived so ..Default::default() covers the optional fields.
"""

from __future__ import annotations

import re
import shutil
import subprocess
import textwrap
from typing import Any

from _spec import ROOT, body_schema, credits_line, enum_values, load, resolve

OUT = ROOT / "rust" / "src" / "generated.rs"

SCALARS = {
    "number": "f64",
    "integer": "i64",
    "string": "String",
    "boolean": "bool",
    "object": "serde_json::Value",
}

#: Types that impl Default, so a params struct using them can derive it.
#: Emitted enums are added here when they get a default variant.
DEFAULTABLE = {"f64", "i64", "String", "bool", "serde_json::Value"}


def rust_type(
    schema: dict[str, Any], comps: dict[str, Any], used: set[str] | None = None
) -> tuple[str, bool]:
    """Map a JSON Schema node to a Rust type.

    Returns (type, nullable). `nullable` means the schema admits null. Both
    nullable and non-required fields become Option<T>.
    """
    if not schema:
        return "serde_json::Value", False

    if "$ref" in schema:
        name = schema["$ref"].rsplit("/", 1)[-1]
        target = resolve(schema, comps)
        # Fixed vocabularies become real enums, so a wrong correlation name is
        # a compile error rather than a 400 from the server.
        if enum_values(target):
            if used is not None:
                used.add(name)
            return name, False
        return "serde_json::Value", False

    if "anyOf" in schema:
        branches = [b for b in schema["anyOf"] if b.get("type") != "null"]
        nullable = len(branches) != len(schema["anyOf"])
        if not branches:
            return "serde_json::Value", True
        if len(branches) == 1:
            inner, _ = rust_type(branches[0], comps, used)
            return inner, nullable
        # The *_json blobs are object-or-array-or-string. Anything admitting
        # more than one shape becomes Value.
        for b in branches:
            rust_type(b, comps, used)
        return "serde_json::Value", nullable

    kind = schema.get("type")
    if isinstance(kind, list):
        nullable = "null" in kind
        real = [k for k in kind if k != "null"]
        if not real:
            return "serde_json::Value", True
        inner, _ = rust_type({**schema, "type": real[0]}, comps, used)
        return inner, nullable

    if kind == "array":
        item, _ = rust_type(schema.get("items") or {}, comps, used)
        return f"Vec<{item}>", False
    if kind in SCALARS:
        return SCALARS[kind], False
    return "serde_json::Value", False


#: Line starts rustdoc reads as a markdown list item.
LIST_MARKER = re.compile(r"^([-*+>]\s|\d+[.)]\s)")


def wrap_doc(text: str, width: int = 76) -> list[str]:
    """Wrap prose for a `///` comment without inventing a markdown list.

    Wrapping "(Tier 1) or the Won model" can leave a line starting "1) ", which
    rustdoc reads as an ordered list item and then warns about the unindented
    continuation. Pulling down the previous line's last word avoids it.
    """
    # rustdoc reads [MPa] and [gas, oil, water] as intra-doc links and warns
    # they resolve to nothing. The spec uses that notation for units and vector
    # layouts throughout, so escape the brackets.
    escaped = text.strip().replace("[", "\\[").replace("]", "\\]")
    lines = textwrap.wrap(escaped, width=width)
    for i in range(1, len(lines)):
        if LIST_MARKER.match(lines[i]) and " " in lines[i - 1]:
            head, _, last = lines[i - 1].rpartition(" ")
            lines[i - 1] = head
            lines[i] = f"{last} {lines[i]}"
    return lines


def pascal(name: str) -> str:
    """Turn a wire value into a Rust identifier: beggs-brill -> BeggsBrill."""
    parts = []
    for part in name.replace("-", "_").split("_"):
        if not part:
            continue
        if part.isupper():
            parts.append(part.capitalize())
        elif any(c.isupper() for c in part) and any(c.islower() for c in part):
            parts.append(part)  # already PascalCase or camelCase — leave it
        else:
            parts.append(part[:1].upper() + part[1:])
    return "".join(parts)


def default_variant(schema: dict[str, Any]) -> str | None:
    """The variant the spec itself calls the default, or None.

    Only read from the spec, never chosen here: picking a default correlation
    or model would be a physics decision. Two forms carry it — a value named
    "Default", or a variant whose description ends in "Default.". If neither is
    present the enum gets no Default impl.
    """
    values = enum_values(schema) or []
    for value in values:
        if value == "Default":
            return value
    for branch in schema.get("oneOf") or []:
        if branch.get("description", "").rstrip().endswith("Default."):
            return branch["const"]
    return None


def emit_enum(name: str, schema: dict[str, Any]) -> str:
    values = enum_values(schema) or []
    default = default_variant(schema)
    doc = (schema.get("description") or name).strip().splitlines()
    lines = [f"/// {line}" for line in doc]
    lines.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]")
    if default is not None:
        lines[-1] = lines[-1].replace("Serialize", "Default, Serialize")
        DEFAULTABLE.add(name)
    lines.append(f"pub enum {name} {{")
    for value in values:
        variant = pascal(value)
        for branch in schema.get("oneOf") or []:
            if branch.get("const") == value and branch.get("description"):
                for line in branch["description"].strip().splitlines():
                    lines.append(f"    /// {line.strip()}")
        if value == default:
            lines.append("    #[default]")
        lines.append(f'    #[serde(rename = "{value}")]')
        lines.append(f"    {variant},")
    lines.append("}")
    return "\n".join(lines)


def emit_params(
    tool: str, op: dict[str, Any], comps: dict[str, Any], used: set[str]
) -> tuple[str, bool] | None:
    """The params struct for one tool, and whether it derives `Default`."""
    schema = body_schema(op)
    props: dict[str, Any] = schema.get("properties") or {}
    if not props:
        return None
    required = set(schema.get("required") or [])

    fields, defaultable = [], True
    for name in sorted(props, key=lambda n: (n not in required, n)):
        ty, nullable = rust_type(props[name], comps, used)
        description = (props[name].get("description") or "").strip()
        for line in wrap_doc(description) if description else []:
            fields.append(f"    /// {line}")
        if name in required and not nullable:
            fields.append(f"    pub {name}: {ty},")
            base = ty[4:-1] if ty.startswith("Vec<") else ty
            defaultable &= ty.startswith("Vec<") or base in DEFAULTABLE
        else:
            fields.append('    #[serde(skip_serializing_if = "Option::is_none")]')
            fields.append(f"    pub {name}: Option<{ty}>,")

    derive = "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]"
    if defaultable:
        derive = derive.replace("Clone,", "Clone, Default,")
    body = "\n".join(
        [
            f"/// Arguments for [`GraphSolve::{tool}`](crate::GraphSolve::{tool}).",
            derive,
            f"pub struct {pascal(tool)}Params {{",
            *fields,
            "}",
        ]
    )
    return body, defaultable


def emit_method(tool: str, op: dict[str, Any], has_params: bool) -> str:
    doc = [f"    /// {line}" for line in wrap_doc(op["summary"], width=72)]
    doc += ["    ///", f"    /// {credits_line(op['x-graphsolve-credits'])}"]
    # Required by clippy::missing_errors_doc. The charged-versus-free split is
    # the only thing a generated comment can say without knowing the tool.
    doc += [
        "    ///",
        "    /// # Errors",
        "    ///",
        "    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)",
        "    /// when the calculation ran and produced no answer, which is charged.",
        "    /// Every other variant means nothing was computed and nothing was billed.",
    ]
    if has_params:
        return "\n".join(
            doc
            + [
                f"    pub async fn {tool}(",
                "        &self,",
                f"        params: {pascal(tool)}Params,",
                "    ) -> Result<ToolResponse> {",
                f'        self.call("{tool}", &params).await',
                "    }",
            ]
        )
    return "\n".join(
        doc
        + [
            f"    pub async fn {tool}(&self) -> Result<ToolResponse> {{",
            f'        self.call("{tool}", &serde_json::json!({{}})).await',
            "    }",
        ]
    )


def main() -> None:
    ops, comps, version = load()
    names = sorted(ops)

    # Two passes. The first finds which enums a request body can reach.
    # Emitting them sets which have a Default, which the structs then need.
    used: set[str] = set()
    for name in names:
        emit_params(name, ops[name], comps, used)
    enums = "\n\n".join(emit_enum(name, comps[name]) for name in sorted(used))
    structs = {name: emit_params(name, ops[name], comps, set()) for name in names}

    header = textwrap.dedent(f"""\
        //! Typed methods for every tool in the GraphSolve API.
        //!
        //! GENERATED FROM spec/graphsolve-v1.json — DO NOT EDIT BY HAND.
        //! Run `python emit/emit_rust.py` after a spec change; CI fails if this
        //! file and the spec disagree.
        //!
        //! Engine API version: {version}
        //! Tools: {len(ops)}

        use serde::{{Deserialize, Serialize}};

        use crate::{{GraphSolve, Result, ToolResponse}};

        /// Every tool this client knows about, in the order the API lists them.
        pub const TOOL_NAMES: [&str; {len(ops)}] = [
        """)
    header += "\n".join(f'    "{name}",' for name in names) + "\n];\n"

    param_types = "\n\n".join(s[0] for s in structs.values() if s)
    methods = "\n\n".join(emit_method(name, ops[name], structs[name] is not None) for name in names)

    OUT.write_text(
        f"{header}\n{enums}\n\n{param_types}\n\nimpl GraphSolve {{\n{methods}\n}}\n"
    )
    rustfmt()
    print(f"wrote {OUT.relative_to(ROOT)} — {len(ops)} methods")


def rustfmt() -> None:
    """Format the emitted file with rustfmt.

    Both `cargo fmt --check` and the CI drift check run over this file, so the
    committed output has to match what rustfmt produces. Reproducing rustfmt's
    chain_width rules here would be fragile.
    """
    if not shutil.which("rustfmt"):
        raise SystemExit(
            "rustfmt not found. It formats the emitted file, and without it the "
            "committed output will not match `cargo fmt --check`. Install it with "
            "`rustup component add rustfmt`."
        )
    subprocess.run(["rustfmt", "--edition", "2021", str(OUT)], check=True)


if __name__ == "__main__":
    main()
