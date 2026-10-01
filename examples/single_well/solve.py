#!/usr/bin/env python3
"""Solve a single oil well, then size its production choke.

    pip install graphsolve
    export GRAPHSOLVE_API_KEY=...
    python solve.py            # one solve, 25 credits
    python solve.py --sweep    # plus four choke sizes, 100 credits more

W-1 is a synthetic well: a reservoir with a straight-line productivity index,
2500 m of vertical tubing, a production choke, a 3 km flowline and a separator
held at 2 MPa. Its rate is not an input. The well delivers whatever its inflow
gives at the bottomhole pressure the network settles at.

Units are SI throughout: pressure MPa, temperature K, rates kSm3/day at
standard conditions, lengths and diameters m.
"""

from __future__ import annotations

import argparse
import copy
import json
from pathlib import Path

from graphsolve import GraphSolve

NETWORK = json.loads((Path(__file__).parent / "network.json").read_text())


def results_by_id(response: dict) -> dict[str, dict]:
    """The `results` block of every node and edge, keyed by element id."""
    return {e["data"]["id"]: e["data"].get("results", {}) for e in response["result"]["elements"]}


def with_choke(network: dict, diameter_m: float) -> dict:
    changed = copy.deepcopy(network)
    for element in changed["elements"]:
        if element["data"]["id"] == "choke":
            element["data"]["choke_data"]["choke_diameter"] = diameter_m
    return changed


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--sweep", action="store_true", help="also solve four choke sizes")
    args = parser.parse_args()

    gs = GraphSolve()

    # Free: checks the model builds before anything is charged.
    check = gs.validate_solver_payload(network_json=NETWORK)["result"]
    if not check["valid"]:
        raise SystemExit(f"model is not valid: {check['errors']}")

    response = gs.solve_network(network_json=NETWORK)
    r = results_by_id(response)
    print(f"credits charged: {response['metadata']['billing']['credits_charged']}\n")

    print(f"{'node':<14}{'P [MPa]':>9}{'T [degC]':>10}")
    for node in ["bottomhole", "wellhead", "choke-outlet", "separator"]:
        p = r[node]["pressure"]["outlet"]
        t = r[node]["temperature"]["outlet"] - 273.15
        print(f"{node:<14}{p:>9.3f}{t:>10.1f}")

    rates = r["inflow"]["mass_flow"]
    print(
        f"\noil {rates['oil_phase_rate']:.3f}, water {rates['water_phase_rate']:.3f}, "
        f"gas {rates['gas_phase_rate']:.1f} kSm3/day"
    )
    print(f"choke critical: {r['choke']['flags']['choking']}")

    if args.sweep:
        print(f"\n{'choke [mm]':>10}{'oil [kSm3/d]':>14}{'WHP [MPa]':>11}{'critical':>10}")
        for diameter in [0.015, 0.020, 0.025, 0.030]:
            s = results_by_id(gs.solve_network(network_json=with_choke(NETWORK, diameter)))
            print(
                f"{diameter * 1000:>10.0f}{s['inflow']['mass_flow']['oil_phase_rate']:>14.3f}"
                f"{s['wellhead']['pressure']['outlet']:>11.3f}{s['choke']['flags']['choking']!s:>10}"
            )


if __name__ == "__main__":
    main()
