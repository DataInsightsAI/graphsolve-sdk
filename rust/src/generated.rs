//! Typed methods for every tool in the GraphSolve API.
//!
//! GENERATED FROM spec/graphsolve-v1.json — DO NOT EDIT BY HAND.
//! Run `python emit/emit_rust.py` after a spec change; CI fails if this
//! file and the spec disagree.
//!
//! Engine API version: 1.0.40
//! Tools: 98

use serde::{Deserialize, Serialize};

use crate::{GraphSolve, Result, ToolResponse};

/// Every tool this client knows about, in the order the API lists them.
pub const TOOL_NAMES: [&str; 98] = [
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
    "calculate_critical_point",
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
    "calculate_pump",
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
    "import_prp_fluid",
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
    "partition_acid_gas_in_water",
    "rta_diagnostics",
    "run_cce",
    "run_cvd",
    "run_dle",
    "run_eos_flash",
    "run_forecast",
    "run_gas_depletion",
    "run_material_balance",
    "run_mmp_probe",
    "run_molecule_tracking",
    "run_nodal_study",
    "run_parametric_study",
    "run_process_graph",
    "run_pvt_regression_suite",
    "run_separator_test",
    "run_swelling_test",
    "run_transient_field",
    "run_transient_flux",
    "run_transient_wave",
    "screen_asphaltene_risk",
    "screen_hydrate_risk",
    "screen_liquid_loading",
    "screen_scale_risk",
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

/// Cubic equation of state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EosModelName {
    /// Peng-Robinson, with the 1978 alpha function for acentric factors above
    /// 0.49. Default.
    #[default]
    #[serde(rename = "PengRobinson")]
    PengRobinson,
    /// Soave-Redlich-Kwong.
    #[serde(rename = "SoaveRedlichKwong")]
    SoaveRedlichKwong,
}

/// Phase viscosity model for the EOS property report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EosViscosityModelName {
    /// Chosen from the phase's reduced density and molecular weight. Default.
    #[default]
    #[serde(rename = "auto")]
    Auto,
    /// Lucas corresponding-states gas viscosity (low pressure).
    #[serde(rename = "lucas")]
    Lucas,
    /// Lohrenz-Bray-Clark dense-fluid mixture viscosity.
    #[serde(rename = "lbc")]
    Lbc,
    /// Wilke mixing rule over pure-component low-pressure gas viscosities.
    #[serde(rename = "wilke")]
    Wilke,
    /// Herning-Zipperer gas mixing rule.
    #[serde(rename = "herning_zipperer")]
    HerningZipperer,
    /// Chapman-Enskog dilute-gas kinetic theory.
    #[serde(rename = "chapman_enskog")]
    ChapmanEnskog,
    /// Pedersen corresponding states with a methane reference.
    #[serde(rename = "pedersen")]
    Pedersen,
    /// TRAPP extended corresponding states.
    #[serde(rename = "trapp")]
    Trapp,
    /// Yarranton expanded-fluid model (heavy oils, all densities).
    #[serde(rename = "expanded_fluid")]
    ExpandedFluid,
    /// Burgoyne-Nielsen-Stanko black-oil correlation.
    #[serde(rename = "burgoyne_nielsen_stanko")]
    BurgoyneNielsenStanko,
}

/// Equation of state for `run_eos_flash`: the two cubics, or GERG-2008 for
/// single-phase gas properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FlashEosModelName {
    /// Peng-Robinson, with the 1978 alpha function for acentric factors above
    /// 0.49. Default.
    #[default]
    #[serde(rename = "PengRobinson")]
    PengRobinson,
    /// Soave-Redlich-Kwong.
    #[serde(rename = "SoaveRedlichKwong")]
    SoaveRedlichKwong,
    /// GERG-2008 reference equation for natural gases (Kunz and Wagner, 2012).
    /// Single phase only: the fluid is treated as one gas phase whatever its
    /// state, with no phase split. Covers 21 components: methane to n-decane,
    /// nitrogen, carbon dioxide, hydrogen sulfide, water, oxygen, argon,
    /// helium, hydrogen and carbon monoxide.
    #[serde(rename = "Gerg2008")]
    Gerg2008,
}

/// Which pair of state variables the flash is specified by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FlashTypeName {
    /// Pressure and temperature. Default.
    #[default]
    #[serde(rename = "pt")]
    Pt,
    /// Pressure and molar enthalpy: solves for the temperature at which the
    /// feed's molar enthalpy equals `enthalpy_j_per_mol`.
    #[serde(rename = "ph")]
    Ph,
    /// Pressure and molar entropy: solves for the temperature at which the
    /// feed's molar entropy equals `entropy_j_per_mol_k`.
    #[serde(rename = "ps")]
    Ps,
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
    #[serde(rename = "QC-high")]
    QcHigh,
    #[serde(rename = "QC-low")]
    QcLow,
    #[serde(rename = "SinglePhaseGas")]
    SinglePhaseGas,
    #[serde(rename = "SUPREME")]
    Supreme,
}

/// Heater/cooler operating mode, spelt as the network `heater_cooler_data.mode`.
/// Every mode closes the energy balance Q = ṁ·cp·(T_out − T_in) on the stream's
/// mass rate and mass-weighted heat capacity at the inlet state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeaterCoolerModeName {
    /// `heat_duty` is given and the outlet temperature follows.
    #[serde(rename = "fixed_duty")]
    FixedDuty,
    /// `outlet_temperature` is given and the duty follows.
    #[serde(rename = "fixed_outlet_temperature")]
    FixedOutletTemperature,
    /// The outlet sits `approach_temperature` short of `utility_temperature`:
    /// above it for a cooler, below it for a heater, and never past the inlet
    /// temperature.
    #[serde(rename = "approach_temperature")]
    ApproachTemperature,
    /// ε-NTU against an isothermal utility at `utility_temperature` with
    /// conductance `ua`: T_out = T_in + (T_util − T_in)·(1 − exp(−UA/(ṁ·cp))).
    #[serde(rename = "ua")]
    Ua,
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

/// The coefficient `match_parameters` fits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchTargetName {
    /// The `pi` model's productivity index.
    #[serde(rename = "productivity_index")]
    ProductivityIndex,
    /// Mechanical skin of a `darcy` or `fetkovich` model.
    #[serde(rename = "skin")]
    Skin,
    /// A and B of the `forchheimer_ab` gas deliverability equation.
    #[serde(rename = "forchheimer_ab")]
    ForchheimerAb,
    /// A and B of the `fetkovich_ab` oil deliverability equation.
    #[serde(rename = "fetkovich_ab")]
    FetkovichAb,
}

/// End of a pipe: `top` is the end the pipe angle points towards, `bottom`
/// the end it is measured from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipeEnd {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "bottom")]
    Bottom,
}

/// Critical-property correlation for a pseudo-component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PseudoCriticalMethodName {
    /// Kesler and Lee (1976) critical temperature and pressure from the
    /// specific gravity and boiling point. Default.
    #[default]
    #[serde(rename = "kesler_lee")]
    KeslerLee,
    /// Twu (1984) perturbation from the n-alkane reference, from the specific
    /// gravity and boiling point.
    #[serde(rename = "twu")]
    Twu,
    /// Sancet critical temperature and pressure from the molecular weight
    /// alone.
    #[serde(rename = "sancet")]
    Sancet,
}

/// Saturation-point algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SaturationAlgorithmName {
    /// Wilson-seeded successive substitution with a Newton update on the
    /// unknown pressure or temperature. Fast; can fail near the critical
    /// point and in retrograde regions. A composition with a component whose
    /// acentric factor exceeds 0.9 is moved to `envelope` automatically.
    /// Default.
    #[default]
    #[serde(rename = "classical")]
    Classical,
    /// Simultaneous Newton with an analytic Jacobian, well conditioned as the
    /// K-values approach one. Falls back to `classical` if it fails.
    #[serde(rename = "bell_jaeger")]
    BellJaeger,
    /// Runs the classical method from several seeds (Wilson, inverse Wilson,
    /// a pressure scan) and keeps the best. About five times the classical
    /// cost.
    #[serde(rename = "multi_start")]
    MultiStart,
    /// Traces the phase envelope and interpolates at the target, taking the
    /// upper dew point where there are two (lean condensates). Most robust;
    /// about fifty times the classical cost.
    #[serde(rename = "envelope")]
    Envelope,
}

/// Which saturation boundary to locate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SaturationBoundaryName {
    /// Bubble point: the feed is liquid and an incipient vapour appears.
    /// Default.
    #[default]
    #[serde(rename = "bubble")]
    Bubble,
    /// Dew point: the feed is vapour and an incipient liquid appears.
    #[serde(rename = "dew")]
    Dew,
}

/// Minerals the scale screen covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScaleMineralName {
    #[serde(rename = "calcite")]
    Calcite,
    #[serde(rename = "aragonite")]
    Aragonite,
    #[serde(rename = "siderite")]
    Siderite,
    #[serde(rename = "barite")]
    Barite,
    #[serde(rename = "celestite")]
    Celestite,
    #[serde(rename = "gypsum")]
    Gypsum,
    #[serde(rename = "anhydrite")]
    Anhydrite,
    #[serde(rename = "halite")]
    Halite,
}

/// Slip model for the Sachdeva choke model: how the gas-liquid velocity ratio
/// at the choke inlet is estimated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlipModelName {
    #[serde(rename = "no_slip")]
    NoSlip,
    #[serde(rename = "gromles")]
    Gromles,
    #[serde(rename = "hydro")]
    Hydro,
    #[serde(rename = "constant_slip")]
    ConstantSlip,
    #[serde(rename = "fauske")]
    Fauske,
    #[serde(rename = "moddy")]
    Moddy,
    #[serde(rename = "simpson")]
    Simpson,
    #[serde(rename = "thom")]
    Thom,
    #[serde(rename = "baroczy")]
    Baroczy,
    #[serde(rename = "lockhart_martenelli")]
    LockhartMartenelli,
}

/// Activity model for the Tier-3 wax SLE flash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WaxActivityModelName {
    /// Regular-solution theory with Won solubility parameters. Default.
    #[default]
    #[serde(rename = "regular_solution")]
    RegularSolution,
    /// Unit activity coefficients, which reduces the flash to the Won
    /// ideal solid solution.
    #[serde(rename = "ideal")]
    Ideal,
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
    /// Tier-3 non-ideal solid-liquid equilibrium flash.
    #[serde(rename = "sle")]
    Sle,
}

/// Arguments for [`GraphSolve::adjust_composition_to_gor`](crate::GraphSolve::adjust_composition_to_gor).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdjustCompositionToGorParams {
    /// Feed mole fractions, one per component, summing to 1
    pub mole_fractions: Vec<f64>,
    /// Target surface GOR in Sm³/Sm³ (SI, as everywhere else on this server).
    /// Convert from scf/bbl with `convert_units` first if needed.
    pub target_gor: f64,
    /// Binary interaction coefficient overrides; each replaces the value of its
    /// pair and the other pairs keep the database or correlation value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names: built-in database names in any case (see
    /// list_eos_components) or the short codes C1, C2, C3, iC4, nC4, iC5, nC5, N2,
    /// CO2, H2S, H2O, O2, H2, Ar. An unknown name is an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// Cubic equation of state (default PengRobinson)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
    /// Separator stage pressures in MPa, one per temperature (default: one stage at
    /// 0.101325 MPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_pressures_mpa: Option<Vec<f64>>,
    /// Separator stage temperatures in K for the surface flash (default: one stage
    /// at 288.71 K). Give with separator_pressures_mpa.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_temperatures_k: Option<Vec<f64>>,
}

/// Arguments for [`GraphSolve::adjust_composition_to_phase_ratio`](crate::GraphSolve::adjust_composition_to_phase_ratio).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdjustCompositionToPhaseRatioParams {
    /// Feed mole fractions, one per component, summing to 1
    pub mole_fractions: Vec<f64>,
    /// Pressure in MPa
    pub pressure_mpa: f64,
    /// Target in-situ GOR in m3/m3
    pub target_gor: f64,
    /// Temperature in Kelvin
    pub temperature_k: f64,
    /// Binary interaction coefficient overrides; each replaces the value of its
    /// pair and the other pairs keep the database or correlation value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names: built-in database names in any case (see
    /// list_eos_components) or the short codes C1, C2, C3, iC4, nC4, iC5, nC5, N2,
    /// CO2, H2S, H2O, O2, H2, Ar. An unknown name is an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// Cubic equation of state (default PengRobinson)
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
    /// Same network payload accepted by `solve_network` (object or JSON string).
    /// Every source is tagged; the solved flow must have no loops.
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
    /// Free gas rate in Sm3/day at standard conditions. The oil carries oil_rate ×
    /// dissolved_gas_ratio in solution on top of it, so subtract that from a total
    /// produced gas rate.
    pub gas_rate: f64,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin
    pub inlet_temperature: f64,
    /// Oil rate in Sm3/day
    pub oil_rate: f64,
    /// Water rate in Sm3/day
    pub water_rate: f64,
    /// Choke discharge coefficient Cd, in (0, 1\] (default: 0.68441971). The mass
    /// rate at a given pressure ratio is proportional to Cd.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_coefficient: Option<f64>,
    /// Dissolved gas-oil ratio in Sm3/Sm3 (default: 90)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolved_gas_ratio: Option<f64>,
    /// Gas molecular weight in g/mol (default: 19.83)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_mw: Option<f64>,
    /// Oil density in kg/m3 (default: 850)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_density: Option<f64>,
    /// Exponent m of the pressure recovery downstream of the vena contracta, p_out
    /// = p_in - (p_in - p_vc)(1 - (d/D)^m) with p_vc the vena-contracta pressure;
    /// positive (default: 2.99996817)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perry_multiplier: Option<f64>,
    /// Upstream pipe inner diameter divided by the choke diameter, D/d, greater
    /// than 1 (default: 5.46554744). Sets how much of the pressure drop to the vena
    /// contracta is recovered downstream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe_diameter_ratio: Option<f64>,
    /// Slip model (default: hydro)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slip_model: Option<SlipModelName>,
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
    /// Choke discharge coefficient Cd, in (0, 1\] (default: 0.68441971). The mass
    /// rate at a given pressure ratio is proportional to Cd.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_coefficient: Option<f64>,
    /// Dissolved GOR of oil phase in Sm3/Sm3 (default: 90)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolved_gor: Option<f64>,
    /// Gas molecular weight in g/mol — applies to both free gas and dissolved gas
    /// (default: 20.279)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_mw: Option<f64>,
    /// Stock-tank oil density in kg/m3 (default: 850)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_density: Option<f64>,
    /// Exponent m of the pressure recovery downstream of the vena contracta, p_out
    /// = p_in - (p_in - p_vc)(1 - (d/D)^m) with p_vc the vena-contracta pressure;
    /// positive (default: 2.99996817)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perry_multiplier: Option<f64>,
    /// Upstream pipe inner diameter divided by the choke diameter, D/d, greater
    /// than 1 (default: 5.46554744). Sets how much of the pressure drop to the vena
    /// contracta is recovered downstream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe_diameter_ratio: Option<f64>,
    /// Slip model (default: hydro)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slip_model: Option<SlipModelName>,
    /// Water salinity in ppm (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_salinity: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_compressor`](crate::GraphSolve::calculate_compressor).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateCompressorParams {
    /// Fluid configuration. `gas_rate` sets the mass flow; only `gas_mw` describes
    /// the gas.
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

