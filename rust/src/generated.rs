//! Typed methods for every tool in the GraphSolve API.
//!
//! GENERATED FROM spec/graphsolve-v1.json — DO NOT EDIT BY HAND.
//! Run `python emit/emit_rust.py` after a spec change; CI fails if this
//! file and the spec disagree.
//!
//! Engine API version: 1.0.35
//! Tools: 89

use serde::{Deserialize, Serialize};

use crate::{GraphSolve, Result, ToolResponse};

/// Every tool this client knows about, in the order the API lists them.
pub const TOOL_NAMES: [&str; 89] = [
    "adjust_composition_to_gor",
    "adjust_composition_to_phase_ratio",
    "aggregate_type_well",
    "allocate_production",
    "analyze_material_balance",
    "calculate_aquifer_influx",
    "calculate_choke_pressure_drop",
    "calculate_choke_size",
    "calculate_compressor",
    "calculate_corrosion_rate",
    "calculate_erosional_velocity",
    "calculate_fluid_properties",
    "calculate_gas_dew_point",
    "calculate_heater_cooler",
    "calculate_hydrate_temperature",
    "calculate_isenthalpic_temperature",
    "calculate_jt_valve",
    "calculate_mmp",
    "calculate_mpfm_allocation",
    "calculate_multistage_compressor",
    "calculate_nodal_analysis",
    "calculate_phase_cuts",
    "calculate_pipe_traverse",
    "calculate_pressure_drop",
    "calculate_pump_head",
    "calculate_rate_from_choke",
    "calculate_reciprocating_compressor",
    "calculate_reid_vapour_pressure",
    "calculate_saturation_pressure",
    "calculate_screw_compressor",
    "calculate_turbine",
    "calculate_volumetrics",
    "calculate_water_properties",
    "calculate_wax_deposition_rate",
    "characterize_pseudo_component",
    "compare_pipeline_correlations",
    "compare_pressure_drop_correlations",
    "compute_statistics",
    "convert_units",
    "evaluate_flow_assurance_profile",
    "fit_decline",
    "flash_to_surface",
    "generate_decline",
    "generate_ipr_curve",
    "generate_phase_envelope",
    "generate_samples",
    "generate_type_curve",
    "get_eos_component",
    "list_correlations",
    "list_edge_types",
    "list_eos_binary_interactions",
    "list_eos_components",
    "list_flow_assurance_models",
    "list_fluid_types",
    "list_node_types",
    "list_transient_solvers",
    "match_fetkovich_ab",
    "match_forchheimer_ab",
    "match_parameters",
    "match_productivity_index",
    "match_pvt",
    "match_skin",
    "optimise_network",
    "rta_diagnostics",
    "run_cce",
    "run_cvd",
    "run_dle",
    "run_eos_flash",
    "run_forecast",
    "run_material_balance",
    "run_molecule_tracking",
    "run_parametric_study",
    "run_process_graph",
    "run_separator_test",
    "run_swelling_test",
    "run_transient_field",
    "run_transient_flux",
    "run_transient_wave",
    "screen_asphaltene_risk",
    "screen_hydrate_risk",
    "screen_liquid_loading",
    "screen_wax_risk",
    "solve_network",
    "solve_network_map",
    "split_plus_fraction",
    "transient_bhp_history",
    "transient_rate_history",
    "tune_mpfm_allocation",
    "validate_solver_payload",
];

/// Internal corrosion rate model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CorrosionModelName {
    /// Sweet CO2 corrosion. Default.
    #[default]
    #[serde(rename = "de_waard_lotz")]
    DeWaardLotz,
    /// de Waard-Lotz with the Mariaca H2S regime factor.
    #[serde(rename = "sour")]
    Sour,
    /// NORSOK M-506, shear- and pH-dependent.
    #[serde(rename = "norsok_m506")]
    NorsokM506,
}

/// Cubic equation-of-state model. Serialises to the exact strings the flash tools
/// accept (`PengRobinson` | `SoaveRedlichKwong` | `PatelTeja`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EosModelName {
    #[serde(rename = "PengRobinson")]
    PengRobinson,
    #[serde(rename = "SoaveRedlichKwong")]
    SoaveRedlichKwong,
    #[serde(rename = "PatelTeja")]
    PatelTeja,
}

/// Canonical name of a multiphase pressure-drop correlation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FlowCorrelationName {
    #[serde(rename = "Aziz")]
    Aziz,
    #[serde(rename = "Baxendell-Thomas")]
    BaxendellThomas,
    #[serde(rename = "Beggs-Brill")]
    BeggsBrill,
    #[serde(rename = "CHAOS")]
    Chaos,
    #[serde(rename = "Dukler")]
    Dukler,
    #[serde(rename = "Duns-Ros")]
    DunsRos,
    #[serde(rename = "Fancher-Brown")]
    FancherBrown,
    #[serde(rename = "GOAT")]
    Goat,
    #[serde(rename = "Gray")]
    Gray,
    #[serde(rename = "Griffith-Wallis")]
    GriffithWallis,
    #[serde(rename = "Hagedorn-Slug")]
    HagedornSlug,
    #[serde(rename = "KISS")]
    Kiss,
    #[serde(rename = "Mist")]
    Mist,
    #[serde(rename = "ml_tuned")]
    MlTuned,
    #[serde(rename = "Mukherjee-Brill")]
    MukherjeeBrill,
    #[serde(rename = "Poettmann-Carpenter")]
    PoettmannCarpenter,
    #[default]
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "SinglePhaseGas")]
    SinglePhaseGas,
    #[serde(rename = "SUPREME")]
    Supreme,
}

/// Hydrate thermodynamics. `vdwp_combined` runs both structures and reports the
/// warmer, controlling envelope; the single-structure variants force one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HydrateModelName {
    /// Tier-1 screening correlation from gas specific gravity.
    #[serde(rename = "towler_mokhatab")]
    TowlerMokhatab,
    /// van der Waals-Platteeuw + Parrish-Prausnitz, structure I.
    #[serde(rename = "vdwp_si")]
    VdwpSi,
    /// Structure II — lifted by even ~1% propane.
    #[serde(rename = "vdwp_sii")]
    VdwpSii,
    /// Both structures, warmer envelope wins. Default.
    #[default]
    #[serde(rename = "vdwp_combined")]
    VdwpCombined,
}

/// Thermodynamic hydrate inhibitor. The field abbreviations are the wire values;
/// the full chemical names and the upper-case spellings are accepted as aliases
/// so hand-written payloads keep working.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InhibitorName {
    #[serde(rename = "methanol")]
    Methanol,
    #[serde(rename = "meg")]
    Meg,
    #[serde(rename = "deg")]
    Deg,
    #[serde(rename = "teg")]
    Teg,
}

/// Wax appearance model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WaxModelName {
    /// Tier-1 correlation from a C7+ characterisation. Default.
    #[default]
    #[serde(rename = "screening")]
    Screening,
    /// Tier-2 Won (1986) multi-solid SLE.
    #[serde(rename = "won")]
    Won,
}

/// Arguments for [`GraphSolve::adjust_composition_to_gor`](crate::GraphSolve::adjust_composition_to_gor).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdjustCompositionToGorParams {
    /// Mole fractions (must match number of components)
    pub mole_fractions: Vec<f64>,
    /// Target surface GOR in Sm³/Sm³ (SI, as everywhere else on this server).
    /// Convert from scf/bbl with `convert_units` first if needed.
    pub target_gor: f64,
    /// Optional binary interaction parameter overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names from the built-in database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// EOS model: "PengRobinson" (default), "SoaveRedlichKwong", or "PatelTeja"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
}

/// Arguments for [`GraphSolve::adjust_composition_to_phase_ratio`](crate::GraphSolve::adjust_composition_to_phase_ratio).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdjustCompositionToPhaseRatioParams {
    /// Mole fractions (must match number of components)
    pub mole_fractions: Vec<f64>,
    /// Pressure in MPa
    pub pressure_mpa: f64,
    /// Target in-situ GOR in m3/m3
    pub target_gor: f64,
    /// Temperature in Kelvin
    pub temperature_k: f64,
    /// Optional binary interaction parameter overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names from the built-in database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// EOS model: "PengRobinson" (default), "SoaveRedlichKwong", or "PatelTeja"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
}

