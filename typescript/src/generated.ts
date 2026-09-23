/**
 * Typed methods for every tool in the GraphSolve API.
 *
 * GENERATED FROM spec/graphsolve-v1.json — DO NOT EDIT BY HAND.
 * Run `python emit/emit_typescript.py` after a spec change; CI fails if
 * this file and the spec disagree.
 *
 * Engine API version: 1.0.35
 * Tools: 89
 */

import type { ToolArguments, ToolResponse } from "./types.js";

/** Every tool this client knows about. */
export type ToolName =
  | "adjust_composition_to_gor"
  | "adjust_composition_to_phase_ratio"
  | "aggregate_type_well"
  | "allocate_production"
  | "analyze_material_balance"
  | "calculate_aquifer_influx"
  | "calculate_choke_pressure_drop"
  | "calculate_choke_size"
  | "calculate_compressor"
  | "calculate_corrosion_rate"
  | "calculate_erosional_velocity"
  | "calculate_fluid_properties"
  | "calculate_gas_dew_point"
  | "calculate_heater_cooler"
  | "calculate_hydrate_temperature"
  | "calculate_isenthalpic_temperature"
  | "calculate_jt_valve"
  | "calculate_mmp"
  | "calculate_mpfm_allocation"
  | "calculate_multistage_compressor"
  | "calculate_nodal_analysis"
  | "calculate_phase_cuts"
  | "calculate_pipe_traverse"
  | "calculate_pressure_drop"
  | "calculate_pump_head"
  | "calculate_rate_from_choke"
  | "calculate_reciprocating_compressor"
  | "calculate_reid_vapour_pressure"
  | "calculate_saturation_pressure"
  | "calculate_screw_compressor"
  | "calculate_turbine"
  | "calculate_volumetrics"
  | "calculate_water_properties"
  | "calculate_wax_deposition_rate"
  | "characterize_pseudo_component"
  | "compare_pipeline_correlations"
  | "compare_pressure_drop_correlations"
  | "compute_statistics"
  | "convert_units"
  | "evaluate_flow_assurance_profile"
  | "fit_decline"
  | "flash_to_surface"
  | "generate_decline"
  | "generate_ipr_curve"
  | "generate_phase_envelope"
  | "generate_samples"
  | "generate_type_curve"
  | "get_eos_component"
  | "list_correlations"
  | "list_edge_types"
  | "list_eos_binary_interactions"
  | "list_eos_components"
  | "list_flow_assurance_models"
  | "list_fluid_types"
  | "list_node_types"
  | "list_transient_solvers"
  | "match_fetkovich_ab"
  | "match_forchheimer_ab"
  | "match_parameters"
  | "match_productivity_index"
  | "match_pvt"
  | "match_skin"
  | "optimise_network"
  | "rta_diagnostics"
  | "run_cce"
  | "run_cvd"
  | "run_dle"
  | "run_eos_flash"
  | "run_forecast"
  | "run_material_balance"
  | "run_molecule_tracking"
  | "run_parametric_study"
  | "run_process_graph"
  | "run_separator_test"
  | "run_swelling_test"
  | "run_transient_field"
  | "run_transient_flux"
  | "run_transient_wave"
  | "screen_asphaltene_risk"
  | "screen_hydrate_risk"
  | "screen_liquid_loading"
  | "screen_wax_risk"
  | "solve_network"
  | "solve_network_map"
  | "split_plus_fraction"
  | "transient_bhp_history"
  | "transient_rate_history"
  | "tune_mpfm_allocation"
  | "validate_solver_payload";

export type CorrosionModelName =
  | "de_waard_lotz"
  | "sour"
  | "norsok_m506";

export type EosModelName =
  | "PengRobinson"
  | "SoaveRedlichKwong"
  | "PatelTeja";

export type FlowCorrelationName =
  | "Aziz"
  | "Baxendell-Thomas"
  | "Beggs-Brill"
  | "CHAOS"
  | "Dukler"
  | "Duns-Ros"
  | "Fancher-Brown"
  | "GOAT"
  | "Gray"
  | "Griffith-Wallis"
  | "Hagedorn-Slug"
  | "KISS"
  | "Mist"
  | "ml_tuned"
  | "Mukherjee-Brill"
  | "Poettmann-Carpenter"
  | "Default"
  | "SinglePhaseGas"
  | "SUPREME";

export type HydrateModelName =
  | "towler_mokhatab"
  | "vdwp_si"
  | "vdwp_sii"
  | "vdwp_combined";

export type InhibitorName =
  | "methanol"
  | "meg"
  | "deg"
  | "teg";

export type WaxModelName =
  | "screening"
  | "won";

export type AdjustCompositionToGorParams = {
  mole_fractions: number[];
  target_gor: number;
  binary_interactions?: Record<string, unknown>[] | null;
  component_names?: string[] | null;
  components?: Record<string, unknown>[] | null;
  eos_model?: EosModelName | null;
};

export type AdjustCompositionToPhaseRatioParams = {
  mole_fractions: number[];
  pressure_mpa: number;
  target_gor: number;
  temperature_k: number;
  binary_interactions?: Record<string, unknown>[] | null;
  component_names?: string[] | null;
  components?: Record<string, unknown>[] | null;
  eos_model?: EosModelName | null;
};

export type AggregateTypeWellParams = {
  type_well_json: Record<string, unknown> | unknown[] | string;
};

export type AllocateProductionParams = {
  network_json: Record<string, unknown> | unknown[] | string;
};

export type AnalyzeMaterialBalanceParams = {
  analysis_json: Record<string, unknown> | unknown[] | string;
};

export type CalculateAquiferInfluxParams = {
  aquifer_json: Record<string, unknown> | unknown[] | string;
};

export type CalculateChokePressureDropParams = {
  choke_diameter: number;
  gas_rate: number;
  inlet_pressure: number;
  inlet_temperature: number;
  oil_rate: number;
  water_rate: number;
  dissolved_gas_ratio?: number | null;
  gas_mw?: number | null;
  oil_density?: number | null;
  slip_model?: string | null;
  water_salinity?: number | null;
};

