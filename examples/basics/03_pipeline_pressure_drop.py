# Pressure and temperature drop along a 5 km horizontal flowline carrying oil,
# gas and water, using the GOAT mechanistic model with heat loss to the
# surroundings.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 03_pipeline_pressure_drop.py

from graphsolve import GraphSolve

gs = GraphSolve()

response = gs.calculate_pipe_traverse(
    diameter=0.2032,  # m, 8 inch inner diameter
    length=5000.0,  # m
    roughness=4.5e-5,  # m, commercial steel
    angle=90.0,  # degrees from vertical, 90 is horizontal
    inlet_pressure=5.0,  # MPa
    inlet_temperature=330.0,  # K
    oil_rate=1000.0,  # Sm3/day
    gas_rate=100000.0,  # Sm3/day of free gas, on top of the dissolved gas
    water_rate=200.0,  # Sm3/day
    dissolved_gas_ratio=90.0,  # Sm3/Sm3
    surrounding_temperature=280.0,  # K
    heat_transfer_coefficient=5.0,  # W/(m2.K)
    flow_correlation="GOAT",
)
result = response["result"]

pressure_drop = result["fluid_inlet_pressure_mpa"] - result["fluid_outlet_pressure_mpa"]

print(f"Outlet pressure:     {result['fluid_outlet_pressure_mpa']:.3f} MPa")
print(f"Outlet temperature:  {result['fluid_outlet_temperature_k']:.1f} K")
print(f"Total pressure drop: {pressure_drop:.3f} MPa")
print(f"  friction:          {result['friction_pressure_drop_mpa']:.3f} MPa")
print(f"  gravity:           {result['gravity_pressure_drop_mpa']:.3f} MPa")
print(f"  acceleration:      {result['acceleration_pressure_drop_mpa']:.3f} MPa")
print(f"Average holdup:      {result['average_holdup']:.3f}")
print(f"Flow regime:         {result['flow_regime_outlet']}")
print()
print(f"{'Distance (m)':>12}{'P (MPa)':>10}{'T (K)':>9}{'Holdup':>9}")
for point in result["traverse"][::5]:
    print(
        f"{point['distance_from_inlet_m']:>12.0f}"
        f"{point['pressure_mpa']:>10.3f}"
        f"{point['temperature_k']:>9.1f}"
        f"{point['liquid_holdup']:>9.3f}"
    )
