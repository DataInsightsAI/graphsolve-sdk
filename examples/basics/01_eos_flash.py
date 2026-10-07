# Flash a natural-gas condensate composition at a given pressure and temperature
# with the Peng-Robinson equation of state.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 01_eos_flash.py

from graphsolve import GraphSolve

gs = GraphSolve()

components = [
    "nitrogen",
    "carbon dioxide",
    "methane",
    "ethane",
    "propane",
    "isobutane",
    "n-butane",
    "isopentane",
    "n-pentane",
    "n-hexane",
]
mole_fractions = [0.01, 0.02, 0.70, 0.08, 0.06, 0.02, 0.03, 0.02, 0.02, 0.04]

response = gs.run_eos_flash(
    component_names=components,
    mole_fractions=mole_fractions,
    pressure_mpa=5.0,  # MPa
    temperature_k=300.0,  # K
    eos_model="PengRobinson",
)
result = response["result"]

print(f"Phase:            {result['phase']}")
print(f"Vapour fraction:  {result['vapor_fraction']:.4f} (molar)")
print(f"Vapour density:   {result['vapor_density_kg_m3']:.1f} kg/m3")
print(f"Liquid density:   {result['liquid_density_kg_m3']:.1f} kg/m3")
print(f"Vapour Z-factor:  {result['z_vapor']:.4f}")
print()
print(f"{'Component':<16}{'Feed':>8}{'Vapour':>10}{'Liquid':>10}{'K':>10}")
for i, name in enumerate(components):
    print(
        f"{name:<16}"
        f"{mole_fractions[i]:>8.4f}"
        f"{result['vapor_composition'][i]:>10.4f}"
        f"{result['liquid_composition'][i]:>10.4f}"
        f"{result['k_values'][i]:>10.4f}"
    )