export type CalculateChokeSizeParams = {
  downstream_pressure: number;
  free_gas_rate: number;
  oil_rate: number;
  upstream_pressure: number;
  upstream_temperature: number;
  water_rate: number;
  dissolved_gor?: number | null;
  gas_mw?: number | null;
  oil_density?: number | null;
  slip_model?: string | null;
  water_salinity?: number | null;
};

export type CalculateCompressorParams = {
  fluid: Record<string, unknown>;
  inlet_pressure: number;
  inlet_temperature: number;
  pressure_ratio: number;
  mechanical_efficiency?: number | null;
  polytropic_efficiency?: number | null;
};

export type CalculateCorrosionRateParams = {
  pressure: number;
  temperature: number;
  bicarbonate_molar?: number | null;
  co2_mole_fraction?: number | null;
  design_life_years?: number | null;
  diameter?: number | null;
  glycol_wt_pct?: number | null;
  h2s_mole_fraction?: number | null;
  inhibitor_efficiency_pct?: number | null;
  ionic_strength_molar?: number | null;
  mixture_density?: number | null;
  mixture_velocity?: number | null;
  mixture_viscosity?: number | null;
  model?: CorrosionModelName | null;
  ph?: number | null;
  roughness?: number | null;
  shear_stress_pa?: number | null;
};

export type CalculateErosionalVelocityParams = {
  fluid: Record<string, unknown>;
  pipe_diameter: number;
  pressure: number;
  temperature: number;
  c_factor?: number | null;
};

export type CalculateFluidPropertiesParams = {
  fluid_properties: Record<string, unknown>;
  pressure: number;
  temperature: number;
};

export type CalculateGasDewPointParams = {
  condensate_gas_ratio: number;
  gas_gravity: number;
  oil_api_gravity: number;
  temperature: number;
};

export type CalculateHeaterCoolerParams = {
  fluid: Record<string, unknown>;
  inlet_pressure: number;
  inlet_temperature: number;
  mode: string;
  approach_temperature?: number | null;
  duty?: number | null;
  pressure_drop?: number | null;
  reference_temperature?: number | null;
  target_temperature?: number | null;
};

export type CalculateHydrateTemperatureParams = {
  gas_gravity: number;
  pressure: number;
  co2_mole_fraction?: number | null;
  h2s_mole_fraction?: number | null;
};

export type CalculateIsenthalpicTemperatureParams = {
  fluid: Record<string, unknown>;
  inlet_pressure: number;
  inlet_temperature: number;
  outlet_pressure: number;
};

export type CalculateJtValveParams = {
  fluid: Record<string, unknown>;
  inlet_pressure: number;
  inlet_temperature: number;
  pressure_drop: number;
};

export type CalculateMmpParams = {
  experiment_json: Record<string, unknown> | unknown[] | string;
};

export type CalculateMpfmAllocationParams = {
  allocation_json: Record<string, unknown> | unknown[] | string;
};

export type CalculateMultistageCompressorParams = {
  fluid: Record<string, unknown>;
  inlet_pressure: number;
  inlet_temperature: number;
  stages: Record<string, unknown>[];
  intercool_pressure_drop?: number | null;
  intercool_temperature?: number | null;
  mechanical_efficiency?: number | null;
};

export type CalculateNodalAnalysisParams = {
  ipr: unknown;
  tubing_diameter: number;
  tubing_length: number;
  tubing_roughness: number;
  wellhead_pressure: number;
  wellhead_temperature: number;
  dissolved_gas_ratio?: number | null;
  flow_correlation?: string | null;
  gas_mw?: number | null;
  oil_density?: number | null;
  tubing_angle?: number | null;
  water_salinity?: number | null;
};

export type CalculatePhaseCutsParams = {
  gas_rate?: number | null;
  gas_unit?: string | null;
  oil_rate?: number | null;
  oil_unit?: string | null;
  water_rate?: number | null;
  water_unit?: string | null;
};

export type CalculatePipeTraverseParams = {
  angle: number;
  diameter: number;
  gas_rate: number;
  inlet_pressure: number;
  inlet_temperature: number;
  length: number;
  oil_rate: number;
  roughness: number;
  water_rate: number;
  dissolved_gas_ratio?: number | null;
  flow_boundary?: string | null;
  flow_correlation?: string | null;
  gas_mw?: number | null;
  heat_transfer_coefficient?: number | null;
  oil_density?: number | null;
  pressure_boundary?: string | null;
  surrounding_temperature?: number | null;
  water_salinity?: number | null;
};

export type CalculatePressureDropParams = {
  angle: number;
  correlation: FlowCorrelationName;
  density: number[];
  diameter: number;
  ift: number;
  pressure: number;
  roughness: number;
  velocity: number[];
  viscosity: number[];
};

export type CalculatePumpHeadParams = {
  flow_rate: number;
  head_curve: number[][];
  inlet_pressure: number;
  inlet_temperature: number;
  liquid_density: number;
};

export type CalculateRateFromChokeParams = {
  choke_diameter: number;
  downstream_pressure: number;
  upstream_pressure: number;
  upstream_temperature: number;
  cgr?: number | null;
  dissolved_gor?: number | null;
  fixed?: string | null;
  free_gas_rate?: number | null;
  gas_mw?: number | null;
  gor?: number | null;
  oil_density?: number | null;
  oil_rate?: number | null;
  slip_model?: string | null;
  target_downstream_temperature?: number | null;
  water_rate?: number | null;
  water_salinity?: number | null;
  watercut?: number | null;
  wgr?: number | null;
};

