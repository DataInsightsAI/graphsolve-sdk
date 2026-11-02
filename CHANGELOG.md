# Changelog

What changed in each release of the GraphSolve clients: the Python package
`graphsolve`, the TypeScript package `@graphsolve/sdk` and the Rust crate
`graphsolve`.

The three clients are built from the same API specification and usually
release together under the same version number. A client's major.minor
matches the engine API it speaks; the patch number is the client's own. Where
a change affects only one language, the entry says so.

Each release lists its changes under these headings:

- **Added**: new tools, arguments or client features.
- **Changed**: different behaviour or pricing in an existing tool.
- **Breaking**: a change that can make code that worked before fail.
  Check this heading before you upgrade.
- **Fixed**: bug fixes.
- **Docs**: new documentation and examples.

## Unreleased

## 1.0.49 — 2026-11-02

### Added

- `trickle_detail` (102 tools in all): the TRICKLE three-phase flow
  structure at one point, as the transient solvers read it. It returns the
  regime weights, holdups, phase velocities and pressure gradient, the slug
  unit, the oil-water structure, and the forces on each phase and interface.
  It costs 1 credit. It also returns:
  - `stability`: how close the flow is to each regime boundary
    (`margins`), the separated and distributed solutions the regime choice
    weighed (`candidates`), and whether more than one steady state exists.
    Near horizontal, `well_posedness` compares the layer's slip with the
    inviscid Kelvin-Helmholtz limit of the two-fluid model.
  - `stability.flags`: the conditions found, named in words. Examples:
    `long_wave_unstable`, `near_inviscid_kh_limit`, `bistable`,
    `slug_film_falls_back` and `film_cannot_be_carried_liquid_loading`.
  - A scan along the gas superficial velocity at the point's liquid rates,
    unless `scan` is false. It gives the gradient's slopes, which side of the
    gradient minimum the point is on, and where the onsets lie.
  - With `evaluate_at`, the forces at imposed holdups and velocities.
