"""Typed methods for every tool in the GraphSolve API.

GENERATED FROM spec/graphsolve-v1.json — DO NOT EDIT BY HAND.
Run `python emit/emit_python.py` after a spec change; CI fails if this
file and the spec disagree.

Engine API version: 1.0.34
Tools: 89
"""

from __future__ import annotations

from typing import Any, Literal


def _present(**kwargs: Any) -> dict[str, Any]:
    """Drop unset arguments.

    An omitted optional argument must be absent from the payload
    rather than null. Several tools treat "not given" and null
    differently.
    """
    return {k: v for k, v in kwargs.items() if v is not None}


class GeneratedMethods:
    """One method per tool. Mixed into :class:`~graphsolve.GraphSolve`.

    Each method wraps ``call()``, which handles retries, idempotency
    and error mapping. Use ``call()`` directly for a tool newer than
    this file.
    """

    def adjust_composition_to_gor(
        self,
        *,
        mole_fractions: list[float],
        target_gor: float,
        binary_interactions: list[dict[str, Any]] | None = None,
        component_names: list[str] | None = None,
        components: list[dict[str, Any]] | None = None,
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong", "PatelTeja"] | None = None,
    ) -> dict[str, Any]:
        """Tune the heavy/light split of a composition to match a target surface GOR
        (Sm³/Sm³).

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "adjust_composition_to_gor",
            _present(
                mole_fractions=mole_fractions,
                target_gor=target_gor,
                binary_interactions=binary_interactions,
                component_names=component_names,
                components=components,
                eos_model=eos_model,
            ),
        )

    def adjust_composition_to_phase_ratio(
        self,
        *,
        mole_fractions: list[float],
        pressure_mpa: float,
        target_gor: float,
        temperature_k: float,
        binary_interactions: list[dict[str, Any]] | None = None,
        component_names: list[str] | None = None,
        components: list[dict[str, Any]] | None = None,
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong", "PatelTeja"] | None = None,
    ) -> dict[str, Any]:
        """Tune a composition to a target in-situ GOR (m3/m3) at given P/T.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "adjust_composition_to_phase_ratio",
            _present(
                mole_fractions=mole_fractions,
                pressure_mpa=pressure_mpa,
                target_gor=target_gor,
                temperature_k=temperature_k,
                binary_interactions=binary_interactions,
                component_names=component_names,
                components=components,
                eos_model=eos_model,
            ),
        )

    def aggregate_type_well(
        self,
        *,
        type_well_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Aggregate analog wells into a probabilistic type well: P10/P50/P90 EUR,
        probability plot, representative declines, optional N-well program
        aggregate.

        Costs 2 credits.
        """
        return self.call(
            "aggregate_type_well",
            _present(
                type_well_json=type_well_json,
            ),
        )

    def allocate_production(
        self,
        *,
        network_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Source-tagged production allocation (back-allocation): attribute
        commingled rates back to each tagged source, including lift-gas
        accounting.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "allocate_production",
            _present(
                network_json=network_json,
            ),
        )

    def analyze_material_balance(
        self,
        *,
        analysis_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Straight-line material-balance diagnostics: gas p/Z → OGIP, or Havlena-
        Odeh F-vs-Et → STOIIP/GIIP, with R² and drive-support intercept.

        Costs 2 credits.
        """
        return self.call(
            "analyze_material_balance",
            _present(
                analysis_json=analysis_json,
            ),
        )

    def calculate_aquifer_influx(
        self,
        *,
        aquifer_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Analytical aquifer water influx We over a pressure history: Fetkovich
        (PSS), Carter-Tracy or van Everdingen-Hurst (USS radial).

        Costs 2 credits.
        """
        return self.call(
            "calculate_aquifer_influx",
            _present(
                aquifer_json=aquifer_json,
            ),
        )

    def calculate_choke_pressure_drop(
        self,
        *,
        choke_diameter: float,
        gas_rate: float,
        inlet_pressure: float,
        inlet_temperature: float,
        oil_rate: float,
        water_rate: float,
        dissolved_gas_ratio: float | None = None,
        gas_mw: float | None = None,
        oil_density: float | None = None,
        slip_model: str | None = None,
        water_salinity: float | None = None,
    ) -> dict[str, Any]:
        """Pressure drop across a choke of known diameter at given rates (Sachdeva
        multiphase model, critical/subcritical).

        Costs 1 credit.
        """
        return self.call(
            "calculate_choke_pressure_drop",
            _present(
                choke_diameter=choke_diameter,
                gas_rate=gas_rate,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                oil_rate=oil_rate,
                water_rate=water_rate,
                dissolved_gas_ratio=dissolved_gas_ratio,
                gas_mw=gas_mw,
                oil_density=oil_density,
                slip_model=slip_model,
                water_salinity=water_salinity,
            ),
        )

    def calculate_choke_size(
        self,
        *,
        downstream_pressure: float,
        free_gas_rate: float,
        oil_rate: float,
        upstream_pressure: float,
        upstream_temperature: float,
        water_rate: float,
        dissolved_gor: float | None = None,
        gas_mw: float | None = None,
        oil_density: float | None = None,
        slip_model: str | None = None,
        water_salinity: float | None = None,
    ) -> dict[str, Any]:
        """Size a choke: find the bean diameter that gives a target downstream
        pressure at the supplied rates.

        Costs 1 credit.
        """
        return self.call(
            "calculate_choke_size",
            _present(
                downstream_pressure=downstream_pressure,
                free_gas_rate=free_gas_rate,
                oil_rate=oil_rate,
                upstream_pressure=upstream_pressure,
                upstream_temperature=upstream_temperature,
                water_rate=water_rate,
                dissolved_gor=dissolved_gor,
                gas_mw=gas_mw,
                oil_density=oil_density,
                slip_model=slip_model,
                water_salinity=water_salinity,
            ),
        )

    def calculate_compressor(
        self,
        *,
        fluid: dict[str, Any],
        inlet_pressure: float,
        inlet_temperature: float,
        pressure_ratio: float,
        mechanical_efficiency: float | None = None,
        polytropic_efficiency: float | None = None,
    ) -> dict[str, Any]:
        """Single-stage centrifugal compressor: outlet P/T and power from inlet P/T,
        pressure ratio, and polytropic efficiency.

        Costs 1 credit.
        """
        return self.call(
            "calculate_compressor",
            _present(
                fluid=fluid,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                pressure_ratio=pressure_ratio,
                mechanical_efficiency=mechanical_efficiency,
                polytropic_efficiency=polytropic_efficiency,
            ),
        )

    def calculate_corrosion_rate(
        self,
        *,
        pressure: float,
        temperature: float,
        bicarbonate_molar: float | None = None,
        co2_mole_fraction: float | None = None,
        design_life_years: float | None = None,
        diameter: float | None = None,
        glycol_wt_pct: float | None = None,
        h2s_mole_fraction: float | None = None,
        inhibitor_efficiency_pct: float | None = None,
        ionic_strength_molar: float | None = None,
        mixture_density: float | None = None,
        mixture_velocity: float | None = None,
        mixture_viscosity: float | None = None,
        model: Literal["de_waard_lotz", "sour", "norsok_m506"] | None = None,
        ph: float | None = None,
        roughness: float | None = None,
        shear_stress_pa: float | None = None,
    ) -> dict[str, Any]:
        """Internal CO2/H2S corrosion rate over three models (de Waard-Lotz, sour
        with the Mariaca regime factor, NORSOK M-506), with NACE MR0175 region,
        inhibited rate and wall allowance.

        Costs 2 credits.
        """
        return self.call(
            "calculate_corrosion_rate",
            _present(
                pressure=pressure,
                temperature=temperature,
                bicarbonate_molar=bicarbonate_molar,
                co2_mole_fraction=co2_mole_fraction,
                design_life_years=design_life_years,
                diameter=diameter,
                glycol_wt_pct=glycol_wt_pct,
                h2s_mole_fraction=h2s_mole_fraction,
                inhibitor_efficiency_pct=inhibitor_efficiency_pct,
                ionic_strength_molar=ionic_strength_molar,
                mixture_density=mixture_density,
                mixture_velocity=mixture_velocity,
                mixture_viscosity=mixture_viscosity,
                model=model,
                ph=ph,
                roughness=roughness,
                shear_stress_pa=shear_stress_pa,
            ),
        )

    def calculate_erosional_velocity(
        self,
        *,
        fluid: dict[str, Any],
        pipe_diameter: float,
        pressure: float,
        temperature: float,
        c_factor: float | None = None,
    ) -> dict[str, Any]:
        """API RP 14E erosional velocity limit and the actual mixture velocity for a
        pipe.

        Costs 1 credit.
        """
        return self.call(
            "calculate_erosional_velocity",
            _present(
                fluid=fluid,
                pipe_diameter=pipe_diameter,
                pressure=pressure,
                temperature=temperature,
                c_factor=c_factor,
            ),
        )

    def calculate_fluid_properties(
        self,
        *,
        fluid_properties: dict[str, Any],
        pressure: float,
        temperature: float,
    ) -> dict[str, Any]:
        """Black-oil PVT properties (Bo, Rs, viscosity, density, Z) of oil / gas /
        water at a given P/T using the selected correlations.

        Costs 1 credit.
        """
        return self.call(
            "calculate_fluid_properties",
            _present(
                fluid_properties=fluid_properties,
                pressure=pressure,
                temperature=temperature,
            ),
        )

    def calculate_gas_dew_point(
        self,
        *,
        condensate_gas_ratio: float,
        gas_gravity: float,
        oil_api_gravity: float,
        temperature: float,
    ) -> dict[str, Any]:
        """Gas dew-point pressure from temperature, gas gravity, oil API and
        condensate-gas ratio (Ahmadi).

        Costs 1 credit.
        """
        return self.call(
            "calculate_gas_dew_point",
            _present(
                condensate_gas_ratio=condensate_gas_ratio,
                gas_gravity=gas_gravity,
                oil_api_gravity=oil_api_gravity,
                temperature=temperature,
            ),
        )

    def calculate_heater_cooler(
        self,
        *,
        fluid: dict[str, Any],
        inlet_pressure: float,
        inlet_temperature: float,
        mode: str,
        approach_temperature: float | None = None,
        duty: float | None = None,
        pressure_drop: float | None = None,
        reference_temperature: float | None = None,
        target_temperature: float | None = None,
    ) -> dict[str, Any]:
        """Heater/cooler in fixed_duty, fixed_temperature, or approach_temperature
        mode; returns outlet P/T and duty.

        Costs 1 credit.
        """
        return self.call(
            "calculate_heater_cooler",
            _present(
                fluid=fluid,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                mode=mode,
                approach_temperature=approach_temperature,
                duty=duty,
                pressure_drop=pressure_drop,
                reference_temperature=reference_temperature,
                target_temperature=target_temperature,
            ),
        )

    def calculate_hydrate_temperature(
        self,
        *,
        gas_gravity: float,
        pressure: float,
        co2_mole_fraction: float | None = None,
        h2s_mole_fraction: float | None = None,
    ) -> dict[str, Any]:
        """Hydrate formation temperature at a given pressure (Baillie-Wichert) with
        sour-gas corrections.

        Costs 1 credit.
        """
        return self.call(
            "calculate_hydrate_temperature",
            _present(
                gas_gravity=gas_gravity,
                pressure=pressure,
                co2_mole_fraction=co2_mole_fraction,
                h2s_mole_fraction=h2s_mole_fraction,
            ),
        )

    def calculate_isenthalpic_temperature(
        self,
        *,
        fluid: dict[str, Any],
        inlet_pressure: float,
        inlet_temperature: float,
        outlet_pressure: float,
    ) -> dict[str, Any]:
        """Outlet temperature after a constant-enthalpy (Joule-Thomson) expansion to
        a lower pressure.

        Costs 1 credit.
        """
        return self.call(
            "calculate_isenthalpic_temperature",
            _present(
                fluid=fluid,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                outlet_pressure=outlet_pressure,
            ),
        )

    def calculate_jt_valve(
        self,
        *,
        fluid: dict[str, Any],
        inlet_pressure: float,
        inlet_temperature: float,
        pressure_drop: float,
    ) -> dict[str, Any]:
        """Joule-Thomson throttle valve: outlet T after an isenthalpic pressure drop.

        Costs 1 credit.
        """
        return self.call(
            "calculate_jt_valve",
            _present(
                fluid=fluid,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                pressure_drop=pressure_drop,
            ),
        )

    def calculate_mmp(
        self,
        *,
        experiment_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Minimum miscibility pressure (MMP) between a reservoir fluid and an
        injection gas (mixing-cell method).

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "calculate_mmp",
            _present(
                experiment_json=experiment_json,
            ),
        )

    def calculate_mpfm_allocation(
        self,
        *,
        allocation_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Multiphase-flow-meter allocation: distribute measured rates among streams
        (in-situ -> standard conditions).

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "calculate_mpfm_allocation",
            _present(
                allocation_json=allocation_json,
            ),
        )

    def calculate_multistage_compressor(
        self,
        *,
        fluid: dict[str, Any],
        inlet_pressure: float,
        inlet_temperature: float,
        stages: list[dict[str, Any]],
        intercool_pressure_drop: float | None = None,
        intercool_temperature: float | None = None,
        mechanical_efficiency: float | None = None,
    ) -> dict[str, Any]:
        """Multi-stage centrifugal train with optional inter-stage cooling; per-stage
        pressure ratios and overall discharge P/T, cooler duty and shaft power.

        Costs 1 credit.
        """
        return self.call(
            "calculate_multistage_compressor",
            _present(
                fluid=fluid,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                stages=stages,
                intercool_pressure_drop=intercool_pressure_drop,
                intercool_temperature=intercool_temperature,
                mechanical_efficiency=mechanical_efficiency,
            ),
        )

    def calculate_nodal_analysis(
        self,
        *,
        ipr: Any,
        tubing_diameter: float,
        tubing_length: float,
        tubing_roughness: float,
        wellhead_pressure: float,
        wellhead_temperature: float,
        dissolved_gas_ratio: float | None = None,
        flow_correlation: str | None = None,
        gas_mw: float | None = None,
        oil_density: float | None = None,
        tubing_angle: float | None = None,
        water_salinity: float | None = None,
    ) -> dict[str, Any]:
        """Operating point = IPR intersect VLP. Generates both curves and finds the
        stabilised rate and flowing BHP.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "calculate_nodal_analysis",
            _present(
                ipr=ipr,
                tubing_diameter=tubing_diameter,
                tubing_length=tubing_length,
                tubing_roughness=tubing_roughness,
                wellhead_pressure=wellhead_pressure,
                wellhead_temperature=wellhead_temperature,
                dissolved_gas_ratio=dissolved_gas_ratio,
                flow_correlation=flow_correlation,
                gas_mw=gas_mw,
                oil_density=oil_density,
                tubing_angle=tubing_angle,
                water_salinity=water_salinity,
            ),
        )

    def calculate_phase_cuts(
        self,
        *,
        gas_rate: float | None = None,
        gas_unit: str | None = None,
        oil_rate: float | None = None,
        oil_unit: str | None = None,
        water_rate: float | None = None,
        water_unit: str | None = None,
    ) -> dict[str, Any]:
        """Watercut / GOR / phase fractions from volumetric phase rates (any flow-
        rate units).

        Costs 1 credit.
        """
        return self.call(
            "calculate_phase_cuts",
            _present(
                gas_rate=gas_rate,
                gas_unit=gas_unit,
                oil_rate=oil_rate,
                oil_unit=oil_unit,
                water_rate=water_rate,
                water_unit=water_unit,
            ),
        )

    def calculate_pipe_traverse(
        self,
        *,
        angle: float,
        diameter: float,
        gas_rate: float,
        inlet_pressure: float,
        inlet_temperature: float,
        length: float,
        oil_rate: float,
        roughness: float,
        water_rate: float,
        dissolved_gas_ratio: float | None = None,
        flow_boundary: str | None = None,
        flow_correlation: str | None = None,
        gas_mw: float | None = None,
        heat_transfer_coefficient: float | None = None,
        oil_density: float | None = None,
        pressure_boundary: str | None = None,
        surrounding_temperature: float | None = None,
        water_salinity: float | None = None,
    ) -> dict[str, Any]:
        """March pressure and temperature along a single pipe (multi-segment) with
        heat transfer.

        Costs 3 credits.
        """
        return self.call(
            "calculate_pipe_traverse",
            _present(
                angle=angle,
                diameter=diameter,
                gas_rate=gas_rate,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                length=length,
                oil_rate=oil_rate,
                roughness=roughness,
                water_rate=water_rate,
                dissolved_gas_ratio=dissolved_gas_ratio,
                flow_boundary=flow_boundary,
                flow_correlation=flow_correlation,
                gas_mw=gas_mw,
                heat_transfer_coefficient=heat_transfer_coefficient,
                oil_density=oil_density,
                pressure_boundary=pressure_boundary,
                surrounding_temperature=surrounding_temperature,
                water_salinity=water_salinity,
            ),
        )

    def calculate_pressure_drop(
        self,
        *,
        angle: float,
        correlation: Literal["Aziz", "Baxendell-Thomas", "Beggs-Brill", "CHAOS", "Dukler", "Duns-Ros", "Fancher-Brown", "GOAT", "Gray", "Griffith-Wallis", "Hagedorn-Slug", "KISS", "Mist", "ml_tuned", "Mukherjee-Brill", "Poettmann-Carpenter", "Default", "SinglePhaseGas", "SUPREME"],
        density: list[float],
        diameter: float,
        ift: float,
        pressure: float,
        roughness: float,
        velocity: list[float],
        viscosity: list[float],
    ) -> dict[str, Any]:
        """Multiphase pressure gradient for a single pipe segment using a chosen
        correlation. Returns gradient components, holdup and flow regime.

        Costs 1 credit.
        """
        return self.call(
            "calculate_pressure_drop",
            _present(
                angle=angle,
                correlation=correlation,
                density=density,
                diameter=diameter,
                ift=ift,
                pressure=pressure,
                roughness=roughness,
                velocity=velocity,
                viscosity=viscosity,
            ),
        )

    def calculate_pump_head(
        self,
        *,
        flow_rate: float,
        head_curve: list[list[float]],
        inlet_pressure: float,
        inlet_temperature: float,
        liquid_density: float,
    ) -> dict[str, Any]:
        """ESP/pump outlet pressure from a head-vs-rate performance curve,
        interpolated at the operating rate and converted with liquid density.

        Costs 1 credit.
        """
        return self.call(
            "calculate_pump_head",
            _present(
                flow_rate=flow_rate,
                head_curve=head_curve,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                liquid_density=liquid_density,
            ),
        )

    def calculate_rate_from_choke(
        self,
        *,
        choke_diameter: float,
        downstream_pressure: float,
        upstream_pressure: float,
        upstream_temperature: float,
        cgr: float | None = None,
        dissolved_gor: float | None = None,
        fixed: str | None = None,
        free_gas_rate: float | None = None,
        gas_mw: float | None = None,
        gor: float | None = None,
        oil_density: float | None = None,
        oil_rate: float | None = None,
        slip_model: str | None = None,
        target_downstream_temperature: float | None = None,
        water_rate: float | None = None,
        water_salinity: float | None = None,
        watercut: float | None = None,
        wgr: float | None = None,
    ) -> dict[str, Any]:
        """Find the rates a fixed choke passes for a given pressure drop, holding a
        phase or ratio (watercut/GOR/WGR/CGR) fixed.

        Costs 1 credit.
        """
        return self.call(
            "calculate_rate_from_choke",
            _present(
                choke_diameter=choke_diameter,
                downstream_pressure=downstream_pressure,
                upstream_pressure=upstream_pressure,
                upstream_temperature=upstream_temperature,
                cgr=cgr,
                dissolved_gor=dissolved_gor,
                fixed=fixed,
                free_gas_rate=free_gas_rate,
                gas_mw=gas_mw,
                gor=gor,
                oil_density=oil_density,
                oil_rate=oil_rate,
                slip_model=slip_model,
                target_downstream_temperature=target_downstream_temperature,
                water_rate=water_rate,
                water_salinity=water_salinity,
                watercut=watercut,
                wgr=wgr,
            ),
        )

    def calculate_reciprocating_compressor(
        self,
        *,
        discharge_pressure: float,
        inlet_pressure: float,
        inlet_temperature: float,
        speed_rpm: float,
        swept_volume_per_rev_m3: float,
        clearance_fraction: float | None = None,
        co2_fraction: float | None = None,
        gas_molecular_weight: float | None = None,
        h2s_fraction: float | None = None,
        max_pressure_ratio: float | None = None,
        mechanical_efficiency: float | None = None,
        min_pressure_ratio: float | None = None,
        n2_fraction: float | None = None,
        polytropic_efficiency: float | None = None,
    ) -> dict[str, Any]:
        """Reciprocating (positive-displacement) compressor. Mass flow is set by
        displacement x speed x volumetric efficiency, not supplied.

        Costs 1 credit.
        """
        return self.call(
            "calculate_reciprocating_compressor",
            _present(
                discharge_pressure=discharge_pressure,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                speed_rpm=speed_rpm,
                swept_volume_per_rev_m3=swept_volume_per_rev_m3,
                clearance_fraction=clearance_fraction,
                co2_fraction=co2_fraction,
                gas_molecular_weight=gas_molecular_weight,
                h2s_fraction=h2s_fraction,
                max_pressure_ratio=max_pressure_ratio,
                mechanical_efficiency=mechanical_efficiency,
                min_pressure_ratio=min_pressure_ratio,
                n2_fraction=n2_fraction,
                polytropic_efficiency=polytropic_efficiency,
            ),
        )

    def calculate_reid_vapour_pressure(
        self,
        *,
        liquid_mole_fractions: list[float],
        binary_interactions: list[dict[str, Any]] | None = None,
        component_names: list[str] | None = None,
        components: list[dict[str, Any]] | None = None,
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong", "PatelTeja"] | None = None,
    ) -> dict[str, Any]:
        """Reid vapour pressure (RVP) of a liquid composition at 100 degF.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "calculate_reid_vapour_pressure",
            _present(
                liquid_mole_fractions=liquid_mole_fractions,
                binary_interactions=binary_interactions,
                component_names=component_names,
                components=components,
                eos_model=eos_model,
            ),
        )

    def calculate_saturation_pressure(
        self,
        *,
        component_names: list[str],
        mole_fractions: list[float],
        temperature_k: float,
        boundary: str | None = None,
        eos_model: str | None = None,
    ) -> dict[str, Any]:
        """Bubble- or dew-point pressure of a composition at a given temperature
        (cubic EOS).

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "calculate_saturation_pressure",
            _present(
                component_names=component_names,
                mole_fractions=mole_fractions,
                temperature_k=temperature_k,
                boundary=boundary,
                eos_model=eos_model,
            ),
        )

    def calculate_screw_compressor(
        self,
        *,
        discharge_pressure: float,
        displacement_per_rev_m3: float,
        inlet_pressure: float,
        inlet_temperature: float,
        shaft_speed_rev_s: float,
        co2_fraction: float | None = None,
        gas_molecular_weight: float | None = None,
        h2s_fraction: float | None = None,
        mechanical_efficiency: float | None = None,
        n2_fraction: float | None = None,
        polytropic_efficiency: float | None = None,
        subtype: str | None = None,
        volumetric_efficiency: float | None = None,
    ) -> dict[str, Any]:
        """Screw (positive-displacement) compressor closed by black-box efficiencies;
        flow set by displacement x shaft speed x volumetric efficiency.

        Costs 1 credit.
        """
        return self.call(
            "calculate_screw_compressor",
            _present(
                discharge_pressure=discharge_pressure,
                displacement_per_rev_m3=displacement_per_rev_m3,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                shaft_speed_rev_s=shaft_speed_rev_s,
                co2_fraction=co2_fraction,
                gas_molecular_weight=gas_molecular_weight,
                h2s_fraction=h2s_fraction,
                mechanical_efficiency=mechanical_efficiency,
                n2_fraction=n2_fraction,
                polytropic_efficiency=polytropic_efficiency,
                subtype=subtype,
                volumetric_efficiency=volumetric_efficiency,
            ),
        )

    def calculate_turbine(
        self,
        *,
        fluid: dict[str, Any],
        inlet_pressure: float,
        inlet_temperature: float,
        pressure_ratio: float,
        mechanical_efficiency: float | None = None,
        polytropic_efficiency: float | None = None,
    ) -> dict[str, Any]:
        """Single-stage centrifugal turbine/expander: outlet P/T and power generated
        from inlet P/T and an expansion pressure ratio (0 < PR < 1).

        Costs 1 credit.
        """
        return self.call(
            "calculate_turbine",
            _present(
                fluid=fluid,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                pressure_ratio=pressure_ratio,
                mechanical_efficiency=mechanical_efficiency,
                polytropic_efficiency=polytropic_efficiency,
            ),
        )

    def calculate_volumetrics(
        self,
        *,
        volumetrics_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Forward OOIP / OGIP from area·thickness·NTG·φ·(1−Sw) ÷ FVF; scalar or per-
        realisation arrays, composes with generate_samples / compute_statistics
        for probabilistic in-place volumes.

        Costs 2 credits.
        """
        return self.call(
            "calculate_volumetrics",
            _present(
                volumetrics_json=volumetrics_json,
            ),
        )

    def calculate_water_properties(
        self,
        *,
        pressure: float,
        temperature: float,
        salinity: float | None = None,
    ) -> dict[str, Any]:
        """Brine properties (density, viscosity, compressibility, FVF) via IAPWS-95
        with a salinity correction.

        Costs 1 credit.
        """
        return self.call(
            "calculate_water_properties",
            _present(
                pressure=pressure,
                temperature=temperature,
                salinity=salinity,
            ),
        )

    def calculate_wax_deposition_rate(
        self,
        *,
        ambient_temperature: float,
        bulk_temperature: float,
        heat_transfer_coefficient: float,
        oil_density: float,
        solubility_gradient_per_k: float,
        oil_molecular_weight: float | None = None,
        oil_thermal_conductivity: float | None = None,
        oil_viscosity: float | None = None,
        wax_diffusivity: float | None = None,
        wax_molecular_weight: float | None = None,
    ) -> dict[str, Any]:
        """Wax deposition rate on a cold wall from the subcooling and the wax
        solubility gradient — thickness in mm/yr and mass flux in g/m2/day.

        Costs 2 credits.
        """
        return self.call(
            "calculate_wax_deposition_rate",
            _present(
                ambient_temperature=ambient_temperature,
                bulk_temperature=bulk_temperature,
                heat_transfer_coefficient=heat_transfer_coefficient,
                oil_density=oil_density,
                solubility_gradient_per_k=solubility_gradient_per_k,
                oil_molecular_weight=oil_molecular_weight,
                oil_thermal_conductivity=oil_thermal_conductivity,
                oil_viscosity=oil_viscosity,
                wax_diffusivity=wax_diffusivity,
                wax_molecular_weight=wax_molecular_weight,
            ),
        )

    def characterize_pseudo_component(
        self,
        *,
        boiling_point: float | None = None,
        method: str | None = None,
        molecular_weight: float | None = None,
        specific_gravity: float | None = None,
    ) -> dict[str, Any]:
        """Critical properties (Tc, Pc, Vc, Watson K) of a pseudo-component from any
        two of MW / specific gravity / boiling point.

        Costs 1 credit.
        """
        return self.call(
            "characterize_pseudo_component",
            _present(
                boiling_point=boiling_point,
                method=method,
                molecular_weight=molecular_weight,
                specific_gravity=specific_gravity,
            ),
        )

    def compare_pipeline_correlations(
        self,
        *,
        fluid: Any,
        inlet: dict[str, Any],
        pipe_segments: Any,
        correlations: list[Literal["Aziz", "Baxendell-Thomas", "Beggs-Brill", "CHAOS", "Dukler", "Duns-Ros", "Fancher-Brown", "GOAT", "Gray", "Griffith-Wallis", "Hagedorn-Slug", "KISS", "Mist", "ml_tuned", "Mukherjee-Brill", "Poettmann-Carpenter", "Default", "SinglePhaseGas", "SUPREME"]] | None = None,
    ) -> dict[str, Any]:
        """Walk a multi-segment pipe with each correlation and compare the predicted
        outlet pressure.

        Costs 3 credits.
        """
        return self.call(
            "compare_pipeline_correlations",
            _present(
                fluid=fluid,
                inlet=inlet,
                pipe_segments=pipe_segments,
                correlations=correlations,
            ),
        )

    def compare_pressure_drop_correlations(
        self,
        *,
        angle: float,
        density: list[float],
        diameter: float,
        ift: float,
        pressure: float,
        roughness: float,
        velocity: list[float],
        viscosity: list[float],
        correlations: list[str] | None = None,
    ) -> dict[str, Any]:
        """Run every multiphase pressure-drop correlation (or a subset) on one
        segment and compare the predicted gradients.

        Costs 3 credits.
        """
        return self.call(
            "compare_pressure_drop_correlations",
            _present(
                angle=angle,
                density=density,
                diameter=diameter,
                ift=ift,
                pressure=pressure,
                roughness=roughness,
                velocity=velocity,
                viscosity=viscosity,
                correlations=correlations,
            ),
        )

    def compute_statistics(
        self,
        *,
        statistics_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Summary statistics (mean/std, percentiles, histogram, CDF) over labelled
        output columns, plus optional input→output correlation sensitivity;
        network-agnostic (e.g. NPV/EUR from an external model).

        Costs 2 credits.
        """
        return self.call(
            "compute_statistics",
            _present(
                statistics_json=statistics_json,
            ),
        )

    def convert_units(
        self,
        *,
        from_unit: str,
        to_unit: str,
        value: float,
    ) -> dict[str, Any]:
        """Convert a value between units (pressure, rate, temperature, length, area,
        ...).

        Free.
        """
        return self.call(
            "convert_units",
            _present(
                from_unit=from_unit,
                to_unit=to_unit,
                value=value,
            ),
        )

    def evaluate_flow_assurance_profile(
        self,
        *,
        inlet_pressure: float,
        inlet_temperature: float,
        mass_flow: float,
        outlet_pressure: float,
        outlet_temperature: float,
        segments: list[dict[str, Any]],
        c_factor: float | None = None,
        component_names: list[str] | None = None,
        heat_capacity: float | None = None,
        inhibitor_depression_k: float | None = None,
        inlet_mixture_density: float | None = None,
        vapor_mole_fractions: list[float] | None = None,
    ) -> dict[str, Any]:
        """Hydrate, corrosion and erosion screening along a whole line, each
        reporting the controlling sample — index, length, pressure and temperature
        — not just a worst-case number.

        Costs 8 credits, plus 1 per 250 ms beyond the first 4 s of compute.
        """
        return self.call(
            "evaluate_flow_assurance_profile",
            _present(
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                mass_flow=mass_flow,
                outlet_pressure=outlet_pressure,
                outlet_temperature=outlet_temperature,
                segments=segments,
                c_factor=c_factor,
                component_names=component_names,
                heat_capacity=heat_capacity,
                inhibitor_depression_k=inhibitor_depression_k,
                inlet_mixture_density=inlet_mixture_density,
                vapor_mole_fractions=vapor_mole_fractions,
            ),
        )

    def fit_decline(
        self,
        *,
        fit_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """History-match a decline model (Arps / mod-hyperbolic / Duong / SEPD / PLE)
        to a time/rate history with optional outlier rejection; omit kind to auto-
        select by R².

        Costs 2 credits.
        """
        return self.call(
            "fit_decline",
            _present(
                fit_json=fit_json,
            ),
        )

    def flash_to_surface(
        self,
        *,
        mole_fractions: list[float],
        binary_interactions: list[dict[str, Any]] | None = None,
        component_names: list[str] | None = None,
        components: list[dict[str, Any]] | None = None,
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong", "PatelTeja"] | None = None,
    ) -> dict[str, Any]:
        """Flash a reservoir composition to stock-tank: GOR, API/oil density,
        shrinkage.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "flash_to_surface",
            _present(
                mole_fractions=mole_fractions,
                binary_interactions=binary_interactions,
                component_names=component_names,
                components=components,
                eos_model=eos_model,
            ),
        )

    def generate_decline(
        self,
        *,
        decline_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Generate a decline curve (rate + cumulative) from known parameters: Arps,
        modified-hyperbolic, Duong, stretched-exponential (SEPD), power-law-
        exponential (PLE).

        Costs 2 credits.
        """
        return self.call(
            "generate_decline",
            _present(
                decline_json=decline_json,
            ),
        )

    def generate_ipr_curve(
        self,
        *,
        inflow_model: Any,
        n_points: int | None = None,
    ) -> dict[str, Any]:
        """Inflow performance relationship (IPR) curve for one well; real physics per
        point. Required inputs depend on ipr_model.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "generate_ipr_curve",
            _present(
                inflow_model=inflow_model,
                n_points=n_points,
            ),
        )

    def generate_phase_envelope(
        self,
        *,
        component_names: list[str],
        mole_fractions: list[float],
        eos_model: str | None = None,
    ) -> dict[str, Any]:
        """Two-phase P-T envelope (dew/bubble locus + critical point) of a
        composition (cubic EOS only; not GERG).

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "generate_phase_envelope",
            _present(
                component_names=component_names,
                mole_fractions=mole_fractions,
                eos_model=eos_model,
            ),
        )

    def generate_samples(
        self,
        *,
        samples_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Draw Monte-Carlo / LHS / Sobol samples from named distributions
        (Normal/Uniform/LogNormal/Triangular/Beta/PERT/Discrete) with optional
        correlations; simple or Saltelli design; network-agnostic, feeds any
        model.

        Costs 5 credits, plus 1 per 500 ms beyond the first 3 s of compute.
        """
        return self.call(
            "generate_samples",
            _present(
                samples_json=samples_json,
            ),
        )

    def generate_type_curve(
        self,
        *,
        inflow_model: Any,
        n: int | None = None,
        td_max: float | None = None,
        td_min: float | None = None,
    ) -> dict[str, Any]:
        """Dimensionless transient type-curve surface (pD/qD, Bourdet derivative) for
        a transient ipr_model.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "generate_type_curve",
            _present(
                inflow_model=inflow_model,
                n=n,
                td_max=td_max,
                td_min=td_min,
            ),
        )

    def get_eos_component(
        self,
        *,
        name: str,
    ) -> dict[str, Any]:
        """Critical properties (Tc, Pc, omega, MW) of a single EOS component by name.

        Free.
        """
        return self.call(
            "get_eos_component",
            _present(
                name=name,
            ),
        )

    def list_correlations(
        self,
        *,
        category: str | None = None,
    ) -> dict[str, Any]:
        """List the available correlations, optionally filtered by category.

        Free.
        """
        return self.call(
            "list_correlations",
            _present(
                category=category,
            ),
        )

    def list_edge_types(
        self,
    ) -> dict[str, Any]:
        """List the network edge types (pipeline, choke, compressor, ...) and their
        params.

        Free.
        """
        return self.call(
            "list_edge_types",
            {},
        )

    def list_eos_binary_interactions(
        self,
        *,
        name: str,
    ) -> dict[str, Any]:
        """Peng-Robinson binary interaction parameters for a component.

        Free.
        """
        return self.call(
            "list_eos_binary_interactions",
            _present(
                name=name,
            ),
        )

    def list_eos_components(
        self,
    ) -> dict[str, Any]:
        """List the component names in the built-in equation-of-state database.

        Free.
        """
        return self.call(
            "list_eos_components",
            {},
        )

    def list_flow_assurance_models(
        self,
    ) -> dict[str, Any]:
        """The model vocabularies the flow-assurance tools accept — hydrate, wax and
        corrosion models with their tier, plus inhibitor names and vdWP guests.

        Free.
        """
        return self.call(
            "list_flow_assurance_models",
            {},
        )

    def list_fluid_types(
        self,
    ) -> dict[str, Any]:
        """List the fluid types (oil, gas, water) and the required PVT configuration.

        Free.
        """
        return self.call(
            "list_fluid_types",
            {},
        )

    def list_node_types(
        self,
    ) -> dict[str, Any]:
        """List the network node types (junction, source, sink) and their params.

        Free.
        """
        return self.call(
            "list_node_types",
            {},
        )

    def list_transient_solvers(
        self,
    ) -> dict[str, Any]:
        """Which of the three transient schemes to use, what each carries, and the
        conventions every transient request shares.

        Free.
        """
        return self.call(
            "list_transient_solvers",
            {},
        )

    def match_fetkovich_ab(
        self,
        *,
        inflow_model: Any,
    ) -> dict[str, Any]:
        """Fit the Fetkovich C and n (deliverability) coefficients to test points.

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "match_fetkovich_ab",
            _present(
                inflow_model=inflow_model,
            ),
        )

    def match_forchheimer_ab(
        self,
        *,
        inflow_model: Any,
    ) -> dict[str, Any]:
        """Fit the Forchheimer A and B (non-Darcy gas) coefficients to test points.

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "match_forchheimer_ab",
            _present(
                inflow_model=inflow_model,
            ),
        )

    def match_parameters(
        self,
        *,
        inflow_model: Any,
        match_target: str | None = None,
    ) -> dict[str, Any]:
        """Unified matching dispatcher: fit productivity_index / skin /
        forchheimer_ab / fetkovich_ab, routed by match_target or ipr_model.

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "match_parameters",
            _present(
                inflow_model=inflow_model,
                match_target=match_target,
            ),
        )

    def match_productivity_index(
        self,
        *,
        inflow_model: Any,
    ) -> dict[str, Any]:
        """Fit the productivity index to observed (rate, flowing-BHP) test points.

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "match_productivity_index",
            _present(
                inflow_model=inflow_model,
            ),
        )

    def match_pvt(
        self,
        *,
        match_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Regress black-oil PVT correlations against measured lab data (bubble
        point, Rs, Bo, viscosity, density).

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "match_pvt",
            _present(
                match_json=match_json,
            ),
        )

    def match_skin(
        self,
        *,
        inflow_model: Any,
    ) -> dict[str, Any]:
        """Fit the skin factor to observed (rate, flowing-BHP) test points.

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "match_skin",
            _present(
                inflow_model=inflow_model,
            ),
        )

    def optimise_network(
        self,
        *,
        optimisation_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Optimise a network: move control variables within bounds to maximise /
        minimise an objective (phase rate, revenue, pressure drop, power) under
        rate / pressure / resource constraints. The optimisation counterpart of
        solve_network.

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "optimise_network",
            _present(
                optimisation_json=optimisation_json,
            ),
        )

    def rta_diagnostics(
        self,
        *,
        inflow_model: Any,
    ) -> dict[str, Any]:
        """Rate-transient-analysis diagnostics (Blasingame, Agarwal-Gardner, Bourdet)
        with material-balance time and pseudo-time, from a time/rate/pressure
        history.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "rta_diagnostics",
            _present(
                inflow_model=inflow_model,
            ),
        )

    def run_cce(
        self,
        *,
        experiment_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Constant composition expansion (CCE) PVT experiment.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "run_cce",
            _present(
                experiment_json=experiment_json,
            ),
        )

    def run_cvd(
        self,
        *,
        experiment_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Constant volume depletion (CVD) PVT experiment (gas condensate / volatile
        oil).

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "run_cvd",
            _present(
                experiment_json=experiment_json,
            ),
        )

    def run_dle(
        self,
        *,
        experiment_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Differential liberation expansion (DLE) PVT experiment.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "run_dle",
            _present(
                experiment_json=experiment_json,
            ),
        )

    def run_eos_flash(
        self,
        *,
        mole_fractions: list[float],
        pressure_mpa: float,
        temperature_k: float,
        binary_interactions: list[dict[str, Any]] | None = None,
        component_names: list[str] | None = None,
        components: list[dict[str, Any]] | None = None,
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong", "PatelTeja"] | None = None,
    ) -> dict[str, Any]:
        """PT flash of a composition (Peng-Robinson default, SRK, Patel-Teja): phase
        split, K-values, densities. Pressure in MPa.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "run_eos_flash",
            _present(
                mole_fractions=mole_fractions,
                pressure_mpa=pressure_mpa,
                temperature_k=temperature_k,
                binary_interactions=binary_interactions,
                component_names=component_names,
                components=components,
                eos_model=eos_model,
            ),
        )

    def run_forecast(
        self,
        *,
        forecast_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Time-series production forecast: march a network through time, solving or
        optimising each step with decline curves, scheduled events and transient-
        IPR wells; returns per-timestep rates and cumulatives.

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "run_forecast",
            _present(
                forecast_json=forecast_json,
            ),
        )

    def run_material_balance(
        self,
        *,
        reservoir_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Material-balance depletion / reserves over one or more reservoir blocks
        (STOIIP/GIIP, recovery, pressure decline, aquifer).

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "run_material_balance",
            _present(
                reservoir_json=reservoir_json,
            ),
        )

    def run_molecule_tracking(
        self,
        *,
        fluids_json: dict[str, Any] | list[Any] | str,
        solved_network_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Trace component compositions through an already-solved network.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "run_molecule_tracking",
            _present(
                fluids_json=fluids_json,
                solved_network_json=solved_network_json,
            ),
        )

    def run_parametric_study(
        self,
        *,
        study_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Sensitivity / parametric sweep over a network (linear / tornado / grid /
        Monte Carlo); returns per-output statistics.

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "run_parametric_study",
            _present(
                study_json=study_json,
            ),
        )

    def run_process_graph(
        self,
        *,
        process_graph_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Simulate a process flow graph (mixers, separators, heaters, compressors)
        of compositional streams.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "run_process_graph",
            _present(
                process_graph_json=process_graph_json,
            ),
        )

    def run_separator_test(
        self,
        *,
        experiment_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Multi-stage separator test to stock-tank: stage GOR, FVF, stock-tank oil
        density.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "run_separator_test",
            _present(
                experiment_json=experiment_json,
            ),
        )

    def run_swelling_test(
        self,
        *,
        experiment_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Swelling test: add injection gas/solvent and report saturation pressure +
        swelling factor per step.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "run_swelling_test",
            _present(
                experiment_json=experiment_json,
            ),
        )

    def run_transient_field(
        self,
        *,
        transient_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Transient multiphase pipe flow, fully-implicit four-field two-fluid
        (Graphsolve-Field): for severe slugging and countercurrent flow.

        Costs 250 credits, plus 1 per 50 ms beyond the first 30 s of compute.
        """
        return self.call(
            "run_transient_field",
            _present(
                transient_json=transient_json,
            ),
        )

    def run_transient_flux(
        self,
        *,
        transient_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Transient multiphase pipe flow, semi-implicit sequential (Graphsolve-
        Flux): the general-purpose scheme, carrying temperature, composition and
        salinity.

        Costs 250 credits, plus 1 per 50 ms beyond the first 30 s of compute.
        """
        return self.call(
            "run_transient_flux",
            _present(
                transient_json=transient_json,
            ),
        )

    def run_transient_wave(
        self,
        *,
        transient_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Transient multiphase pipe flow, explicit Godunov/Rusanov drift flux
        (Graphsolve-Wave): the only scheme that resolves pressure waves.

        Costs 250 credits, plus 1 per 50 ms beyond the first 30 s of compute.
        """
        return self.call(
            "run_transient_wave",
            _present(
                transient_json=transient_json,
            ),
        )

    def screen_asphaltene_risk(
        self,
        *,
        bubble_point_pressure: float,
        live_oil_density: float,
        reservoir_pressure: float,
    ) -> dict[str, Any]:
        """De Boer (1995) asphaltene-onset screening from in-situ undersaturation and
        live-oil density; four-bin risk classification.

        Costs 2 credits.
        """
        return self.call(
            "screen_asphaltene_risk",
            _present(
                bubble_point_pressure=bubble_point_pressure,
                live_oil_density=live_oil_density,
                reservoir_pressure=reservoir_pressure,
            ),
        )

    def screen_hydrate_risk(
        self,
        *,
        pressure: float,
        temperature: float,
        component_names: list[str] | None = None,
        inhibitor: Literal["methanol", "meg", "deg", "teg"] | None = None,
        inhibitor_wt_pct: float | None = None,
        model: Literal["towler_mokhatab", "vdwp_si", "vdwp_sii", "vdwp_combined"] | None = None,
        mole_fractions: list[float] | None = None,
        molecular_weights: list[float] | None = None,
        specific_gravity: float | None = None,
    ) -> dict[str, Any]:
        """Hydrate formation temperature and margin at a live state, over four models
        (Towler-Mokhatab screening or vdW-Platteeuw sI / sII / combined), with
        Hammerschmidt inhibitor depression.

        Costs 2 credits.
        """
        return self.call(
            "screen_hydrate_risk",
            _present(
                pressure=pressure,
                temperature=temperature,
                component_names=component_names,
                inhibitor=inhibitor,
                inhibitor_wt_pct=inhibitor_wt_pct,
                model=model,
                mole_fractions=mole_fractions,
                molecular_weights=molecular_weights,
                specific_gravity=specific_gravity,
            ),
        )

    def screen_liquid_loading(
        self,
        *,
        gas_density: float,
        gas_velocity: float,
        liquid_density: float,
        surface_tension: float,
    ) -> dict[str, Any]:
        """Turner critical-velocity screen for gas-well liquid loading: critical
        velocity, ratio and a loading flag.

        Costs 2 credits.
        """
        return self.call(
            "screen_liquid_loading",
            _present(
                gas_density=gas_density,
                gas_velocity=gas_velocity,
                liquid_density=liquid_density,
                surface_tension=surface_tension,
            ),
        )

    def screen_wax_risk(
        self,
        *,
        temperature: float,
        density_c7_plus: float | None = None,
        model: Literal["screening", "won"] | None = None,
        mole_fractions: list[float] | None = None,
        molecular_weights: list[float] | None = None,
        mw_c7_plus: float | None = None,
        paraffin_mass_fraction: float | None = None,
        watson_k: float | None = None,
        wax_former_mask: list[bool] | None = None,
    ) -> dict[str, Any]:
        """Wax appearance temperature and margin: a C7+ screening correlation (Tier
        1) or the Won multi-solid SLE (Tier 2) with per-component solid fractions.

        Costs 2 credits.
        """
        return self.call(
            "screen_wax_risk",
            _present(
                temperature=temperature,
                density_c7_plus=density_c7_plus,
                model=model,
                mole_fractions=mole_fractions,
                molecular_weights=molecular_weights,
                mw_c7_plus=mw_c7_plus,
                paraffin_mass_fraction=paraffin_mass_fraction,
                watson_k=watson_k,
                wax_former_mask=wax_former_mask,
            ),
        )

    def solve_network(
        self,
        *,
        network_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Solve a complete production network (Newton-Raphson): pressures,
        temperatures and rates at every node and edge. The headline tool.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "solve_network",
            _present(
                network_json=network_json,
            ),
        )

    def solve_network_map(
        self,
        *,
        network_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Data-reconciliation solve (MAP): inject measured values with a variance
        and reconcile them, returning a posterior variance per reconciled
        quantity.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "solve_network_map",
            _present(
                network_json=network_json,
            ),
        )

    def split_plus_fraction(
        self,
        *,
        split_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Split a heavy/plus fraction into N pseudo-components (gamma distribution).
        Front half of building a matched compositional fluid.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "split_plus_fraction",
            _present(
                split_json=split_json,
            ),
        )

    def transient_bhp_history(
        self,
        *,
        inflow_model: Any,
    ) -> dict[str, Any]:
        """Compute the flowing-BHP history from a rate history (transient
        superposition).

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "transient_bhp_history",
            _present(
                inflow_model=inflow_model,
            ),
        )

    def transient_rate_history(
        self,
        *,
        inflow_model: Any,
    ) -> dict[str, Any]:
        """Compute the rate history from a pressure history (transient
        superposition).

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "transient_rate_history",
            _present(
                inflow_model=inflow_model,
            ),
        )

    def tune_mpfm_allocation(
        self,
        *,
        tuning_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Regress MPFM allocation parameters against measured observation rows
        (inverse of calculate_mpfm_allocation).

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "tune_mpfm_allocation",
            _present(
                tuning_json=tuning_json,
            ),
        )

    def validate_solver_payload(
        self,
        *,
        network_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Pre-flight check a network payload without solving (runs the same build
        step solve_network does). Run this first when assembling a network.

        Free.
        """
        return self.call(
            "validate_solver_payload",
            _present(
                network_json=network_json,
            ),
        )