export type CalculateReciprocatingCompressorParams = {
  discharge_pressure: number;
  inlet_pressure: number;
  inlet_temperature: number;
  speed_rpm: number;
  swept_volume_per_rev_m3: number;
  clearance_fraction?: number | null;
  co2_fraction?: number | null;
  gas_molecular_weight?: number | null;
  h2s_fraction?: number | null;
  max_pressure_ratio?: number | null;
  mechanical_efficiency?: number | null;
  min_pressure_ratio?: number | null;
  n2_fraction?: number | null;
  polytropic_efficiency?: number | null;
};

export type CalculateReidVapourPressureParams = {
  liquid_mole_fractions: number[];
  binary_interactions?: Record<string, unknown>[] | null;
  component_names?: string[] | null;
  components?: Record<string, unknown>[] | null;
  eos_model?: EosModelName | null;
};

export type CalculateSaturationPressureParams = {
  component_names: string[];
  mole_fractions: number[];
  temperature_k: number;
  boundary?: string | null;
  eos_model?: string | null;
};

export type CalculateScrewCompressorParams = {
  discharge_pressure: number;
  displacement_per_rev_m3: number;
  inlet_pressure: number;
  inlet_temperature: number;
  shaft_speed_rev_s: number;
  co2_fraction?: number | null;
  gas_molecular_weight?: number | null;
  h2s_fraction?: number | null;
  mechanical_efficiency?: number | null;
  n2_fraction?: number | null;
  polytropic_efficiency?: number | null;
  subtype?: string | null;
  volumetric_efficiency?: number | null;
};

export type CalculateTurbineParams = {
  fluid: Record<string, unknown>;
  inlet_pressure: number;
  inlet_temperature: number;
  pressure_ratio: number;
  mechanical_efficiency?: number | null;
  polytropic_efficiency?: number | null;
};

export type CalculateVolumetricsParams = {
  volumetrics_json: Record<string, unknown> | unknown[] | string;
};

export type CalculateWaterPropertiesParams = {
  pressure: number;
  temperature: number;
  salinity?: number | null;
};

export type CalculateWaxDepositionRateParams = {
  ambient_temperature: number;
  bulk_temperature: number;
  heat_transfer_coefficient: number;
  oil_density: number;
  solubility_gradient_per_k: number;
  oil_molecular_weight?: number | null;
  oil_thermal_conductivity?: number | null;
  oil_viscosity?: number | null;
  wax_diffusivity?: number | null;
  wax_molecular_weight?: number | null;
};

export type CharacterizePseudoComponentParams = {
  boiling_point?: number | null;
  method?: string | null;
  molecular_weight?: number | null;
  specific_gravity?: number | null;
};

export type ComparePipelineCorrelationsParams = {
  fluid: unknown;
  inlet: Record<string, unknown>;
  pipe_segments: unknown;
  correlations?: FlowCorrelationName[] | null;
};

export type ComparePressureDropCorrelationsParams = {
  angle: number;
  density: number[];
  diameter: number;
  ift: number;
  pressure: number;
  roughness: number;
  velocity: number[];
  viscosity: number[];
  correlations?: string[] | null;
};

export type ComputeStatisticsParams = {
  statistics_json: Record<string, unknown> | unknown[] | string;
};

export type ConvertUnitsParams = {
  from_unit: string;
  to_unit: string;
  value: number;
};

export type EvaluateFlowAssuranceProfileParams = {
  inlet_pressure: number;
  inlet_temperature: number;
  mass_flow: number;
  outlet_pressure: number;
  outlet_temperature: number;
  segments: Record<string, unknown>[];
  c_factor?: number | null;
  component_names?: string[] | null;
  heat_capacity?: number | null;
  inhibitor_depression_k?: number | null;
  inlet_mixture_density?: number | null;
  vapor_mole_fractions?: number[] | null;
};

export type FitDeclineParams = {
  fit_json: Record<string, unknown> | unknown[] | string;
};

export type FlashToSurfaceParams = {
  mole_fractions: number[];
  binary_interactions?: Record<string, unknown>[] | null;
  component_names?: string[] | null;
  components?: Record<string, unknown>[] | null;
  eos_model?: EosModelName | null;
};

export type GenerateDeclineParams = {
  decline_json: Record<string, unknown> | unknown[] | string;
};

export type GenerateIprCurveParams = {
  inflow_model: unknown;
  n_points?: number | null;
};

export type GeneratePhaseEnvelopeParams = {
  component_names: string[];
  mole_fractions: number[];
  eos_model?: string | null;
};

export type GenerateSamplesParams = {
  samples_json: Record<string, unknown> | unknown[] | string;
};

export type GenerateTypeCurveParams = {
  inflow_model: unknown;
  n?: number | null;
  td_max?: number | null;
  td_min?: number | null;
};

export type GetEosComponentParams = {
  name: string;
};

export type ListCorrelationsParams = {
  category?: string | null;
};

export type ListEosBinaryInteractionsParams = {
  name: string;
};

export type MatchFetkovichAbParams = {
  inflow_model: unknown;
};

export type MatchForchheimerAbParams = {
  inflow_model: unknown;
};

export type MatchParametersParams = {
  inflow_model: unknown;
  match_target?: string | null;
};

export type MatchProductivityIndexParams = {
  inflow_model: unknown;
};

export type MatchPvtParams = {
  match_json: Record<string, unknown> | unknown[] | string;
};

export type MatchSkinParams = {
  inflow_model: unknown;
};

export type OptimiseNetworkParams = {
  optimisation_json: Record<string, unknown> | unknown[] | string;
};

export type RtaDiagnosticsParams = {
  inflow_model: unknown;
};

export type RunCceParams = {
  experiment_json: Record<string, unknown> | unknown[] | string;
};

export type RunCvdParams = {
  experiment_json: Record<string, unknown> | unknown[] | string;
};

export type RunDleParams = {
  experiment_json: Record<string, unknown> | unknown[] | string;
};

export type RunEosFlashParams = {
  mole_fractions: number[];
  pressure_mpa: number;
  temperature_k: number;
  binary_interactions?: Record<string, unknown>[] | null;
  component_names?: string[] | null;
  components?: Record<string, unknown>[] | null;
  eos_model?: EosModelName | null;
};

