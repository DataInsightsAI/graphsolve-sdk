# Single-stage centrifugal compressor on a published benchmark: case SC M of
# Evans & Huble, Appendix A, a natural gas of molecular weight 24.287 g/mol
# compressed from 5.99 MPa by a pressure ratio of 2.7685 at a polytropic
# efficiency of 0.8209. The published outlet temperature is 403.76 K and the
# polytropic head 34,782.3 ft.lbf/lbm (103,968 J/kg).
#
# The API describes the gas by its molecular weight alone, so the result is
# compared with the published values rather than matched exactly.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 05_compressor.py

from graphsolve import GraphSolve

gs = GraphSolve()

published_outlet_temperature = 403.761  # K
published_head = 103968.0  # J/kg

response = gs.calculate_compressor(
    inlet_pressure=5.990165,  # MPa
    inlet_temperature=324.761,  # K
    pressure_ratio=2.768531,  # outlet / inlet
    polytropic_efficiency=0.8209,
    mechanical_efficiency=0.95,
    fluid={
        "gas_rate": 1000000.0,  # Sm3/day
        "gas_mw": 24.287,  # g/mol
    },
)
result = response["result"]

outlet_temperature = result["outlet_temperature_k"]
head = result["polytropic_head_j_per_kg"]

print(f"Model:                 {result['model']}")
print(f"Mass flow:             {result['mass_flow_kg_s']:.2f} kg/s")
print(f"Outlet pressure:       {result['outlet_pressure_mpa']:.3f} MPa")
print(f"Polytropic efficiency: {result['polytropic_efficiency']:.4f}")
print(f"Isentropic efficiency: {result['isentropic_efficiency']:.4f}")
print(f"Shaft power:           {result['shaft_power_kw']:.0f} kW")
print()
print(f"{'':<20}{'GraphSolve':>12}{'Published':>12}{'Difference':>12}")
print(
    f"{'Outlet temperature':<20}"
    f"{outlet_temperature:>10.2f} K"
    f"{published_outlet_temperature:>10.2f} K"
    f"{outlet_temperature - published_outlet_temperature:>10.2f} K"
)
print(
    f"{'Polytropic head':<20}"
    f"{head / 1000:>6.1f} kJ/kg"
    f"{published_head / 1000:>6.1f} kJ/kg"
    f"{100 * (head - published_head) / published_head:>10.1f} %"
)
