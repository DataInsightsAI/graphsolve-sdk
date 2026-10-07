# Pressure drop across a 1 inch production choke for a given oil, gas and
# water rate, using the Sachdeva multiphase choke model.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 04_choke_pressure_drop.py

from graphsolve import GraphSolve

gs = GraphSolve()

response = gs.calculate_choke_pressure_drop(
    choke_diameter=0.0254,  # m, 1 inch bean
    inlet_pressure=10.0,  # MPa
    inlet_temperature=350.0,  # K
    oil_rate=500.0,  # Sm3/day
    gas_rate=50000.0,  # Sm3/day of free gas, on top of the dissolved gas
    water_rate=100.0,  # Sm3/day
    dissolved_gas_ratio=90.0,  # Sm3/Sm3
)
result = response["result"]

print(f"Inlet pressure:          {result['inlet_pressure']:.3f} MPa")
print(f"Outlet pressure:         {result['outlet_pressure']:.3f} MPa")
print(f"Pressure drop:           {result['pressure_drop']:.3f} MPa")
print(f"Outlet temperature:      {result['outlet_temperature']:.1f} K")
print(f"Critical pressure ratio: {result['critical_pressure_ratio']:.3f}")
print(f"Flow regime:             {result['flow_regime']}")