export type RunForecastParams = {
  forecast_json: Record<string, unknown> | unknown[] | string;
};

export type RunMaterialBalanceParams = {
  reservoir_json: Record<string, unknown> | unknown[] | string;
};

export type RunMoleculeTrackingParams = {
  fluids_json: Record<string, unknown> | unknown[] | string;
  solved_network_json: Record<string, unknown> | unknown[] | string;
};

export type RunParametricStudyParams = {
  study_json: Record<string, unknown> | unknown[] | string;
};

export type RunProcessGraphParams = {
  process_graph_json: Record<string, unknown> | unknown[] | string;
};

export type RunSeparatorTestParams = {
  experiment_json: Record<string, unknown> | unknown[] | string;
};

export type RunSwellingTestParams = {
  experiment_json: Record<string, unknown> | unknown[] | string;
};

export type RunTransientFieldParams = {
  transient_json: Record<string, unknown> | unknown[] | string;
};

export type RunTransientFluxParams = {
  transient_json: Record<string, unknown> | unknown[] | string;
};

export type RunTransientWaveParams = {
  transient_json: Record<string, unknown> | unknown[] | string;
};

export type ScreenAsphalteneRiskParams = {
  bubble_point_pressure: number;
  live_oil_density: number;
  reservoir_pressure: number;
};

export type ScreenHydrateRiskParams = {
  pressure: number;
  temperature: number;
  component_names?: string[] | null;
  inhibitor?: InhibitorName | null;
  inhibitor_wt_pct?: number | null;
  model?: HydrateModelName | null;
  mole_fractions?: number[] | null;
  molecular_weights?: number[] | null;
  specific_gravity?: number | null;
};

export type ScreenLiquidLoadingParams = {
  gas_density: number;
  gas_velocity: number;
  liquid_density: number;
  surface_tension: number;
};

export type ScreenWaxRiskParams = {
  temperature: number;
  density_c7_plus?: number | null;
  model?: WaxModelName | null;
  mole_fractions?: number[] | null;
  molecular_weights?: number[] | null;
  mw_c7_plus?: number | null;
  paraffin_mass_fraction?: number | null;
  watson_k?: number | null;
  wax_former_mask?: boolean[] | null;
};

export type SolveNetworkParams = {
  network_json: Record<string, unknown> | unknown[] | string;
};

export type SolveNetworkMapParams = {
  network_json: Record<string, unknown> | unknown[] | string;
};

export type SplitPlusFractionParams = {
  split_json: Record<string, unknown> | unknown[] | string;
};

export type TransientBhpHistoryParams = {
  inflow_model: unknown;
};

export type TransientRateHistoryParams = {
  inflow_model: unknown;
};

export type TuneMpfmAllocationParams = {
  tuning_json: Record<string, unknown> | unknown[] | string;
};

export type ValidateSolverPayloadParams = {
  network_json: Record<string, unknown> | unknown[] | string;
};

/**
 * One method per tool. Extended by {@link GraphSolve}.
 *
 * Each method wraps `call()`, which handles retries, idempotency and
 * error mapping. Use `call()` directly for a tool newer than this file.
 */
export abstract class GeneratedMethods {
  abstract call<T = unknown>(
    tool: string,
    args?: ToolArguments,
  ): Promise<ToolResponse<T>>;

  /**
   * Tune the heavy/light split of a composition to match a target surface
   * GOR (Sm³/Sm³).
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  adjust_composition_to_gor(
    params: AdjustCompositionToGorParams,
  ): Promise<ToolResponse> {
    return this.call("adjust_composition_to_gor", params);
  }

  /**
   * Tune a composition to a target in-situ GOR (m3/m3) at given P/T.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  adjust_composition_to_phase_ratio(
    params: AdjustCompositionToPhaseRatioParams,
  ): Promise<ToolResponse> {
    return this.call("adjust_composition_to_phase_ratio", params);
  }

  /**
   * Aggregate analog wells into a probabilistic type well: P10/P50/P90 EUR,
   * probability plot, representative declines, optional N-well program
   * aggregate.
   *
   * Costs 2 credits.
   */
  aggregate_type_well(params: AggregateTypeWellParams): Promise<ToolResponse> {
    return this.call("aggregate_type_well", params);
  }

  /**
   * Source-tagged production allocation (back-allocation): attribute
   * commingled rates back to each tagged source, including lift-gas
   * accounting.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  allocate_production(params: AllocateProductionParams): Promise<ToolResponse> {
    return this.call("allocate_production", params);
  }

  /**
   * Straight-line material-balance diagnostics: gas p/Z → OGIP, or Havlena-
   * Odeh F-vs-Et → STOIIP/GIIP, with R² and drive-support intercept.
   *
   * Costs 2 credits.
   */
  analyze_material_balance(
    params: AnalyzeMaterialBalanceParams,
  ): Promise<ToolResponse> {
    return this.call("analyze_material_balance", params);
  }

  /**
   * Analytical aquifer water influx We over a pressure history: Fetkovich
   * (PSS), Carter-Tracy or van Everdingen-Hurst (USS radial).
   *
   * Costs 2 credits.
   */
  calculate_aquifer_influx(
    params: CalculateAquiferInfluxParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_aquifer_influx", params);
  }

  /**
   * Pressure drop across a choke of known diameter at given rates (Sachdeva
   * multiphase model, critical/subcritical).
   *
   * Costs 1 credit.
   */
  calculate_choke_pressure_drop(
    params: CalculateChokePressureDropParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_choke_pressure_drop", params);
  }

  /**
   * Size a choke: find the bean diameter that gives a target downstream
   * pressure at the supplied rates.
   *
   * Costs 1 credit.
   */
  calculate_choke_size(
    params: CalculateChokeSizeParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_choke_size", params);
  }

