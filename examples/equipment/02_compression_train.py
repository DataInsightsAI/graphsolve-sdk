# A three-stage compression train on a gas condensate, with intercoolers and
# scrubbers.
#
# Each stage compresses, a cooler brings the gas back to 320 K, and a scrubber
# removes the liquid that condenses, so the next stage takes only the gas, at
# its new composition and lower mass rate. The last stage has an aftercooler
# but no scrubber.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 02_compression_train.py

from graphsolve import GraphSolve

gs = GraphSolve()


def stage(ratio, efficiency, knockout=True, method=None):
    machine = {
        "kind": "centrifugal_simple",
        "compressor_pressure_ratio": ratio,
        "polytropic_efficiency": efficiency,
    }
    if method:
        machine["method"] = method
    return {
        "turbo_machine": machine,
        "cooler": {"target_temperature_k": 320.0, "pressure_drop_mpa": 0.03},
        "knockout": knockout,
    }


response = gs.calculate_compression_train(
    inlet_pressure=2.0,  # MPa
    inlet_temperature=360.0,  # K
    composition={
        "component_names": ["methane", "ethane", "propane", "n-butane", "n-hexane", "n-decane"],
        "mole_fractions": [0.80, 0.08, 0.05, 0.03, 0.025, 0.015],
        "mass_rate": 5.0,  # kg/s
    },
    stages=[
        stage(2.2, 0.78),
        stage(2.2, 0.78, method="schultz"),
        stage(2.0, 0.76, knockout=False),
    ],
)
train = response["result"]

print(f"{'Stage':<7}{'P out':>9}{'T out':>9}{'Shaft':>10}{'Cooled':>10}{'Liquid':>10}")
for n, s in enumerate(train["stages"], start=1):
    machine = s["machine"]
    knockout = s.get("knockout") or {}
    print(
        f"{n:<7}"
        f"{machine['outlet_pressure']:>6.2f} MPa"
        f"{machine['outlet_temperature']:>7.1f} K"
        f"{machine['shaft_power'] / 1000:>7.0f} kW"
        f"{(s['heat_removed'] or 0.0) / 1000:>7.0f} kW"
        f"{knockout.get('liquid_mass_rate', 0.0):>6.3f} kg/s"
    )
print()
print(f"Delivery:          {train['outlet_pressure']:.2f} MPa, {train['outlet_temperature']:.1f} K")
print(f"Pressure ratio:    {train['overall_pressure_ratio']:.2f}")
print(f"Shaft power:       {train['total_shaft_power'] / 1000:.0f} kW")
print(f"Heat removed:      {train['total_heat_removed'] / 1000:.0f} kW")
print(f"Liquid removed:    {train['total_liquid_removed']:.3f} kg/s")
print(f"Gas delivered:     {train['outlet_mass_rate']:.3f} kg/s")
for name, x in zip(train["component_names"], train["outlet_mole_fractions"]):
    print(f"  {name:<10} {x:.4f}")
