# A Joule-Thomson valve on a gas condensate.
#
# The outlet temperature solves h(T_out, P_out) = h(T_in, P_in) by a flash on
# the equation of state, so the cooling and the liquid that drops out both
# come from the fluid's own enthalpy surface.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 03_compositional_jt_valve.py

from graphsolve import GraphSolve

gs = GraphSolve()

response = gs.calculate_jt_valve(
    inlet_pressure=10.0,  # MPa
    inlet_temperature=350.0,  # K
    pressure_drop=3.0,  # MPa
    composition={
        "component_names": ["methane", "ethane", "n-decane"],
        "mole_fractions": [0.85, 0.10, 0.05],
    },
)
result = response["result"]

print(f"Outlet:               {result['outlet_pressure_mpa']:.2f} MPa, "
      f"{result['outlet_temperature_k']:.2f} K")
print(f"Temperature change:   {result['temperature_change_k']:.2f} K")
print(f"Outlet vapour (mol):  {result['outlet_vapour_fraction']:.4f}")