/// Arguments for [`GraphSolve::calculate_critical_point`](crate::GraphSolve::calculate_critical_point).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateCriticalPointParams {
    /// Mole fractions, one per component, summing to 1
    pub mole_fractions: Vec<f64>,
    /// Binary interaction coefficient overrides; each replaces the value of its
    /// pair and the other pairs keep the database or correlation value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names: built-in database names in any case (see
    /// list_eos_components) or the short codes C1, C2, C3, iC4, nC4, iC5, nC5, N2,
    /// CO2, H2S, H2O, O2, H2, Ar. An unknown name is an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// Cubic equation of state (default PengRobinson)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
}

/// Arguments for [`GraphSolve::calculate_erosional_velocity`](crate::GraphSolve::calculate_erosional_velocity).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateErosionalVelocityParams {
    /// Fluid configuration: phase rates in Sm3/day and the PVT inputs
    pub fluid: serde_json::Value,
    /// Pipe inner diameter in meters
    pub pipe_diameter: f64,
    /// Pressure in MPa
    pub pressure: f64,
    /// Temperature in Kelvin
    pub temperature: f64,
    /// API RP 14E C-factor (default 100), in the standard's customary units
    /// (ft/s)(lb/ft3)^0.5, which is why the SI limit is 1.22 C / sqrt(rho_mix). By
    /// service class: 100 continuous service in carbon steel, 125 intermittent
    /// service, 150 solids-free inhibited or corrosion-resistant-alloy service; 30
    /// to 50 for sand-bearing production.
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
    /// Temperature in Kelvin (valid range: 273.15 to 500, the correlation range)
    pub temperature: f64,
}

/// Arguments for [`GraphSolve::calculate_gas_dew_point`](crate::GraphSolve::calculate_gas_dew_point).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateGasDewPointParams {
    /// Condensate-gas ratio \[STB/MMscf\], the oilfield unit the Ahmadi correlation
    /// is defined in. The earlier name condensate_gas_ratio is still accepted.
    pub condensate_gas_ratio_stb_per_mmscf: f64,
    /// Gas specific gravity (air = 1.0)
    pub gas_gravity: f64,
    /// Oil API gravity
    pub oil_api_gravity: f64,
    /// Temperature in Kelvin
    pub temperature: f64,
}

/// Arguments for [`GraphSolve::calculate_heater_cooler`](crate::GraphSolve::calculate_heater_cooler).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculateHeaterCoolerParams {
    /// Fluid configuration
    pub fluid: serde_json::Value,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin
    pub inlet_temperature: f64,
    /// Operating mode
    pub mode: HeaterCoolerModeName,
    /// Approach to the utility temperature in Kelvin for approach_temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approach_temperature: Option<f64>,
    /// Heat duty in W for fixed_duty (positive heats, negative cools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_duty: Option<f64>,
    /// Duty rating in W (> 0). Caps |duty| in every mode but fixed_duty; the outlet
    /// temperature then follows the capped duty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duty: Option<f64>,
    /// Outlet temperature in Kelvin for fixed_outlet_temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlet_temperature: Option<f64>,
    /// Pressure drop across the heater/cooler in MPa (default: 0), the same unit as
    /// a network heater_cooler edge's `heater_cooler_data.pressure_drop`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure_drop: Option<f64>,
    /// Overall conductance UA in W/K for ua (> 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<f64>,
    /// Utility (cooling water, steam, refrigerant) temperature in Kelvin for
    /// approach_temperature and ua
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_temperature: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_hydrate_temperature`](crate::GraphSolve::calculate_hydrate_temperature).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateHydrateTemperatureParams {
    /// Gas specific gravity (air = 1.0). Accepted from 0.5 to 1.5; the
    /// correlation's typical natural-gas range is about 0.55 to 0.95.
    pub gas_gravity: f64,
    /// Pressure in MPa
    pub pressure: f64,
    /// CO2 mole fraction. Echoed but not applied: the Towler-Mokhatab correlation
    /// has no acid-gas term, so CO2 acts only through gas_gravity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_mole_fraction: Option<f64>,
    /// H2S mole fraction. Echoed but not applied: the Towler-Mokhatab correlation
    /// has no acid-gas term, so H2S acts only through gas_gravity.
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
    /// Pressure drop across the valve in MPa (positive value), the same unit as a
    /// network jt_valve edge's `jt_valve_data.pressure_drop`
    pub pressure_drop: f64,
}

/// Arguments for [`GraphSolve::calculate_mmp`](crate::GraphSolve::calculate_mmp).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateMmpParams {
    /// Experiment configuration as a JSON object (or string): `composition` and
    /// then `input` (CCE, DLE, CVD, separator, swelling, gas depletion), `options`
    /// (the MMP tools) or `cases` (the regression suite). The composition is either
    /// a full composition (components with critical_pressure in MPa,
    /// binary_interactions, separation stages) or `{ "component_names": \[...\],
    /// "eos_model"?: ..., "binary_interactions"?: \[...\] }`, where each entry is a
    /// database name, a short code such as "C1", or a pseudo-component `{ "name",
    /// "molecular_weight", "specific_gravity", "boiling_point" }` (any two of the
    /// three).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::calculate_mpfm_allocation`](crate::GraphSolve::calculate_mpfm_allocation).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateMpfmAllocationParams {
    /// MPFM allocation configuration as a JSON object (or string): the
    /// `composition` (full, or `{ "component_names": \[...\] }`), reservoir
    /// `mole_fractions`, line `pressure_mpa` (MPa, with optional
    /// `pressure_offset_mpa`) and `temperature` (K), the meter's gas volume
    /// fraction `gvf`, in-situ water cut `wc_insitu` (fractions), and its reported
    /// phase densities `rho_gas_reported`, `rho_oil_reported`, `rho_water_reported`
    /// (kg/m3). Mass rates are kg/h and standard rates Sm3/h.
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
    /// The well's inflow model: the same object as the `inflow_model` of
    /// `generate_ipr_curve` (`inflow_model_id`, `ipr_model`, `reservoir_pressure`,
    /// `reservoir_temperature`, `fluid`, ...). Its `fluid` also sets the PVT of the
    /// VLP; the four fluid fields below override it.
    pub ipr: serde_json::Value,
    /// Tubing inner diameter in meters
    pub tubing_diameter: f64,
    /// Tubing length along the hole (measured length, not vertical depth) in
    /// meters, from the wellhead to the inflow node. The vertical depth is
    /// tubing_length·cos(tubing_angle).
    pub tubing_length: f64,
    /// Absolute tubing roughness in meters
    pub tubing_roughness: f64,
    /// Wellhead back-pressure in MPa
    pub wellhead_pressure: f64,
    /// Wellhead temperature in Kelvin; also the surrounding temperature of the
    /// tubing for heat loss.
    pub wellhead_temperature: f64,
    /// Dissolved gas-oil ratio in Sm3/Sm3 for the VLP; overrides
    /// `ipr.fluid.dissolved_gas_ratio`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolved_gas_ratio: Option<f64>,
    /// Multiphase flow correlation for the VLP (default: "KISS").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_correlation: Option<FlowCorrelationName>,
    /// Gas molecular weight in g/mol for the VLP; overrides
    /// `ipr.fluid.reference_gas_mw`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_mw: Option<f64>,
    /// Stock-tank oil density in kg/m3 for the VLP; overrides
    /// `ipr.fluid.reference_oil_density`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_density: Option<f64>,
    /// Tubing inclination from vertical in degrees (default: 0 = vertical)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tubing_angle: Option<f64>,
    /// Water salinity in ppm for the VLP; overrides `ipr.fluid.salinity`.
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
    /// Free gas rate in Sm3/day at standard conditions. The oil carries oil_rate ×
    /// dissolved_gas_ratio in solution on top of it, so subtract that from a total
    /// produced gas rate.
    pub gas_rate: f64,
    /// Pressure in MPa at the `pressure_boundary` end of the pipe. That is the
    /// fluid inlet only when `pressure_boundary` equals `flow_boundary`; otherwise
    /// it is the fluid outlet and the march runs upstream from it.
    pub inlet_pressure: f64,
    /// Temperature in Kelvin of the fluid entering at `flow_boundary`, whichever
    /// end `inlet_pressure` is given at
    pub inlet_temperature: f64,
    /// Pipe length in meters
    pub length: f64,
    /// Oil rate in Sm3/day
    pub oil_rate: f64,
    /// Absolute pipe roughness in meters
    pub roughness: f64,
    /// Water rate in Sm3/day
    pub water_rate: f64,
    /// Dissolved gas-oil ratio in Sm3/Sm3 (default: 90)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolved_gas_ratio: Option<f64>,
    /// End of the pipe where the fluid enters (default: "top")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_boundary: Option<PipeEnd>,
    /// Multiphase flow correlation (default: "Beggs-Brill")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_correlation: Option<FlowCorrelationName>,
    /// Gas molecular weight in g/mol (default: 20.279)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_mw: Option<f64>,
    /// Overall heat transfer coefficient in W/(m².K), referred to the pipe inner
    /// surface (default: 5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_transfer_coefficient: Option<f64>,
    /// Stock-tank oil density in kg/m3 (default: 850)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oil_density: Option<f64>,
    /// End of the pipe where `inlet_pressure` is given (default: "top")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure_boundary: Option<PipeEnd>,
    /// Surrounding/ambient temperature in Kelvin, for heat transfer (default:
    /// `inlet_temperature`, which makes the pipe isothermal)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surrounding_temperature: Option<f64>,
    /// Water salinity in ppm (default: 0)
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

/// Arguments for [`GraphSolve::calculate_pump`](crate::GraphSolve::calculate_pump).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculatePumpParams {
    /// The pumped stream: standard-condition rates and black-oil PVT. It is
    /// evaluated at the inlet of every march step, so free gas compresses and
    /// redissolves along the pump.
    pub fluid: serde_json::Value,
    /// Suction pressure in MPa.
    pub inlet_pressure: f64,
    /// Suction temperature in Kelvin.
    pub inlet_temperature: f64,
    /// Multistage centrifugal pump on stage curves, with an optional ESP drive
    /// train. Replaces `head_curve`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub centrifugal: Option<serde_json::Value>,
    /// Hydraulic efficiency (0-1) of a head-curve pump, and of any section without
    /// a stage-curve efficiency. Needed only then.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efficiency: Option<f64>,
    /// Single head curve as \[flow m3/s in situ, head m\] pairs, flow strictly
    /// increasing. Needed unless `centrifugal` is given, which replaces it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_curve: Option<Vec<Vec<f64>>>,
    /// Mechanical efficiency (0-1, default: 0.95). Shaft power = hydraulic power /
    /// (hydraulic efficiency × mechanical efficiency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanical_efficiency: Option<f64>,
    /// Minor loss coefficient on the nozzle velocity head (default: 0). Applied
    /// only with `nozzle_diameter`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_loss_coefficient: Option<f64>,
    /// Pump nozzle bore in m, the reference area for `minor_loss_coefficient`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nozzle_diameter: Option<f64>,
    /// Net positive suction head required in m, at the first section's reference
    /// speed; it scales with speed squared (default: 0, no check).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npsh_required: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_pump_head`](crate::GraphSolve::calculate_pump_head).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculatePumpHeadParams {
    /// Flow rate in m3/day
    pub flow_rate: f64,
    /// Head curve as array of \[rate_m3_day, head_m\] pairs (flow in m3/day, unlike
    /// the network pump's m3/s)
    pub head_curve: Vec<Vec<f64>>,
    /// Inlet pressure in MPa
    pub inlet_pressure: f64,
    /// Inlet temperature in Kelvin (passed through to the outlet)
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
    /// Choke discharge coefficient Cd, in (0, 1\] (default: 0.68441971). The mass
    /// rate at a given pressure ratio is proportional to Cd.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_coefficient: Option<f64>,
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
    /// Gas molecular weight in g/mol (default: 20.279)
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
    /// Exponent m of the pressure recovery downstream of the vena contracta, p_out
    /// = p_in - (p_in - p_vc)(1 - (d/D)^m) with p_vc the vena-contracta pressure;
    /// positive (default: 2.99996817)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perry_multiplier: Option<f64>,
    /// Upstream pipe inner diameter divided by the choke diameter, D/d, greater
    /// than 1 (default: 5.46554744). Sets how much of the pressure drop to the vena
    /// contracta is recovered downstream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe_diameter_ratio: Option<f64>,
    /// Slip model (default: hydro)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slip_model: Option<SlipModelName>,
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
    /// Liquid mole fractions, one per component, summing to 1
    pub liquid_mole_fractions: Vec<f64>,
    /// Binary interaction coefficient overrides; each replaces the value of its
    /// pair and the other pairs keep the database or correlation value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names: built-in database names in any case (see
    /// list_eos_components) or the short codes C1, C2, C3, iC4, nC4, iC5, nC5, N2,
    /// CO2, H2S, H2O, O2, H2, Ar. An unknown name is an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// Cubic equation of state (default PengRobinson)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
}

