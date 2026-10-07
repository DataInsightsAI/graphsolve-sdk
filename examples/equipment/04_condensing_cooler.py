# A cooler taking a gas condensate across its dew point.
#
# With a composition the energy balance is on the equation-of-state enthalpy,
# Q = n (h_out - h_in), so the latent heat of the liquid that condenses is in
# the duty. A constant heat capacity would understate it.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 04_condensing_cooler.py

from graphsolve import GraphSolve

gs = GraphSolve()

response = gs.calculate_heater_cooler(
    inlet_pressure=5.0,  # MPa
    inlet_temperature=500.0,  # K
    mode="fixed_outlet_temperature",
    outlet_temperature=330.0,  # K
    pressure_drop=0.05,  # MPa
    composition={
        "component_names": ["methane", "ethane", "n-decane"],
        "mole_fractions": [0.85, 0.10, 0.05],
        "mass_rate": 5.0,  # kg/s
    },
)
result = response["result"]

print(f"Outlet:               {result['outlet_pressure_mpa']:.2f} MPa, "
      f"{result['outlet_temperature_k']:.1f} K")
print(f"Duty:                 {result['heat_duty_kw']:.0f} kW (negative cools)")
print(f"Vapour fraction (mol): in {result['inlet_vapour_fraction']:.3f}, "
      f"out {result['outlet_vapour_fraction']:.3f}")
