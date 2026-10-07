# Calibrate a compressor from test data, then run the calibrated machine.
#
# Four measured points on two speed lines give suction and discharge pressure
# and temperature, and the mass rate. The path method back-calculates the
# polytropic head and efficiency the machine achieved at each, and the tool
# fits a performance map to them. That map then drives the same machine at a
# new flow.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 05_calibrate_compressor.py

from graphsolve import GraphSolve

gs = GraphSolve()

gas = {
    "component_names": ["methane", "ethane", "propane"],
    "mole_fractions": [0.90, 0.07, 0.03],
}


def point(p_out, t_out, mass_rate, speed):
    return {
        "inlet_pressure": 4.0,  # MPa
        "inlet_temperature": 300.0,  # K
        "outlet_pressure": p_out,  # MPa
        "outlet_temperature": t_out,  # K
        "mass_rate": mass_rate,  # kg/s
        "shaft_speed": speed,  # rad/s
    }


analysis = gs.analyse_turbo_performance(
    composition=gas,
    methods=["huntington_3point", "schultz"],
    points=[
        point(7.6, 358.0, 8.0, 1000.0),
        point(7.0, 352.0, 11.0, 1000.0),
        point(9.0, 375.0, 10.0, 1200.0),
        point(8.2, 367.0, 13.0, 1200.0),
    ],
)["result"]

print(f"{'Q m3/s':>8}{'speed':>8}  {'method':<18}{'H_p kJ/kg':>10}{'eta_p':>8}{'eta_s':>8}")
for p in analysis["points"]:
    for m in p["methods"]:
        print(
            f"{p['inlet_volume_flow']:>8.3f}{p['shaft_speed']:>8.0f}  {m['method']:<18}"
            f"{m['polytropic_head'] / 1000:>10.1f}{m['polytropic_efficiency']:>8.4f}"
            f"{m['isentropic_efficiency']:>8.4f}"
        )

machine = analysis["fitted_turbo_machine"]
machine["shaft_speed"] = 1100.0  # run between the two measured speed lines

run = gs.calculate_turbo_machine(
    inlet_pressure=4.0,
    inlet_temperature=300.0,
    composition={**gas, "mass_rate": 10.5},
    turbo_machine=machine,
)["result"]
print()
print(f"Calibrated {machine['kind']} at 1100 rad/s and 10.5 kg/s:")
print(f"  outlet {run['outlet_pressure']:.3f} MPa, {run['outlet_temperature']:.2f} K, "
      f"efficiency {run['efficiency']:.4f}, shaft {run['shaft_power'] / 1000:.0f} kW")
