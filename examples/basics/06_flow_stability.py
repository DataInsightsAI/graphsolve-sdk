# Flow stability at one point of vertical production tubing: the TRICKLE flow
# structure from `trickle_detail`, read with the five-step screen in
# docs/guides/transients.md. The verdict is one of Steady, Watch, Mechanism or
# Act; Act means confirm with a transient run that includes the system.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 06_flow_stability.py

from graphsolve import GraphSolve

MECHANISM = {"film_cannot_be_carried_liquid_loading", "below_film_reversal"}
BOUNDARY = {
    "near_long_wave_instability",
    "near_film_lift",
    "near_film_reversal",
    "near_bubble_transition",
    "slug_unit_fading",
    "pseudo_slug_to_film",
    "near_inviscid_kh_limit",
}
MULTIPLE = {"bistable", "near_film_fold_multiple_solutions"}

gs = GraphSolve()

detail = gs.trickle_detail(
    pressure=10.0,  # MPa
    diameter=0.1,  # m
    roughness=4.5e-5,  # m, absolute
    angle=0.0,  # degrees from vertical in the flow direction, 0 is vertical upflow
    density=[80.0, 750.0, 1030.0],  # kg/m3, [gas, oil, water]
    viscosity=[1.5e-5, 2.0e-3, 5.0e-4],  # Pa.s, [gas, oil, water]
    velocity=[2.0, 1.0, 0.3],  # m/s superficial, [gas, oil, water]
    ift=0.02,  # N/m, gas-oil
    ift_gas_water=0.06,  # N/m
    ift_oil_water=0.03,  # N/m
)["result"]

flags = set(detail["stability"]["flags"])
weights = detail["regime_weights"]
slug = weights["slug_body"] + weights["slug_film_layer"] + weights["slug_film_wall"]
level = detail["stability"]["margins"]["long_wave"] is not None
minimum = (detail["scan"] or {}).get("gradient_minimum")

mechanisms = sorted(flags & MECHANISM)
if "slug_film_falls_back" in flags and slug >= 0.25:
    mechanisms.append("slug_film_falls_back")
if level and "long_wave_unstable" in flags and "unstable_layer_too_thin_to_bridge" not in flags:
    mechanisms.append("long_wave_unstable")

act = []
if not level and flags & MECHANISM:
    act.append("liquid loading")
heading = not level and "slug_film_falls_back" in mechanisms and "left_of_gradient_minimum" in flags
if heading and minimum and minimum["ratio"] < 0.8:
    act.append(
        f"heading mechanism, ratio to the gradient minimum {minimum['ratio']:.2f} (below 0.8)"
    )
if mechanisms and flags & MULTIPLE:
    act.append("a mechanism with two steady states")

if act:
    verdict = "Act"
elif mechanisms:
    verdict = "Mechanism"
elif flags & (BOUNDARY | MULTIPLE):
    verdict = "Watch"
else:
    verdict = "Steady"

print(f"Regime:      {detail['regime_label']} (slug share {slug:.2f})")
print(f"Flags:       {', '.join(sorted(flags)) or 'none'}")
print(f"Mechanisms:  {', '.join(mechanisms) or 'none'}")
if minimum:
    print(f"Gradient minimum at v_SG {minimum['vsg_m_s']:.2f} m/s, ratio {minimum['ratio']:.2f}")
print(f"Verdict:     {verdict}{': ' + '; '.join(act) if act else ''}")
if heading:
    print("A heading mechanism is weighed by the ratio at the TOP of the tubing, where the gas")
    print("has expanded most: screen that point too before concluding.")