/// Arguments for [`GraphSolve::calculate_saturation_pressure`](crate::GraphSolve::calculate_saturation_pressure).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateSaturationPressureParams {
    /// Feed mole fractions, one per component, summing to 1
    pub mole_fractions: Vec<f64>,
    /// Saturation algorithm (default classical)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<SaturationAlgorithmName>,
    /// Binary interaction coefficient overrides; each replaces the value of its
    /// pair and the other pairs keep the database or correlation value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Saturation boundary (default bubble)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boundary: Option<SaturationBoundaryName>,
    /// Component names: built-in database names in any case (see
    /// list_eos_components) or the short codes C1, C2, C3, iC4, nC4, iC5, nC5, N2,
    /// CO2, H2S, H2O, O2, H2, Ar. An unknown name is an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// Cubic equation of state (default PengRobinson)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
    /// Pressure in MPa: solve for the saturation temperature. Give this or
    /// temperature_k.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure_mpa: Option<f64>,
    /// Temperature in Kelvin: solve for the saturation pressure. Give this or
    /// pressure_mpa.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_k: Option<f64>,
}

/// Arguments for [`GraphSolve::calculate_screw_compressor`](crate::GraphSolve::calculate_screw_compressor).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculateScrewCompressorParams {
    /// Discharge pressure in MPa (must exceed inlet_pressure).
    pub discharge_pressure: f64,
    /// Displacement (swept volume) per revolution in m^3, > 0.
    pub displacement_per_rev_m3: f64,
    /// Suction pressure in MPa.
    pub inlet_pressure: f64,
    /// Suction temperature in Kelvin.
    pub inlet_temperature: f64,
    /// Shaft speed in revolutions per second (50 rev/s = 3000 rpm), > 0. A network
    /// `screw_simple_pd` machine takes its `shaft_speed` in rad/s instead (2π ×
    /// rev/s).
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
    /// Fluid configuration. `gas_rate` sets the mass flow; only `gas_mw` describes
    /// the gas.
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
    /// Temperature in Kelvin (valid range: 273.15 to 500, the correlation range)
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
    /// Wax solubility gradient dWs/dT \[1/K\]: the slope of dissolved wax mass
    /// fraction against temperature. screen_wax_risk returns it for model sle.
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
    /// Critical-property correlation (default kesler_lee)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PseudoCriticalMethodName>,
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
    /// Black-oil fluid, the fields of `calculate_fluid_properties.fluid_properties`
    /// without their defaults: `fluid_type`, `reference_oil_density` (kg/m3),
    /// `dissolved_gas_ratio` (Sm3/Sm3), `reference_gas_mw` (g/mol),
    /// `saturation_pressure_correlation` and `oil_viscosity_correlation` are
    /// required; `salinity` (ppm) and the other correlation selectors are optional.
    /// The gas, oil and water phases are all built from it.
    pub fluid: serde_json::Value,
    /// Inlet boundary state at the upstream end of the pipe.
    pub inlet: serde_json::Value,
    /// Pipe geometry segments, the same shape as `pipe_data.pipe_geometry` in a
    /// network model, listed from the outlet end to the inlet end: the fluid enters
    /// at the last listed segment, as it does on a network pipe flowing from its
    /// source to its target. Each segment takes `segment_type` ("pipe" or
    /// "restriction"), `pipe_diameter` (m), `pipe_roughness` (m, absolute),
    /// `surrounding_temperature` (K), `heat_transfer_coefficient` (W/(m²·K)), an
    /// optional `thermal` block that replaces the scalar coefficient with one
    /// computed from wall, insulation and burial layers, an optional
    /// `annulus_inner_diameter` (m, the tubing outer diameter; `pipe_diameter` is
    /// then the casing inner diameter), and its geometry as one of: `pipe_length`
    /// (m) with `pipe_angle` (degrees from vertical in the flow direction: 0
    /// upflow, 90 horizontal, 180 downflow); `measured_depth` and
    /// `true_vertical_depth` (m) survey stations, sorted by measured depth so the
    /// deepest station is the inlet; or `end_x`, `end_y`, `end_z` (m, z up) end
    /// points, each segment running from the previous end point (the reference
    /// point for the first), with its angle taken in that direction. A
    /// `geometry_mode` on any segment picks the family for the whole pipe ("mdtvd",
    /// "xyz", any other value for length and angle); without one, the first
    /// segment's fields decide. Fields of the other families are ignored.
    pub pipe_segments: serde_json::Value,
    /// Optional subset of correlations to compare. Default: every correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlations: Option<Vec<FlowCorrelationName>>,
    /// Measured depth \[m\] the first survey station is measured from, as
    /// `pipe_data.reference_measured_depth` (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_measured_depth: Option<f64>,
    /// True vertical depth \[m\] at `reference_measured_depth`, as
    /// `pipe_data.reference_true_vertical_depth` (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_true_vertical_depth: Option<f64>,
    /// X coordinate \[m\] the first `end_x`/`end_y`/`end_z` segment starts from, as
    /// `pipe_data.reference_x` (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_x: Option<f64>,
    /// Y coordinate \[m\] of that start point (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_y: Option<f64>,
    /// Z coordinate \[m, up\] of that start point (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_z: Option<f64>,
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
    /// Optional subset of correlations. Default: every correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlations: Option<Vec<FlowCorrelationName>>,
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
    /// API RP 14E C-factor for the erosion pass (default 100), in the standard's
    /// customary units (ft/s)(lb/ft3)^0.5. By service class: 100 continuous service
    /// in carbon steel, 125 intermittent service, 150 solids-free inhibited or
    /// corrosion-resistant-alloy service; 30 to 50 for sand-bearing production.
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
    /// Fit request: `{kind?, time, rate, reject_outliers (default true)}`. `kind`
    /// is one of the `generate_decline` models; omit it to fit them all and rank by
    /// R². The fitted `params` use the same names as `generate_decline` (for
    /// `arps`: `initial_rate`, `initial_decline_rate`, `hyperbolic_exponent`).
    pub fit_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::flash_to_surface`](crate::GraphSolve::flash_to_surface).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FlashToSurfaceParams {
    /// Feed mole fractions, one per component, summing to 1
    pub mole_fractions: Vec<f64>,
    /// Binary interaction coefficient overrides; each replaces the value of its
    /// pair and the other pairs keep the database or correlation value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names: built-in database names in any case (see
    /// list_eos_components) or the short codes C1, C2, C3, iC4, nC4, iC5, nC5, N2,
    /// CO2, H2S, H2O, O2, H2, Ar. An unknown name is an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// Cubic equation of state (default PengRobinson)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
    /// Separator stage pressures in MPa, one per temperature (default: one stage at
    /// 0.101325 MPa). End at standard conditions for stock-tank values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_pressures_mpa: Option<Vec<f64>>,
    /// Separator stage temperatures in K, first stage first (default: one stage at
    /// 288.71 K). Give with separator_pressures_mpa.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_temperatures_k: Option<Vec<f64>>,
}

/// Arguments for [`GraphSolve::generate_decline`](crate::GraphSolve::generate_decline).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GenerateDeclineParams {
    /// Decline-generation request: `{kind, params, times | t_max, n_points (default
    /// 200), economic_limit?}`. `params` is keyed by name for each `kind` (q rate,
    /// t time): - `arps`: `initial_rate`, `initial_decline_rate`,
    /// `hyperbolic_exponent`; q = qi/(1 + b·Di·t)^(1/b), exponential at   b = 0 and
    /// harmonic at b = 1 (Arps, 1945). - `modified_hyperbolic`: the `arps` names
    /// plus   `minimum_decline_rate`; hyperbolic until the instantaneous decline
    /// falls to it, exponential after. - `duong`: `intercept` (q1), `slope` (a),
    /// `time_exponent` (m);   q = q1·t^(−m)·exp(a/(1 − m)·(t^(1−m) − 1)) (Duong,
    /// 2011). - `stretched_exponential`: `initial_rate`, `characteristic_time` (τ),
    /// `exponent` (n); q = qi·exp(−(t/τ)^n) (Valkó, 2009). -
    /// `power_law_exponential`: `initial_rate`, `initial_loss_ratio` (Di),
    /// `infinite_loss_ratio` (D∞), `exponent` (n); q = qi·exp(−D∞·t −   (Di −
    /// D∞)/n·t^n) (after Ilk et al., 2008).  Rates and times are in any consistent
    /// units, and decline rates are per unit of that time. Returns time, rate and
    /// cumulative arrays.
    pub decline_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::generate_ipr_curve`](crate::GraphSolve::generate_ipr_curve).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GenerateIprCurveParams {
    /// The inflow model: the same object as a row of a network's `sources` library.
    /// Always `inflow_model_id` (any string) and `ipr_model`. Every model except
    /// `table` also needs `reservoir_pressure` \[MPa\], `reservoir_temperature`
    /// \[K\] and a black-oil `fluid` (`fluid_type`, `reference_oil_density`
    /// \[kg/m³\], `reference_gas_mw` \[g/mol\], `dissolved_gas_ratio` \[Sm³/Sm³\],
    /// `saturation_pressure_correlation`, `oil_viscosity_correlation`, optional
    /// `salinity` \[ppm\]). `watercut` is water/(water+oil) in percent (0-100) on
    /// the oil models; on `forchheimer`, `forchheimer_ab`, `vertical_gas_pss`,
    /// `fractured_vertical_gas_pss` and a transient model with a gas fluid it is
    /// the water-gas ratio \[Sm³/Sm³\]. Those four gas deliverability models read
    /// `fluid.dissolved_gas_ratio` as the condensate-gas ratio \[Sm³/Sm³\]; a
    /// transient model with a gas fluid produces no condensate.  Fields each
    /// `ipr_model` needs (permeability mD, lengths m, areas m²): - `table`:
    /// `ipr_table`, a list of points `{oil_rate, gas_rate,   water_rate
    /// \[kSm³/day\], pressure \[MPa\], temperature \[K\]}`; no fluid. - `pi`:
    /// `reservoir_properties.productivity_index` \[Sm³/day/MPa of   liquid at
    /// standard conditions\], `watercut`. - `darcy`:
    /// `reservoir_properties.permeability`,   `well_properties.{wellbore_radius,
    /// skin}`,   `reservoir_dimensions.{drainage_area, reservoir_thickness_tst,
    /// dietz_shape_factor}`, `watercut`. - `fetkovich`: the `darcy` fields plus
    /// `relative_permeability`, one   Corey entry `{phase, residual_saturation,
    /// permeability_endpoint,   exponent}` for each of oil, water and gas. -
    /// `fetkovich_ab`: `ipr_coefficients.{a \[Sm³/day/MPa\], b   \[Sm³/day/MPa²\]}`
    /// in q = a·(pr − psat) + b·(psat² − pwf²), with the   bubble point psat from
    /// `fluid`. - `forchheimer`: `reservoir_properties.permeability`,
    /// `well_properties.{wellbore_radius, skin, perforation_interval}`,
    /// `reservoir_dimensions.{drainage_area, reservoir_thickness_tst}`,
    /// `relative_permeability` (as for `fetkovich`), `watercut`. -
    /// `forchheimer_ab`: `ipr_coefficients.{a \[MPa²·day/Sm³\], b
    /// \[MPa²·day²/Sm⁶\]}` in pr² − pwf² = a·q + b·q². - `vertical_gas_pss`:
    /// `reservoir_properties.permeability`,   `well_properties.{wellbore_radius,
    /// skin}`,   `reservoir_dimensions.{drainage_area, reservoir_thickness_tst}`;
    /// optional `well_properties.perforation_interval`. -
    /// `fractured_vertical_gas_pss`: the `vertical_gas_pss` fields (skin
    /// optional) plus `fracture_properties.half_length` and either
    /// `fracture_properties.fcd` or `fracture_properties.conductivity`   \[mD·m\].
    /// The transient models all need `transient_timestep` \[hours\] (the time the
    /// IPR is evaluated at), `reservoir_properties.{permeability, porosity,
    /// formation_compressibility \[1/MPa\]}` and
    /// `reservoir_dimensions.reservoir_thickness_tst`; `well_properties.skin`
    /// defaults to 0 except on `horizontal_msf`. Per model: - `radial`:
    /// `well_properties.wellbore_radius`;   `reservoir_dimensions.outer_boundary`
    /// `infinite` (default),   `closed` or `constant_pressure`, the last two with
    /// `reservoir_dimensions.drainage_area`. - `partial_penetration`:
    /// `well_properties.{wellbore_radius,   perforation_interval}` (the perforated
    /// height). - `faulted`: `well_properties.wellbore_radius`,
    /// `reservoir_dimensions.fault_distance`; `fault_config` `single`   (default),
    /// `corner`, `intersecting` (with `fault_angle_deg`) or   `channel` (with
    /// `channel_width`). - `composite_radial`: `well_properties.wellbore_radius`,
    /// `reservoir_properties.{stimulated_permeability, stimulated_radius}`;
    /// optional `stimulated_porosity`, `stimulated_compressibility`. -
    /// `horizontal`: `well_properties.{wellbore_radius,   perforation_interval}`
    /// (the completed lateral length). - `uniform_flux_fracture`,
    /// `infinite_conductivity_fracture`:   `fracture_properties.half_length`. -
    /// `bounded_radial_fractured`: `fracture_properties.half_length`,
    /// `fracture_properties.fcd` (or `conductivity` \[mD·m\] with the
    /// permeability), `reservoir_dimensions.drainage_area`,
    /// `well_properties.wellbore_radius`; optional
    /// `fracture_properties.conductivity_type` `infinite` (default) or   `uniform`,
    /// `reservoir_dimensions.dietz_shape_factor` (default   31.62). -
    /// `bounded_rect_fractured`: `fracture_properties.half_length`,
    /// `reservoir_dimensions.{reservoir_length_x, reservoir_length_y}`;   optional
    /// `fracture_properties.conductivity_type`. - `horizontal_msf`:
    /// `fracture_properties.{half_length, width,   permeability}` and optional
    /// `number_of_fractures` (default 1),
    /// `reservoir_dimensions.reservoir_length_x`,
    /// `well_properties.{perforation_interval (lateral length), skin}` and
    /// `dual_porosity_properties.{dual_porosity_model, storativity_ratio,
    /// dual_porosity_flow_capacity}`, where `dual_porosity_model` is 0
    /// (homogeneous), 1 (pseudo-steady-state interporosity flow) or 2   (transient
    /// interporosity flow); all three fields are required.
    pub inflow_model: serde_json::Value,
    /// Number of (P, Q) points to compute. Each point is a real model evaluation.
    /// Range \[2, 200\], default 30 (the canonical solver resolution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_points: Option<i64>,
}

/// Arguments for [`GraphSolve::generate_phase_envelope`](crate::GraphSolve::generate_phase_envelope).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GeneratePhaseEnvelopeParams {
    /// Feed mole fractions, one per component, summing to 1
    pub mole_fractions: Vec<f64>,
    /// Binary interaction coefficient overrides; each replaces the value of its
    /// pair and the other pairs keep the database or correlation value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names: built-in database names in any case (see
    /// list_eos_components) or the short codes C1, C2, C3, iC4, nC4, iC5, nC5, N2,
    /// CO2, H2S, H2O, O2, H2, Ar. An unknown name is an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// Cubic equation of state (default PengRobinson)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<EosModelName>,
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
    /// The inflow model, as for `generate_ipr_curve`, with a transient `ipr_model`:
    /// `radial`, `partial_penetration`, `faulted`, `composite_radial`,
    /// `horizontal`, `uniform_flux_fracture`, `infinite_conductivity_fracture`,
    /// `bounded_radial_fractured`, `bounded_rect_fractured` or `horizontal_msf`.
    /// The model is built in full, so it needs every input its IPR does,
    /// `transient_timestep` included (see the per-model list on
    /// `generate_ipr_curve`: outer boundary, fault configuration and distances,
    /// fracture conductivity type, fcd, number of fractures, stimulated radius).
    /// The dimensionless groups are derived from those inputs and returned as
    /// `type_curve_parameters`; there is no input block for them.
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
    /// Component to look up: a database name in any case or a short code (e.g.
    /// "methane", "C1", "CO2")
    pub name: String,
}

/// Arguments for [`GraphSolve::import_prp_fluid`](crate::GraphSolve::import_prp_fluid).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ImportPrpFluidParams {
    /// The text of a PVTsim `.prp` fluid file (format versions 16 and 19): the
    /// header, the equation type, the component table (mole percent, Tc degF, Pc
    /// psig, acentric factor, MW, SG, Vc l/kmol, volume shift, Tb degF, parachor,
    /// ...), the binary interaction coefficients and the volume shift flag.
    pub prp_text: String,
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
    /// Component to list binary interaction parameters for: a database name in any
    /// case or a short code (e.g. "methane", "C1", "CO2")
    pub name: String,
}

