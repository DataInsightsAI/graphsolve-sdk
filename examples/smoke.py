#!/usr/bin/env python3
"""Prove an API key works end to end against a live deployment.

    pip install graphsolve
    export GRAPHSOLVE_API_KEY=...
    python examples/smoke.py            # free calls only
    python examples/smoke.py --solve    # also run one billed tool (1 credit)

Exercises the whole path a real integration takes: the key is exchanged for an
access token, the token is presented to the engine, and the engine answers.
Each step prints what it saw, so a wrong host, a key without the scope a tool
needs, or a rejected key is visible at the step that failed rather than as a
stack trace.

The key is read from the environment and never printed. The hosts are printed,
because pointing a key at the wrong deployment is the most common mistake:
tokens are issued centrally, so `GRAPHSOLVE_TOKEN_URL` stays at its default
for every production deployment, and only `GRAPHSOLVE_BASE_URL` changes.
"""

from __future__ import annotations

import argparse
import os
import sys

from graphsolve import (
    DEFAULT_BASE_URL,
    DEFAULT_TOKEN_URL,
    GraphSolve,
    GraphSolveError,
    ToolNotPermitted,
)

# 1 MPa in psi, to four decimals. Wider tolerance than the engine needs, so a
# change in its rounding does not fail a smoke test.
PSI_PER_MPA = 145.0377
TOLERANCE = 1e-3


def step(name: str) -> None:
    print(f"\n== {name}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument(
        "--solve",
        action="store_true",
        help="also run calculate_pressure_drop, which costs 1 credit",
    )
    args = parser.parse_args()

    base_url = os.environ.get("GRAPHSOLVE_BASE_URL") or DEFAULT_BASE_URL
    token_url = os.environ.get("GRAPHSOLVE_TOKEN_URL") or DEFAULT_TOKEN_URL
    print(f"engine     {base_url}")
    print(f"token from {token_url}")

    try:
        gs = GraphSolve()
    except GraphSolveError as exc:
        print(f"FAIL  {exc}")
        return 1

    try:
        step("exchange the API key for a token")
        token = gs.get_access_token()
        print(f"ok    token of {len(token)} characters")

        step("GET /v1/me")
        me = gs.me()
        scopes = me.get("scopes") or []
        print(f"ok    user {me.get('user_id')}  tenant {me.get('tenant_id')}  key {me.get('key_id')}")
        print(f"      scopes {scopes}  billable {me.get('billable')}")

        step("GET /v1/tools")
        tools = gs.tools()
        print(f"ok    {len(tools)} tools")

        step("convert_units, free")
        response = gs.convert_units(value=1.0, from_unit="MPa", to_unit="psi")
        value = float(response["result"]["converted_value"])
        charged = response["metadata"]["billing"]["credits_charged"]
        if abs(value - PSI_PER_MPA) > TOLERANCE:
            print(f"FAIL  1 MPa converted to {value} psi, expected {PSI_PER_MPA}")
            return 1
        print(f"ok    1 MPa = {value:.4f} psi, charged {charged}")

        if not args.solve:
            print("\nskipped the billed call; pass --solve to run calculate_pressure_drop")
            return 0

        step("calculate_pressure_drop, billed")
        if "engine:solve" not in scopes:
            print("FAIL  this key has no engine:solve scope, so it cannot run a calculation")
            return 1
        response = gs.calculate_pressure_drop(
            pressure=10.0,
            density=[80.0, 800.0, 1000.0],
            viscosity=[1.5e-5, 1.0e-3, 5.0e-4],
            ift=0.02,
            diameter=0.15,
            roughness=4.5e-5,
            velocity=[3.0, 1.0, 0.2],
            angle=90.0,
            correlation="Beggs-Brill",
        )
        charged = response["metadata"]["billing"]["credits_charged"]
        keys = sorted(response["result"])
        print(f"ok    charged {charged}, result fields {keys}")
    except ToolNotPermitted as exc:
        print(f"FAIL  {exc}")
        print("      the key exists but lacks the scope this step needs")
        return 1
    except GraphSolveError as exc:
        print(f"FAIL  {type(exc).__name__}: {exc}")
        return 1

    print("\nall steps passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
