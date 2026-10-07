/**
 * Typed methods for every tool in the GraphSolve API.
 *
 * GENERATED FROM spec/graphsolve-v1.json — DO NOT EDIT BY HAND.
 * Run `python emit/emit_typescript.py` after a spec change; CI fails if
 * this file and the spec disagree.
 *
 * Engine API version: 1.0.42
 * Tools: 101
 */

import type { ToolArguments, ToolResponse } from "./types.js";

/** Every tool this client knows about. */
export type ToolName =
  | "adjust_composition_to_gor"
  | "adjust_composition_to_phase_ratio"
  | "aggregate_type_well"
  | "allocate_production"
  | "analyse_turbo_performance"
  | "analyze_material_balance"
  | "calculate_aquifer_influx"
  | "calculate_choke_pressure_drop"
  | "calculate_choke_size"
  | "calculate_compression_train"
  | "calculate_compressor"
  | "calculate_corrosion_rate"
  | "calculate_critical_point"
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
  | "calculate_pump"
  | "calculate_pump_head"
  | "calculate_rate_from_choke"
  | "calculate_reciprocating_compressor"
  | "calculate_reid_vapour_pressure"
  | "calculate_saturation_pressure"
  | "calculate_screw_compressor"
  | "calculate_turbine"
  | "calculate_turbo_machine"
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
  | "import_prp_fluid"
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
  | "partition_acid_gas_in_water"
  | "rta_diagnostics"
  | "run_cce"
  | "run_cvd"
  | "run_dle"
  | "run_eos_flash"
  | "run_forecast"
  | "run_gas_depletion"
  | "run_material_balance"
  | "run_mmp_probe"
  | "run_molecule_tracking"
  | "run_nodal_study"
  | "run_parametric_study"
  | "run_process_graph"
  | "run_pvt_regression_suite"
  | "run_separator_test"
  | "run_swelling_test"
  | "run_transient_field"
  | "run_transient_flux"
  | "run_transient_wave"
  | "screen_asphaltene_risk"
  | "screen_hydrate_risk"
  | "screen_liquid_loading"
  | "screen_scale_risk"
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
  | "SoaveRedlichKwong";

export type EosViscosityModelName =
  | "auto"
  | "lucas"
  | "lbc"
  | "wilke"
  | "herning_zipperer"
  | "chapman_enskog"
  | "pedersen"
  | "trapp"
  | "expanded_fluid"
  | "burgoyne_nielsen_stanko";

export type FlashEosModelName =
  | "PengRobinson"
  | "SoaveRedlichKwong"
  | "Gerg2008";

export type FlashTypeName =
  | "pt"
  | "ph"
  | "ps";

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
  | "QC-high"
  | "QC-low"
  | "SinglePhaseGas"
  | "SUPREME";

export type HeaterCoolerModeName =
  | "fixed_duty"
  | "fixed_outlet_temperature"
  | "approach_temperature"
  | "ua";

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

export type MatchTargetName =
  | "productivity_index"
  | "skin"
  | "forchheimer_ab"
  | "fetkovich_ab";

export type PipeEnd =
  | "top"
  | "bottom";

export type PseudoCriticalMethodName =
  | "kesler_lee"
  | "twu"
  | "sancet";

export type SaturationAlgorithmName =
  | "classical"
  | "bell_jaeger"
  | "multi_start"
  | "envelope";

export type SaturationBoundaryName =
  | "bubble"
  | "dew";

export type ScaleMineralName =
  | "calcite"
  | "aragonite"
  | "siderite"
  | "barite"
  | "celestite"
  | "gypsum"
  | "anhydrite"
  | "halite";

export type SlipModelName =
  | "no_slip"
  | "gromles"
  | "hydro"
  | "constant_slip"
  | "fauske"
  | "moddy"
  | "simpson"
  | "thom"
  | "baroczy"
  | "lockhart_martenelli";

export type TurboModeName =
  | "compressor"
  | "expander";

export type WaxActivityModelName =
  | "regular_solution"
  | "ideal";

export type WaxModelName =
  | "screening"
  | "won"
  | "sle";

