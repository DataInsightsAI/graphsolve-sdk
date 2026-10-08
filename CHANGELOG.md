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

### Fixed

- **Python only:** `graphsolve.__version__` now reports the installed
  version. It was stuck at `"1.0.0"`.

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