/// Arguments for [`GraphSolve::aggregate_type_well`](crate::GraphSolve::aggregate_type_well).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AggregateTypeWellParams {
    /// Type-well request as a JSON string: `{ "wells": \[{ "eur"? | "decline"? |
    /// "history"?, "normalization_value"? }, ...\], "horizon"?, "economic_limit"?,
    /// "normalize_to"?, "percentiles"?: \[10,50,90\], "program"?: { "n_wells",
    /// "iterations"?, "seed"? } }`.
    pub type_well_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::allocate_production`](crate::GraphSolve::allocate_production).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AllocateProductionParams {
    /// Same network JSON string accepted by `solve_network`.
    pub network_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::analyze_material_balance`](crate::GraphSolve::analyze_material_balance).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AnalyzeMaterialBalanceParams {
    /// Analysis request as a JSON string: `{ "method": "gas_pz", "gp": \[...\],
    /// "p_over_z": \[...\] }` or `{ "method": "havlena_odeh", "f": \[...\], "et":
    /// \[...\] }`.
    pub analysis_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::calculate_aquifer_influx`](crate::GraphSolve::calculate_aquifer_influx).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateAquiferInfluxParams {
    /// Aquifer request as a JSON string: `{ "model": "fetkovich" | "carter_tracy" |
    /// "van_everdingen_hurst", <model params>, "time": \[...\], "pressure": \[...\]
    /// }`.
    pub aquifer_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::calculate_choke_pressure_drop`](crate::GraphSolve::calculate_choke_pressure_drop).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateChokePressureDropParams {
    /// Choke diameter in meters
    pub choke_diameter: f64,
    /// Gas rate in Sm3/day
    pub gas_rate: f64,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin
    pub inlet_temperature: f64,
    /// Oil rate in Sm3/day
    pub oil_rate: f64,
    /// Water rate in Sm3/day
    pub water_rate: f64,
    /// Dissolved gas-oil ratio in Sm3/Sm3 (default: 90)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolved_gas_ratio: Option<f64>,
    /// Gas molecular weight in g/mol (default: 19.83)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_mw: Option<f64>,
    /// Oil density in kg/m3 (default: 850)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_density: Option<f64>,
    /// Slip model: "hydro", "fauske", "moddy" (default: "hydro")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slip_model: Option<String>,
    /// Water salinity in ppm (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_salinity: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_choke_size`](crate::GraphSolve::calculate_choke_size).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateChokeSizeParams {
    /// Target downstream pressure in MPa (must be < upstream_pressure)
    pub downstream_pressure: f64,
    /// Free gas rate in Sm3/day — gas above bubble point (may be 0)
    pub free_gas_rate: f64,
    /// Oil rate in Sm3/day (may be 0 if no oil)
    pub oil_rate: f64,
    /// Inlet pressure in MPa
    pub upstream_pressure: f64,
    /// Inlet temperature in Kelvin
    pub upstream_temperature: f64,
    /// Water rate in Sm3/day (may be 0 if no water)
    pub water_rate: f64,
    /// Dissolved GOR of oil phase in Sm3/Sm3 (default: 90)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolved_gor: Option<f64>,
    /// Gas molecular weight in g/mol — applies to both free gas and dissolved gas
    /// (default: 19.83)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_mw: Option<f64>,
    /// Stock-tank oil density in kg/m3 (default: 850)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_density: Option<f64>,
    /// Slip model: "hydro", "fauske", "moddy", etc. (default: "hydro")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slip_model: Option<String>,
    /// Water salinity in ppm (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_salinity: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_compressor`](crate::GraphSolve::calculate_compressor).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateCompressorParams {
    /// Fluid configuration
    pub fluid: serde_json::Value,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin
    pub inlet_temperature: f64,
    /// Pressure ratio (outlet/inlet, must be > 1.0)
    pub pressure_ratio: f64,
    /// Mechanical efficiency (0–1, default: 0.95)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanical_efficiency: Option<f64>,
    /// Polytropic efficiency (0–1, default: 0.75). This is the native parameter of
    /// the turbo centrifugal model. The result reports both the polytropic and the
    /// back-computed isentropic efficiency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polytropic_efficiency: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_corrosion_rate`](crate::GraphSolve::calculate_corrosion_rate).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateCorrosionRateParams {
    /// Local total pressure in MPa
    pub pressure: f64,
    /// Local temperature in K
    pub temperature: f64,
    /// Bicarbonate concentration \[mol/L\] — NORSOK in-situ pH
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bicarbonate_molar: Option<f64>,
    /// CO2 mole fraction in the gas phase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_mole_fraction: Option<f64>,
    /// Design life \[years\], to report a wall-thickness allowance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design_life_years: Option<f64>,
    /// Internal diameter \[m\] — NORSOK shear
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diameter: Option<f64>,
    /// Glycol concentration \[wt %\] — NORSOK mitigation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glycol_wt_pct: Option<f64>,
    /// H2S mole fraction in the gas phase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2s_mole_fraction: Option<f64>,
    /// Corrosion-inhibitor efficiency \[%\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inhibitor_efficiency_pct: Option<f64>,
    /// Ionic strength \[mol/L\] — NORSOK in-situ pH
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ionic_strength_molar: Option<f64>,
    /// Mixture density \[kg/m3\] — NORSOK shear
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixture_density: Option<f64>,
    /// Mixture velocity \[m/s\] — NORSOK shear
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixture_velocity: Option<f64>,
    /// Mixture viscosity \[Pa.s\] — NORSOK shear
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixture_viscosity: Option<f64>,
    /// Corrosion rate model (default de_waard_lotz)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<CorrosionModelName>,
    /// In-situ pH — NORSOK. Computed from the carbonate equilibrium when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ph: Option<f64>,
    /// Absolute roughness \[m\] — NORSOK shear (default 4.5e-5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roughness: Option<f64>,
    /// Wall shear stress \[Pa\] — NORSOK. Computed from the flow state below when
    /// omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shear_stress_pa: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_erosional_velocity`](crate::GraphSolve::calculate_erosional_velocity).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateErosionalVelocityParams {
    /// Fluid configuration
    pub fluid: serde_json::Value,
    /// Pipe inner diameter in meters
    pub pipe_diameter: f64,
    /// Pressure in MPa
    pub pressure: f64,
    /// Temperature in Kelvin
    pub temperature: f64,
    /// API RP 14E C-factor (default: 100, range ~80–300)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_factor: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_fluid_properties`](crate::GraphSolve::calculate_fluid_properties).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateFluidPropertiesParams {
    /// Fluid properties configuration
    pub fluid_properties: serde_json::Value,
    /// Pressure in MPa (valid range: 0.1 to 150)
    pub pressure: f64,
    /// Temperature in Kelvin (valid range: 250 to 500)
    pub temperature: f64,
}

/// Arguments for [`GraphSolve::calculate_gas_dew_point`](crate::GraphSolve::calculate_gas_dew_point).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateGasDewPointParams {
    /// Condensate-gas ratio in STB/MMscf
    pub condensate_gas_ratio: f64,
    /// Gas specific gravity (air = 1.0)
    pub gas_gravity: f64,
    /// Oil API gravity
    pub oil_api_gravity: f64,
    /// Temperature in Kelvin
    pub temperature: f64,
}

/// Arguments for [`GraphSolve::calculate_heater_cooler`](crate::GraphSolve::calculate_heater_cooler).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateHeaterCoolerParams {
    /// Fluid configuration
    pub fluid: serde_json::Value,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin
    pub inlet_temperature: f64,
    /// Operating mode: "fixed_duty", "fixed_temperature", or "approach_temperature"
    pub mode: String,
    /// Temperature approach in Kelvin (for approach_temperature mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approach_temperature: Option<f64>,
    /// Heat duty in Watts (for fixed_duty mode; positive=heating, negative=cooling)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duty: Option<f64>,
    /// Pressure drop across heater/cooler in MPa (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure_drop: Option<f64>,
    /// Reference temperature in Kelvin (for approach_temperature mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_temperature: Option<f64>,
    /// Target temperature in Kelvin (for fixed_temperature mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_temperature: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_hydrate_temperature`](crate::GraphSolve::calculate_hydrate_temperature).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateHydrateTemperatureParams {
    /// Gas specific gravity (air = 1.0)
    pub gas_gravity: f64,
    /// Pressure in MPa
    pub pressure: f64,
    /// CO2 mole fraction (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_mole_fraction: Option<f64>,
    /// H2S mole fraction (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2s_mole_fraction: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_isenthalpic_temperature`](crate::GraphSolve::calculate_isenthalpic_temperature).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateIsenthalpicTemperatureParams {
    /// Fluid configuration
    pub fluid: serde_json::Value,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin
    pub inlet_temperature: f64,
    /// Outlet pressure in MPa (must be < inlet_pressure)
    pub outlet_pressure: f64,
}

/// Arguments for [`GraphSolve::calculate_jt_valve`](crate::GraphSolve::calculate_jt_valve).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateJtValveParams {
    /// Fluid configuration
    pub fluid: serde_json::Value,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin
    pub inlet_temperature: f64,
    /// Pressure drop across valve in MPa (positive value)
    pub pressure_drop: f64,
}

/// Arguments for [`GraphSolve::calculate_mmp`](crate::GraphSolve::calculate_mmp).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateMmpParams {
    /// Experiment configuration as a JSON string: the fluid composition and the
    /// experiment-specific schedule (e.g. pressure stages, separator stages,
    /// injection-gas composition).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::calculate_mpfm_allocation`](crate::GraphSolve::calculate_mpfm_allocation).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateMpfmAllocationParams {
    /// Complete MPFM allocation configuration as a JSON string. `pressure_mpa` and
    /// `pressure_offset_mpa` in MPa.
    pub allocation_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::calculate_multistage_compressor`](crate::GraphSolve::calculate_multistage_compressor).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateMultistageCompressorParams {
    /// Fluid rates and PVT configuration (gas rate drives the mass flow).
    pub fluid: serde_json::Value,
    /// Inlet pressure in MPa.
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin.
    pub inlet_temperature: f64,
    /// Compression stages, in order.
    pub stages: Vec<serde_json::Value>,
    /// Inter-stage cooler pressure drop in MPa (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intercool_pressure_drop: Option<f64>,
    /// Inter-stage cooler target temperature in Kelvin (applied between stages when
    /// set; omit for an uncooled train).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intercool_temperature: Option<f64>,
    /// Mechanical efficiency (0-1, default: 0.95).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanical_efficiency: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_nodal_analysis`](crate::GraphSolve::calculate_nodal_analysis).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateNodalAnalysisParams {
    /// IPR model configuration (same fields as calculate_ipr_curve: ipr_model,
    /// reservoir_pressure, etc.)
    pub ipr: serde_json::Value,
    /// Tubing inner diameter in meters
    pub tubing_diameter: f64,
    /// Tubing length (TVD) in meters
    pub tubing_length: f64,
    /// Absolute tubing roughness in meters
    pub tubing_roughness: f64,
    /// Wellhead back-pressure in MPa
    pub wellhead_pressure: f64,
    /// Wellhead temperature in Kelvin
    pub wellhead_temperature: f64,
    /// Dissolved gas-oil ratio in Sm3/Sm3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolved_gas_ratio: Option<f64>,
    /// Multiphase flow correlation (default: "Beggs-Brill")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_correlation: Option<String>,
    /// Gas molecular weight in g/mol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_mw: Option<f64>,
    /// Oil density in kg/m3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_density: Option<f64>,
    /// Tubing angle from vertical in degrees (default: 0 = vertical)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tubing_angle: Option<f64>,
    /// Water salinity in ppm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_salinity: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_phase_cuts`](crate::GraphSolve::calculate_phase_cuts).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculatePhaseCutsParams {
    /// Volumetric gas rate (omit or set to 0 if no gas)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_rate: Option<f64>,
    /// Unit for gas rate (default: "Sm3/day"). Accepts any flow-rate unit: MMSCFD,
    /// kSm3/day, MSCFD, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_unit: Option<String>,
    /// Volumetric oil rate (omit or set to 0 if no oil)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_rate: Option<f64>,
    /// Unit for oil rate (default: "Sm3/day"). Accepts: BOPD, bbl/d, Sm3/day,
    /// m3/day, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_unit: Option<String>,
    /// Volumetric water rate (omit or set to 0 if no water)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_rate: Option<f64>,
    /// Unit for water rate (default: "Sm3/day"). Accepts: BWPD, bbl/d, Sm3/day,
    /// m3/day, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_unit: Option<String>,
}