export type AdjustCompositionToGorParams = {
  mole_fractions: number[];
  target_gor: number;
  binary_interactions?: Record<string, unknown>[] | null;
  component_names?: string[] | null;
  components?: Record<string, unknown>[] | null;
  eos_model?: EosModelName | null;
  separator_pressures_mpa?: number[] | null;
  separator_temperatures_k?: number[] | null;
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

export type AnalyseTurboPerformanceParams = {
  points: Record<string, unknown>[];
  co2_fraction?: number | null;
  composition?: Record<string, unknown> | null;
  gas_molecular_weight?: number | null;
  h2s_fraction?: number | null;
  methods?: string[] | null;
  mode?: TurboModeName | null;
  n2_fraction?: number | null;
};

export type AnalyzeMaterialBalanceParams = {
  analysis_json: Record<string, unknown> | unknown[] | string;
};

export type CalculateAquiferInfluxParams = {
  aquifer_json: Record<string, unknown> | unknown[] | string;
};

export type CalculateChokePressureDropParams = {
  choke_diameter: number;
  inlet_pressure: number;
  inlet_temperature: number;
  composition?: Record<string, unknown> | null;
  discharge_coefficient?: number | null;
  dissolved_gas_ratio?: number | null;
  gas_mw?: number | null;
  gas_rate?: number;
  oil_density?: number | null;
  oil_rate?: number;
  perry_multiplier?: number | null;
  pipe_diameter_ratio?: number | null;
  slip_model?: SlipModelName | null;
  water_rate?: number;
  water_salinity?: number | null;
};

export type CalculateChokeSizeParams = {
  downstream_pressure: number;
  free_gas_rate: number;
  oil_rate: number;
  upstream_pressure: number;
  upstream_temperature: number;
  water_rate: number;
  discharge_coefficient?: number | null;
  dissolved_gor?: number | null;
  gas_mw?: number | null;
  oil_density?: number | null;
  perry_multiplier?: number | null;
  pipe_diameter_ratio?: number | null;
  slip_model?: SlipModelName | null;
  water_salinity?: number | null;
};

export type CalculateCompressionTrainParams = {
  composition: Record<string, unknown>;
  inlet_pressure: number;
  inlet_temperature: number;
  stages: Record<string, unknown>[];
};

export type CalculateCompressorParams = {
  inlet_pressure: number;
  inlet_temperature: number;
  pressure_ratio: number;
  composition?: Record<string, unknown> | null;
  fluid?: Record<string, unknown> | null;
  isentropic_efficiency?: number | null;
  mechanical_efficiency?: number | null;
  method?: string | null;
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

export type CalculateCriticalPointParams = {
  mole_fractions: number[];
  binary_interactions?: Record<string, unknown>[] | null;
  component_names?: string[] | null;
  components?: Record<string, unknown>[] | null;
  eos_model?: EosModelName | null;
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
  condensate_gas_ratio_stb_per_mmscf: number;
  gas_gravity: number;
  oil_api_gravity: number;
  temperature: number;
};

export type CalculateHeaterCoolerParams = {
  inlet_pressure: number;
  inlet_temperature: number;
  mode: HeaterCoolerModeName;
  approach_temperature?: number | null;
  composition?: Record<string, unknown> | null;
  fluid?: Record<string, unknown> | null;
  heat_duty?: number | null;
  max_duty?: number | null;
  outlet_temperature?: number | null;
  pressure_drop?: number | null;
  ua?: number | null;
  utility_temperature?: number | null;
};

export type CalculateHydrateTemperatureParams = {
  gas_gravity: number;
  pressure: number;
  co2_mole_fraction?: number | null;
  h2s_mole_fraction?: number | null;
};

export type CalculateIsenthalpicTemperatureParams = {
  inlet_pressure: number;
  inlet_temperature: number;
  outlet_pressure: number;
  composition?: Record<string, unknown> | null;
  fluid?: Record<string, unknown> | null;
};

export type CalculateJtValveParams = {
  inlet_pressure: number;
  inlet_temperature: number;
  pressure_drop: number;
  composition?: Record<string, unknown> | null;
  fluid?: Record<string, unknown> | null;
};

export type CalculateMmpParams = {
  experiment_json: Record<string, unknown> | unknown[] | string;
};

export type CalculateMpfmAllocationParams = {
  allocation_json: Record<string, unknown> | unknown[] | string;
};

export type CalculateMultistageCompressorParams = {
  inlet_pressure: number;
  inlet_temperature: number;
  stages: Record<string, unknown>[];
  composition?: Record<string, unknown> | null;
  fluid?: Record<string, unknown> | null;
  intercool_pressure_drop?: number | null;
  intercool_temperature?: number | null;
  mechanical_efficiency?: number | null;
  method?: string | null;
};

export type CalculateNodalAnalysisParams = {
  ipr: unknown;
  tubing_diameter: number;
  tubing_length: number;
  tubing_roughness: number;
  wellhead_pressure: number;
  wellhead_temperature: number;
  dissolved_gas_ratio?: number | null;
  flow_correlation?: FlowCorrelationName | null;
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
  flow_boundary?: PipeEnd | null;
  flow_correlation?: FlowCorrelationName | null;
  gas_mw?: number | null;
  heat_transfer_coefficient?: number | null;
  oil_density?: number | null;
  pressure_boundary?: PipeEnd | null;
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

export type CalculatePumpParams = {
  inlet_pressure: number;
  inlet_temperature: number;
  centrifugal?: Record<string, unknown> | null;
  composition?: Record<string, unknown> | null;
  efficiency?: number | null;
  fluid?: Record<string, unknown> | null;
  head_curve?: number[][];
  mechanical_efficiency?: number | null;
  minor_loss_coefficient?: number | null;
  nozzle_diameter?: number | null;
  npsh_required?: number | null;
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
  discharge_coefficient?: number | null;
  dissolved_gor?: number | null;
  fixed?: string | null;
  free_gas_rate?: number | null;
  gas_mw?: number | null;
  gor?: number | null;
  oil_density?: number | null;
  oil_rate?: number | null;
  perry_multiplier?: number | null;
  pipe_diameter_ratio?: number | null;
  slip_model?: SlipModelName | null;
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
  composition?: Record<string, unknown> | null;
  gas_molecular_weight?: number | null;
  h2s_fraction?: number | null;
  max_pressure_ratio?: number | null;
  mechanical_efficiency?: number | null;
  method?: string | null;
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
  mole_fractions: number[];
  algorithm?: SaturationAlgorithmName | null;
  binary_interactions?: Record<string, unknown>[] | null;
  boundary?: SaturationBoundaryName | null;
  component_names?: string[] | null;
  components?: Record<string, unknown>[] | null;
  eos_model?: EosModelName | null;
  pressure_mpa?: number | null;
  temperature_k?: number | null;
};

export type CalculateScrewCompressorParams = {
  discharge_pressure: number;
  displacement_per_rev_m3: number;
  inlet_pressure: number;
  inlet_temperature: number;
  shaft_speed_rev_s: number;
  co2_fraction?: number | null;
  composition?: Record<string, unknown> | null;
  gas_molecular_weight?: number | null;
  h2s_fraction?: number | null;
  mechanical_efficiency?: number | null;
  method?: string | null;
  n2_fraction?: number | null;
  polytropic_efficiency?: number | null;
  subtype?: string | null;
  volumetric_efficiency?: number | null;
};

export type CalculateTurbineParams = {
  inlet_pressure: number;
  inlet_temperature: number;
  pressure_ratio: number;
  composition?: Record<string, unknown> | null;
  fluid?: Record<string, unknown> | null;
  isentropic_efficiency?: number | null;
  mechanical_efficiency?: number | null;
  method?: string | null;
  polytropic_efficiency?: number | null;
};

export type CalculateTurboMachineParams = {
  inlet_pressure: number;
  inlet_temperature: number;
  turbo_machine: Record<string, unknown>;
  composition?: Record<string, unknown> | null;
  discharge_pressure?: number | null;
  fluid?: Record<string, unknown> | null;
  mechanical_efficiency?: number | null;
  mode?: TurboModeName | null;
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
  method?: PseudoCriticalMethodName | null;
  molecular_weight?: number | null;
  specific_gravity?: number | null;
};

export type ComparePipelineCorrelationsParams = {
  fluid: unknown;
  inlet: Record<string, unknown>;
  pipe_segments: unknown;
  correlations?: FlowCorrelationName[] | null;
  reference_measured_depth?: number | null;
  reference_true_vertical_depth?: number | null;
  reference_x?: number | null;
  reference_y?: number | null;
  reference_z?: number | null;
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
  correlations?: FlowCorrelationName[] | null;
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
  separator_pressures_mpa?: number[] | null;
  separator_temperatures_k?: number[] | null;
};

export type GenerateDeclineParams = {
  decline_json: Record<string, unknown> | unknown[] | string;
};

export type GenerateIprCurveParams = {
  inflow_model: unknown;
  n_points?: number | null;
};

export type GeneratePhaseEnvelopeParams = {
  mole_fractions: number[];
  binary_interactions?: Record<string, unknown>[] | null;
  component_names?: string[] | null;
  components?: Record<string, unknown>[] | null;
  eos_model?: EosModelName | null;
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

export type ImportPrpFluidParams = {
  prp_text: string;
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
  match_target?: MatchTargetName | null;
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

export type PartitionAcidGasInWaterParams = {
  pressure: number;
  temperature: number;
  water_analysis: Record<string, unknown>;
  co2_fugacity_bar?: number | null;
  co2_mass_rate?: number | null;
  co2_mole_fraction?: number | null;
  h2s_fugacity_bar?: number | null;
  h2s_mass_rate?: number | null;
  h2s_mole_fraction?: number | null;
  ph?: number | null;
  water_mass_rate?: number | null;
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
  binary_interactions?: Record<string, unknown>[] | null;
  component_names?: string[] | null;
  components?: Record<string, unknown>[] | null;
  enthalpy_j_per_mol?: number | null;
  entropy_j_per_mol_k?: number | null;
  eos_model?: FlashEosModelName | null;
  flash_type?: FlashTypeName | null;
  gas_viscosity_model?: EosViscosityModelName | null;
  include_properties?: boolean | null;
  liquid_viscosity_model?: EosViscosityModelName | null;
  temperature_k?: number | null;
};

export type RunForecastParams = {
  forecast_json: Record<string, unknown> | unknown[] | string;
};

export type RunGasDepletionParams = {
  experiment_json: Record<string, unknown> | unknown[] | string;
};

export type RunMaterialBalanceParams = {
  reservoir_json: Record<string, unknown> | unknown[] | string;
};

export type RunMmpProbeParams = {
  experiment_json: Record<string, unknown> | unknown[] | string;
};

export type RunMoleculeTrackingParams = {
  fluids_json: Record<string, unknown> | unknown[] | string;
  solved_network_json: Record<string, unknown> | unknown[] | string;
};

export type RunNodalStudyParams = {
  nodal_json: Record<string, unknown> | unknown[] | string;
};

export type RunParametricStudyParams = {
  study_json: Record<string, unknown> | unknown[] | string;
};

export type RunProcessGraphParams = {
  process_graph_json: Record<string, unknown> | unknown[] | string;
};

export type RunPvtRegressionSuiteParams = {
  experiment_json: Record<string, unknown> | unknown[] | string;
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

export type ScreenScaleRiskParams = {
  pressure: number;
  temperature: number;
  water_analysis: Record<string, unknown>;
  co2_fugacity_bar?: number | null;
  co2_mole_fraction?: number | null;
  h2s_fugacity_bar?: number | null;
  h2s_mole_fraction?: number | null;
  minerals?: ScaleMineralName[] | null;
  ph?: number | null;
  water_mass_rate?: number | null;
};

export type ScreenWaxRiskParams = {
  temperature: number;
  activity_model?: WaxActivityModelName | null;
  component_names?: string[] | null;
  density_c7_plus?: number | null;
  enthalpies_of_fusion?: number[] | null;
  enthalpies_of_transition?: number[] | null;
  heat_capacity_correction?: boolean | null;
  heat_capacity_of_fusion_j_per_mol_k?: number[] | null;
  liquid_molar_volume_cm3_per_mol?: number[] | null;
  liquid_solubility_parameter_mpa_half?: number[] | null;
  melting_points?: number[] | null;
  model?: WaxModelName | null;
  mole_fractions?: number[] | null;
  molecular_weights?: number[] | null;
  mw_c7_plus?: number | null;
  paraffin_mass_fraction?: number | null;
  solid_molar_volume_ratio?: number | null;
  transition_temperatures?: number[] | null;
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
   * Tune a composition to a target in-situ gas-oil volume ratio (m3/m3) at
   * given P/T.
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
   * Source-tagged production allocation (back-allocation): solve the network
   * and attribute the mass on every edge back to the sources it came from.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  allocate_production(params: AllocateProductionParams): Promise<ToolResponse> {
    return this.call("allocate_production", params);
  }

  /**
   * Back-calculate head, efficiencies and powers from measured suction and
   * discharge states by path method, and fit a turbo_machine map to the
   * points.
   *
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
   */
  analyse_turbo_performance(
    params: AnalyseTurboPerformanceParams,
  ): Promise<ToolResponse> {
    return this.call("analyse_turbo_performance", params);
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
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
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
   * Compression train on a composition: per stage a turbo machine, an
   * intercooler on the EOS enthalpy and a scrubber that removes the
   * condensed liquid; stage and train power, duty, liquid and compositions.
   *
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
   */
  calculate_compression_train(
    params: CalculateCompressionTrainParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_compression_train", params);
  }

  /**
   * Single-stage centrifugal compressor: outlet P/T and power from inlet
   * P/T, pressure ratio, and polytropic efficiency.
   *
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
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
   * True critical point of a mixture (Heidemann-Khalil) with a cubic EOS:
   * critical temperature, pressure, molar volume and Z-factor.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  calculate_critical_point(
    params: CalculateCriticalPointParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_critical_point", params);
  }

  /**
   * API RP 14E erosional velocity limit (1.22 C / sqrt(rho_mix) in SI), the
   * actual mixture velocity for a pipe, and the flow at which the pipe
   * reaches the limit.
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
   * Heater/cooler on the network edge's energy balance, in fixed_duty,
   * fixed_outlet_temperature, approach_temperature or ua mode; returns
   * outlet P/T and duty.
   *
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
   */
  calculate_heater_cooler(
    params: CalculateHeaterCoolerParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_heater_cooler", params);
  }

  /**
   * Hydrate formation temperature at a given pressure and gas gravity from
   * the Towler-Mokhatab screening correlation; H2S and CO2 inputs are
   * echoed, not applied.
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
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
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
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
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
   * Multiphase-flow-meter allocation: convert one meter reading (in-situ) to
   * standard-condition rates with an EOS.
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
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
   */
  calculate_multistage_compressor(
    params: CalculateMultistageCompressorParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_multistage_compressor", params);
  }

  /**
   * Operating point = IPR intersect VLP for one well and one tubing run.
   * Generates both curves and finds the stabilised rate and flowing BHP.
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
   * March pressure and temperature along a single straight pipe (one
   * diameter, length and angle) with heat transfer.
   *
   * Costs 3 credits.
   */
  calculate_pipe_traverse(
    params: CalculatePipeTraverseParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_pipe_traverse", params);
  }

  /**
   * Multiphase pressure gradient at one point in a pipe using a chosen
   * correlation. Returns gradient components, holdup, flow regime and
   * hydraulics.
   *
   * Costs 1 credit.
   */
  calculate_pressure_drop(
    params: CalculatePressureDropParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_pressure_drop", params);
  }

  /**
   * Centrifugal pump or ESP at one suction state on the network pump model:
   * discharge P/T, head, power, NPSH, per-section operating range and the
   * ESP drive train.
   *
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
   */
  calculate_pump(params: CalculatePumpParams): Promise<ToolResponse> {
    return this.call("calculate_pump", params);
  }

  /**
   * Single head-curve lookup: discharge pressure from one head-vs-rate curve
   * read at one flow with one liquid density. No stages, speed, gas or power
   * model; see calculate_pump.
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
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
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
   * Bubble- or dew-point pressure of a composition at a temperature, or
   * bubble- or dew-point temperature at a pressure (cubic EOS).
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
   * efficiencies; mass flow = volumetric efficiency x suction density x
   * displacement per revolution x shaft speed in rev/s.
   *
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
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
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
   */
  calculate_turbine(params: CalculateTurbineParams): Promise<ToolResponse> {
    return this.call("calculate_turbine", params);
  }

  /**
   * Any network turbo machine (centrifugal simple/mapped/map/multistage,
   * axial, screw, reciprocating) at one operating point, on a GERG gas or an
   * EOS composition, with a chosen path method.
   *
   * Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
   */
  calculate_turbo_machine(
    params: CalculateTurboMachineParams,
  ): Promise<ToolResponse> {
    return this.call("calculate_turbo_machine", params);
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
   * Brine properties (density, viscosity, compressibility, heat capacity,
   * enthalpy) with a salinity correction.
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
   * Critical properties (Tc, Pc, Vc), acentric factor and Watson K of a
   * pseudo-component from any two of MW / specific gravity / boiling point.
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
   * Flash a composition through a separator train to stock-tank: GOR, oil
   * density, specific gravity and API gravity, gas gravity.
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
   * per point. Required inputs depend on ipr_model (19 models).
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  generate_ipr_curve(params: GenerateIprCurveParams): Promise<ToolResponse> {
    return this.call("generate_ipr_curve", params);
  }

  /**
   * Two-phase P-T envelope (dew/bubble locus + critical point) of a
   * composition (Peng-Robinson or SRK).
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
   * for a transient ipr_model, with the dimensionless groups derived from
   * the geometry.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  generate_type_curve(params: GenerateTypeCurveParams): Promise<ToolResponse> {
    return this.call("generate_type_curve", params);
  }

  /**
   * Critical properties (Tc, Pc in MPa, omega, MW) of a single EOS component
   * by database name or short code.
   *
   * Free.
   */
  get_eos_component(params: GetEosComponentParams): Promise<ToolResponse> {
    return this.call("get_eos_component", params);
  }

  /**
   * Import a PVTsim .prp fluid file (text) as a composition for the PVT,
   * process-graph and MPFM tools.
   *
   * Costs 1 credit.
   */
  import_prp_fluid(params: ImportPrpFluidParams): Promise<ToolResponse> {
    return this.call("import_prp_fluid", params);
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
   * List the network edge types by name (no_pressure_loss, pipe, choke,
   * compressor, ...) and the fields of their data blocks.
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
   * List the component names in the built-in equation-of-state database and
   * the short codes (C1, CO2, iC4, ...) the compositional tools also accept.
   *
   * Free.
   */
  list_eos_components(): Promise<ToolResponse> {
    return this.call("list_eos_components", {});
  }

  /**
   * The model vocabularies the flow-assurance tools accept — hydrate, wax,
   * asphaltene, corrosion and scale models with their tier, plus how wax
   * formers are identified, inhibitor names, vdWP guests, the scale risk
   * bands, the NACE MR0175 regions and the API RP 14E service classes.
   *
   * Free.
   */
  list_flow_assurance_models(): Promise<ToolResponse> {
    return this.call("list_flow_assurance_models", {});
  }

  /**
   * List the black-oil fluid types (oil, gas, water): configuration fields,
   * correlation names and defaults, and the reported property keys with
   * units.
   *
   * Free.
   */
  list_fluid_types(): Promise<ToolResponse> {
    return this.call("list_fluid_types", {});
  }

  /**
   * List the network node types by name (fixed_rate_source,
   * fixed_pressure_source, pressure_dependent_source, network_node,
   * fixed_pressure_sink, fixed_rate_sink) and their fields.
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
   * Fit A and B of the fetkovich_ab oil deliverability equation q = A·(pr −
   * psat) + B·(psat² − pwf²) to test points.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  match_fetkovich_ab(params: MatchFetkovichAbParams): Promise<ToolResponse> {
    return this.call("match_fetkovich_ab", params);
  }

  /**
   * Fit A and B of the forchheimer_ab gas deliverability equation pr² − pwf²
   * = A·q + B·q² to test points.
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
   * Fit the pi model's productivity index to observed (oil rate, flowing-
   * BHP) test points.
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
   * Fit the mechanical skin of a darcy or fetkovich model to observed (oil
   * rate, flowing-BHP) test points.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  match_skin(params: MatchSkinParams): Promise<ToolResponse> {
    return this.call("match_skin", params);
  }

  /**
   * Optimise a network: move control variables within bounds to maximise /
   * minimise an objective (phase rate, revenue, pressure drop, power,
   * deviation from measurements) under rate / pressure / temperature /
   * velocity / power / pump / resource constraints. The optimisation
   * counterpart of solve_network.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  optimise_network(params: OptimiseNetworkParams): Promise<ToolResponse> {
    return this.call("optimise_network", params);
  }

  /**
   * How much CO2 and H2S is dissolved in the produced water, and the in-situ
   * pH that leaves — the brine pH the corrosion models should consume
   * instead of a condensed-water estimate.
   *
   * Costs 2 credits.
   */
  partition_acid_gas_in_water(
    params: PartitionAcidGasInWaterParams,
  ): Promise<ToolResponse> {
    return this.call("partition_acid_gas_in_water", params);
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
   * EOS flash of a composition (Peng-Robinson default, SRK, or GERG-2008 for
   * single-phase gas): PT, PH or PS; phase split, K-values, phase densities,
   * and optionally each phase's viscosity, enthalpy, entropy and heat
   * capacities. Pressure in MPa.
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
   * Gas-reservoir depletion study: p/z, Bg, recovery factor and retrograde
   * liquid per pressure step.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  run_gas_depletion(params: RunGasDepletionParams): Promise<ToolResponse> {
    return this.call("run_gas_depletion", params);
  }

  /**
   * Tank material balance over one or more reservoir zones (STOIIP/GIIP,
   * cumulative production, pressure decline, recovery factor); no aquifer.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  run_material_balance(
    params: RunMaterialBalanceParams,
  ): Promise<ToolResponse> {
    return this.call("run_material_balance", params);
  }

  /**
   * Mixing-cell miscibility test at one pressure: forward and backward
   * contact series and whether either reached miscibility.
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  run_mmp_probe(params: RunMmpProbeParams): Promise<ToolResponse> {
    return this.call("run_mmp_probe", params);
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
   * Nodal analysis of one well inside a network: IPR sweep, VLP by full
   * network solves, their operating point, and optional sensitivity
   * overlays.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  run_nodal_study(params: RunNodalStudyParams): Promise<ToolResponse> {
    return this.call("run_nodal_study", params);
  }

  /**
   * Sensitivity / parametric study over a network (single-variable sweep,
   * tornado, two-factor grid, Monte Carlo, envelope map); returns every run
   * plus the study's summary.
   *
   * Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
   */
  run_parametric_study(
    params: RunParametricStudyParams,
  ): Promise<ToolResponse> {
    return this.call("run_parametric_study", params);
  }

  /**
   * Steady-state compositional flowsheet of Source, Separator, Mixer,
   * Splitter and Sink nodes (no heaters or compressors), with recycle loops.
   *
   * Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
   */
  run_process_graph(params: RunProcessGraphParams): Promise<ToolResponse> {
    return this.call("run_process_graph", params);
  }

  /**
   * Run several PVT experiments on one EOS fluid against lab data: per-field
   * residuals and a weighted objective (evaluation, not tuning).
   *
   * Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
   */
  run_pvt_regression_suite(
    params: RunPvtRegressionSuiteParams,
  ): Promise<ToolResponse> {
    return this.call("run_pvt_regression_suite", params);
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
   * Flux): the general-purpose scheme, carrying temperature, the oil/water
   * split and salinity.
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
   * Mineral-scale saturation indices from a produced-water ion analysis —
   * calcite, aragonite, siderite, barite, celestite, gypsum, anhydrite and
   * halite — with the precipitable mass and the limiting ion.
   *
   * Costs 2 credits.
   */
  screen_scale_risk(params: ScreenScaleRiskParams): Promise<ToolResponse> {
    return this.call("screen_scale_risk", params);
  }

  /**
   * Wax appearance temperature and margin over three tiers: a C7+ screening
   * correlation, the Won multi-solid SLE, or the non-ideal SLE flash with
   * per-component solid fractions, measured melting data and the solubility
   * gradient for the deposition rate.
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
   * Data-reconciliation solve (MAP): reconcile pressure gauges and rate
   * meters, each with a variance, against the network physics and estimate
   * uncertain source inputs, returning posterior variances and a per-
   * measurement misfit.
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