/// Arguments for [`GraphSolve::match_fetkovich_ab`](crate::GraphSolve::match_fetkovich_ab).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchFetkovichAbParams {
    /// The inflow model (see `generate_ipr_curve` for the fields each `ipr_model`
    /// needs) plus the data the tool consumes: - matching: `test_points`, at least
    /// 2 observed points, each   `{oil_rate, gas_rate, water_rate, pressure,
    /// temperature}` with rates   in kSm³/day at standard conditions, `pressure`
    /// the measured flowing   bottom-hole pressure \[MPa\] and `temperature` \[K\].
    /// All five are   required (0 for an absent phase). The fit reads `oil_rate`
    /// for the   oil models and `gas_rate` for `forchheimer_ab`. The coefficient
    /// being   fitted need not be supplied. - `transient_bhp_history`:
    /// `rate_history`, steps `{time_hours, rate}`   with the rate in kSm³/day of
    /// the fluid's primary phase. - `transient_rate_history`: `pressure_history`,
    /// steps `{time_hours,   pressure \[MPa\]}`. - `rta_diagnostics`:
    /// `rta_history`, samples `{time_hours, rate   \[kSm³/day\], pressure
    /// \[MPa\]}`.  The three history tools need a transient `ipr_model`.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::match_forchheimer_ab`](crate::GraphSolve::match_forchheimer_ab).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchForchheimerAbParams {
    /// The inflow model (see `generate_ipr_curve` for the fields each `ipr_model`
    /// needs) plus the data the tool consumes: - matching: `test_points`, at least
    /// 2 observed points, each   `{oil_rate, gas_rate, water_rate, pressure,
    /// temperature}` with rates   in kSm³/day at standard conditions, `pressure`
    /// the measured flowing   bottom-hole pressure \[MPa\] and `temperature` \[K\].
    /// All five are   required (0 for an absent phase). The fit reads `oil_rate`
    /// for the   oil models and `gas_rate` for `forchheimer_ab`. The coefficient
    /// being   fitted need not be supplied. - `transient_bhp_history`:
    /// `rate_history`, steps `{time_hours, rate}`   with the rate in kSm³/day of
    /// the fluid's primary phase. - `transient_rate_history`: `pressure_history`,
    /// steps `{time_hours,   pressure \[MPa\]}`. - `rta_diagnostics`:
    /// `rta_history`, samples `{time_hours, rate   \[kSm³/day\], pressure
    /// \[MPa\]}`.  The three history tools need a transient `ipr_model`.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::match_parameters`](crate::GraphSolve::match_parameters).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchParametersParams {
    /// The inflow model with observed `test_points`, as for the specific matching
    /// tools.
    pub inflow_model: serde_json::Value,
    /// Which coefficient to fit. When omitted it follows `ipr_model`: `pi` fits the
    /// productivity index, `darcy` and `fetkovich` the skin, `forchheimer_ab` and
    /// `fetkovich_ab` their A and B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_target: Option<MatchTargetName>,
}