/// Arguments for [`GraphSolve::calculate_pipe_traverse`](crate::GraphSolve::calculate_pipe_traverse).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculatePipeTraverseParams {
    /// Pipe angle from vertical in degrees, measured from the bottom end to the top
    /// end (0=vertical, 90=horizontal). Flow enters at `flow_boundary`: with the
    /// default "top", 0 is vertical downflow and 180 vertical upflow; with
    /// "bottom", 0 is vertical upflow.
    pub angle: f64,
    /// Pipe inner diameter in meters
    pub diameter: f64,
    /// Gas rate in Sm3/day
    pub gas_rate: f64,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin
    pub inlet_temperature: f64,
    /// Pipe length in meters
    pub length: f64,
    /// Oil rate in Sm3/day
    pub oil_rate: f64,
    /// Absolute pipe roughness in meters
    pub roughness: f64,
    /// Water rate in Sm3/day
    pub water_rate: f64,
    /// Dissolved gas-oil ratio in Sm3/Sm3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolved_gas_ratio: Option<f64>,
    /// Flow boundary location: "top" or "bottom" (default: "top")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_boundary: Option<String>,
    /// Multiphase flow correlation: "Griffith-Wallis", "KISS", "Mukherjee-Brill",
    /// "Hage-Brown", "Beggs-Brill"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_correlation: Option<String>,
    /// Gas molecular weight in g/mol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_mw: Option<f64>,
    /// Overall heat transfer coefficient in W/(m².K)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_transfer_coefficient: Option<f64>,
    /// Oil density in kg/m3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_density: Option<f64>,
    /// Pressure boundary location: "top" or "bottom" (default: "top")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure_boundary: Option<String>,
    /// Surrounding/ambient temperature in Kelvin (for heat transfer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surrounding_temperature: Option<f64>,
    /// Water salinity in ppm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_salinity: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_pressure_drop`](crate::GraphSolve::calculate_pressure_drop).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculatePressureDropParams {
    /// Pipe angle from vertical in degrees, in the flow direction (0=vertical
    /// upflow, 90=horizontal, 180=vertical downflow)
    pub angle: f64,
    /// Multiphase flow correlation (e.g. "Beggs-Brill", "Griffith-Wallis", "KISS").
    pub correlation: FlowCorrelationName,
    /// Densities \[gas, oil, water\] in kg/m3
    pub density: Vec<f64>,
    /// Pipe diameter in meters
    pub diameter: f64,
    /// Gas-liquid interfacial tension in N/m
    pub ift: f64,
    /// Pressure in MPa
    pub pressure: f64,
    /// Absolute pipe roughness in meters
    pub roughness: f64,
    /// Superficial velocities \[gas, oil, water\] in m/s
    pub velocity: Vec<f64>,
    /// Viscosities \[gas, oil, water\] in Pa.s
    pub viscosity: Vec<f64>,
}

/// Arguments for [`GraphSolve::calculate_pump_head`](crate::GraphSolve::calculate_pump_head).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculatePumpHeadParams {
    /// Flow rate in m3/day
    pub flow_rate: f64,
    /// Head curve as array of \[rate_m3_day, head_m\] pairs
    pub head_curve: Vec<Vec<f64>>,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin
    pub inlet_temperature: f64,
    /// Liquid density in kg/m3
    pub liquid_density: f64,
}

/// Arguments for [`GraphSolve::calculate_rate_from_choke`](crate::GraphSolve::calculate_rate_from_choke).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateRateFromChokeParams {
    /// Choke diameter in meters
    pub choke_diameter: f64,
    /// Target downstream pressure in MPa (must be < upstream_pressure)
    pub downstream_pressure: f64,
    /// Inlet pressure in MPa
    pub upstream_pressure: f64,
    /// Inlet temperature in Kelvin
    pub upstream_temperature: f64,
    /// Fix condensate-to-gas ratio (oil / free_gas) in Sm3/Sm3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgr: Option<f64>,
    /// Dissolved GOR in Sm3/Sm3 (default: 90)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolved_gor: Option<f64>,
    /// Phase held fixed while others scale to match pressure drop. One of:
    /// "free_gas" (default), "oil", "water", "all".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<String>,
    /// Seed free gas rate in Sm3/day (optional; default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_gas_rate: Option<f64>,
    /// Gas molecular weight in g/mol (default: 19.83)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_mw: Option<f64>,
    /// Fix free-gas-to-oil ratio in Sm3/Sm3 (gor = free_gas / oil)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gor: Option<f64>,
    /// Stock-tank oil density in kg/m3 (default: 850)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_density: Option<f64>,
    /// Seed oil rate in Sm3/day (optional; default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_rate: Option<f64>,
    /// Slip model (default: "hydro")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slip_model: Option<String>,
    /// Optional target downstream temperature in K — reported as a match check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_downstream_temperature: Option<f64>,
    /// Seed water rate in Sm3/day (optional; default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_rate: Option<f64>,
    /// Water salinity in ppm (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_salinity: Option<f64>,
    /// Fix watercut = water / (oil + water), in PERCENT \[0, 100\] — the same
    /// convention as `source_sink_data.water_cut` (10 means 10 %)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watercut: Option<f64>,
    /// Fix water-to-gas ratio in Sm3/Sm3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wgr: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_reciprocating_compressor`](crate::GraphSolve::calculate_reciprocating_compressor).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateReciprocatingCompressorParams {
    /// Discharge (flange) pressure in MPa. Must be greater than inlet_pressure.
    pub discharge_pressure: f64,
    /// Suction (inlet) pressure in MPa.
    pub inlet_pressure: f64,
    /// Suction (inlet) temperature in Kelvin.
    pub inlet_temperature: f64,
    /// Running speed in rpm (must be > 0).
    pub speed_rpm: f64,
    /// Swept (displacement) volume per revolution in m^3 (must be > 0).
    pub swept_volume_per_rev_m3: f64,
    /// Clearance volume as a fraction of swept volume (default: 0.12).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearance_fraction: Option<f64>,
    /// CO2 mole fraction in the gas (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_fraction: Option<f64>,
    /// Gas molecular weight in g/mol (default: 20.279).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_molecular_weight: Option<f64>,
    /// H2S mole fraction in the gas (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2s_fraction: Option<f64>,
    /// Upper guard on the per-stage pressure ratio the solver may explore (default:
    /// 6.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pressure_ratio: Option<f64>,
    /// Mechanical efficiency (0-1, default: 0.95); converts fluid power to shaft
    /// power.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanical_efficiency: Option<f64>,
    /// Lower guard on the per-stage pressure ratio (default: 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pressure_ratio: Option<f64>,
    /// N2 mole fraction in the gas (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n2_fraction: Option<f64>,
    /// Polytropic efficiency (0-1, default: 0.80). Native turbo parameter; the
    /// result also reports the back-computed isentropic efficiency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polytropic_efficiency: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_reid_vapour_pressure`](crate::GraphSolve::calculate_reid_vapour_pressure).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateReidVapourPressureParams {
    /// Liquid mole fractions (must match number of components)
    pub liquid_mole_fractions: Vec<f64>,
    /// Optional binary interaction parameter overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names from the built-in database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// EOS model: "PengRobinson" (default), "SoaveRedlichKwong", or "PatelTeja"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
}

/// Arguments for [`GraphSolve::calculate_saturation_pressure`](crate::GraphSolve::calculate_saturation_pressure).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateSaturationPressureParams {
    /// Component names from the built-in EOS database.
    pub component_names: Vec<String>,
    /// Overall mole fractions (must match component_names length).
    pub mole_fractions: Vec<f64>,
    /// Temperature in Kelvin.
    pub temperature_k: f64,
    /// Saturation boundary: "bubble" (default) or "dew".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boundary: Option<String>,
    /// EOS model: "PengRobinson" (default), "SoaveRedlichKwong", "PatelTeja".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<String>,
}

/// Arguments for [`GraphSolve::calculate_screw_compressor`](crate::GraphSolve::calculate_screw_compressor).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateScrewCompressorParams {
    /// Discharge pressure in MPa (must exceed inlet_pressure).
    pub discharge_pressure: f64,
    /// Displacement (swept volume) per revolution in m^3.
    pub displacement_per_rev_m3: f64,
    /// Suction pressure in MPa.
    pub inlet_pressure: f64,
    /// Suction temperature in Kelvin.
    pub inlet_temperature: f64,
    /// Shaft speed in revolutions per second.
    pub shaft_speed_rev_s: f64,
    /// CO2 mole fraction (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_fraction: Option<f64>,
    /// Gas molecular weight in g/mol (default: 20.279).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_molecular_weight: Option<f64>,
    /// H2S mole fraction (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2s_fraction: Option<f64>,
    /// Mechanical efficiency (0-1, default: 0.95).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanical_efficiency: Option<f64>,
    /// N2 mole fraction (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n2_fraction: Option<f64>,
    /// Internal polytropic efficiency (0-1, default: 0.70).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polytropic_efficiency: Option<f64>,
    /// Screw subtype: "dry" (default), "oil_flooded", or "water_injected".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Volumetric efficiency (0-1, default: 0.85).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumetric_efficiency: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_turbine`](crate::GraphSolve::calculate_turbine).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateTurbineParams {
    /// Fluid configuration
    pub fluid: serde_json::Value,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin
    pub inlet_temperature: f64,
    /// Pressure ratio (outlet/inlet, must be 0 < PR < 1.0)
    pub pressure_ratio: f64,
    /// Mechanical efficiency (0–1, default: 0.95)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanical_efficiency: Option<f64>,
    /// Polytropic efficiency (0–1, default: 0.80). Native parameter of the turbo
    /// centrifugal expander model. The result reports both the polytropic and the
    /// back-computed isentropic efficiency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polytropic_efficiency: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_volumetrics`](crate::GraphSolve::calculate_volumetrics).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateVolumetricsParams {
    /// Volumetrics request as a JSON string: `{ "area_m2": ..., "thickness_m": ...,
    /// "net_to_gross"?: ..., "porosity": ..., "water_saturation": ..., "oil_fvf"?:
    /// ..., "gas_fvf"?: ... }`. Each field is a scalar or a per-realisation array
    /// (scalars broadcast).
    pub volumetrics_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::calculate_water_properties`](crate::GraphSolve::calculate_water_properties).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateWaterPropertiesParams {
    /// Pressure in MPa (valid range: 0.1 to 150)
    pub pressure: f64,
    /// Temperature in Kelvin (valid range: 250 to 650)
    pub temperature: f64,
    /// Water salinity in ppm (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salinity: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_wax_deposition_rate`](crate::GraphSolve::calculate_wax_deposition_rate).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateWaxDepositionRateParams {
    /// Ambient (outside-wall) temperature in K
    pub ambient_temperature: f64,
    /// Bulk fluid temperature in K
    pub bulk_temperature: f64,
    /// Overall heat-transfer coefficient \[W/m2/K\]
    pub heat_transfer_coefficient: f64,
    /// Oil density \[kg/m3\]
    pub oil_density: f64,
    /// Wax solubility gradient dWs/dT \[1/K\] — the slope of dissolved wax against
    /// temperature
    pub solubility_gradient_per_k: f64,
    /// Oil molecular weight \[g/mol\] — for the Wilke-Chang diffusivity estimate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_molecular_weight: Option<f64>,
    /// Oil thermal conductivity \[W/m/K\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_thermal_conductivity: Option<f64>,
    /// Oil viscosity \[Pa.s\] — for the Wilke-Chang diffusivity estimate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_viscosity: Option<f64>,
    /// Wax diffusivity \[m2/s\]. Estimated by Wilke-Chang when omitted and the oil
    /// properties below are given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wax_diffusivity: Option<f64>,
    /// Wax molecular weight \[g/mol\] — for the Wilke-Chang diffusivity estimate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wax_molecular_weight: Option<f64>,
}

