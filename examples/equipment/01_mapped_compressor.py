# A centrifugal compressor driven by its performance map, on a gas composition.
#
# The map has two speed lines (polytropic head and efficiency against actual
# inlet volume flow); the machine runs between them at 1100 rad/s, scaled by
# the fan laws. The gas is a lean natural gas on the Peng-Robinson equation of
# state, compressed at 10 kg/s with the Schultz path method.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 01_mapped_compressor.py

from graphsolve import GraphSolve

gs = GraphSolve()

response = gs.calculate_turbo_machine(
    inlet_pressure=4.0,  # MPa
    inlet_temperature=300.0,  # K
    composition={
        "component_names": ["methane", "ethane", "propane"],
        "mole_fractions": [0.90, 0.07, 0.03],
        "mass_rate": 10.0,  # kg/s
    },
    turbo_machine={
        "kind": "centrifugal_map",
        "shaft_speed": 1100.0,  # rad/s
        "method": "schultz",
        "speed_lines": [
            {
                "shaft_speed": 1000.0,
                "head_curve": {"x": [0.20, 0.35, 0.50], "y": [85000.0, 75000.0, 58000.0]},
                "efficiency_curve": {"x": [0.20, 0.35, 0.50], "y": [0.74, 0.80, 0.76]},
            },
            {
                "shaft_speed": 1200.0,
                "head_curve": {"x": [0.24, 0.42, 0.60], "y": [122000.0, 108000.0, 83000.0]},
                "efficiency_curve": {"x": [0.24, 0.42, 0.60], "y": [0.74, 0.80, 0.76]},
            },
        ],
    },
)
result = response["result"]
performance = result["turbo"]["performance"]

print(f"Machine:               {result['machine_kind']} ({result['method']})")
print(f"Outlet pressure:       {result['outlet_pressure']:.3f} MPa")
print(f"Outlet temperature:    {result['outlet_temperature']:.2f} K")
print(f"Polytropic efficiency: {result['efficiency']:.4f}")
print(f"Polytropic head:       {performance['polytropic_head'] / 1000:.1f} kJ/kg")
print(f"Shaft power:           {result['shaft_power'] / 1000:.0f} kW")
for warning in result["turbo"].get("warnings", []):
    print(f"Warning:               {warning}")
