# Compressors and compositional equipment

The equipment tools take a gas or liquid described by its **composition** on a
cubic equation of state (Peng-Robinson or SRK), as well as the black-oil
`fluid` they have always taken. With a composition the stream is flashed at
every state, so it can condense in a cooler, flash across a valve or go
two-phase in a compressor, and the duty, temperature and liquid rate follow
from the fluid's own enthalpy. The scripts are in
[`examples/equipment/`](https://github.com/DataInsightsAI/graphsolve-sdk/tree/main/examples/equipment).

A composition names its components from the built-in database (or defines
them), gives their mole fractions, and, where the result depends on the flow,
the stream's `mass_rate` in kg/s:

```python
composition = {
    "component_names": ["methane", "ethane", "propane"],
    "mole_fractions": [0.90, 0.07, 0.03],
    "eos_model": "PengRobinson",  # the default
    "mass_rate": 10.0,  # kg/s
}
```

Every compressor and turbine tool also takes a path `method`: `huntington_3point`
(the default), `schultz`, `reference_2017`, `sandberg_colby_multistep` and the
others listed on [the equipment reference](../reference/tools/equipment.md).

## A compressor on its performance map

`calculate_turbo_machine` runs any machine a network compressor or turbine edge
accepts: fixed-ratio, single-speed and multi-speed maps (with surge and
stonewall limits and fan-law speed scaling), multistage trains, axial, screw
and reciprocating machines. The machine block is the edge's `turbo_machine`,
so a machine tested here drops straight into a network model.

```python
--8<-- "examples/equipment/01_mapped_compressor.py"
```

## A compression train with knockout

`calculate_compression_train` runs each stage's machine, then its cooler on the
equation-of-state enthalpy, then a scrubber that removes the liquid the cooler
condensed. The next stage takes only the gas, at its new composition and mass
rate.

```python
--8<-- "examples/equipment/02_compression_train.py"
```

## Valves and coolers that condense

A Joule-Thomson valve's outlet temperature comes from an enthalpy flash, and
the result reports the vapour fraction left at the outlet:

```python
--8<-- "examples/equipment/03_compositional_jt_valve.py"
```

A cooler's duty includes the latent heat of whatever condenses:

```python
--8<-- "examples/equipment/04_condensing_cooler.py"
```

The pump (`calculate_pump`) and choke (`calculate_choke_pressure_drop`) take a
`composition` the same way.

## Calibrating a compressor from test data

`analyse_turbo_performance` is the inverse of `calculate_turbo_machine`. From
measured suction and discharge pressure and temperature it back-calculates the
polytropic head and efficiency, the isentropic efficiency and the polytropic
exponent by each path method you name; with flow and shaft power it adds the
mechanical and overall efficiencies. With a flow on every point it fits a
`turbo_machine` map, one speed line per measured speed, that the other tools
take directly.

```python
--8<-- "examples/equipment/05_calibrate_compressor.py"
```

## Cost

These tools cost 1 credit, plus 1 credit for each 250 ms of compute beyond the
first 250 ms. A single unit on a large composition takes milliseconds, so in
practice only long trains, mechanistic machines and many-point calibrations on
compositions of twenty or more components pay more. See
[Billing and credits](../guides/billing.md).