/// Arguments for [`GraphSolve::characterize_pseudo_component`](crate::GraphSolve::characterize_pseudo_component).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CharacterizePseudoComponentParams {
    /// Normal boiling point in Kelvin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boiling_point: Option<f64>,
    /// Critical-property correlation: "kesler_lee" (default), "twu", or "sancet".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Molecular weight in g/mol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_weight: Option<f64>,
    /// Specific gravity (water = 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specific_gravity: Option<f64>,
}

/// Arguments for [`GraphSolve::compare_pipeline_correlations`](crate::GraphSolve::compare_pipeline_correlations).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComparePipelineCorrelationsParams {
    /// Fluid spec, the same shape as `calculate_fluid_properties.fluid_properties`.
    pub fluid: serde_json::Value,
    /// Inlet boundary state at the upstream end of the pipe.
    pub inlet: serde_json::Value,
    /// Pipe geometry segments, the same shape as `pipe_data.segments` in a network
    /// model. Each segment takes segment_type, pipe_diameter, pipe_roughness,
    /// surrounding_temperature, heat_transfer_coefficient, pipe_length, pipe_angle
    /// and optional MD/TVD.
    pub pipe_segments: serde_json::Value,
    /// Optional subset of correlations to compare. Default: all 19 variants in the
    /// FlowCorrelation enum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlations: Option<Vec<FlowCorrelationName>>,
}

/// Arguments for [`GraphSolve::compare_pressure_drop_correlations`](crate::GraphSolve::compare_pressure_drop_correlations).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComparePressureDropCorrelationsParams {
    /// Pipe angle from vertical in degrees, in the flow direction (0=vertical
    /// upflow, 90=horizontal, 180=vertical downflow)
    pub angle: f64,
    /// Densities \[gas, oil, water\] in kg/m3
    pub density: Vec<f64>,
    /// Pipe diameter in meters
    pub diameter: f64,
    /// Gas-liquid interfacial tension in N/m
    pub ift: f64,
    /// Pressure in MPa
    pub pressure: f64,
    /// Absolute pipe roughness in meters
    pub roughness: f64,
    /// Superficial velocities \[gas, oil, water\] in m/s
    pub velocity: Vec<f64>,
    /// Viscosities \[gas, oil, water\] in Pa.s
    pub viscosity: Vec<f64>,
    /// Optional subset of correlations. Default: all 19 variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlations: Option<Vec<String>>,
}

