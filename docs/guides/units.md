# Units

**Every value the API takes or returns is SI, with one exception: permeability
is in millidarcy.** There are no unit options and no unit strings on arguments.
Convert at the edge of your code.

| Quantity | Unit | Notes |
|---|---|---|
| Pressure | MPa | Absolute. 1 bar = 0.1 MPa; 1 psi = 0.006895 MPa |
| Temperature | K | |
| Length, diameter, depth | m | |
| Rate | kSm³/day | At standard conditions, oil, water and gas alike. 1 kSm³/day = 1000 Sm³/day |
| Mass rate | kg/s | Where a field says it is a mass rate |
| Density | kg/m³ | |
| Viscosity | Pa·s | 1 cP = 0.001 Pa·s |
| GOR, CGR | Sm³/Sm³ | |
| Water cut | % | |
| Productivity index | Sm³/day/MPa | Of liquid (oil + water) at standard conditions |
| Compressibility | 1/MPa | |
| Permeability | mD | The one exception to SI |
| Transient timestep | hours | |

`convert_units` converts between any of these and the field units your data
arrives in. It is free.

## Things that are easy to get backwards

**Pipe angle is measured from vertical, in the direction of flow.** 0° is
vertical upflow, 90° is horizontal and 180° is vertical downflow. A production
tubing string is 0°. An annulus carrying lift gas down is 180°.

**Rates are volumetric at standard conditions, including gas.** A gas rate of
`120.0` means 120 000 Sm³/day. A measured rate entered in kg/s is off by an order
of magnitude or more and still solves, answering a different question.

**The productivity index is per day, not per thousand.** It is Sm³/day per MPa
of drawdown. A value entered in kSm³/day/MPa is 1000 times too small and still
solves, as a well that barely flows.

**The equipment tools' black-oil `fluid` takes Sm³/day, not kSm³/day.** On
`calculate_compressor`, the pump, valve, heater/cooler and the other equipment
tools, `fluid.gas_rate`, `oil_rate` and `water_rate` are Sm³/day at standard
conditions, as each tool's reference says.

**A composition's `mass_rate` is the whole stream, in kg/s.** It is not a
standard volume rate and not per component. The mole fractions describe the
stream; the mass rate sets how much of it flows.

**Machine speeds differ by machine.** Performance-map `shaft_speed` is in rad/s,
as on a network edge. `calculate_screw_compressor` takes `shaft_speed_rev_s` in
revolutions per second and `calculate_reciprocating_compressor` takes
`speed_rpm`.

**A variance is σ², in the squared unit of the value.** A pressure gauge good to
±0.5 bar has σ = 0.05 MPa and variance `0.0025` MPa². A ±25 Sm³/Sm³ belief on
GOR has variance `625.0`.