  /**
   * Single-stage centrifugal compressor: outlet P/T and power from inlet
   * P/T, pressure ratio, and polytropic efficiency.
   *
   * Costs 1 credit.
   */
  calculate_compressor(
    params: CalculateCompressorParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_compressor", params);
  }

  /**
   * Internal CO2/H2S corrosion rate over three models (de Waard-Lotz, sour
   * with the Mariaca regime factor, NORSOK M-506), with NACE MR0175 region,
   * inhibited rate and wall allowance.
   *
   * Costs 2 credits.
   */
  calculate_corrosion_rate(
    params: CalculateCorrosionRateParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_corrosion_rate", params);
  }

  /**
   * API RP 14E erosional velocity limit and the actual mixture velocity for
   * a pipe.
   *
   * Costs 1 credit.
   */
  calculate_erosional_velocity(
    params: CalculateErosionalVelocityParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_erosional_velocity", params);
  }

  /**
   * Black-oil PVT properties (Bo, Rs, viscosity, density, Z) of oil / gas /
   * water at a given P/T using the selected correlations.
   *
   * Costs 1 credit.
   */
  calculate_fluid_properties(
    params: CalculateFluidPropertiesParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_fluid_properties", params);
  }

  /**
   * Gas dew-point pressure from temperature, gas gravity, oil API and
   * condensate-gas ratio (Ahmadi).
   *
   * Costs 1 credit.
   */
  calculate_gas_dew_point(
    params: CalculateGasDewPointParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_gas_dew_point", params);
  }

  /**
   * Heater/cooler in fixed_duty, fixed_temperature, or approach_temperature
   * mode; returns outlet P/T and duty.
   *
   * Costs 1 credit.
   */
  calculate_heater_cooler(
    params: CalculateHeaterCoolerParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_heater_cooler", params);
  }

  /**
   * Hydrate formation temperature at a given pressure (Baillie-Wichert) with
   * sour-gas corrections.
   *
   * Costs 1 credit.
   */
  calculate_hydrate_temperature(
    params: CalculateHydrateTemperatureParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_hydrate_temperature", params);
  }

  /**
   * Outlet temperature after a constant-enthalpy (Joule-Thomson) expansion
   * to a lower pressure.
   *
   * Costs 1 credit.
   */
  calculate_isenthalpic_temperature(
    params: CalculateIsenthalpicTemperatureParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_isenthalpic_temperature", params);
  }

  /**
   * Joule-Thomson throttle valve: outlet T after an isenthalpic pressure
   * drop.
   *
   * Costs 1 credit.
   */
  calculate_jt_valve(params: CalculateJtValveParams): Promise<ToolResponse> {
    return this.call("calculate_jt_valve", params);
  }

  /**
   * Minimum miscibility pressure (MMP) between a reservoir fluid and an
   * injection gas (mixing-cell method).
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  calculate_mmp(params: CalculateMmpParams): Promise<ToolResponse> {
    return this.call("calculate_mmp", params);
  }

  /**
   * Multiphase-flow-meter allocation: distribute measured rates among
   * streams (in-situ -> standard conditions).
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  calculate_mpfm_allocation(
    params: CalculateMpfmAllocationParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_mpfm_allocation", params);
  }

  /**
   * Multi-stage centrifugal train with optional inter-stage cooling; per-
   * stage pressure ratios and overall discharge P/T, cooler duty and shaft
   * power.
   *
   * Costs 1 credit.
   */
  calculate_multistage_compressor(
    params: CalculateMultistageCompressorParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_multistage_compressor", params);
  }

  /**
   * Operating point = IPR intersect VLP. Generates both curves and finds the
   * stabilised rate and flowing BHP.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  calculate_nodal_analysis(
    params: CalculateNodalAnalysisParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_nodal_analysis", params);
  }

  /**
   * Watercut / GOR / phase fractions from volumetric phase rates (any flow-
   * rate units).
   *
   * Costs 1 credit.
   */
  calculate_phase_cuts(
    params: CalculatePhaseCutsParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_phase_cuts", params);
  }

  /**
   * March pressure and temperature along a single pipe (multi-segment) with
   * heat transfer.
   *
   * Costs 3 credits.
   */
  calculate_pipe_traverse(
    params: CalculatePipeTraverseParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_pipe_traverse", params);
  }

  /**
   * Multiphase pressure gradient for a single pipe segment using a chosen
   * correlation. Returns gradient components, holdup and flow regime.
   *
   * Costs 1 credit.
   */
  calculate_pressure_drop(
    params: CalculatePressureDropParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_pressure_drop", params);
  }

  /**
   * ESP/pump outlet pressure from a head-vs-rate performance curve,
   * interpolated at the operating rate and converted with liquid density.
   *
   * Costs 1 credit.
   */
  calculate_pump_head(params: CalculatePumpHeadParams): Promise<ToolResponse> {
    return this.call("calculate_pump_head", params);
  }

  /**
   * Find the rates a fixed choke passes for a given pressure drop, holding a
   * phase or ratio (watercut/GOR/WGR/CGR) fixed.
   *
   * Costs 1 credit.
   */
  calculate_rate_from_choke(
    params: CalculateRateFromChokeParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_rate_from_choke", params);
  }

  /**
   * Reciprocating (positive-displacement) compressor. Mass flow is set by
   * displacement x speed x volumetric efficiency, not supplied.
   *
   * Costs 1 credit.
   */
  calculate_reciprocating_compressor(
    params: CalculateReciprocatingCompressorParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_reciprocating_compressor", params);
  }

  /**
   * Reid vapour pressure (RVP) of a liquid composition at 100 degF.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  calculate_reid_vapour_pressure(
    params: CalculateReidVapourPressureParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_reid_vapour_pressure", params);
  }

  /**
   * Bubble- or dew-point pressure of a composition at a given temperature
   * (cubic EOS).
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  calculate_saturation_pressure(
    params: CalculateSaturationPressureParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_saturation_pressure", params);
  }

  /**
   * Screw (positive-displacement) compressor closed by black-box
   * efficiencies; flow set by displacement x shaft speed x volumetric
   * efficiency.
   *
   * Costs 1 credit.
   */
  calculate_screw_compressor(
    params: CalculateScrewCompressorParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_screw_compressor", params);
  }