- `TRICKLE` is a new `correlation` on `calculate_pressure_drop`.
- Transient runs: `run_transient_field`, `run_transient_flux` and
  `run_transient_wave` now all run on the TRICKLE three-phase flow structure
  by default. The request takes these new options:
  - `flow_model`: `trickle` (the water moves at its own velocity),
    `goat` (GOAT's two-fluid closure on one lumped liquid) or `tfs`
    (Wave's two-fluid friction; Wave only).
  - A line form: a `source` node, a `line` of pipe edges with an optional
    outlet choke, and a `sink` node, written in the network solver's schema.
    The source can be a fixed rate or a well inflow model.
  - `gas_transfer`: how quickly gas leaves and re-enters solution, for Field
    and Flux.
  - `slug_capturing` (Field only): slugs form on the grid.
  - `steps`: time-step controls for Field and Flux.
  - `equipment` (Field only): a well inflow table or model, gas-lift points
    with or without an annulus valve, a separator, and gas volume behind the
    inlet.
  - Field networks: several branches that gather or split, with one outlet
    or several.
  - New output variables: `oil_holdup`, `water_holdup`, `oil_velocity` and
    `water_velocity`.
- `calculate_water_properties` takes `density_correlation` and
  `viscosity_correlation`.
- `solve_network` accepts a `solver_profile` block. A model exported from the
  app solves with the profile its tab names.
- `run_forecast` with the optimise engine applies the optimiser targets
  (control bounds, constraint limits and objective weights and prices).
  These were accepted before but not applied.

### Changed

- Transient runs build each fluid's property table from the engine's own
  fluid model. For a black-oil or table fluid, `fluid_type`, the correlation
  choices and `pvt_table` are now used; before, only
  `reference_oil_density`, `dissolved_gas_ratio`, `reference_gas_mw` and
  `salinity` were read, through a fixed correlation set. For an
  equation-of-state fluid, `pvt.salinity_ppm` now sets the water's salinity.
  Results for the same request differ from 1.0.42.
- Transient runs: the fixed pressure windows are gone. Each scheme now works
  inside the pressure and temperature range of its fluid table (`pvt`). Wave
  clamps a cell to that range, Flux retries a step that would leave it, and
  Field counts it as a failed step. Widen `pvt` for a run that may leave the
  range. The default `pvt.temperature_nodes` is now 36 (was 24).
- Transient runs: a `water_salinity` first given at a later event now ramps
  from the fluid's own salinity.
- `run_forecast`: a `max_iter` or `tolerance` in `solver_settings` now
  replaces the network's own for each step's solve.
- `calculate_choke_pressure_drop` with a `composition` now uses Sachdeva's
  model for every slip model, with the network's default pressure recovery.
- No prices changed.

### Breaking

- Transient runs, network form: each edge's `pipe_geometry` is now laid out
  as a network solve lays it, from the `target` end to the `source` end.
  Before, it ran from `source` to `target`. If an edge has more than one
  segment, check its order.
- Transient runs, network form: an annulus or restriction segment in the
  path is now refused. Before, it was dropped silently. Remove it from the
  path.
- Transient runs: a black-oil or table fluid must now give
  `reference_oil_density` and `reference_gas_mw`.
- **Rust only:** `FlowCorrelationName` has a new `Trickle` variant. A `match`
  without a wildcard arm needs one more arm. `CalculateWaterPropertiesParams`
  has two new `Option` fields. A struct literal without
  `..Default::default()` must set them.

## 1.0.42 — 2026-10-08

### Added

- Three equipment tools (101 tools in all):
  - `calculate_turbo_machine`
  - `analyse_turbo_performance`
  - `calculate_compression_train`
- Compositional equipment calculations. The compressor, turbine, multistage,
  screw and reciprocating compressor tools take a `composition` and a path
  method. The JT valve, isenthalpic temperature, heater/cooler, choke pressure
  drop and pump tools take a `composition`.

### Changed

- `fluid` is now optional on `calculate_compressor`, `calculate_turbine`,
  `calculate_multistage_compressor`, `calculate_jt_valve`,
  `calculate_isenthalpic_temperature`, `calculate_heater_cooler` and
  `calculate_pump`. `gas_rate`, `oil_rate` and `water_rate` are now optional
  on `calculate_choke_pressure_drop`. A call can describe the fluid with a
  `composition` instead.
- Pricing: thirteen equipment tools now cost 1 credit plus 1 credit for each
  250 ms of compute beyond the first 250 ms. These were previously a flat
  1 credit:
  - `calculate_compressor`, `calculate_multistage_compressor`,
    `calculate_screw_compressor`, `calculate_reciprocating_compressor`
  - `calculate_turbine`, `calculate_pump`
  - `calculate_jt_valve`, `calculate_isenthalpic_temperature`,
    `calculate_heater_cooler`
  - `calculate_choke_pressure_drop`

  The three new tools use the same pricing. For how charges are calculated,
  see [Billing and credits](https://datainsightsai.github.io/graphsolve-sdk/guides/billing/).

### Breaking

- **Rust only:** the arguments that became optional are now `Option` fields
  on their `…Params` structs. Code that sets them must wrap the value in
  `Some(...)`. Python and TypeScript callers do not need to change anything.

### Docs

- A [documentation site](https://datainsightsai.github.io/graphsolve-sdk/) with:
  - getting-started pages and guides, with code in Python, TypeScript and Rust
  - a worked single-well example
  - a reference page for every tool, giving its arguments and price
  - API references for each client and the HTTP API
- A new example,
  [Compressors and compositional equipment](https://datainsightsai.github.io/graphsolve-sdk/examples/equipment/),
  covers a mapped compressor, a compression train with knockout, a JT valve
  and a condensing cooler on compositions, and a compressor calibrated from
  test points.
- The [Units](https://datainsightsai.github.io/graphsolve-sdk/guides/units/)
  guide now covers compositions and explains that the equipment tools take
  rates in Sm3/day.
- Short starter scripts in
  [`examples/basics`](https://github.com/DataInsightsAI/graphsolve-sdk/tree/main/examples/basics):
  EOS flash, pressure gradient, pipeline and choke pressure drop, and a
  compressor.

## 1.0.40 — 2026-09-30

### Added

- Nine tools (98 tools in all):
  - `calculate_critical_point`: the true critical point of a mixture with a
    cubic EOS.
  - `calculate_pump`: centrifugal pump or ESP performance at one suction
    state.
  - `import_prp_fluid`: imports a PVTsim `.prp` fluid file as a composition.
  - `partition_acid_gas_in_water`: dissolved CO2 and H2S in produced water,
    and the resulting in-situ pH.
  - `run_gas_depletion`: a gas-reservoir depletion study.
  - `run_mmp_probe`: a mixing-cell miscibility test at one pressure.
  - `run_nodal_study`: nodal analysis of one well inside a network.
  - `run_pvt_regression_suite`: compares several PVT experiments with lab
    data.
  - `screen_scale_risk`: mineral-scale saturation indices from a
    produced-water analysis.

### Breaking

- `calculate_gas_dew_point` now requires `condensate_gas_ratio_stb_per_mmscf`.

### Fixed

- **Rust only:** enum variants with descriptions longer than one line now
  generate valid doc comments.