/// Arguments for [`GraphSolve::match_productivity_index`](crate::GraphSolve::match_productivity_index).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchProductivityIndexParams {
    /// The inflow model (see `generate_ipr_curve` for the fields each `ipr_model`
    /// needs) plus the data the tool consumes: - matching: `test_points`, at least
    /// 2 observed points, each   `{oil_rate, gas_rate, water_rate, pressure,
    /// temperature}` with rates   in kSm³/day at standard conditions, `pressure`
    /// the measured flowing   bottom-hole pressure \[MPa\] and `temperature` \[K\].
    /// All five are   required (0 for an absent phase). The fit reads `oil_rate`
    /// for the   oil models and `gas_rate` for `forchheimer_ab`. The coefficient
    /// being   fitted need not be supplied. - `transient_bhp_history`:
    /// `rate_history`, steps `{time_hours, rate}`   with the rate in kSm³/day of
    /// the fluid's primary phase. - `transient_rate_history`: `pressure_history`,
    /// steps `{time_hours,   pressure \[MPa\]}`. - `rta_diagnostics`:
    /// `rta_history`, samples `{time_hours, rate   \[kSm³/day\], pressure
    /// \[MPa\]}`.  The three history tools need a transient `ipr_model`.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::match_pvt`](crate::GraphSolve::match_pvt).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchPvtParams {
    /// PVT-match configuration as a JSON object (or string): `{ "config": ...,
    /// "lab": ... }`. `config` is the starting fluid, in FIELD UNITS:
    /// `gas_specific_gravity` (air = 1), `stock_tank_oil_api` (degrees API) and
    /// `gas_to_oil_ratio_scf_stb` (scf/STB, not Sm3/Sm3), all required; the search
    /// ranges `sg_range_fraction` (default 0.10), `api_range_absolute` (degrees
    /// API, default 3.0) and `gor_range_fraction` (default 0.05), `max_iterations`
    /// (default 200) and `tolerance` (default 1e-8) are optional. `lab` is SI:
    /// `measured_pb_mpa` (MPa), `temperature_k` (K) and `pressures_mpa` (MPa) are
    /// required; `measured_rs` (Sm3/Sm3), `measured_bo`, `measured_viscosity`
    /// (Pa.s) and `measured_density` (kg/m3) are optional, one value per pressure.
    pub match_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::match_skin`](crate::GraphSolve::match_skin).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MatchSkinParams {
    /// The inflow model (see `generate_ipr_curve` for the fields each `ipr_model`
    /// needs) plus the data the tool consumes: - matching: `test_points`, at least
    /// 2 observed points, each   `{oil_rate, gas_rate, water_rate, pressure,
    /// temperature}` with rates   in kSm³/day at standard conditions, `pressure`
    /// the measured flowing   bottom-hole pressure \[MPa\] and `temperature` \[K\].
    /// All five are   required (0 for an absent phase). The fit reads `oil_rate`
    /// for the   oil models and `gas_rate` for `forchheimer_ab`. The coefficient
    /// being   fitted need not be supplied. - `transient_bhp_history`:
    /// `rate_history`, steps `{time_hours, rate}`   with the rate in kSm³/day of
    /// the fluid's primary phase. - `transient_rate_history`: `pressure_history`,
    /// steps `{time_hours,   pressure \[MPa\]}`. - `rta_diagnostics`:
    /// `rta_history`, samples `{time_hours, rate   \[kSm³/day\], pressure
    /// \[MPa\]}`.  The three history tools need a transient `ipr_model`.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::optimise_network`](crate::GraphSolve::optimise_network).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OptimiseNetworkParams {
    /// Optimisation request: `network_payload` (a model-format network, as for
    /// `solve_network`), `variables`, `objective`, `constraints` and optional
    /// `settings`. Elements are referenced by id: `{"node_id": "<id>"}` or
    /// `{"edge_id": "<id>"}`.  - `variables`: `{id, variable_type, element,
    /// lower_bound,   upper_bound, initial_value?}` each. The control types and
    /// their   units: `choke_opening` (choke bean diameter, m; edge),
    /// `gas_lift_rate` (kSm³/day; node only, the gas-injection source),
    /// `pipe_diameter` and `pipe_roughness` (m; edge, optional   `segment_index` in
    /// the element), `source_rate` (total kSm³/day at   fixed composition; node),
    /// `source_water_cut` (percent 0-100, the   water-gas ratio on a gas fluid;
    /// node), `source_gor` (Sm³/Sm³, the   condensate-gas ratio on a gas fluid;
    /// node), `boundary_pressure`   (MPa; source or sink node, the reservoir
    /// pressure of an IPR   source), `compressor_ratio` (edge, fixed-ratio
    /// machines),   `compressor_speed` (rad/s; edge), `separator_pressure` (MPa;
    /// node),   `source_temperature` (K; node), `heater_duty` (W; edge),
    /// `pump_speed` (rpm; edge) and `pump_frequency` (Hz; edge, an ESP   drive). -
    /// `objective`: `{terms: \[...\]}`, each term tagged by `type`:
    /// `maximise_phase_rate` and `minimise_phase_rate` `{phase, elements,   weight
    /// (default 1)}`; `maximise_revenue` `{terms: \[{phase, price   (per kSm³),
    /// elements}\]}`; `minimise_pressure_drop` and   `minimise_power_consumption`
    /// `{elements, weight}`;   `minimise_deviation` `{observations: \[{element,
    /// quantity,   measured_value, weight}\]}`. A `phase` is `oil`, `water`, `gas`,
    /// `free_gas`, `liquid` or `total_mass` (the total volumetric rate); a
    /// `quantity` is `{"phase_rate": "<phase>"}` or one of `pressure`,
    /// `temperature`, `pressure_drop`, `velocity`, `power`,   `pump_npsh_margin`,
    /// `pump_flow_range_margin`, `pump_motor_load`,   `pump_intake_gas_fraction`. -
    /// `constraints`: `{id, type, ...}` each, `type` one of
    /// `phase_rate_upper_bound` and `phase_rate_lower_bound` `{phase,   elements,
    /// limit \[kSm³/day\]}`; `pressure_upper_bound` and   `pressure_lower_bound`
    /// \[MPa\], `temperature_upper_bound` and   `temperature_lower_bound` \[K\],
    /// `velocity_upper_bound` \[m/s\],   `power_upper_bound` \[W\], each
    /// `{elements, limit}` on the sum over   the elements;
    /// `pump_npsh_margin_lower_bound` \[m\],   `pump_flow_range_lower_bound`,
    /// `pump_motor_load_upper_bound` and   `pump_intake_gas_fraction_upper_bound`
    /// (fractions), each `{elements,   limit}` held at every listed pump;
    /// `total_resource_limit`   `{variable_ids, limit}` on the sum of those
    /// variables, in their   unit. - `settings`: `max_iter` (default 200), `tol`
    /// (1e-4), `max_cpu_time`   (60 s), `print_level` (5), `gradient_method`
    /// (`sensitivity`, the   default, or `finite_difference`) and
    /// `degeneracy_check` (default   true: probe the free directions at the answer
    /// and report flat ones   in `diagnostics`). `acceptable_tol` is accepted but
    /// not yet used.  The result carries `status`, `objective_value`, each
    /// variable's initial and optimised value, each constraint's value and whether
    /// it holds, `iterations`, and the solved network in `network_output`.
    pub optimisation_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::partition_acid_gas_in_water`](crate::GraphSolve::partition_acid_gas_in_water).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PartitionAcidGasInWaterParams {
    /// Local total pressure in MPa
    pub pressure: f64,
    /// Local temperature in K
    pub temperature: f64,
    /// Produced-water ion analysis in mg/l
    pub water_analysis: serde_json::Value,
    /// CO2 fugacity \[bar\]. Supply this or co2_mole_fraction. The earlier name
    /// co2_fugacity is still accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_fugacity_bar: Option<f64>,
    /// Total CO2 mass rate in the stream \[kg/s\], to report the aqueous fraction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_mass_rate: Option<f64>,
    /// CO2 mole fraction in the gas phase, converted to a fugacity through the
    /// NORSOK fugacity coefficient at the given pressure and temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_mole_fraction: Option<f64>,
    /// H2S fugacity \[bar\]. Supply this or h2s_mole_fraction. The earlier name
    /// h2s_fugacity is still accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2s_fugacity_bar: Option<f64>,
    /// Total H2S mass rate in the stream \[kg/s\], to report the aqueous fraction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2s_mass_rate: Option<f64>,
    /// H2S mole fraction in the gas phase, converted to a partial pressure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2s_mole_fraction: Option<f64>,
    /// Measured in-situ pH. Bypasses the charge-balance solve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ph: Option<f64>,
    /// Produced-water mass rate \[kg/s\], to report the aqueous inventory as kg/day
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_mass_rate: Option<f64>,
}

/// Arguments for [`GraphSolve::rta_diagnostics`](crate::GraphSolve::rta_diagnostics).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RtaDiagnosticsParams {
    /// The inflow model (see `generate_ipr_curve` for the fields each `ipr_model`
    /// needs) plus the data the tool consumes: - matching: `test_points`, at least
    /// 2 observed points, each   `{oil_rate, gas_rate, water_rate, pressure,
    /// temperature}` with rates   in kSm³/day at standard conditions, `pressure`
    /// the measured flowing   bottom-hole pressure \[MPa\] and `temperature` \[K\].
    /// All five are   required (0 for an absent phase). The fit reads `oil_rate`
    /// for the   oil models and `gas_rate` for `forchheimer_ab`. The coefficient
    /// being   fitted need not be supplied. - `transient_bhp_history`:
    /// `rate_history`, steps `{time_hours, rate}`   with the rate in kSm³/day of
    /// the fluid's primary phase. - `transient_rate_history`: `pressure_history`,
    /// steps `{time_hours,   pressure \[MPa\]}`. - `rta_diagnostics`:
    /// `rta_history`, samples `{time_hours, rate   \[kSm³/day\], pressure
    /// \[MPa\]}`.  The three history tools need a transient `ipr_model`.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_cce`](crate::GraphSolve::run_cce).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunCceParams {
    /// Experiment configuration as a JSON object (or string): `composition` and
    /// then `input` (CCE, DLE, CVD, separator, swelling, gas depletion), `options`
    /// (the MMP tools) or `cases` (the regression suite). The composition is either
    /// a full composition (components with critical_pressure in MPa,
    /// binary_interactions, separation stages) or `{ "component_names": \[...\],
    /// "eos_model"?: ..., "binary_interactions"?: \[...\] }`, where each entry is a
    /// database name, a short code such as "C1", or a pseudo-component `{ "name",
    /// "molecular_weight", "specific_gravity", "boiling_point" }` (any two of the
    /// three).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_cvd`](crate::GraphSolve::run_cvd).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunCvdParams {
    /// Experiment configuration as a JSON object (or string): `composition` and
    /// then `input` (CCE, DLE, CVD, separator, swelling, gas depletion), `options`
    /// (the MMP tools) or `cases` (the regression suite). The composition is either
    /// a full composition (components with critical_pressure in MPa,
    /// binary_interactions, separation stages) or `{ "component_names": \[...\],
    /// "eos_model"?: ..., "binary_interactions"?: \[...\] }`, where each entry is a
    /// database name, a short code such as "C1", or a pseudo-component `{ "name",
    /// "molecular_weight", "specific_gravity", "boiling_point" }` (any two of the
    /// three).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_dle`](crate::GraphSolve::run_dle).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunDleParams {
    /// Experiment configuration as a JSON object (or string): `composition` and
    /// then `input` (CCE, DLE, CVD, separator, swelling, gas depletion), `options`
    /// (the MMP tools) or `cases` (the regression suite). The composition is either
    /// a full composition (components with critical_pressure in MPa,
    /// binary_interactions, separation stages) or `{ "component_names": \[...\],
    /// "eos_model"?: ..., "binary_interactions"?: \[...\] }`, where each entry is a
    /// database name, a short code such as "C1", or a pseudo-component `{ "name",
    /// "molecular_weight", "specific_gravity", "boiling_point" }` (any two of the
    /// three).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_eos_flash`](crate::GraphSolve::run_eos_flash).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunEosFlashParams {
    /// Feed mole fractions, one per component, summing to 1
    pub mole_fractions: Vec<f64>,
    /// Pressure in MPa
    pub pressure_mpa: f64,
    /// Binary interaction coefficient overrides; each replaces the value of its
    /// pair and the other pairs keep the database or correlation value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_interactions: Option<Vec<serde_json::Value>>,
    /// Component names: built-in database names in any case (see
    /// list_eos_components) or the short codes C1, C2, C3, iC4, nC4, iC5, nC5, N2,
    /// CO2, H2S, H2O, O2, H2, Ar. An unknown name is an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// Full component definitions (alternative to component_names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<serde_json::Value>>,
    /// Target molar enthalpy in J/mol for flash_type "ph", on the elements-zero
    /// reference state (ideal gas at 298.15 K and 0.1 MPa, formation enthalpies
    /// included). A PT flash with include_properties reports it as
    /// enthalpy_j_per_mol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enthalpy_j_per_mol: Option<f64>,
    /// Target molar entropy in J/(mol K) for flash_type "ps", on the same reference
    /// state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy_j_per_mol_k: Option<f64>,
    /// Equation of state (default PengRobinson). Gerg2008 gives single-phase gas
    /// properties with no phase split.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_model: Option<FlashEosModelName>,
    /// Flash specification: "pt" (default), "ph" or "ps"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_type: Option<FlashTypeName>,
    /// Vapour viscosity model for the property report (default auto)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_viscosity_model: Option<EosViscosityModelName>,
    /// Also report each phase's density, viscosity, enthalpy, entropy, heat
    /// capacities and sound speed (default false; always on for PH, PS and
    /// Gerg2008)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_properties: Option<bool>,
    /// Liquid viscosity model for the property report (default auto)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquid_viscosity_model: Option<EosViscosityModelName>,
    /// Temperature in Kelvin. Required for the PT flash; for PH and PS it is an
    /// optional initial guess.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_k: Option<f64>,
}

/// Arguments for [`GraphSolve::run_forecast`](crate::GraphSolve::run_forecast).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunForecastParams {
    /// Forecast request: `network_payload` (a model-format network, as for
    /// `solve_network`) and `time` are required. Elements are referenced by id:
    /// `{"node_id": "<id>"}` or `{"edge_id": "<id>"}`.  - `time`: `start` (ISO-8601
    /// date or date-time), `unit` (`minutes`,   `hours`, `days` (default), `weeks`,
    /// `months`, `years`),   `timestep_duration` (default 1) and either
    /// `num_timesteps` or   `end_date`. Months and years are calendar lengths and
    /// need a   whole-number `timestep_duration`. - `engine`: omit for a plain
    /// solve each step, or `{type: solve,   solver_settings: {max_iter (default
    /// 10), tolerance (default   1e-6)}}`, or `{type: optimise, variables,
    /// objective, constraints,   settings, iteration_report (default 1)}` with the
    /// `optimise_network` shapes. Each step's solve currently takes its   iteration
    /// limit and tolerance from the network's `solve_parameters`   (defaults 100
    /// and 1e-6), which override `solver_settings`. - `updates`: time-series
    /// drivers `{id, target, trigger, driver,   condition?}`. `target` is `{target:
    /// network, elements: <selector>,   property: <control type as for
    /// optimise_network>}`; the forms   `{target: optimiser_control_bound,
    /// variable_id, bound: lower |   upper | initial}`, `{target:
    /// optimiser_constraint_limit,   constraint_id}` and `{target:
    /// optimiser_objective_term, term_index,   field: {field: weight} | {field:
    /// price, phase}}` are accepted but   not yet applied. `trigger` is `{on:
    /// time}` (x = elapsed time in   `time.unit` at the start of the step), `{on:
    /// cumulative_production,   key}` or `{on: property, source: <element>,
    /// property}`. `driver` is   `{kind: constant, value}`, `{kind: linear,
    /// intercept,   slope_per_unit_time, floor?, cap?}`, `{kind: decline, decline:
    /// exponential | hyperbolic | harmonic, qi, d, b (default 0.5)}` (d per   unit
    /// of x) or `{kind: table, xs, ys, interp: linear | step}`.   `condition` is
    /// `{compare: greater_than | less_than | greater_equal |   less_equal,
    /// threshold}`, tested on x. - `events`: `{timestep | at (ISO-8601), action}`
    /// each, `action` one   of `{action: shutdown, element: <selector>}`, `{action:
    /// startup,   element: <selector>}`, `{action: set_property, target, value}` (a
    /// lasting override, `target` as in updates) and `{action: workover,   element:
    /// <selector>, downtime_steps, improvement_factor}` (shut in   for
    /// `downtime_steps`; the improvement factor is not yet applied). - A
    /// `<selector>` is `{select: element, element: <element>}`,   `{select: group,
    /// group: <name>}` (a key of `groups`, which maps   names to element lists) or
    /// `{select: all_pressure_sources}`. - `transient_wells`: `{node_id,
    /// rate_history: \[{time_hours, rate}\]}`   for each well whose IPR is a
    /// transient model. - `failure_policy`: `{policy: continue_with_last}`
    /// (default: a step   that fails carries the last good rates), `{policy:
    /// abort}`, or   `{policy: retry_substep, splits}` (the retry is not yet
    /// implemented; the failed step's own rates are kept). - `result_detail`:
    /// `summary` (default), `elements` (adds each   element's results per step) or
    /// `full_snapshots` (adds the solved   network per step; very large). -
    /// `stream_results` (default true) only matters to callers that   receive per-
    /// step progress; this tool returns the whole run at once.  Rates and
    /// cumulatives are keyed `node_<element id>_<phase>` (phase oil, gas, water or
    /// total) in kSm³/day and kSm³. A `cumulative_production` trigger key currently
    /// names the node by its 0-based position among the network's node elements
    /// (`node_0_oil`), not by element id.
    pub forecast_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_gas_depletion`](crate::GraphSolve::run_gas_depletion).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunGasDepletionParams {
    /// Experiment configuration as a JSON object (or string): `composition` and
    /// then `input` (CCE, DLE, CVD, separator, swelling, gas depletion), `options`
    /// (the MMP tools) or `cases` (the regression suite). The composition is either
    /// a full composition (components with critical_pressure in MPa,
    /// binary_interactions, separation stages) or `{ "component_names": \[...\],
    /// "eos_model"?: ..., "binary_interactions"?: \[...\] }`, where each entry is a
    /// database name, a short code such as "C1", or a pseudo-component `{ "name",
    /// "molecular_weight", "specific_gravity", "boiling_point" }` (any two of the
    /// three).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_material_balance`](crate::GraphSolve::run_material_balance).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunMaterialBalanceParams {
    /// Reservoir-system definition: `{production_zones, connections,
    /// production_history}`. Volumes are million standard m³ (MMSm³), pressures
    /// MPa.  - `production_zones`, one tank each: `id` (integer), `stoiip`
    /// \[MMSm³\], optional `giip` \[MMSm³\] for a gas cap, `pressure` (initial
    /// reservoir pressure) \[MPa\], `temperature` \[K\], `reference_depth`   \[m\],
    /// `average_tank_porosity` and `average_tank_water_saturation`   (fractions),
    /// and `tank_fluid_data`: exactly two fluids, the   reservoir oil then the
    /// connate water, each the fluid object of   `calculate_fluid_properties`.
    /// Optional `initial_gas_oil_contact`   and `initial_water_hydrocarbon_contact`
    /// \[m\],   `fixed_formation_compressibility` \[1/MPa\] (else estimated from
    /// porosity) and `fixed_water_compressibility` \[1/MPa\]. - `connections`:
    /// `{container1_id, container2_id, flow_coefficient}`   between zones. Accepted
    /// but not yet used: each zone is balanced on   its own. May be empty. -
    /// `production_history`: `{production_zone_id, production_start_date,
    /// data_points}` per zone, each data point `{production_date,   time_days?,
    /// reservoir_pressure? (a measured pressure \[MPa\], reported   against the
    /// balanced one), cumulative_oil_produced,   cumulative_gas_produced,
    /// cumulative_water_produced,   cumulative_water_injected,
    /// cumulative_gas_injected}` with the   cumulatives in MMSm³ and the oil
    /// cumulative above zero.  There is no aquifer: water influx is not part of
    /// this balance (see `calculate_aquifer_influx` for an analytical estimate).
    /// The result is, per zone and date, the balanced reservoir pressure, the PVT
    /// at it (Bo, Bg, Rs, compressibilities), saturations, the Havlena-Odeh terms
    /// F, Eo and Eg, and the recovery factor.
    pub reservoir_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_mmp_probe`](crate::GraphSolve::run_mmp_probe).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunMmpProbeParams {
    /// Experiment configuration as a JSON object (or string): `composition` and
    /// then `input` (CCE, DLE, CVD, separator, swelling, gas depletion), `options`
    /// (the MMP tools) or `cases` (the regression suite). The composition is either
    /// a full composition (components with critical_pressure in MPa,
    /// binary_interactions, separation stages) or `{ "component_names": \[...\],
    /// "eos_model"?: ..., "binary_interactions"?: \[...\] }`, where each entry is a
    /// database name, a short code such as "C1", or a pseudo-component `{ "name",
    /// "molecular_weight", "specific_gravity", "boiling_point" }` (any two of the
    /// three).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_molecule_tracking`](crate::GraphSolve::run_molecule_tracking).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunMoleculeTrackingParams {
    /// Array of fluid rows (array or JSON string) holding the equation-of-state
    /// fluids the sources reference: `{id, model_type: "equation_of_state", model:
    /// {eos_type, eos_parameters: {components, mole_fractions}}}`. Each component
    /// carries its full properties: `molar_mass` \[g/mol\], `critical_temperature`
    /// \[K\], `critical_pressure` \[MPa\], `critical_volume` \[m^3/mol\],
    /// `acentric_factor` and `specific_gravity`.
    pub fluids_json: serde_json::Value,
    /// The reply of `solve_network` (object or JSON string): the elements with
    /// their `results`, and `solve_data`. Every source needs
    /// `source_sink_data.reference_fluid_id` naming an equation-of-state fluid in
    /// `fluids_json`.
    pub solved_network_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_nodal_study`](crate::GraphSolve::run_nodal_study).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunNodalStudyParams {
    /// Nodal-study request: `network_payload` (a model-format network, as for
    /// `solve_network`), `node_id` (the element id of a
    /// `pressure_dependent_source`) and `rate_phase` (`oil`, `gas`, `water`,
    /// `free_gas`, `liquid` or `total_mass`: the rate axis of both curves).
    /// Optional: `ipr_pressure_min` \[MPa\] (default 0.1), `ipr_pressure_max`
    /// \[MPa\] (default the well's reservoir pressure) and `ipr_n_points` (default
    /// 30); `vlp_rate_min` and `vlp_rate_max` \[kSm³/day of rate_phase\] (default 5
    /// % and 110 % of the IPR's open-flow rate) and `vlp_n_points` (default 15,
    /// each a network solve); `vlp_coupling`, either `coupled` (default: the other
    /// wells stay on their IPRs and rebalance at every VLP point) or `isolated`
    /// (the other wells are held at their rates from one base solve, so the curve
    /// is this well's own outflow); `compute_operating_point` (default true);
    /// `sensitivity`, `{variable_type, element, values}` with a control type as for
    /// `optimise_network` and up to 5 values (any further values are ignored), each
    /// adding an IPR and VLP overlay.  Returns `ipr` and `vlp` curves of `{pressure
    /// \[MPa\], rate \[kSm³/day\]}` (each VLP point flagged `converged`), the
    /// `operating_point` where they cross, the open-flow rate `aofp`, the reservoir
    /// pressure used and any sensitivity overlays.
    pub nodal_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_parametric_study`](crate::GraphSolve::run_parametric_study).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunParametricStudyParams {
    /// Parametric-study request: `network_payload` (a model-format network, as for
    /// `solve_network`), `parameters`, `outputs` and `study`. Elements are
    /// referenced by id: `{"node_id": "<id>"}` or `{"edge_id": "<id>"}`.  -
    /// `parameters`: `{id, variable_type, element, base_value, sweep?,
    /// distribution?}` each. `variable_type` is a control type as for
    /// `optimise_network` (for example `choke_opening`,   `boundary_pressure`,
    /// `source_rate`), in that control's unit.   `sweep` is `{min, max, steps
    /// (default 20), scale: linear (default)   or logarithmic}`; without it a sweep
    /// spans base_value ±20 % in 20   steps. `distribution` (Monte Carlo) is
    /// `{dist_type: normal, mean,   std_dev}`, `{dist_type: uniform, min, max}`,
    /// `{dist_type:   log_normal, mean, std_dev}` or `{dist_type: triangular, min,
    /// mode,   max}`. - `outputs`: `{id, quantity, element}` each, with a quantity
    /// as for   `optimise_network`: `{"phase_rate": "oil"}` (kSm³/day), `pressure`,
    /// `temperature`, `pressure_drop`, `velocity`, `power`, ... - `study`, tagged
    /// by `type`: `single_variable_sweep` (each parameter   swept alone, the others
    /// at base); `tornado_chart` with   `variation_percent` (default 10) and
    /// `steps` per side (default 1);   `two_factor_grid` with `param_a`, `param_b`,
    /// `steps_a`, `steps_b`   (default 20 each); `monte_carlo` with `n_samples`,
    /// `sampling`   (`latin_hypercube` default, `random`, `sobol`), `seed`,
    /// `n_bins`   (default by Sturges' rule), `compute_sobol` (Saltelli indices,
    /// n_samples·(parameters + 2) solves) and `correlations` `\[{param_a,
    /// param_b, coefficient}\]` (not with compute_sobol); `envelope_map`   with
    /// `param_a`, `param_b`, `steps_a`, `steps_b` (default 20 each)   and
    /// `constraints` `\[{id, output_id, upper_limit?, lower_limit?}\]`.  The result
    /// carries `samples` (every run's parameter values, outputs and convergence)
    /// and the study's own block: `sweep`, `tornado`, `grid`, `envelope`, or for
    /// Monte Carlo `statistics` (per output mean, std_dev, p10, p50, p90, min, max,
    /// where p10 is the 10th percentile, plus histograms, CDFs and sensitivity
    /// indices).
    pub study_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_process_graph`](crate::GraphSolve::run_process_graph).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunProcessGraphParams {
    /// Process graph as a JSON object (or string): `{ "composition": ..., "graph":
    /// { "n_components", "eos_model", "nodes", "edges" } }`. Node kinds are Source
    /// (z, moles, optional target_gor / oil_rate / gas_rate / composition),
    /// Separator (t_k, p_mpa), Mixer, Splitter (edge fractions) and Sink
    /// (calculations: AverageMW, ReidVaporPressure); there are no heaters or
    /// compressors. Totals are reported for the sinks whose ids are `gas` and
    /// `oil`. A composition is either a full composition or `{ "component_names":
    /// \[...\], "eos_model"? }`.
    pub process_graph_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_pvt_regression_suite`](crate::GraphSolve::run_pvt_regression_suite).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunPvtRegressionSuiteParams {
    /// Experiment configuration as a JSON object (or string): `composition` and
    /// then `input` (CCE, DLE, CVD, separator, swelling, gas depletion), `options`
    /// (the MMP tools) or `cases` (the regression suite). The composition is either
    /// a full composition (components with critical_pressure in MPa,
    /// binary_interactions, separation stages) or `{ "component_names": \[...\],
    /// "eos_model"?: ..., "binary_interactions"?: \[...\] }`, where each entry is a
    /// database name, a short code such as "C1", or a pseudo-component `{ "name",
    /// "molecular_weight", "specific_gravity", "boiling_point" }` (any two of the
    /// three).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_separator_test`](crate::GraphSolve::run_separator_test).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunSeparatorTestParams {
    /// Experiment configuration as a JSON object (or string): `composition` and
    /// then `input` (CCE, DLE, CVD, separator, swelling, gas depletion), `options`
    /// (the MMP tools) or `cases` (the regression suite). The composition is either
    /// a full composition (components with critical_pressure in MPa,
    /// binary_interactions, separation stages) or `{ "component_names": \[...\],
    /// "eos_model"?: ..., "binary_interactions"?: \[...\] }`, where each entry is a
    /// database name, a short code such as "C1", or a pseudo-component `{ "name",
    /// "molecular_weight", "specific_gravity", "boiling_point" }` (any two of the
    /// three).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_swelling_test`](crate::GraphSolve::run_swelling_test).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunSwellingTestParams {
    /// Experiment configuration as a JSON object (or string): `composition` and
    /// then `input` (CCE, DLE, CVD, separator, swelling, gas depletion), `options`
    /// (the MMP tools) or `cases` (the regression suite). The composition is either
    /// a full composition (components with critical_pressure in MPa,
    /// binary_interactions, separation stages) or `{ "component_names": \[...\],
    /// "eos_model"?: ..., "binary_interactions"?: \[...\] }`, where each entry is a
    /// database name, a short code such as "C1", or a pseudo-component `{ "name",
    /// "molecular_weight", "specific_gravity", "boiling_point" }` (any two of the
    /// three).
    pub experiment_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_transient_field`](crate::GraphSolve::run_transient_field).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunTransientFieldParams {
    /// Transient request, as a JSON object or a JSON string. Pressure MPa, rates
    /// kSm3/day at standard conditions, temperature K, lengths m.  Pipe and fluid,
    /// in one of two forms: - Network: `network_payload` (a model-format network:
    /// `format`,   `elements`, `fluids`, `sources`) and `path`: `{"select":
    /// "edges",   "edges": \[{"edge_id": "<id>"}, ...\]}`, the pipe edges of one
    /// route   listed INLET FIRST. They are joined in that order and not checked
    /// for   continuity; each keeps its drawn direction (`source` upstream) and
    /// lays   out its `pipe_geometry` segments from `source` to `target` in the
    /// order given (survey stations by measured depth). Only the geometry   and the
    /// fluid are read from the network:   the fluid named by the first edge's
    /// source node, or the only fluid.   Annulus and zero-length restriction
    /// segments are not represented. - Bare: `segments` listed OUTLET FIRST, and
    /// `fluid`, a network fluid   entry `{id, model_type, model}`. A segment has
    /// `pipe_length` m,   `pipe_diameter` m and `pipe_angle` in degrees from
    /// vertical along the   flow (0 flowing up, 90 horizontal, 180 flowing down),
    /// and optionally   `pipe_roughness` m (default 4.6e-5), `max_cell_length` m
    /// (this   segment only), and `surrounding_temperature` K with
    /// `heat_transfer_coefficient` W/(m2 K) (both, for heat exchange).  Black-oil
    /// and table fluids are read from `reference_oil_density`,
    /// `dissolved_gas_ratio`, `reference_gas_mw` and `salinity` only, through a
    /// fixed black-oil correlation set; `fluid_type`, correlation choices and
    /// `pvt_table` are ignored. An equation-of-state fluid is tabulated over (P, T)
    /// first.  `time`: `total` in `unit` (`seconds`, `minutes`, `hours` or `days`;
    /// default `hours`), and `record_interval_s`, the recording interval in seconds
    /// whatever the unit (default 30).  `boundary`, the state at t = 0:
    /// `outlet_pressure` MPa (for a choke, the pressure downstream of it); inlet
    /// `oil_rate`, `water_rate` and `gas_rate` kSm3/day, where `gas_rate` is the
    /// total gas, dissolved included, and an omitted rate is 0; `inlet_temperature`
    /// K; `water_salinity` ppm of the water entering; and `outlet`: `{"type":
    /// "pressure"}` (default), `{"type": "closed"}` or `{"type": "choke", "area":
    /// <m2>, "cd": <default 0.84>}`. `outlet_pressure`, `inlet_temperature` and a
    /// non-zero rate are required in both forms; nothing is taken from the
    /// network's nodes. The run starts from the steady state of this boundary, and
    /// Graphsolve-Flux needs a `pressure` outlet at t = 0.  `schedule`: `\[{"at":
    /// <time in time.unit>, "set": {<boundary fields>}}\]`. Fields not set carry
    /// forward. Rates, `outlet_pressure`, choke `area` and `water_salinity` ramp
    /// linearly from the previous event (t = 0 counts as one), so a lone event at
    /// 0.5 h is a 30-minute ramp starting at t = 0; a `water_salinity` not given at
    /// t = 0 applies from the previous event instead. `inlet_temperature`, choke
    /// `cd` and the outlet `type` step at the event. For a step in a ramped value,
    /// add an event with an empty `set` at the same `at`, listed first. The outlet
    /// and the inlet rates are the only valve controls: close the outlet for a
    /// valve slam, set the rates to 0 for a shut-in.  `results`: `detail` is
    /// `summary` (default: pressure, gas_fraction and mixture_velocity at the
    /// outlet and inlet cells), `profiles` (the `variables` listed, default those
    /// three, at every `cell_stride`-th cell plus both ends, or at the `cells`
    /// listed by index from 0 at the outlet) or `full` (every variable the scheme
    /// carries, every cell; `variables`, `cell_stride` and `cells` are ignored).
    /// Variables: pressure, temperature, gas_fraction, liquid_holdup, gas_velocity,
    /// liquid_velocity, mixture_velocity, mixture_density, oil_fraction, salinity,
    /// solution_gor, sound_speed. Responses over about two million values are
    /// refused.  `discretisation`: `max_cell_length` m (default 50).  `pvt`, for an
    /// equation-of-state fluid only: the table's `p_min_mpa`, `p_max_mpa`,
    /// `t_min_k` and `t_max_k` (inferred from the geometry and boundary when
    /// omitted; outside them properties are held at the edge) and `pressure_nodes`
    /// (default 240) and `temperature_nodes` (default 24). The table's water is
    /// fresh water; `salinity_ppm` does not change it.  Each scheme holds its cells
    /// in a pressure window and clamps anything outside it: Flux 0.5-15 MPa (and
    /// 250-450 K), Wave 0.01-50 MPa, Field 0.05-100 MPa. Solver options cannot be
    /// set. `solver` is set from the tool.  The result is `stats` (cells, length_m,
    /// simulated_s, steps, ...) and `recording`: `geometry` of the recorded cells,
    /// `time_s`, `n_cells`, and `series` of `{variable, unit, values}` with
    /// `values\[sample * n_cells + cell\]`.
    pub transient_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_transient_flux`](crate::GraphSolve::run_transient_flux).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunTransientFluxParams {
    /// Transient request, as a JSON object or a JSON string. Pressure MPa, rates
    /// kSm3/day at standard conditions, temperature K, lengths m.  Pipe and fluid,
    /// in one of two forms: - Network: `network_payload` (a model-format network:
    /// `format`,   `elements`, `fluids`, `sources`) and `path`: `{"select":
    /// "edges",   "edges": \[{"edge_id": "<id>"}, ...\]}`, the pipe edges of one
    /// route   listed INLET FIRST. They are joined in that order and not checked
    /// for   continuity; each keeps its drawn direction (`source` upstream) and
    /// lays   out its `pipe_geometry` segments from `source` to `target` in the
    /// order given (survey stations by measured depth). Only the geometry   and the
    /// fluid are read from the network:   the fluid named by the first edge's
    /// source node, or the only fluid.   Annulus and zero-length restriction
    /// segments are not represented. - Bare: `segments` listed OUTLET FIRST, and
    /// `fluid`, a network fluid   entry `{id, model_type, model}`. A segment has
    /// `pipe_length` m,   `pipe_diameter` m and `pipe_angle` in degrees from
    /// vertical along the   flow (0 flowing up, 90 horizontal, 180 flowing down),
    /// and optionally   `pipe_roughness` m (default 4.6e-5), `max_cell_length` m
    /// (this   segment only), and `surrounding_temperature` K with
    /// `heat_transfer_coefficient` W/(m2 K) (both, for heat exchange).  Black-oil
    /// and table fluids are read from `reference_oil_density`,
    /// `dissolved_gas_ratio`, `reference_gas_mw` and `salinity` only, through a
    /// fixed black-oil correlation set; `fluid_type`, correlation choices and
    /// `pvt_table` are ignored. An equation-of-state fluid is tabulated over (P, T)
    /// first.  `time`: `total` in `unit` (`seconds`, `minutes`, `hours` or `days`;
    /// default `hours`), and `record_interval_s`, the recording interval in seconds
    /// whatever the unit (default 30).  `boundary`, the state at t = 0:
    /// `outlet_pressure` MPa (for a choke, the pressure downstream of it); inlet
    /// `oil_rate`, `water_rate` and `gas_rate` kSm3/day, where `gas_rate` is the
    /// total gas, dissolved included, and an omitted rate is 0; `inlet_temperature`
    /// K; `water_salinity` ppm of the water entering; and `outlet`: `{"type":
    /// "pressure"}` (default), `{"type": "closed"}` or `{"type": "choke", "area":
    /// <m2>, "cd": <default 0.84>}`. `outlet_pressure`, `inlet_temperature` and a
    /// non-zero rate are required in both forms; nothing is taken from the
    /// network's nodes. The run starts from the steady state of this boundary, and
    /// Graphsolve-Flux needs a `pressure` outlet at t = 0.  `schedule`: `\[{"at":
    /// <time in time.unit>, "set": {<boundary fields>}}\]`. Fields not set carry
    /// forward. Rates, `outlet_pressure`, choke `area` and `water_salinity` ramp
    /// linearly from the previous event (t = 0 counts as one), so a lone event at
    /// 0.5 h is a 30-minute ramp starting at t = 0; a `water_salinity` not given at
    /// t = 0 applies from the previous event instead. `inlet_temperature`, choke
    /// `cd` and the outlet `type` step at the event. For a step in a ramped value,
    /// add an event with an empty `set` at the same `at`, listed first. The outlet
    /// and the inlet rates are the only valve controls: close the outlet for a
    /// valve slam, set the rates to 0 for a shut-in.  `results`: `detail` is
    /// `summary` (default: pressure, gas_fraction and mixture_velocity at the
    /// outlet and inlet cells), `profiles` (the `variables` listed, default those
    /// three, at every `cell_stride`-th cell plus both ends, or at the `cells`
    /// listed by index from 0 at the outlet) or `full` (every variable the scheme
    /// carries, every cell; `variables`, `cell_stride` and `cells` are ignored).
    /// Variables: pressure, temperature, gas_fraction, liquid_holdup, gas_velocity,
    /// liquid_velocity, mixture_velocity, mixture_density, oil_fraction, salinity,
    /// solution_gor, sound_speed. Responses over about two million values are
    /// refused.  `discretisation`: `max_cell_length` m (default 50).  `pvt`, for an
    /// equation-of-state fluid only: the table's `p_min_mpa`, `p_max_mpa`,
    /// `t_min_k` and `t_max_k` (inferred from the geometry and boundary when
    /// omitted; outside them properties are held at the edge) and `pressure_nodes`
    /// (default 240) and `temperature_nodes` (default 24). The table's water is
    /// fresh water; `salinity_ppm` does not change it.  Each scheme holds its cells
    /// in a pressure window and clamps anything outside it: Flux 0.5-15 MPa (and
    /// 250-450 K), Wave 0.01-50 MPa, Field 0.05-100 MPa. Solver options cannot be
    /// set. `solver` is set from the tool.  The result is `stats` (cells, length_m,
    /// simulated_s, steps, ...) and `recording`: `geometry` of the recorded cells,
    /// `time_s`, `n_cells`, and `series` of `{variable, unit, values}` with
    /// `values\[sample * n_cells + cell\]`.
    pub transient_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::run_transient_wave`](crate::GraphSolve::run_transient_wave).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RunTransientWaveParams {
    /// Transient request, as a JSON object or a JSON string. Pressure MPa, rates
    /// kSm3/day at standard conditions, temperature K, lengths m.  Pipe and fluid,
    /// in one of two forms: - Network: `network_payload` (a model-format network:
    /// `format`,   `elements`, `fluids`, `sources`) and `path`: `{"select":
    /// "edges",   "edges": \[{"edge_id": "<id>"}, ...\]}`, the pipe edges of one
    /// route   listed INLET FIRST. They are joined in that order and not checked
    /// for   continuity; each keeps its drawn direction (`source` upstream) and
    /// lays   out its `pipe_geometry` segments from `source` to `target` in the
    /// order given (survey stations by measured depth). Only the geometry   and the
    /// fluid are read from the network:   the fluid named by the first edge's
    /// source node, or the only fluid.   Annulus and zero-length restriction
    /// segments are not represented. - Bare: `segments` listed OUTLET FIRST, and
    /// `fluid`, a network fluid   entry `{id, model_type, model}`. A segment has
    /// `pipe_length` m,   `pipe_diameter` m and `pipe_angle` in degrees from
    /// vertical along the   flow (0 flowing up, 90 horizontal, 180 flowing down),
    /// and optionally   `pipe_roughness` m (default 4.6e-5), `max_cell_length` m
    /// (this   segment only), and `surrounding_temperature` K with
    /// `heat_transfer_coefficient` W/(m2 K) (both, for heat exchange).  Black-oil
    /// and table fluids are read from `reference_oil_density`,
    /// `dissolved_gas_ratio`, `reference_gas_mw` and `salinity` only, through a
    /// fixed black-oil correlation set; `fluid_type`, correlation choices and
    /// `pvt_table` are ignored. An equation-of-state fluid is tabulated over (P, T)
    /// first.  `time`: `total` in `unit` (`seconds`, `minutes`, `hours` or `days`;
    /// default `hours`), and `record_interval_s`, the recording interval in seconds
    /// whatever the unit (default 30).  `boundary`, the state at t = 0:
    /// `outlet_pressure` MPa (for a choke, the pressure downstream of it); inlet
    /// `oil_rate`, `water_rate` and `gas_rate` kSm3/day, where `gas_rate` is the
    /// total gas, dissolved included, and an omitted rate is 0; `inlet_temperature`
    /// K; `water_salinity` ppm of the water entering; and `outlet`: `{"type":
    /// "pressure"}` (default), `{"type": "closed"}` or `{"type": "choke", "area":
    /// <m2>, "cd": <default 0.84>}`. `outlet_pressure`, `inlet_temperature` and a
    /// non-zero rate are required in both forms; nothing is taken from the
    /// network's nodes. The run starts from the steady state of this boundary, and
    /// Graphsolve-Flux needs a `pressure` outlet at t = 0.  `schedule`: `\[{"at":
    /// <time in time.unit>, "set": {<boundary fields>}}\]`. Fields not set carry
    /// forward. Rates, `outlet_pressure`, choke `area` and `water_salinity` ramp
    /// linearly from the previous event (t = 0 counts as one), so a lone event at
    /// 0.5 h is a 30-minute ramp starting at t = 0; a `water_salinity` not given at
    /// t = 0 applies from the previous event instead. `inlet_temperature`, choke
    /// `cd` and the outlet `type` step at the event. For a step in a ramped value,
    /// add an event with an empty `set` at the same `at`, listed first. The outlet
    /// and the inlet rates are the only valve controls: close the outlet for a
    /// valve slam, set the rates to 0 for a shut-in.  `results`: `detail` is
    /// `summary` (default: pressure, gas_fraction and mixture_velocity at the
    /// outlet and inlet cells), `profiles` (the `variables` listed, default those
    /// three, at every `cell_stride`-th cell plus both ends, or at the `cells`
    /// listed by index from 0 at the outlet) or `full` (every variable the scheme
    /// carries, every cell; `variables`, `cell_stride` and `cells` are ignored).
    /// Variables: pressure, temperature, gas_fraction, liquid_holdup, gas_velocity,
    /// liquid_velocity, mixture_velocity, mixture_density, oil_fraction, salinity,
    /// solution_gor, sound_speed. Responses over about two million values are
    /// refused.  `discretisation`: `max_cell_length` m (default 50).  `pvt`, for an
    /// equation-of-state fluid only: the table's `p_min_mpa`, `p_max_mpa`,
    /// `t_min_k` and `t_max_k` (inferred from the geometry and boundary when
    /// omitted; outside them properties are held at the edge) and `pressure_nodes`
    /// (default 240) and `temperature_nodes` (default 24). The table's water is
    /// fresh water; `salinity_ppm` does not change it.  Each scheme holds its cells
    /// in a pressure window and clamps anything outside it: Flux 0.5-15 MPa (and
    /// 250-450 K), Wave 0.01-50 MPa, Field 0.05-100 MPa. Solver options cannot be
    /// set. `solver` is set from the tool.  The result is `stats` (cells, length_m,
    /// simulated_s, steps, ...) and `recording`: `geometry` of the recorded cells,
    /// `time_s`, `n_cells`, and `series` of `{variable, unit, values}` with
    /// `values\[sample * n_cells + cell\]`.
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

/// Arguments for [`GraphSolve::screen_scale_risk`](crate::GraphSolve::screen_scale_risk).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ScreenScaleRiskParams {
    /// Local total pressure in MPa
    pub pressure: f64,
    /// Local temperature in K
    pub temperature: f64,
    /// Produced-water ion analysis in mg/l
    pub water_analysis: serde_json::Value,
    /// CO2 fugacity \[bar\]. Supply this or co2_mole_fraction. The earlier name
    /// co2_fugacity is still accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_fugacity_bar: Option<f64>,
    /// CO2 mole fraction in the gas phase, converted to a fugacity through the
    /// NORSOK fugacity coefficient at the given pressure and temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_mole_fraction: Option<f64>,
    /// H2S fugacity \[bar\]. Supply this or h2s_mole_fraction. The earlier name
    /// h2s_fugacity is still accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2s_fugacity_bar: Option<f64>,
    /// H2S mole fraction in the gas phase, converted to a partial pressure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2s_mole_fraction: Option<f64>,
    /// Restrict the screen to these minerals. Omit for all eight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minerals: Option<Vec<ScaleMineralName>>,
    /// Measured in-situ pH. Bypasses the charge-balance solve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ph: Option<f64>,
    /// Produced-water mass rate \[kg/s\], to report scale as a mass rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_mass_rate: Option<f64>,
}

/// Arguments for [`GraphSolve::screen_wax_risk`](crate::GraphSolve::screen_wax_risk).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ScreenWaxRiskParams {
    /// Local operating temperature in K
    pub temperature: f64,
    /// Activity model - sle model (default regular_solution)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_model: Option<WaxActivityModelName>,
    /// Component names, aligned with mole_fractions - won and sle models. When
    /// wax_former_mask is omitted, a name that resolves to a normal paraffin
    /// (n-hexadecane, n-C24, ...) is a wax former if it is n-C16 or heavier; any
    /// other name falls back to the molecular-weight threshold. The sle result also
    /// names its dominant wax former from this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_names: Option<Vec<String>>,
    /// C7+ density \[kg/m3\] - screening model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density_c7_plus: Option<f64>,
    /// Measured enthalpies of fusion \[J/mol\] per component - won and sle models.
    /// Nulls fall back to the Won correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enthalpies_of_fusion: Option<Vec<f64>>,
    /// Solid-solid transition enthalpies \[J/mol\] per component - sle model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enthalpies_of_transition: Option<Vec<f64>>,
    /// Apply the heat-capacity-of-fusion correction - sle model (default true).
    /// Uses heat_capacity_of_fusion_j_per_mol_k where given, else the Pedersen
    /// correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_capacity_correction: Option<bool>,
    /// Heat capacity of fusion, Cp(liquid) - Cp(solid) \[J/(mol K)\], per
    /// component - sle model, applied when heat_capacity_correction is on. Nulls fall back to
    /// the Pedersen correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_capacity_of_fusion_j_per_mol_k: Option<Vec<f64>>,
    /// Liquid molar volumes \[cm3/mol\] per component - sle model activity
    /// coefficients. Nulls fall back to the Won paraffin-density correlation, or a
    /// light-hydrocarbon fit below 70 g/mol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquid_molar_volume_cm3_per_mol: Option<Vec<f64>>,
    /// Liquid solubility parameters \[MPa^0.5\] per component - sle model activity
    /// coefficients. Nulls fall back to a correlation in molar mass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquid_solubility_parameter_mpa_half: Option<Vec<f64>>,
    /// Measured melting points \[K\] per component - won and sle models. Nulls fall
    /// back to the Won correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub melting_points: Option<Vec<f64>>,
    /// Wax model (default screening)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<WaxModelName>,
    /// Component mole fractions - won and sle models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mole_fractions: Option<Vec<f64>>,
    /// Component molecular weights \[g/mol\] - won and sle models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_weights: Option<Vec<f64>>,
    /// C7+ molecular weight \[g/mol\] - screening model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mw_c7_plus: Option<f64>,
    /// Paraffin mass fraction of the C7+ cut - screening model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paraffin_mass_fraction: Option<f64>,
    /// Solid-to-liquid molar volume ratio - sle model (default 1: the solid and
    /// liquid molar volumes are taken as equal). Must be positive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_molar_volume_ratio: Option<f64>,
    /// Solid-solid (rotator-phase) transition temperatures \[K\] per component -
    /// sle model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_temperatures: Option<Vec<f64>>,
    /// Watson K, used to estimate the paraffin fraction when it is not supplied -
    /// screening model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watson_k: Option<f64>,
    /// Which components are wax formers - won and sle models. Omit to identify them
    /// from component_names, or without names from a molecular-weight threshold of
    /// about n-C16 (226.45 g/mol).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wax_former_mask: Option<Vec<bool>>,
}

/// Arguments for [`GraphSolve::solve_network`](crate::GraphSolve::solve_network).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SolveNetworkParams {
    /// Complete network definition in the model format. Accepts the object
    /// directly, or the same payload as a JSON string.  The elements of a saved
    /// model as they are stored: `elements\[\]` of `{group, data}` with string ids
    /// (or `tabs\[\]` of `{id, elements}` with a `tab_id` naming the tab to solve),
    /// `node_type` by name (`fixed_rate_source`, `fixed_pressure_source`,
    /// `pressure_dependent_source`, `network_node`, `fixed_pressure_sink`,
    /// `fixed_rate_sink`), `edge_type` by name (`no_pressure_loss`, `pipe`,
    /// `choke`, `heat_exchanger`, `compressor`, `pump`, `turbine`, `jt_valve`,
    /// `heater_cooler`, `reactor`), edges joined by `source`/`target` element ids,
    /// with the `fluids`, `sources` and `process_equipment` libraries at the top
    /// level and optional `solve_parameters` and `chemical_treatment` blocks. A
    /// model exported from the app is accepted as it is. The reply carries
    /// `results` on each element.
    pub network_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::solve_network_map`](crate::GraphSolve::solve_network_map).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SolveNetworkMapParams {
    /// Network payload accepted by `solve_network` (object or JSON string).
    /// Measurements are `{value, variance}`, and a tight (small) variance means a
    /// trusted instrument. A pressure gauge is `fixed_pressure` on a `network_node`
    /// \[MPa, MPa^2\]; it is read on no other node type, so the boundary pressures
    /// of sources and sinks stay hard. A rate meter is `measured_rate` on an edge
    /// (`total_mass` flow model, all phases summed) or `measured_phase_rates` with
    /// any of `oil`, `water`, `gas`, `injection_gas` (`multiphase` flow model, set
    /// by `solve_parameters.flow_model`), in kSm^3/day at standard conditions with
    /// variance \[(kSm^3/day)^2\]. Source inputs given a variance in
    /// `source_sink_data.beliefs` (`multiphase` only) are estimated from the
    /// measurements. Include a `map_config` inside `solve_parameters` to tune
    /// `physics_variance` and the lambda schedule; if omitted, MAP mode is still
    /// enabled with engine defaults.
    pub network_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::split_plus_fraction`](crate::GraphSolve::split_plus_fraction).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SplitPlusFractionParams {
    /// Split configuration as a JSON object (or string): the `composition` (full,
    /// or `{ "component_names": \[...\] }` where a plus fraction is `{ "name",
    /// "molecular_weight", "specific_gravity" }`), `overall_mole_fractions`,
    /// `heavy_index`, `number_of_pseudo_components`, and optional gamma-
    /// distribution parameters (`alpha`, `eta`, `highest_mw`).
    pub split_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::transient_bhp_history`](crate::GraphSolve::transient_bhp_history).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransientBhpHistoryParams {
    /// The inflow model (see `generate_ipr_curve` for the fields each `ipr_model`
    /// needs) plus the data the tool consumes: - matching: `test_points`, at least
    /// 2 observed points, each   `{oil_rate, gas_rate, water_rate, pressure,
    /// temperature}` with rates   in kSm³/day at standard conditions, `pressure`
    /// the measured flowing   bottom-hole pressure \[MPa\] and `temperature` \[K\].
    /// All five are   required (0 for an absent phase). The fit reads `oil_rate`
    /// for the   oil models and `gas_rate` for `forchheimer_ab`. The coefficient
    /// being   fitted need not be supplied. - `transient_bhp_history`:
    /// `rate_history`, steps `{time_hours, rate}`   with the rate in kSm³/day of
    /// the fluid's primary phase. - `transient_rate_history`: `pressure_history`,
    /// steps `{time_hours,   pressure \[MPa\]}`. - `rta_diagnostics`:
    /// `rta_history`, samples `{time_hours, rate   \[kSm³/day\], pressure
    /// \[MPa\]}`.  The three history tools need a transient `ipr_model`.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::transient_rate_history`](crate::GraphSolve::transient_rate_history).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransientRateHistoryParams {
    /// The inflow model (see `generate_ipr_curve` for the fields each `ipr_model`
    /// needs) plus the data the tool consumes: - matching: `test_points`, at least
    /// 2 observed points, each   `{oil_rate, gas_rate, water_rate, pressure,
    /// temperature}` with rates   in kSm³/day at standard conditions, `pressure`
    /// the measured flowing   bottom-hole pressure \[MPa\] and `temperature` \[K\].
    /// All five are   required (0 for an absent phase). The fit reads `oil_rate`
    /// for the   oil models and `gas_rate` for `forchheimer_ab`. The coefficient
    /// being   fitted need not be supplied. - `transient_bhp_history`:
    /// `rate_history`, steps `{time_hours, rate}`   with the rate in kSm³/day of
    /// the fluid's primary phase. - `transient_rate_history`: `pressure_history`,
    /// steps `{time_hours,   pressure \[MPa\]}`. - `rta_diagnostics`:
    /// `rta_history`, samples `{time_hours, rate   \[kSm³/day\], pressure
    /// \[MPa\]}`.  The three history tools need a transient `ipr_model`.
    pub inflow_model: serde_json::Value,
}

/// Arguments for [`GraphSolve::tune_mpfm_allocation`](crate::GraphSolve::tune_mpfm_allocation).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TuneMpfmAllocationParams {
    /// MPFM tuning configuration as a JSON object (or string): one or more
    /// observation rows (measured vs reference) plus the parameters and bounds to
    /// fit. The inverse of `calculate_mpfm_allocation`.
    pub tuning_json: serde_json::Value,
}

/// Arguments for [`GraphSolve::validate_solver_payload`](crate::GraphSolve::validate_solver_payload).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ValidateSolverPayloadParams {
    /// Same network payload accepted by `solve_network` (object or JSON string).
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

    /// Tune a composition to a target in-situ gas-oil volume ratio (m3/m3) at
    /// given P/T.
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

    /// Source-tagged production allocation (back-allocation): solve the network
    /// and attribute the mass on every edge back to the sources it came from.
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

    /// True critical point of a mixture (Heidemann-Khalil) with a cubic EOS:
    /// critical temperature, pressure, molar volume and Z-factor.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_critical_point(
        &self,
        params: CalculateCriticalPointParams,
    ) -> Result<ToolResponse> {
        self.call("calculate_critical_point", &params).await
    }

    /// API RP 14E erosional velocity limit (1.22 C / sqrt(rho_mix) in SI), the
    /// actual mixture velocity for a pipe, and the flow at which the pipe
    /// reaches the limit.
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

    /// Heater/cooler on the network edge's energy balance, in fixed_duty,
    /// fixed_outlet_temperature, approach_temperature or ua mode; returns
    /// outlet P/T and duty.
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

    /// Hydrate formation temperature at a given pressure and gas gravity from
    /// the Towler-Mokhatab screening correlation; H2S and CO2 inputs are
    /// echoed, not applied.
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

    /// Multiphase-flow-meter allocation: convert one meter reading (in-situ) to
    /// standard-condition rates with an EOS.
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

    /// Operating point = IPR intersect VLP for one well and one tubing run.
    /// Generates both curves and finds the stabilised rate and flowing BHP.
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

    /// March pressure and temperature along a single straight pipe (one
    /// diameter, length and angle) with heat transfer.
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

    /// Multiphase pressure gradient at one point in a pipe using a chosen
    /// correlation. Returns gradient components, holdup, flow regime and
    /// hydraulics.
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

    /// Centrifugal pump or ESP at one suction state on the network pump model:
    /// discharge P/T, head, power, NPSH, per-section operating range and the
    /// ESP drive train.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn calculate_pump(&self, params: CalculatePumpParams) -> Result<ToolResponse> {
        self.call("calculate_pump", &params).await
    }

    /// Single head-curve lookup: discharge pressure from one head-vs-rate curve
    /// read at one flow with one liquid density. No stages, speed, gas or power
    /// model; see calculate_pump.
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

    /// Bubble- or dew-point pressure of a composition at a temperature, or
    /// bubble- or dew-point temperature at a pressure (cubic EOS).
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
    /// efficiencies; mass flow = volumetric efficiency x suction density x
    /// displacement per revolution x shaft speed in rev/s.
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

    /// Brine properties (density, viscosity, compressibility, heat capacity,
    /// enthalpy) with a salinity correction.
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

    /// Critical properties (Tc, Pc, Vc), acentric factor and Watson K of a
    /// pseudo-component from any two of MW / specific gravity / boiling point.
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

    /// Flash a composition through a separator train to stock-tank: GOR, oil
    /// density, specific gravity and API gravity, gas gravity.
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
    /// per point. Required inputs depend on ipr_model (19 models).
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
    /// composition (Peng-Robinson or SRK).
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
    /// for a transient ipr_model, with the dimensionless groups derived from
    /// the geometry.
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

    /// Critical properties (Tc, Pc in MPa, omega, MW) of a single EOS component
    /// by database name or short code.
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

    /// Import a PVTsim .prp fluid file (text) as a composition for the PVT,
    /// process-graph and MPFM tools.
    ///
    /// Costs 1 credit.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn import_prp_fluid(&self, params: ImportPrpFluidParams) -> Result<ToolResponse> {
        self.call("import_prp_fluid", &params).await
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

    /// List the network edge types by name (no_pressure_loss, pipe, choke,
    /// compressor, ...) and the fields of their data blocks.
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

    /// List the component names in the built-in equation-of-state database and
    /// the short codes (C1, CO2, iC4, ...) the compositional tools also accept.
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

    /// The model vocabularies the flow-assurance tools accept — hydrate, wax,
    /// asphaltene, corrosion and scale models with their tier, plus how wax
    /// formers are identified, inhibitor names, vdWP guests, the scale risk
    /// bands, the NACE MR0175 regions and the API RP 14E service classes.
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

    /// List the black-oil fluid types (oil, gas, water): configuration fields,
    /// correlation names and defaults, and the reported property keys with
    /// units.
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

    /// List the network node types by name (fixed_rate_source,
    /// fixed_pressure_source, pressure_dependent_source, network_node,
    /// fixed_pressure_sink, fixed_rate_sink) and their fields.
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

    /// Fit A and B of the fetkovich_ab oil deliverability equation q = A·(pr −
    /// psat) + B·(psat² − pwf²) to test points.
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

    /// Fit A and B of the forchheimer_ab gas deliverability equation pr² − pwf²
    /// = A·q + B·q² to test points.
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

    /// Fit the pi model's productivity index to observed (oil rate, flowing-
    /// BHP) test points.
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

    /// Fit the mechanical skin of a darcy or fetkovich model to observed (oil
    /// rate, flowing-BHP) test points.
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
    /// minimise an objective (phase rate, revenue, pressure drop, power,
    /// deviation from measurements) under rate / pressure / temperature /
    /// velocity / power / pump / resource constraints. The optimisation
    /// counterpart of solve_network.
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

    /// How much CO2 and H2S is dissolved in the produced water, and the in-situ
    /// pH that leaves — the brine pH the corrosion models should consume
    /// instead of a condensed-water estimate.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn partition_acid_gas_in_water(
        &self,
        params: PartitionAcidGasInWaterParams,
    ) -> Result<ToolResponse> {
        self.call("partition_acid_gas_in_water", &params).await
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

    /// EOS flash of a composition (Peng-Robinson default, SRK, or GERG-2008 for
    /// single-phase gas): PT, PH or PS; phase split, K-values, phase densities,
    /// and optionally each phase's viscosity, enthalpy, entropy and heat
    /// capacities. Pressure in MPa.
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

    /// Gas-reservoir depletion study: p/z, Bg, recovery factor and retrograde
    /// liquid per pressure step.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_gas_depletion(&self, params: RunGasDepletionParams) -> Result<ToolResponse> {
        self.call("run_gas_depletion", &params).await
    }

    /// Tank material balance over one or more reservoir zones (STOIIP/GIIP,
    /// cumulative production, pressure decline, recovery factor); no aquifer.
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

    /// Mixing-cell miscibility test at one pressure: forward and backward
    /// contact series and whether either reached miscibility.
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_mmp_probe(&self, params: RunMmpProbeParams) -> Result<ToolResponse> {
        self.call("run_mmp_probe", &params).await
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

    /// Nodal analysis of one well inside a network: IPR sweep, VLP by full
    /// network solves, their operating point, and optional sensitivity
    /// overlays.
    ///
    /// Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_nodal_study(&self, params: RunNodalStudyParams) -> Result<ToolResponse> {
        self.call("run_nodal_study", &params).await
    }

    /// Sensitivity / parametric study over a network (single-variable sweep,
    /// tornado, two-factor grid, Monte Carlo, envelope map); returns every run
    /// plus the study's summary.
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

    /// Steady-state compositional flowsheet of Source, Separator, Mixer,
    /// Splitter and Sink nodes (no heaters or compressors), with recycle loops.
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

    /// Run several PVT experiments on one EOS fluid against lab data: per-field
    /// residuals and a weighted objective (evaluation, not tuning).
    ///
    /// Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn run_pvt_regression_suite(
        &self,
        params: RunPvtRegressionSuiteParams,
    ) -> Result<ToolResponse> {
        self.call("run_pvt_regression_suite", &params).await
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
    /// Flux): the general-purpose scheme, carrying temperature, the oil/water
    /// split and salinity.
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

    /// Mineral-scale saturation indices from a produced-water ion analysis —
    /// calcite, aragonite, siderite, barite, celestite, gypsum, anhydrite and
    /// halite — with the precipitable mass and the limiting ion.
    ///
    /// Costs 2 credits.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`](crate::Error::SolverDidNotConverge)
    /// when the calculation ran and produced no answer, which is charged.
    /// Every other variant means nothing was computed and nothing was billed.
    pub async fn screen_scale_risk(&self, params: ScreenScaleRiskParams) -> Result<ToolResponse> {
        self.call("screen_scale_risk", &params).await
    }

    /// Wax appearance temperature and margin over three tiers: a C7+ screening
    /// correlation, the Won multi-solid SLE, or the non-ideal SLE flash with
    /// per-component solid fractions, measured melting data and the solubility
    /// gradient for the deposition rate.
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

    /// Data-reconciliation solve (MAP): reconcile pressure gauges and rate
    /// meters, each with a variance, against the network physics and estimate
    /// uncertain source inputs, returning posterior variances and a per-
    /// measurement misfit.
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