  /**
   * Single-stage centrifugal turbine/expander: outlet P/T and power
   * generated from inlet P/T and an expansion pressure ratio (0 < PR < 1).
   *
   * Costs 1 credit.
   */
  calculate_turbine(params: CalculateTurbineParams): Promise<ToolResponse> {
    return this.call("calculate_turbine", params);
  }

  /**
   * Forward OOIP / OGIP from area·thickness·NTG·φ·(1−Sw) ÷ FVF; scalar or
   * per-realisation arrays, composes with generate_samples /
   * compute_statistics for probabilistic in-place volumes.
   *
   * Costs 2 credits.
   */
  calculate_volumetrics(
    params: CalculateVolumetricsParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_volumetrics", params);
  }

  /**
   * Brine properties (density, viscosity, compressibility, FVF) via IAPWS-95
   * with a salinity correction.
   *
   * Costs 1 credit.
   */
  calculate_water_properties(
    params: CalculateWaterPropertiesParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_water_properties", params);
  }

  /**
   * Wax deposition rate on a cold wall from the subcooling and the wax
   * solubility gradient — thickness in mm/yr and mass flux in g/m2/day.
   *
   * Costs 2 credits.
   */
  calculate_wax_deposition_rate(
    params: CalculateWaxDepositionRateParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_wax_deposition_rate", params);
  }

  /**
   * Critical properties (Tc, Pc, Vc, Watson K) of a pseudo-component from
   * any two of MW / specific gravity / boiling point.
   *
   * Costs 1 credit.
   */
  characterize_pseudo_component(
    params: CharacterizePseudoComponentParams,
  ): Promise<ToolResponse> {
    return this.call("characterize_pseudo_component", params);
  }

  /**
   * Walk a multi-segment pipe with each correlation and compare the
   * predicted outlet pressure.
   *
   * Costs 3 credits.
   */
  compare_pipeline_correlations(
    params: ComparePipelineCorrelationsParams,
  ): Promise<ToolResponse> {
    return this.call("compare_pipeline_correlations", params);
  }

  /**
   * Run every multiphase pressure-drop correlation (or a subset) on one
   * segment and compare the predicted gradients.
   *
   * Costs 3 credits.
   */
  compare_pressure_drop_correlations(
    params: ComparePressureDropCorrelationsParams,
  ): Promise<ToolResponse> {
    return this.call("compare_pressure_drop_correlations", params);
  }

  /**
   * Summary statistics (mean/std, percentiles, histogram, CDF) over labelled
   * output columns, plus optional input→output correlation sensitivity;
   * network-agnostic (e.g. NPV/EUR from an external model).
   *
   * Costs 2 credits.
   */
  compute_statistics(params: ComputeStatisticsParams): Promise<ToolResponse> {
    return this.call("compute_statistics", params);
  }

  /**
   * Convert a value between units (pressure, rate, temperature, length,
   * area, ...).
   *
   * Free.
   */
  convert_units(params: ConvertUnitsParams): Promise<ToolResponse> {
    return this.call("convert_units", params);
  }

  /**
   * Hydrate, corrosion and erosion screening along a whole line, each
   * reporting the controlling sample — index, length, pressure and
   * temperature — not just a worst-case number.
   *
   * Costs 8 credits, plus 1 per 250 ms beyond the first 4 s of compute.
   */
  evaluate_flow_assurance_profile(
    params: EvaluateFlowAssuranceProfileParams,
  ): Promise<ToolResponse> {
    return this.call("evaluate_flow_assurance_profile", params);
  }

  /**
   * History-match a decline model (Arps / mod-hyperbolic / Duong / SEPD /
   * PLE) to a time/rate history with optional outlier rejection; omit kind
   * to auto-select by R².
   *
   * Costs 2 credits.
   */
  fit_decline(params: FitDeclineParams): Promise<ToolResponse> {
    return this.call("fit_decline", params);
  }

  /**
   * Flash a reservoir composition to stock-tank: GOR, API/oil density,
   * shrinkage.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  flash_to_surface(params: FlashToSurfaceParams): Promise<ToolResponse> {
    return this.call("flash_to_surface", params);
  }

  /**
   * Generate a decline curve (rate + cumulative) from known parameters:
   * Arps, modified-hyperbolic, Duong, stretched-exponential (SEPD), power-
   * law-exponential (PLE).
   *
   * Costs 2 credits.
   */
  generate_decline(params: GenerateDeclineParams): Promise<ToolResponse> {
    return this.call("generate_decline", params);
  }

  /**
   * Inflow performance relationship (IPR) curve for one well; real physics
   * per point. Required inputs depend on ipr_model.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  generate_ipr_curve(params: GenerateIprCurveParams): Promise<ToolResponse> {
    return this.call("generate_ipr_curve", params);
  }

  /**
   * Two-phase P-T envelope (dew/bubble locus + critical point) of a
   * composition (cubic EOS only; not GERG).
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  generate_phase_envelope(
    params: GeneratePhaseEnvelopeParams,
  ): Promise<ToolResponse> {
    return this.call("generate_phase_envelope", params);
  }

  /**
   * Draw Monte-Carlo / LHS / Sobol samples from named distributions
   * (Normal/Uniform/LogNormal/Triangular/Beta/PERT/Discrete) with optional
   * correlations; simple or Saltelli design; network-agnostic, feeds any
   * model.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 3 s of compute.
   */
  generate_samples(params: GenerateSamplesParams): Promise<ToolResponse> {
    return this.call("generate_samples", params);
  }

