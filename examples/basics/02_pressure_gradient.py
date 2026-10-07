# Pressure gradient at a single point in vertical production tubing, from the
# GOAT mechanistic model. This calls the flow correlation directly: phase
# densities, viscosities and superficial velocities at the point are inputs,
# so no fluid model or pressure march is involved.
#
#     pip install graphsolve
#     export GRAPHSOLVE_API_KEY=...
#     python 02_pressure_gradient.py

from graphsolve import GraphSolve

gs = GraphSolve()

response = gs.calculate_pressure_drop(
    correlation="GOAT",
    pressure=10.0,  # MPa
    diameter=0.1,  # m, 4 inch tubing
    roughness=4.5e-5,  # m
    angle=0.0,  # degrees from vertical in the flow direction, 0 is vertical upflow
    density=[80.0, 750.0, 1030.0],  # kg/m3, [gas, oil, water]
    viscosity=[1.5e-5, 2.0e-3, 5.0e-4],  # Pa.s, [gas, oil, water]
    velocity=[2.0, 1.0, 0.3],  # m/s superficial, [gas, oil, water]
    ift=0.02,  # N/m, gas-liquid interfacial tension
)
result = response["result"]

print(f"Total gradient:        {result['total_gradient_mpa_per_m'] * 1000:.3f} kPa/m")
print(f"  gravity:             {result['gravity_gradient_mpa_per_m'] * 1000:.3f} kPa/m")
print(f"  friction:            {result['frictional_gradient_mpa_per_m'] * 1000:.3f} kPa/m")
print(f"  acceleration:        {result['acceleration_gradient_mpa_per_m'] * 1000:.3f} kPa/m")
print(f"Liquid holdup:         {result['holdup']:.3f}")
print(f"Flow regime:           {result['flow_regime']}")
print(f"Mixture velocity:      {result['mixture_velocity_m_per_s']:.2f} m/s")
print(f"Reynolds number:       {result['reynolds_number']:.0f}")
print(f"Friction factor:       {result['friction_factor']:.4f}")