/// Arguments for [`GraphSolve::compute_statistics`](crate::GraphSolve::compute_statistics).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComputeStatisticsParams {
    /// Statistics request as a JSON string: `{ "outputs": \[{ "label": ...,
    /// "values": \[...\] }, ...\], "inputs": \[...\], "options": { "percentiles":
    /// \[10,50,90\], "n_bins": 20, "sensitivity": "correlation",
    /// "reserves_convention": false } }`.
    pub statistics_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::convert_units`](crate::GraphSolve::convert_units).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConvertUnitsParams {
    /// Source unit (e.g. "psi", "bar", "MPa", "bbl/d", "degF", "mD")
    pub from_unit: String,
    /// Target unit (e.g. "Pa", "MPa", "Sm3/d", "K", "m2")
    pub to_unit: String,
    /// Numeric value to convert
    pub value: f64,
}

/// Arguments for [`GraphSolve::evaluate_flow_assurance_profile`](crate::GraphSolve::evaluate_flow_assurance_profile).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EvaluateFlowAssuranceProfileParams {
    /// Inlet pressure \[MPa\] from the converged solve
    pub inlet_pressure: f64,
    /// Inlet temperature \[K\]
    pub inlet_temperature: f64,
    /// Total mass flow \[kg/s\]
    pub mass_flow: f64,
    /// Outlet pressure \[MPa\]
    pub outlet_pressure: f64,
    /// Outlet temperature \[K\]
    pub outlet_temperature: f64,
    /// Pipe geometry, inlet to outlet
    pub segments: Vec<serde_json::Value>,
    /// API RP 14E C-factor for the erosion pass (default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_factor: Option<f64>,
    /// Component names, for the hydrate and corrosion passes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Mixture heat capacity \[J/kg/K\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_capacity: Option<f64>,
    /// Hydrate inhibitor depression already applied to the line \[K\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inhibitor_depression_k: Option<f64>,
    /// Inlet mixture density \[kg/m3\], for the erosion pass
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlet_mixture_density: Option<f64>,
    /// Inlet vapour mole fractions, aligned with component_names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vapor_mole_fractions: Option<Vec<f64>>,
}

/// Arguments for [`GraphSolve::fit_decline`](crate::GraphSolve::fit_decline).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FitDeclineParams {
    /// Fit request as a JSON string: `{ "kind"?: <model, omit to auto-select>,
    /// "time": \[...\], "rate": \[...\], "reject_outliers"?: true }`.
    pub fit_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::flash_to_surface`](crate::GraphSolve::flash_to_surface).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FlashToSurfaceParams {
    /// Mole fractions (must match number of components)
    pub mole_fractions: Vec<f64>,
    /// Optional binary interaction parameter overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names from the built-in database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// EOS model: "PengRobinson" (default), "SoaveRedlichKwong", or "PatelTeja"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
}

/// Arguments for [`GraphSolve::generate_decline`](crate::GraphSolve::generate_decline).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GenerateDeclineParams {
    /// Decline-generation request as a JSON string: `{ "kind": "arps" |
    /// "modified_hyperbolic" | "duong" | "stretched_exponential" |
    /// "power_law_exponential", "params": { <name>: value, ... }, "times"?: \[...\]
    /// | "t_max": ..., "n_points"?: 200, "economic_limit"?: ... }`.
    pub decline_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::generate_ipr_curve`](crate::GraphSolve::generate_ipr_curve).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GenerateIprCurveParams {
    /// Full inflow-model spec — same shape as the `inflow_model` field of a
    /// SourcesTable record. Required fields depend on `ipr_model`: "pi" needs
    /// `reservoir_properties.productivity_index`; "darcy" / "fetkovich" /
    /// "forchheimer" need permeability + well/reservoir dimensions; "test_data"
    /// needs `test_points`.
    pub inflow_model: serde_json::Value,
    /// Number of (P, Q) points to compute. Each point is a real model evaluation.
    /// Range \[2, 200\], default 30 (the canonical solver resolution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_points: Option<i64>,
}

/// Arguments for [`GraphSolve::generate_phase_envelope`](crate::GraphSolve::generate_phase_envelope).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GeneratePhaseEnvelopeParams {
    /// Component names from the built-in EOS database.
    pub component_names: Vec<String>,
    /// Overall mole fractions (must match component_names length).
    pub mole_fractions: Vec<f64>,
    /// Cubic EOS model: "PengRobinson" (default), "SoaveRedlichKwong", "PatelTeja".
    /// GERG-2008 is not supported for the envelope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<String>,
}

/// Arguments for [`GraphSolve::generate_samples`](crate::GraphSolve::generate_samples).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GenerateSamplesParams {
    /// Sample-generation request as a JSON string: `{ "variables": \[{ "name": ...,
    /// "distribution": { "dist_type": "normal", "mean": ..., "std_dev": ... } },
    /// ...\], "n_samples": N, "method": "latin_hypercube" | "random" | "sobol",
    /// "correlations": \[...\], "seed": 0 }`.
    pub samples_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::generate_type_curve`](crate::GraphSolve::generate_type_curve).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GenerateTypeCurveParams {
    /// Full inflow-model spec. Use a transient type-curve `ipr_model` (`radial`,
    /// `partial_penetration`, `faulted`, `composite_radial`, `horizontal`,
    /// `uniform_flux_fracture`, `infinite_conductivity_fracture`,
    /// `bounded_radial_fractured`, `bounded_rect_fractured`, `horizontal_msf`). The
    /// dimensionless parameters are derived from the geometric / petrophysical
    /// inputs (no `type_curve` block).
    pub inflow_model: serde_json::Value,
    /// Number of log-spaced `tD` points. Range \[2, 200\], default 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Upper bound of the dimensionless time grid. Default `1e6`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td_max: Option<f64>,
    /// Lower bound of the dimensionless time grid. Default `1e-2`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td_min: Option<f64>,
}

/// Arguments for [`GraphSolve::get_eos_component`](crate::GraphSolve::get_eos_component).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GetEosComponentParams {
    /// Component name to look up (e.g. "methane", "C1", "CO2")
    pub name: String,
}

/// Arguments for [`GraphSolve::list_correlations`](crate::GraphSolve::list_correlations).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListCorrelationsParams {
    /// Optional category filter. One of: "flow", "bubble_point", "oil_viscosity",
    /// "oil_compressibility", "hydrocarbon_ift", "gas_zfactor", "gas_viscosity",
    /// "gas_critical_properties", "oil_heat_capacity", "choke_slip", "eos", "ipr".
    /// Omit to list all categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

/// Arguments for [`GraphSolve::list_eos_binary_interactions`](crate::GraphSolve::list_eos_binary_interactions).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListEosBinaryInteractionsParams {
    /// Component name to get binary interaction parameters for
    pub name: String,
}

/// Arguments for [`GraphSolve::match_fetkovich_ab`](crate::GraphSolve::match_fetkovich_ab).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchFetkovichAbParams {
    /// Full inflow-model spec (see `generate_ipr_curve`). For matching, include
    /// `test_points` (observed rate + flowing-BHP pairs). For the transient BHP
    /// history, include `rate_history`; for the rate history a `pressure_history`;
    /// for RTA an `rta_history` (time/rate/pressure), plus the transient model's
    /// reservoir inputs.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::match_forchheimer_ab`](crate::GraphSolve::match_forchheimer_ab).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchForchheimerAbParams {
    /// Full inflow-model spec (see `generate_ipr_curve`). For matching, include
    /// `test_points` (observed rate + flowing-BHP pairs). For the transient BHP
    /// history, include `rate_history`; for the rate history a `pressure_history`;
    /// for RTA an `rta_history` (time/rate/pressure), plus the transient model's
    /// reservoir inputs.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::match_parameters`](crate::GraphSolve::match_parameters).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchParametersParams {
    /// Full inflow-model spec with observed `test_points`.
    pub inflow_model: serde_json::Value,
    /// Which parameter to fit: `"productivity_index"` | `"skin"` |
    /// `"forchheimer_ab"` | `"fetkovich_ab"`. When omitted it is derived from
    /// `ipr_model`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_target: Option<String>,
}

/// Arguments for [`GraphSolve::match_productivity_index`](crate::GraphSolve::match_productivity_index).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchProductivityIndexParams {
    /// Full inflow-model spec (see `generate_ipr_curve`). For matching, include
    /// `test_points` (observed rate + flowing-BHP pairs). For the transient BHP
    /// history, include `rate_history`; for the rate history a `pressure_history`;
    /// for RTA an `rta_history` (time/rate/pressure), plus the transient model's
    /// reservoir inputs.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::match_pvt`](crate::GraphSolve::match_pvt).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchPvtParams {
    /// PVT-match configuration as a JSON string: `{ "config": ..., "lab": ... }`
    /// where `config` holds the correlation search ranges and iteration limits and
    /// `lab` holds the measured bubble point, Rs, Bo, viscosity and density.
    pub match_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::match_skin`](crate::GraphSolve::match_skin).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchSkinParams {
    /// Full inflow-model spec (see `generate_ipr_curve`). For matching, include
    /// `test_points` (observed rate + flowing-BHP pairs). For the transient BHP
    /// history, include `rate_history`; for the rate history a `pressure_history`;
    /// for RTA an `rta_history` (time/rate/pressure), plus the transient model's
    /// reservoir inputs.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::optimise_network`](crate::GraphSolve::optimise_network).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OptimiseNetworkParams {
    /// Optimisation request as a JSON string: the base `network_payload` plus
    /// `variables` (the controls to move, each with a type/element and lower/upper
    /// bounds), an `objective` (maximise/minimise phase rate, revenue, pressure
    /// drop, power, or deviation), `constraints` (rate / pressure / temperature /
    /// velocity / power caps, or total-resource limits), and optional solver
    /// `settings`. Elements are referenced by id: `{"node_id": "<id>"}` or
    /// `{"edge_id": "<id>"}`.
    pub optimisation_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::rta_diagnostics`](crate::GraphSolve::rta_diagnostics).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RtaDiagnosticsParams {
    /// Full inflow-model spec (see `generate_ipr_curve`). For matching, include
    /// `test_points` (observed rate + flowing-BHP pairs). For the transient BHP
    /// history, include `rate_history`; for the rate history a `pressure_history`;
    /// for RTA an `rta_history` (time/rate/pressure), plus the transient model's
    /// reservoir inputs.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_cce`](crate::GraphSolve::run_cce).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunCceParams {
    /// Experiment configuration as a JSON string: the fluid composition and the
    /// experiment-specific schedule (e.g. pressure stages, separator stages,
    /// injection-gas composition).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_cvd`](crate::GraphSolve::run_cvd).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunCvdParams {
    /// Experiment configuration as a JSON string: the fluid composition and the
    /// experiment-specific schedule (e.g. pressure stages, separator stages,
    /// injection-gas composition).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_dle`](crate::GraphSolve::run_dle).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunDleParams {
    /// Experiment configuration as a JSON string: the fluid composition and the
    /// experiment-specific schedule (e.g. pressure stages, separator stages,
    /// injection-gas composition).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_eos_flash`](crate::GraphSolve::run_eos_flash).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunEosFlashParams {
    /// Mole fractions (must match number of components)
    pub mole_fractions: Vec<f64>,
    /// Pressure in MPa
    pub pressure_mpa: f64,
    /// Temperature in Kelvin
    pub temperature_k: f64,
    /// Optional binary interaction parameter overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names from the built-in database (e.g. \["methane", "ethane",
    /// "propane"\])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// EOS model: "PengRobinson" (default), "SoaveRedlichKwong", or "PatelTeja"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
}

/// Arguments for [`GraphSolve::run_forecast`](crate::GraphSolve::run_forecast).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunForecastParams {
    /// Forecast request as a JSON string: `network_payload`, `time`
    /// (start/unit/timestep_duration/horizon), an optional `engine` (plain `solve`
    /// by default, or `optimise` with variables/objective/constraints), `updates`
    /// (decline curves / time-series drivers), `events` (shut-ins, startups,
    /// setpoints, workovers), `transient_wells`, and run policy. Elements are
    /// referenced by id: `{"node_id": "<id>"}` or `{"edge_id": "<id>"}`. Prefer
    /// `result_detail: "summary"` — per-timestep graph snapshots make the result
    /// very large.
    pub forecast_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_material_balance`](crate::GraphSolve::run_material_balance).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunMaterialBalanceParams {
    /// Reservoir-system definition as a JSON string: one or more blocks
    /// (STOIIP/GIIP, PVT, rock compressibility), their production/injection
    /// histories, and any block-to-block connections.
    pub reservoir_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_molecule_tracking`](crate::GraphSolve::run_molecule_tracking).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunMoleculeTrackingParams {
    /// Fluid compositions JSON string
    pub fluids_json: serde_json::Value,
    /// Solved network JSON string (output of solve_network)
    pub solved_network_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_parametric_study`](crate::GraphSolve::run_parametric_study).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunParametricStudyParams {
    /// Parametric-study request as a JSON string: the base network payload plus the
    /// sweep definition (variables, ranges or distributions, sampling, and study
    /// type: linear sweep, tornado, two-factor grid, or Monte Carlo). Elements are
    /// referenced by id: `{"node_id": "<id>"}` or `{"edge_id": "<id>"}`.
    pub study_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_process_graph`](crate::GraphSolve::run_process_graph).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunProcessGraphParams {
    /// Complete process graph definition as a JSON string containing stream
    /// compositions, separators, mixers, heaters, compressors, and connections.
    pub process_graph_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_separator_test`](crate::GraphSolve::run_separator_test).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunSeparatorTestParams {
    /// Experiment configuration as a JSON string: the fluid composition and the
    /// experiment-specific schedule (e.g. pressure stages, separator stages,
    /// injection-gas composition).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_swelling_test`](crate::GraphSolve::run_swelling_test).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunSwellingTestParams {
    /// Experiment configuration as a JSON string: the fluid composition and the
    /// experiment-specific schedule (e.g. pressure stages, separator stages,
    /// injection-gas composition).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_transient_field`](crate::GraphSolve::run_transient_field).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunTransientFieldParams {
    /// Transient request as a JSON string. Either `network_payload` plus a `path`
    /// naming the pipe edges to run, or a bare `segments` list plus a `fluid`; then
    /// `time` (total, unit, record_interval_s), the initial `boundary`, an optional
    /// `schedule` of changes, `results` (detail, variables, cell_stride) and
    /// `discretisation` (max_cell_length).  Prefer `results.detail: "summary"`
    /// unless the per-cell history is the point — `profiles` and `full` return
    /// samples x cells x variables floats and are refused above a couple of million
    /// of them.
    pub transient_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_transient_flux`](crate::GraphSolve::run_transient_flux).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunTransientFluxParams {
    /// Transient request as a JSON string. Either `network_payload` plus a `path`
    /// naming the pipe edges to run, or a bare `segments` list plus a `fluid`; then
    /// `time` (total, unit, record_interval_s), the initial `boundary`, an optional
    /// `schedule` of changes, `results` (detail, variables, cell_stride) and
    /// `discretisation` (max_cell_length).  Prefer `results.detail: "summary"`
    /// unless the per-cell history is the point — `profiles` and `full` return
    /// samples x cells x variables floats and are refused above a couple of million
    /// of them.
    pub transient_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_transient_wave`](crate::GraphSolve::run_transient_wave).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunTransientWaveParams {
    /// Transient request as a JSON string. Either `network_payload` plus a `path`
    /// naming the pipe edges to run, or a bare `segments` list plus a `fluid`; then
    /// `time` (total, unit, record_interval_s), the initial `boundary`, an optional
    /// `schedule` of changes, `results` (detail, variables, cell_stride) and
    /// `discretisation` (max_cell_length).  Prefer `results.detail: "summary"`
    /// unless the per-cell history is the point — `profiles` and `full` return
    /// samples x cells x variables floats and are refused above a couple of million
    /// of them.
    pub transient_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::screen_asphaltene_risk`](crate::GraphSolve::screen_asphaltene_risk).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ScreenAsphalteneRiskParams {
    /// Bubble-point pressure in MPa
    pub bubble_point_pressure: f64,
    /// Live-oil density at the reservoir state \[kg/m3\]
    pub live_oil_density: f64,
    /// Reservoir (or local) pressure in MPa
    pub reservoir_pressure: f64,
}

/// Arguments for [`GraphSolve::screen_hydrate_risk`](crate::GraphSolve::screen_hydrate_risk).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ScreenHydrateRiskParams {
    /// Local pressure in MPa
    pub pressure: f64,
    /// Local operating temperature in K
    pub temperature: f64,
    /// Component names. Required by the vdWP models — a specific gravity carries no
    /// cavity-occupancy information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Thermodynamic inhibitor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inhibitor: Option<InhibitorName>,
    /// Inhibitor mass fraction in the WATER phase \[wt %\], not in the bulk stream
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inhibitor_wt_pct: Option<f64>,
    /// Hydrate thermodynamics (default vdwp_combined)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<HydrateModelName>,
    /// Vapour mole fractions, aligned with component_names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mole_fractions: Option<Vec<f64>>,
    /// Component molecular weights \[g/mol\], to derive a specific gravity for
    /// towler_mokhatab
    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_weights: Option<Vec<f64>>,
    /// Gas specific gravity (air = 1). towler_mokhatab only; derived from the
    /// composition when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specific_gravity: Option<f64>,
}

/// Arguments for [`GraphSolve::screen_liquid_loading`](crate::GraphSolve::screen_liquid_loading).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ScreenLiquidLoadingParams {
    /// Gas density \[kg/m3\]
    pub gas_density: f64,
    /// Actual superficial gas velocity \[m/s\]
    pub gas_velocity: f64,
    /// Liquid density \[kg/m3\]
    pub liquid_density: f64,
    /// Gas-liquid surface tension \[N/m\]
    pub surface_tension: f64,
}

/// Arguments for [`GraphSolve::screen_wax_risk`](crate::GraphSolve::screen_wax_risk).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ScreenWaxRiskParams {
    /// Local operating temperature in K
    pub temperature: f64,
    /// C7+ density \[kg/m3\] — screening model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density_c7_plus: Option<f64>,
    /// Wax model (default screening)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<WaxModelName>,
    /// Component mole fractions — Won model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mole_fractions: Option<Vec<f64>>,
    /// Component molecular weights \[g/mol\] — Won model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_weights: Option<Vec<f64>>,
    /// C7+ molecular weight \[g/mol\] — screening model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mw_c7_plus: Option<f64>,
    /// Paraffin mass fraction of the C7+ cut — screening model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paraffin_mass_fraction: Option<f64>,
    /// Watson K, used to estimate the paraffin fraction when it is not supplied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watson_k: Option<f64>,
    /// Which components are wax formers. Omit to use the molecular-weight fallback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wax_former_mask: Option<Vec<bool>>,
}

/// Arguments for [`GraphSolve::solve_network`](crate::GraphSolve::solve_network).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SolveNetworkParams {
    /// Complete network definition in the model format. Accepts the object
    /// directly, or the same payload as a JSON string.  The elements of a saved
    /// model as they are stored: `elements\[\]` of `{group, data}` with string ids,
    /// `node_type` by name (`fixed_rate_source`, `fixed_pressure_source`,
    /// `pressure_dependent_source`, `network_node`, `fixed_pressure_sink`,
    /// `fixed_rate_sink`), `edge_type` by name (`no_pressure_loss`, `pipe`,
    /// `choke`, `heat_exchanger`, `compressor`, `pump`, `turbine`, `jt_valve`,
    /// `heater_cooler`, `reactor`), edges joined by `source`/`target`, with the
    /// `fluids`, `sources` and `process_equipment` libraries at the top level and
    /// an optional `solve_parameters` block. A model exported from the app is
    /// accepted as it is. The reply carries `results` on each element.
    pub network_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::solve_network_map`](crate::GraphSolve::solve_network_map).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SolveNetworkMapParams {
    /// Network JSON string accepted by `solve_network`. Put a `variance` on each
    /// trusted measurement: `fixed_pressure.variance` \[MPa^2\] on a node, or a
    /// measured-rate `variance` \[kg^2/s^2\] on an edge/source. A tight (small)
    /// variance means a trusted gauge. Include a `map_config` inside
    /// `solve_parameters` to tune `physics_variance` and the lambda schedule; if
    /// omitted, MAP mode is still enabled with engine defaults.
    pub network_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::split_plus_fraction`](crate::GraphSolve::split_plus_fraction).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SplitPlusFractionParams {
    /// Split configuration as a JSON string: the composition, overall mole
    /// fractions, `heavy_index`, `number_of_pseudo_components`, and optional gamma-
    /// distribution parameters (`alpha`, `eta`, `highest_mw`).
    pub split_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::transient_bhp_history`](crate::GraphSolve::transient_bhp_history).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransientBhpHistoryParams {
    /// Full inflow-model spec (see `generate_ipr_curve`). For matching, include
    /// `test_points` (observed rate + flowing-BHP pairs). For the transient BHP
    /// history, include `rate_history`; for the rate history a `pressure_history`;
    /// for RTA an `rta_history` (time/rate/pressure), plus the transient model's
    /// reservoir inputs.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::transient_rate_history`](crate::GraphSolve::transient_rate_history).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransientRateHistoryParams {
    /// Full inflow-model spec (see `generate_ipr_curve`). For matching, include
    /// `test_points` (observed rate + flowing-BHP pairs). For the transient BHP
    /// history, include `rate_history`; for the rate history a `pressure_history`;
    /// for RTA an `rta_history` (time/rate/pressure), plus the transient model's
    /// reservoir inputs.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::tune_mpfm_allocation`](crate::GraphSolve::tune_mpfm_allocation).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TuneMpfmAllocationParams {
    /// MPFM tuning configuration as a JSON string: one or more observation rows
    /// (measured vs reference) plus the parameters and bounds to fit. The inverse
    /// of `calculate_mpfm_allocation`.
    pub tuning_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::validate_solver_payload`](crate::GraphSolve::validate_solver_payload).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ValidateSolverPayloadParams {
    /// Same network JSON string accepted by `solve_network`.
    pub network_json: serde_json::Value,
}

impl GraphSolve {
    /// Tune the heavy/light split of a composition to match a target surface
    /// GOR (Sm³/Sm³).
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn adjust_composition_to_gor(
        &self,
        params: AdjustCompositionToGorParams,
    ) -> Result<ToolResponse> {
        self.call("adjust_composition_to_gor", &params).await
    }

    /// Tune a composition to a target in-situ GOR (m3/m3) at given P/T.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn adjust_composition_to_phase_ratio(
        &self,
        params: AdjustCompositionToPhaseRatioParams,
    ) -> Result<ToolResponse> {
        self.call("adjust_composition_to_phase_ratio", &params)
            .await
    }

    /// Aggregate analog wells into a probabilistic type well: P10/P50/P90 EUR,
    /// probability plot, representative declines, optional N-well program
    /// aggregate.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn aggregate_type_well(
        &self,
        params: AggregateTypeWellParams,
    ) -> Result<ToolResponse> {
        self.call("aggregate_type_well", &params).await
    }

    /// Source-tagged production allocation (back-allocation): attribute
    /// commingled rates back to each tagged source, including lift-gas
    /// accounting.
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn allocate_production(
        &self,
        params: AllocateProductionParams,
    ) -> Result<ToolResponse> {
        self.call("allocate_production", &params).await
    }

    /// Straight-line material-balance diagnostics: gas p/Z → OGIP, or Havlena-
    /// Odeh F-vs-Et → STOIIP/GIIP, with R² and drive-support intercept.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn analyze_material_balance(
        &self,
        params: AnalyzeMaterialBalanceParams,
    ) -> Result<ToolResponse> {
        self.call("analyze_material_balance", &params).await
    }

    /// Analytical aquifer water influx We over a pressure history: Fetkovich
    /// (PSS), Carter-Tracy or van Everdingen-Hurst (USS radial).
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_aquifer_influx(
        &self,
        params: CalculateAquiferInfluxParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_aquifer_influx", &params).await
    }

    /// Pressure drop across a choke of known diameter at given rates (Sachdeva
    /// multiphase model, critical/subcritical).
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_choke_pressure_drop(
        &self,
        params: CalculateChokePressureDropParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_choke_pressure_drop", &params).await
    }

    /// Size a choke: find the bean diameter that gives a target downstream
    /// pressure at the supplied rates.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_choke_size(
        &self,
        params: CalculateChokeSizeParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_choke_size", &params).await
    }

    /// Single-stage centrifugal compressor: outlet P/T and power from inlet
    /// P/T, pressure ratio, and polytropic efficiency.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_compressor(
        &self,
        params: CalculateCompressorParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_compressor", &params).await
    }

    /// Internal CO2/H2S corrosion rate over three models (de Waard-Lotz, sour
    /// with the Mariaca regime factor, NORSOK M-506), with NACE MR0175 region,
    /// inhibited rate and wall allowance.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_corrosion_rate(
        &self,
        params: CalculateCorrosionRateParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_corrosion_rate", &params).await
    }

    /// API RP 14E erosional velocity limit and the actual mixture velocity for
    /// a pipe.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_erosional_velocity(
        &self,
        params: CalculateErosionalVelocityParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_erosional_velocity", &params).await
    }

    /// Black-oil PVT properties (Bo, Rs, viscosity, density, Z) of oil / gas /
    /// water at a given P/T using the selected correlations.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_fluid_properties(
        &self,
        params: CalculateFluidPropertiesParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_fluid_properties", &params).await
    }

    /// Gas dew-point pressure from temperature, gas gravity, oil API and
    /// condensate-gas ratio (Ahmadi).
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_gas_dew_point(
        &self,
        params: CalculateGasDewPointParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_gas_dew_point", &params).await
    }

    /// Heater/cooler in fixed_duty, fixed_temperature, or approach_temperature
    /// mode; returns outlet P/T and duty.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_heater_cooler(
        &self,
        params: CalculateHeaterCoolerParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_heater_cooler", &params).await
    }

    /// Hydrate formation temperature at a given pressure (Baillie-Wichert) with
    /// sour-gas corrections.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_hydrate_temperature(
        &self,
        params: CalculateHydrateTemperatureParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_hydrate_temperature", &params).await
    }

    /// Outlet temperature after a constant-enthalpy (Joule-Thomson) expansion
    /// to a lower pressure.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_isenthalpic_temperature(
        &self,
        params: CalculateIsenthalpicTemperatureParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_isenthalpic_temperature", &params)
            .await
    }

    /// Joule-Thomson throttle valve: outlet T after an isenthalpic pressure
    /// drop.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_jt_valve(&self, params: CalculateJtValveParams) -> Result<ToolResponse> {
        self.call("calculate_jt_valve", &params).await
    }

    /// Minimum miscibility pressure (MMP) between a reservoir fluid and an
    /// injection gas (mixing-cell method).
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_mmp(&self, params: CalculateMmpParams) -> Result<ToolResponse> {
        self.call("calculate_mmp", &params).await
    }

    /// Multiphase-flow-meter allocation: distribute measured rates among
    /// streams (in-situ -> standard conditions).
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_mpfm_allocation(
        &self,
        params: CalculateMpfmAllocationParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_mpfm_allocation", &params).await
    }

    /// Multi-stage centrifugal train with optional inter-stage cooling; per-
    /// stage pressure ratios and overall discharge P/T, cooler duty and shaft
    /// power.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_multistage_compressor(
        &self,
        params: CalculateMultistageCompressorParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_multistage_compressor", &params).await
    }

    /// Operating point = IPR intersect VLP. Generates both curves and finds the
    /// stabilised rate and flowing BHP.
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_nodal_analysis(
        &self,
        params: CalculateNodalAnalysisParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_nodal_analysis", &params).await
    }

    /// Watercut / GOR / phase fractions from volumetric phase rates (any flow-
    /// rate units).
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_phase_cuts(
        &self,
        params: CalculatePhaseCutsParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_phase_cuts", &params).await
    }

    /// March pressure and temperature along a single pipe (multi-segment) with
    /// heat transfer.
    ///
    /// Costs 3 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_pipe_traverse(
        &self,
        params: CalculatePipeTraverseParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_pipe_traverse", &params).await
    }

    /// Multiphase pressure gradient for a single pipe segment using a chosen
    /// correlation. Returns gradient components, holdup and flow regime.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_pressure_drop(
        &self,
        params: CalculatePressureDropParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_pressure_drop", &params).await
    }

    /// ESP/pump outlet pressure from a head-vs-rate performance curve,
    /// interpolated at the operating rate and converted with liquid density.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_pump_head(
        &self,
        params: CalculatePumpHeadParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_pump_head", &params).await
    }

    /// Find the rates a fixed choke passes for a given pressure drop, holding a
    /// phase or ratio (watercut/GOR/WGR/CGR) fixed.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_rate_from_choke(
        &self,
        params: CalculateRateFromChokeParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_rate_from_choke", &params).await
    }

    /// Reciprocating (positive-displacement) compressor. Mass flow is set by
    /// displacement x speed x volumetric efficiency, not supplied.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_reciprocating_compressor(
        &self,
        params: CalculateReciprocatingCompressorParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_reciprocating_compressor", &params)
            .await
    }

    /// Reid vapour pressure (RVP) of a liquid composition at 100 degF.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_reid_vapour_pressure(
        &self,
        params: CalculateReidVapourPressureParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_reid_vapour_pressure", &params).await
    }

    /// Bubble- or dew-point pressure of a composition at a given temperature
    /// (cubic EOS).
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_saturation_pressure(
        &self,
        params: CalculateSaturationPressureParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_saturation_pressure", &params).await
    }

    /// Screw (positive-displacement) compressor closed by black-box
    /// efficiencies; flow set by displacement x shaft speed x volumetric
    /// efficiency.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_screw_compressor(
        &self,
        params: CalculateScrewCompressorParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_screw_compressor", &params).await
    }

    /// Single-stage centrifugal turbine/expander: outlet P/T and power
    /// generated from inlet P/T and an expansion pressure ratio (0 < PR < 1).
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_turbine(&self, params: CalculateTurbineParams) -> Result<ToolResponse> {
        self.call("calculate_turbine", &params).await
    }

    /// Forward OOIP / OGIP from area·thickness·NTG·φ·(1−Sw) ÷ FVF; scalar or
    /// per-realisation arrays, composes with generate_samples /
    /// compute_statistics for probabilistic in-place volumes.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_volumetrics(
        &self,
        params: CalculateVolumetricsParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_volumetrics", &params).await
    }

    /// Brine properties (density, viscosity, compressibility, FVF) via IAPWS-95
    /// with a salinity correction.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_water_properties(
        &self,
        params: CalculateWaterPropertiesParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_water_properties", &params).await
    }

    /// Wax deposition rate on a cold wall from the subcooling and the wax
    /// solubility gradient — thickness in mm/yr and mass flux in g/m2/day.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_wax_deposition_rate(
        &self,
        params: CalculateWaxDepositionRateParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_wax_deposition_rate", &params).await
    }

    /// Critical properties (Tc, Pc, Vc, Watson K) of a pseudo-component from
    /// any two of MW / specific gravity / boiling point.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn characterize_pseudo_component(
        &self,
        params: CharacterizePseudoComponentParams,
    ) -> Result<ToolResponse> {
        self.call("characterize_pseudo_component", &params).await
    }

    /// Walk a multi-segment pipe with each correlation and compare the
    /// predicted outlet pressure.
    ///
    /// Costs 3 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn compare_pipeline_correlations(
        &self,
        params: ComparePipelineCorrelationsParams,
    ) -> Result<ToolResponse> {
        self.call("compare_pipeline_correlations", &params).await
    }

    /// Run every multiphase pressure-drop correlation (or a subset) on one
    /// segment and compare the predicted gradients.
    ///
    /// Costs 3 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn compare_pressure_drop_correlations(
        &self,
        params: ComparePressureDropCorrelationsParams,
    ) -> Result<ToolResponse> {
        self.call("compare_pressure_drop_correlations", &params)
            .await
    }

    /// Summary statistics (mean/std, percentiles, histogram, CDF) over labelled
    /// output columns, plus optional input→output correlation sensitivity;
    /// network-agnostic (e.g. NPV/EUR from an external model).
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn compute_statistics(
        &self,
        params: ComputeStatisticsParams,
    ) -> Result<ToolResponse> {
        self.call("compute_statistics", &params).await
    }

    /// Convert a value between units (pressure, rate, temperature, length,
    /// area, ...).
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn convert_units(&self, params: ConvertUnitsParams) -> Result<ToolResponse> {
        self.call("convert_units", &params).await
    }

    /// Hydrate, corrosion and erosion screening along a whole line, each
    /// reporting the controlling sample — index, length, pressure and
    /// temperature — not just a worst-case number.
    ///
    /// Costs 8 credits, plus 1 per 250 ms beyond the first 4 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn evaluate_flow_assurance_profile(
        &self,
        params: EvaluateFlowAssuranceProfileParams,
    ) -> Result<ToolResponse> {
        self.call("evaluate_flow_assurance_profile", &params).await
    }

    /// History-match a decline model (Arps / mod-hyperbolic / Duong / SEPD /
    /// PLE) to a time/rate history with optional outlier rejection; omit kind
    /// to auto-select by R².
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn fit_decline(&self, params: FitDeclineParams) -> Result<ToolResponse> {
        self.call("fit_decline", &params).await
    }

    /// Flash a reservoir composition to stock-tank: GOR, API/oil density,
    /// shrinkage.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn flash_to_surface(&self, params: FlashToSurfaceParams) -> Result<ToolResponse> {
        self.call("flash_to_surface", &params).await
    }

    /// Generate a decline curve (rate + cumulative) from known parameters:
    /// Arps, modified-hyperbolic, Duong, stretched-exponential (SEPD), power-
    /// law-exponential (PLE).
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn generate_decline(&self, params: GenerateDeclineParams) -> Result<ToolResponse> {
        self.call("generate_decline", &params).await
    }

    /// Inflow performance relationship (IPR) curve for one well; real physics
    /// per point. Required inputs depend on ipr_model.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn generate_ipr_curve(&self, params: GenerateIprCurveParams) -> Result<ToolResponse> {
        self.call("generate_ipr_curve", &params).await
    }

    /// Two-phase P-T envelope (dew/bubble locus + critical point) of a
    /// composition (cubic EOS only; not GERG).
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn generate_phase_envelope(
        &self,
        params: GeneratePhaseEnvelopeParams,
    ) -> Result<ToolResponse> {
        self.call("generate_phase_envelope", &params).await
    }

    /// Draw Monte-Carlo / LHS / Sobol samples from named distributions
    /// (Normal/Uniform/LogNormal/Triangular/Beta/PERT/Discrete) with optional
    /// correlations; simple or Saltelli design; network-agnostic, feeds any
    /// model.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 3 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn generate_samples(&self, params: GenerateSamplesParams) -> Result<ToolResponse> {
        self.call("generate_samples", &params).await
    }

    /// Dimensionless transient type-curve surface (pD/qD, Bourdet derivative)
    /// for a transient ipr_model.
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn generate_type_curve(
        &self,
        params: GenerateTypeCurveParams,
    ) -> Result<ToolResponse> {
        self.call("generate_type_curve", &params).await
    }

    /// Critical properties (Tc, Pc, omega, MW) of a single EOS component by
    /// name.
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn get_eos_component(&self, params: GetEosComponentParams) -> Result<ToolResponse> {
        self.call("get_eos_component", &params).await
    }

    /// List the available correlations, optionally filtered by category.
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn list_correlations(&self, params: ListCorrelationsParams) -> Result<ToolResponse> {
        self.call("list_correlations", &params).await
    }

    /// List the network edge types (pipeline, choke, compressor, ...) and their
    /// params.
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn list_edge_types(&self) -> Result<ToolResponse> {
        self.call("list_edge_types", &serde_json::json!({})).await
    }

    /// Peng-Robinson binary interaction parameters for a component.
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn list_eos_binary_interactions(
        &self,
        params: ListEosBinaryInteractionsParams,
    ) -> Result<ToolResponse> {
        self.call("list_eos_binary_interactions", &params).await
    }

    /// List the component names in the built-in equation-of-state database.
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn list_eos_components(&self) -> Result<ToolResponse> {
        self.call("list_eos_components", &serde_json::json!({}))
            .await
    }

    /// The model vocabularies the flow-assurance tools accept — hydrate, wax
    /// and corrosion models with their tier, plus inhibitor names and vdWP
    /// guests.
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn list_flow_assurance_models(&self) -> Result<ToolResponse> {
        self.call("list_flow_assurance_models", &serde_json::json!({}))
            .await
    }

    /// List the fluid types (oil, gas, water) and the required PVT
    /// configuration.
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn list_fluid_types(&self) -> Result<ToolResponse> {
        self.call("list_fluid_types", &serde_json::json!({})).await
    }

    /// List the network node types (junction, source, sink) and their params.
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn list_node_types(&self) -> Result<ToolResponse> {
        self.call("list_node_types", &serde_json::json!({})).await
    }

    /// Which of the three transient schemes to use, what each carries, and the
    /// conventions every transient request shares.
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn list_transient_solvers(&self) -> Result<ToolResponse> {
        self.call("list_transient_solvers", &serde_json::json!({}))
            .await
    }

    /// Fit the Fetkovich C and n (deliverability) coefficients to test points.
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn match_fetkovich_ab(&self, params: MatchFetkovichAbParams) -> Result<ToolResponse> {
        self.call("match_fetkovich_ab", &params).await
    }

    /// Fit the Forchheimer A and B (non-Darcy gas) coefficients to test points.
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn match_forchheimer_ab(
        &self,
        params: MatchForchheimerAbParams,
    ) -> Result<ToolResponse> {
        self.call("match_forchheimer_ab", &params).await
    }

    /// Unified matching dispatcher: fit productivity_index / skin /
    /// forchheimer_ab / fetkovich_ab, routed by match_target or ipr_model.
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn match_parameters(&self, params: MatchParametersParams) -> Result<ToolResponse> {
        self.call("match_parameters", &params).await
    }

    /// Fit the productivity index to observed (rate, flowing-BHP) test points.
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn match_productivity_index(
        &self,
        params: MatchProductivityIndexParams,
    ) -> Result<ToolResponse> {
        self.call("match_productivity_index", &params).await
    }

    /// Regress black-oil PVT correlations against measured lab data (bubble
    /// point, Rs, Bo, viscosity, density).
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn match_pvt(&self, params: MatchPvtParams) -> Result<ToolResponse> {
        self.call("match_pvt", &params).await
    }

    /// Fit the skin factor to observed (rate, flowing-BHP) test points.
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn match_skin(&self, params: MatchSkinParams) -> Result<ToolResponse> {
        self.call("match_skin", &params).await
    }

    /// Optimise a network: move control variables within bounds to maximise /
    /// minimise an objective (phase rate, revenue, pressure drop, power) under
    /// rate / pressure / resource constraints. The optimisation counterpart of
    /// solve_network.
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn optimise_network(&self, params: OptimiseNetworkParams) -> Result<ToolResponse> {
        self.call("optimise_network", &params).await
    }

    /// Rate-transient-analysis diagnostics (Blasingame, Agarwal-Gardner,
    /// Bourdet) with material-balance time and pseudo-time, from a
    /// time/rate/pressure history.
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn rta_diagnostics(&self, params: RtaDiagnosticsParams) -> Result<ToolResponse> {
        self.call("rta_diagnostics", &params).await
    }

    /// Constant composition expansion (CCE) PVT experiment.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_cce(&self, params: RunCceParams) -> Result<ToolResponse> {
        self.call("run_cce", &params).await
    }

    /// Constant volume depletion (CVD) PVT experiment (gas condensate /
    /// volatile oil).
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_cvd(&self, params: RunCvdParams) -> Result<ToolResponse> {
        self.call("run_cvd", &params).await
    }

    /// Differential liberation expansion (DLE) PVT experiment.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_dle(&self, params: RunDleParams) -> Result<ToolResponse> {
        self.call("run_dle", &params).await
    }

    /// PT flash of a composition (Peng-Robinson default, SRK, Patel-Teja):
    /// phase split, K-values, densities. Pressure in MPa.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_eos_flash(&self, params: RunEosFlashParams) -> Result<ToolResponse> {
        self.call("run_eos_flash", &params).await
    }

    /// Time-series production forecast: march a network through time, solving
    /// or optimising each step with decline curves, scheduled events and
    /// transient-IPR wells; returns per-timestep rates and cumulatives.
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_forecast(&self, params: RunForecastParams) -> Result<ToolResponse> {
        self.call("run_forecast", &params).await
    }

    /// Material-balance depletion / reserves over one or more reservoir blocks
    /// (STOIIP/GIIP, recovery, pressure decline, aquifer).
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_material_balance(
        &self,
        params: RunMaterialBalanceParams,
    ) -> Result<ToolResponse> {
        self.call("run_material_balance", &params).await
    }

    /// Trace component compositions through an already-solved network.
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_molecule_tracking(
        &self,
        params: RunMoleculeTrackingParams,
    ) -> Result<ToolResponse> {
        self.call("run_molecule_tracking", &params).await
    }

    /// Sensitivity / parametric sweep over a network (linear / tornado / grid /
    /// Monte Carlo); returns per-output statistics.
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_parametric_study(
        &self,
        params: RunParametricStudyParams,
    ) -> Result<ToolResponse> {
        self.call("run_parametric_study", &params).await
    }

    /// Simulate a process flow graph (mixers, separators, heaters, compressors)
    /// of compositional streams.
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_process_graph(&self, params: RunProcessGraphParams) -> Result<ToolResponse> {
        self.call("run_process_graph", &params).await
    }

    /// Multi-stage separator test to stock-tank: stage GOR, FVF, stock-tank oil
    /// density.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_separator_test(&self, params: RunSeparatorTestParams) -> Result<ToolResponse> {
        self.call("run_separator_test", &params).await
    }

    /// Swelling test: add injection gas/solvent and report saturation
    /// pressure + swelling factor per step.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_swelling_test(&self, params: RunSwellingTestParams) -> Result<ToolResponse> {
        self.call("run_swelling_test", &params).await
    }

    /// Transient multiphase pipe flow, fully-implicit four-field two-fluid
    /// (Graphsolve-Field): for severe slugging and countercurrent flow.
    ///
    /// Costs 250 credits, plus 1 per 50 ms beyond the first 30 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_transient_field(
        &self,
        params: RunTransientFieldParams,
    ) -> Result<ToolResponse> {
        self.call("run_transient_field", &params).await
    }

    /// Transient multiphase pipe flow, semi-implicit sequential (Graphsolve-
    /// Flux): the general-purpose scheme, carrying temperature, composition and
    /// salinity.
    ///
    /// Costs 250 credits, plus 1 per 50 ms beyond the first 30 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_transient_flux(&self, params: RunTransientFluxParams) -> Result<ToolResponse> {
        self.call("run_transient_flux", &params).await
    }

    /// Transient multiphase pipe flow, explicit Godunov/Rusanov drift flux
    /// (Graphsolve-Wave): the only scheme that resolves pressure waves.
    ///
    /// Costs 250 credits, plus 1 per 50 ms beyond the first 30 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_transient_wave(&self, params: RunTransientWaveParams) -> Result<ToolResponse> {
        self.call("run_transient_wave", &params).await
    }

    /// De Boer (1995) asphaltene-onset screening from in-situ undersaturation
    /// and live-oil density; four-bin risk classification.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn screen_asphaltene_risk(
        &self,
        params: ScreenAsphalteneRiskParams,
    ) -> Result<ToolResponse> {
        self.call("screen_asphaltene_risk", &params).await
    }

    /// Hydrate formation temperature and margin at a live state, over four
    /// models (Towler-Mokhatab screening or vdW-Platteeuw sI / sII / combined),
    /// with Hammerschmidt inhibitor depression.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn screen_hydrate_risk(
        &self,
        params: ScreenHydrateRiskParams,
    ) -> Result<ToolResponse> {
        self.call("screen_hydrate_risk", &params).await
    }

    /// Turner critical-velocity screen for gas-well liquid loading: critical
    /// velocity, ratio and a loading flag.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn screen_liquid_loading(
        &self,
        params: ScreenLiquidLoadingParams,
    ) -> Result<ToolResponse> {
        self.call("screen_liquid_loading", &params).await
    }

    /// Wax appearance temperature and margin: a C7+ screening correlation
    /// (Tier 1) or the Won multi-solid SLE (Tier 2) with per-component solid
    /// fractions.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn screen_wax_risk(&self, params: ScreenWaxRiskParams) -> Result<ToolResponse> {
        self.call("screen_wax_risk", &params).await
    }

    /// Solve a complete production network (Newton-Raphson): pressures,
    /// temperatures and rates at every node and edge. The headline tool.
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn solve_network(&self, params: SolveNetworkParams) -> Result<ToolResponse> {
        self.call("solve_network", &params).await
    }

    /// Data-reconciliation solve (MAP): inject measured values with a variance
    /// and reconcile them, returning a posterior variance per reconciled
    /// quantity.
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn solve_network_map(&self, params: SolveNetworkMapParams) -> Result<ToolResponse> {
        self.call("solve_network_map", &params).await
    }

    /// Split a heavy/plus fraction into N pseudo-components (gamma
    /// distribution). Front half of building a matched compositional fluid.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn split_plus_fraction(
        &self,
        params: SplitPlusFractionParams,
    ) -> Result<ToolResponse> {
        self.call("split_plus_fraction", &params).await
    }

    /// Compute the flowing-BHP history from a rate history (transient
    /// superposition).
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn transient_bhp_history(
        &self,
        params: TransientBhpHistoryParams,
    ) -> Result<ToolResponse> {
        self.call("transient_bhp_history", &params).await
    }

    /// Compute the rate history from a pressure history (transient
    /// superposition).
    ///
    /// Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn transient_rate_history(
        &self,
        params: TransientRateHistoryParams,
    ) -> Result<ToolResponse> {
        self.call("transient_rate_history", &params).await
    }

    /// Regress MPFM allocation parameters against measured observation rows
    /// (inverse of calculate_mpfm_allocation).
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn tune_mpfm_allocation(
        &self,
        params: TuneMpfmAllocationParams,
    ) -> Result<ToolResponse> {
        self.call("tune_mpfm_allocation", &params).await
    }

    /// Pre-flight check a network payload without solving (runs the same build
    /// step solve_network does). Run this first when assembling a network.
    ///
    /// Free.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn validate_solver_payload(
        &self,
        params: ValidateSolverPayloadParams,
    ) -> Result<ToolResponse> {
        self.call("validate_solver_payload", &params).await
    }
}