  /**
   * Dimensionless transient type-curve surface (pD/qD, Bourdet derivative)
   * for a transient ipr_model.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  generate_type_curve(params: GenerateTypeCurveParams): Promise<ToolResponse> {
    return this.call("generate_type_curve", params);
  }

  /**
   * Critical properties (Tc, Pc, omega, MW) of a single EOS component by
   * name.
   *
   * Free.
   */
  get_eos_component(params: GetEosComponentParams): Promise<ToolResponse> {
    return this.call("get_eos_component", params);
  }

  /**
   * List the available correlations, optionally filtered by category.
   *
   * Free.
   */
  list_correlations(params: ListCorrelationsParams): Promise<ToolResponse> {
    return this.call("list_correlations", params);
  }

  /**
   * List the network edge types (pipeline, choke, compressor, ...) and their
   * params.
   *
   * Free.
   */
  list_edge_types(): Promise<ToolResponse> {
    return this.call("list_edge_types", {});
  }

  /**
   * Peng-Robinson binary interaction parameters for a component.
   *
   * Free.
   */
  list_eos_binary_interactions(
    params: ListEosBinaryInteractionsParams,
  ): Promise<ToolResponse> {
    return this.call("list_eos_binary_interactions", params);
  }

  /**
   * List the component names in the built-in equation-of-state database.
   *
   * Free.
   */
  list_eos_components(): Promise<ToolResponse> {
    return this.call("list_eos_components", {});
  }

  /**
   * The model vocabularies the flow-assurance tools accept — hydrate, wax
   * and corrosion models with their tier, plus inhibitor names and vdWP
   * guests.
   *
   * Free.
   */
  list_flow_assurance_models(): Promise<ToolResponse> {
    return this.call("list_flow_assurance_models", {});
  }

  /**
   * List the fluid types (oil, gas, water) and the required PVT
   * configuration.
   *
   * Free.
   */
  list_fluid_types(): Promise<ToolResponse> {
    return this.call("list_fluid_types", {});
  }

  /**
   * List the network node types (junction, source, sink) and their params.
   *
   * Free.
   */
  list_node_types(): Promise<ToolResponse> {
    return this.call("list_node_types", {});
  }

  /**
   * Which of the three transient schemes to use, what each carries, and the
   * conventions every transient request shares.
   *
   * Free.
   */
  list_transient_solvers(): Promise<ToolResponse> {
    return this.call("list_transient_solvers", {});
  }

  /**
   * Fit the Fetkovich C and n (deliverability) coefficients to test points.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  match_fetkovich_ab(params: MatchFetkovichAbParams): Promise<ToolResponse> {
    return this.call("match_fetkovich_ab", params);
  }

  /**
   * Fit the Forchheimer A and B (non-Darcy gas) coefficients to test points.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  match_forchheimer_ab(
    params: MatchForchheimerAbParams,
  ): Promise<ToolResponse> {
    return this.call("match_forchheimer_ab", params);
  }

  /**
   * Unified matching dispatcher: fit productivity_index / skin /
   * forchheimer_ab / fetkovich_ab, routed by match_target or ipr_model.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  match_parameters(params: MatchParametersParams): Promise<ToolResponse> {
    return this.call("match_parameters", params);
  }

  /**
   * Fit the productivity index to observed (rate, flowing-BHP) test points.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  match_productivity_index(
    params: MatchProductivityIndexParams,
  ): Promise<ToolResponse> {
    return this.call("match_productivity_index", params);
  }

  /**
   * Regress black-oil PVT correlations against measured lab data (bubble
   * point, Rs, Bo, viscosity, density).
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  match_pvt(params: MatchPvtParams): Promise<ToolResponse> {
    return this.call("match_pvt", params);
  }

  /**
   * Fit the skin factor to observed (rate, flowing-BHP) test points.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  match_skin(params: MatchSkinParams): Promise<ToolResponse> {
    return this.call("match_skin", params);
  }

  /**
   * Optimise a network: move control variables within bounds to maximise /
   * minimise an objective (phase rate, revenue, pressure drop, power) under
   * rate / pressure / resource constraints. The optimisation counterpart of
   * solve_network.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  optimise_network(params: OptimiseNetworkParams): Promise<ToolResponse> {
    return this.call("optimise_network", params);
  }

  /**
   * Rate-transient-analysis diagnostics (Blasingame, Agarwal-Gardner,
   * Bourdet) with material-balance time and pseudo-time, from a
   * time/rate/pressure history.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  rta_diagnostics(params: RtaDiagnosticsParams): Promise<ToolResponse> {
    return this.call("rta_diagnostics", params);
  }

  /**
   * Constant composition expansion (CCE) PVT experiment.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  run_cce(params: RunCceParams): Promise<ToolResponse> {
    return this.call("run_cce", params);
  }

  /**
   * Constant volume depletion (CVD) PVT experiment (gas condensate /
   * volatile oil).
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  run_cvd(params: RunCvdParams): Promise<ToolResponse> {
    return this.call("run_cvd", params);
  }

  /**
   * Differential liberation expansion (DLE) PVT experiment.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  run_dle(params: RunDleParams): Promise<ToolResponse> {
    return this.call("run_dle", params);
  }

  /**
   * PT flash of a composition (Peng-Robinson default, SRK, Patel-Teja):
   * phase split, K-values, densities. Pressure in MPa.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  run_eos_flash(params: RunEosFlashParams): Promise<ToolResponse> {
    return this.call("run_eos_flash", params);
  }

  /**
   * Time-series production forecast: march a network through time, solving
   * or optimising each step with decline curves, scheduled events and
   * transient-IPR wells; returns per-timestep rates and cumulatives.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  run_forecast(params: RunForecastParams): Promise<ToolResponse> {
    return this.call("run_forecast", params);
  }

  /**
   * Material-balance depletion / reserves over one or more reservoir blocks
   * (STOIIP/GIIP, recovery, pressure decline, aquifer).
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  run_material_balance(
    params: RunMaterialBalanceParams,
  ): Promise<ToolResponse> {
    return this.call("run_material_balance", params);
  }

  /**
   * Trace component compositions through an already-solved network.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  run_molecule_tracking(
    params: RunMoleculeTrackingParams,
  ): Promise<ToolResponse> {
    return this.call("run_molecule_tracking", params);
  }

  /**
   * Sensitivity / parametric sweep over a network (linear / tornado / grid /
   * Monte Carlo); returns per-output statistics.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  run_parametric_study(
    params: RunParametricStudyParams,
  ): Promise<ToolResponse> {
    return this.call("run_parametric_study", params);
  }

  /**
   * Simulate a process flow graph (mixers, separators, heaters, compressors)
   * of compositional streams.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  run_process_graph(params: RunProcessGraphParams): Promise<ToolResponse> {
    return this.call("run_process_graph", params);
  }

  /**
   * Multi-stage separator test to stock-tank: stage GOR, FVF, stock-tank oil
   * density.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  run_separator_test(params: RunSeparatorTestParams): Promise<ToolResponse> {
    return this.call("run_separator_test", params);
  }

  /**
   * Swelling test: add injection gas/solvent and report saturation pressure
   * + swelling factor per step.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  run_swelling_test(params: RunSwellingTestParams): Promise<ToolResponse> {
    return this.call("run_swelling_test", params);
  }

  /**
   * Transient multiphase pipe flow, fully-implicit four-field two-fluid
   * (Graphsolve-Field): for severe slugging and countercurrent flow.
   *
   * Costs 250 credits, plus 1 per 50 ms beyond the first 30 s of compute.
   */
  run_transient_field(params: RunTransientFieldParams): Promise<ToolResponse> {
    return this.call("run_transient_field", params);
  }

  /**
   * Transient multiphase pipe flow, semi-implicit sequential (Graphsolve-
   * Flux): the general-purpose scheme, carrying temperature, composition and
   * salinity.
   *
   * Costs 250 credits, plus 1 per 50 ms beyond the first 30 s of compute.
   */
  run_transient_flux(params: RunTransientFluxParams): Promise<ToolResponse> {
    return this.call("run_transient_flux", params);
  }

  /**
   * Transient multiphase pipe flow, explicit Godunov/Rusanov drift flux
   * (Graphsolve-Wave): the only scheme that resolves pressure waves.
   *
   * Costs 250 credits, plus 1 per 50 ms beyond the first 30 s of compute.
   */
  run_transient_wave(params: RunTransientWaveParams): Promise<ToolResponse> {
    return this.call("run_transient_wave", params);
  }

  /**
   * De Boer (1995) asphaltene-onset screening from in-situ undersaturation
   * and live-oil density; four-bin risk classification.
   *
   * Costs 2 credits.
   */
  screen_asphaltene_risk(
    params: ScreenAsphalteneRiskParams,
  ): Promise<ToolResponse> {
    return this.call("screen_asphaltene_risk", params);
  }

  /**
   * Hydrate formation temperature and margin at a live state, over four
   * models (Towler-Mokhatab screening or vdW-Platteeuw sI / sII / combined),
   * with Hammerschmidt inhibitor depression.
   *
   * Costs 2 credits.
   */
  screen_hydrate_risk(params: ScreenHydrateRiskParams): Promise<ToolResponse> {
    return this.call("screen_hydrate_risk", params);
  }

  /**
   * Turner critical-velocity screen for gas-well liquid loading: critical
   * velocity, ratio and a loading flag.
   *
   * Costs 2 credits.
   */
  screen_liquid_loading(
    params: ScreenLiquidLoadingParams,
  ): Promise<ToolResponse> {
    return this.call("screen_liquid_loading", params);
  }

  /**
   * Wax appearance temperature and margin: a C7+ screening correlation (Tier
   * 1) or the Won multi-solid SLE (Tier 2) with per-component solid
   * fractions.
   *
   * Costs 2 credits.
   */
  screen_wax_risk(params: ScreenWaxRiskParams): Promise<ToolResponse> {
    return this.call("screen_wax_risk", params);
  }

  /**
   * Solve a complete production network (Newton-Raphson): pressures,
   * temperatures and rates at every node and edge. The headline tool.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  solve_network(params: SolveNetworkParams): Promise<ToolResponse> {
    return this.call("solve_network", params);
  }

  /**
   * Data-reconciliation solve (MAP): inject measured values with a variance
   * and reconcile them, returning a posterior variance per reconciled
   * quantity.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  solve_network_map(params: SolveNetworkMapParams): Promise<ToolResponse> {
    return this.call("solve_network_map", params);
  }

  /**
   * Split a heavy/plus fraction into N pseudo-components (gamma
   * distribution). Front half of building a matched compositional fluid.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  split_plus_fraction(params: SplitPlusFractionParams): Promise<ToolResponse> {
    return this.call("split_plus_fraction", params);
  }

  /**
   * Compute the flowing-BHP history from a rate history (transient
   * superposition).
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  transient_bhp_history(
    params: TransientBhpHistoryParams,
  ): Promise<ToolResponse> {
    return this.call("transient_bhp_history", params);
  }

  /**
   * Compute the rate history from a pressure history (transient
   * superposition).
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  transient_rate_history(
    params: TransientRateHistoryParams,
  ): Promise<ToolResponse> {
    return this.call("transient_rate_history", params);
  }

  /**
   * Regress MPFM allocation parameters against measured observation rows
   * (inverse of calculate_mpfm_allocation).
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  tune_mpfm_allocation(
    params: TuneMpfmAllocationParams,
  ): Promise<ToolResponse> {
    return this.call("tune_mpfm_allocation", params);
  }

  /**
   * Pre-flight check a network payload without solving (runs the same build
   * step solve_network does). Run this first when assembling a network.
   *
   * Free.
   */
  validate_solver_payload(
    params: ValidateSolverPayloadParams,
  ): Promise<ToolResponse> {
    return this.call("validate_solver_payload", params);
  }
}
