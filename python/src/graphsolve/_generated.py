"""Typed methods for every tool in the GraphSolve API.

GENERATED FROM spec/graphsolve-v1.json — DO NOT EDIT BY HAND.
Run `python emit/emit_python.py` after a spec change; CI fails if this
file and the spec disagree.

Engine API version: 1.0.42
Tools: 101
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
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong"] | None = None,
        separator_pressures_mpa: list[float] | None = None,
        separator_temperatures_k: list[float] | None = None,
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
                separator_pressures_mpa=separator_pressures_mpa,
                separator_temperatures_k=separator_temperatures_k,
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
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong"] | None = None,
    ) -> dict[str, Any]:
        """Tune a composition to a target in-situ gas-oil volume ratio (m3/m3) at
        given P/T.

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
        """Source-tagged production allocation (back-allocation): solve the network
        and attribute the mass on every edge back to the sources it came from.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "allocate_production",
            _present(
                network_json=network_json,
            ),
        )

    def analyse_turbo_performance(
        self,
        *,
        points: list[dict[str, Any]],
        co2_fraction: float | None = None,
        composition: dict[str, Any] | None = None,
        gas_molecular_weight: float | None = None,
        h2s_fraction: float | None = None,
        methods: list[str] | None = None,
        mode: Literal["compressor", "expander"] | None = None,
        n2_fraction: float | None = None,
    ) -> dict[str, Any]:
        """Back-calculate head, efficiencies and powers from measured suction and
        discharge states by path method, and fit a turbo_machine map to the
        points.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "analyse_turbo_performance",
            _present(
                points=points,
                co2_fraction=co2_fraction,
                composition=composition,
                gas_molecular_weight=gas_molecular_weight,
                h2s_fraction=h2s_fraction,
                methods=methods,
                mode=mode,
                n2_fraction=n2_fraction,
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
        inlet_pressure: float,
        inlet_temperature: float,
        composition: dict[str, Any] | None = None,
        discharge_coefficient: float | None = None,
        dissolved_gas_ratio: float | None = None,
        gas_mw: float | None = None,
        gas_rate: float | None = None,
        oil_density: float | None = None,
        oil_rate: float | None = None,
        perry_multiplier: float | None = None,
        pipe_diameter_ratio: float | None = None,
        slip_model: Literal["no_slip", "gromles", "hydro", "constant_slip", "fauske", "moddy", "simpson", "thom", "baroczy", "lockhart_martenelli"] | None = None,
        water_rate: float | None = None,
        water_salinity: float | None = None,
    ) -> dict[str, Any]:
        """Pressure drop across a choke of known diameter at given rates (Sachdeva
        multiphase model, critical/subcritical).

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "calculate_choke_pressure_drop",
            _present(
                choke_diameter=choke_diameter,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                composition=composition,
                discharge_coefficient=discharge_coefficient,
                dissolved_gas_ratio=dissolved_gas_ratio,
                gas_mw=gas_mw,
                gas_rate=gas_rate,
                oil_density=oil_density,
                oil_rate=oil_rate,
                perry_multiplier=perry_multiplier,
                pipe_diameter_ratio=pipe_diameter_ratio,
                slip_model=slip_model,
                water_rate=water_rate,
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
        discharge_coefficient: float | None = None,
        dissolved_gor: float | None = None,
        gas_mw: float | None = None,
        oil_density: float | None = None,
        perry_multiplier: float | None = None,
        pipe_diameter_ratio: float | None = None,
        slip_model: Literal["no_slip", "gromles", "hydro", "constant_slip", "fauske", "moddy", "simpson", "thom", "baroczy", "lockhart_martenelli"] | None = None,
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
                discharge_coefficient=discharge_coefficient,
                dissolved_gor=dissolved_gor,
                gas_mw=gas_mw,
                oil_density=oil_density,
                perry_multiplier=perry_multiplier,
                pipe_diameter_ratio=pipe_diameter_ratio,
                slip_model=slip_model,
                water_salinity=water_salinity,
            ),
        )

    def calculate_compression_train(
        self,
        *,
        composition: dict[str, Any],
        inlet_pressure: float,
        inlet_temperature: float,
        stages: list[dict[str, Any]],
    ) -> dict[str, Any]:
        """Compression train on a composition: per stage a turbo machine, an
        intercooler on the EOS enthalpy and a scrubber that removes the condensed
        liquid; stage and train power, duty, liquid and compositions.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "calculate_compression_train",
            _present(
                composition=composition,
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                stages=stages,
            ),
        )

    def calculate_compressor(
        self,
        *,
        inlet_pressure: float,
        inlet_temperature: float,
        pressure_ratio: float,
        composition: dict[str, Any] | None = None,
        fluid: dict[str, Any] | None = None,
        isentropic_efficiency: float | None = None,
        mechanical_efficiency: float | None = None,
        method: str | None = None,
        polytropic_efficiency: float | None = None,
    ) -> dict[str, Any]:
        """Single-stage centrifugal compressor: outlet P/T and power from inlet P/T,
        pressure ratio, and polytropic efficiency.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "calculate_compressor",
            _present(
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                pressure_ratio=pressure_ratio,
                composition=composition,
                fluid=fluid,
                isentropic_efficiency=isentropic_efficiency,
                mechanical_efficiency=mechanical_efficiency,
                method=method,
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

    def calculate_critical_point(
        self,
        *,
        mole_fractions: list[float],
        binary_interactions: list[dict[str, Any]] | None = None,
        component_names: list[str] | None = None,
        components: list[dict[str, Any]] | None = None,
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong"] | None = None,
    ) -> dict[str, Any]:
        """True critical point of a mixture (Heidemann-Khalil) with a cubic EOS:
        critical temperature, pressure, molar volume and Z-factor.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "calculate_critical_point",
            _present(
                mole_fractions=mole_fractions,
                binary_interactions=binary_interactions,
                component_names=component_names,
                components=components,
                eos_model=eos_model,
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
        """API RP 14E erosional velocity limit (1.22 C / sqrt(rho_mix) in SI), the
        actual mixture velocity for a pipe, and the flow at which the pipe reaches
        the limit.

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
        condensate_gas_ratio_stb_per_mmscf: float,
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
                condensate_gas_ratio_stb_per_mmscf=condensate_gas_ratio_stb_per_mmscf,
                gas_gravity=gas_gravity,
                oil_api_gravity=oil_api_gravity,
                temperature=temperature,
            ),
        )

    def calculate_heater_cooler(
        self,
        *,
        inlet_pressure: float,
        inlet_temperature: float,
        mode: Literal["fixed_duty", "fixed_outlet_temperature", "approach_temperature", "ua"],
        approach_temperature: float | None = None,
        composition: dict[str, Any] | None = None,
        fluid: dict[str, Any] | None = None,
        heat_duty: float | None = None,
        max_duty: float | None = None,
        outlet_temperature: float | None = None,
        pressure_drop: float | None = None,
        ua: float | None = None,
        utility_temperature: float | None = None,
    ) -> dict[str, Any]:
        """Heater/cooler on the network edge's energy balance, in fixed_duty,
        fixed_outlet_temperature, approach_temperature or ua mode; returns outlet
        P/T and duty.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "calculate_heater_cooler",
            _present(
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                mode=mode,
                approach_temperature=approach_temperature,
                composition=composition,
                fluid=fluid,
                heat_duty=heat_duty,
                max_duty=max_duty,
                outlet_temperature=outlet_temperature,
                pressure_drop=pressure_drop,
                ua=ua,
                utility_temperature=utility_temperature,
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
        """Hydrate formation temperature at a given pressure and gas gravity from the
        Towler-Mokhatab screening correlation; H2S and CO2 inputs are echoed, not
        applied.

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
        inlet_pressure: float,
        inlet_temperature: float,
        outlet_pressure: float,
        composition: dict[str, Any] | None = None,
        fluid: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        """Outlet temperature after a constant-enthalpy (Joule-Thomson) expansion to
        a lower pressure.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "calculate_isenthalpic_temperature",
            _present(
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                outlet_pressure=outlet_pressure,
                composition=composition,
                fluid=fluid,
            ),
        )

    def calculate_jt_valve(
        self,
        *,
        inlet_pressure: float,
        inlet_temperature: float,
        pressure_drop: float,
        composition: dict[str, Any] | None = None,
        fluid: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        """Joule-Thomson throttle valve: outlet T after an isenthalpic pressure drop.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "calculate_jt_valve",
            _present(
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                pressure_drop=pressure_drop,
                composition=composition,
                fluid=fluid,
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
        """Multiphase-flow-meter allocation: convert one meter reading (in-situ) to
        standard-condition rates with an EOS.

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
        inlet_pressure: float,
        inlet_temperature: float,
        stages: list[dict[str, Any]],
        composition: dict[str, Any] | None = None,
        fluid: dict[str, Any] | None = None,
        intercool_pressure_drop: float | None = None,
        intercool_temperature: float | None = None,
        mechanical_efficiency: float | None = None,
        method: str | None = None,
    ) -> dict[str, Any]:
        """Multi-stage centrifugal train with optional inter-stage cooling; per-stage
        pressure ratios and overall discharge P/T, cooler duty and shaft power.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "calculate_multistage_compressor",
            _present(
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                stages=stages,
                composition=composition,
                fluid=fluid,
                intercool_pressure_drop=intercool_pressure_drop,
                intercool_temperature=intercool_temperature,
                mechanical_efficiency=mechanical_efficiency,
                method=method,
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
        flow_correlation: Literal["Aziz", "Baxendell-Thomas", "Beggs-Brill", "CHAOS", "Dukler", "Duns-Ros", "Fancher-Brown", "GOAT", "Gray", "Griffith-Wallis", "Hagedorn-Slug", "KISS", "Mist", "ml_tuned", "Mukherjee-Brill", "Poettmann-Carpenter", "Default", "QC-high", "QC-low", "SinglePhaseGas", "SUPREME"] | None = None,
        gas_mw: float | None = None,
        oil_density: float | None = None,
        tubing_angle: float | None = None,
        water_salinity: float | None = None,
    ) -> dict[str, Any]:
        """Operating point = IPR intersect VLP for one well and one tubing run.
        Generates both curves and finds the stabilised rate and flowing BHP.

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
        flow_boundary: Literal["top", "bottom"] | None = None,
        flow_correlation: Literal["Aziz", "Baxendell-Thomas", "Beggs-Brill", "CHAOS", "Dukler", "Duns-Ros", "Fancher-Brown", "GOAT", "Gray", "Griffith-Wallis", "Hagedorn-Slug", "KISS", "Mist", "ml_tuned", "Mukherjee-Brill", "Poettmann-Carpenter", "Default", "QC-high", "QC-low", "SinglePhaseGas", "SUPREME"] | None = None,
        gas_mw: float | None = None,
        heat_transfer_coefficient: float | None = None,
        oil_density: float | None = None,
        pressure_boundary: Literal["top", "bottom"] | None = None,
        surrounding_temperature: float | None = None,
        water_salinity: float | None = None,
    ) -> dict[str, Any]:
        """March pressure and temperature along a single straight pipe (one diameter,
        length and angle) with heat transfer.

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
        correlation: Literal["Aziz", "Baxendell-Thomas", "Beggs-Brill", "CHAOS", "Dukler", "Duns-Ros", "Fancher-Brown", "GOAT", "Gray", "Griffith-Wallis", "Hagedorn-Slug", "KISS", "Mist", "ml_tuned", "Mukherjee-Brill", "Poettmann-Carpenter", "Default", "QC-high", "QC-low", "SinglePhaseGas", "SUPREME"],
        density: list[float],
        diameter: float,
        ift: float,
        pressure: float,
        roughness: float,
        velocity: list[float],
        viscosity: list[float],
    ) -> dict[str, Any]:
        """Multiphase pressure gradient at one point in a pipe using a chosen
        correlation. Returns gradient components, holdup, flow regime and
        hydraulics.

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

    def calculate_pump(
        self,
        *,
        inlet_pressure: float,
        inlet_temperature: float,
        centrifugal: dict[str, Any] | None = None,
        composition: dict[str, Any] | None = None,
        efficiency: float | None = None,
        fluid: dict[str, Any] | None = None,
        head_curve: list[list[float]] | None = None,
        mechanical_efficiency: float | None = None,
        minor_loss_coefficient: float | None = None,
        nozzle_diameter: float | None = None,
        npsh_required: float | None = None,
    ) -> dict[str, Any]:
        """Centrifugal pump or ESP at one suction state on the network pump model:
        discharge P/T, head, power, NPSH, per-section operating range and the ESP
        drive train.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "calculate_pump",
            _present(
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                centrifugal=centrifugal,
                composition=composition,
                efficiency=efficiency,
                fluid=fluid,
                head_curve=head_curve,
                mechanical_efficiency=mechanical_efficiency,
                minor_loss_coefficient=minor_loss_coefficient,
                nozzle_diameter=nozzle_diameter,
                npsh_required=npsh_required,
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
        """Single head-curve lookup: discharge pressure from one head-vs-rate curve
        read at one flow with one liquid density. No stages, speed, gas or power
        model; see calculate_pump.

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
        discharge_coefficient: float | None = None,
        dissolved_gor: float | None = None,
        fixed: str | None = None,
        free_gas_rate: float | None = None,
        gas_mw: float | None = None,
        gor: float | None = None,
        oil_density: float | None = None,
        oil_rate: float | None = None,
        perry_multiplier: float | None = None,
        pipe_diameter_ratio: float | None = None,
        slip_model: Literal["no_slip", "gromles", "hydro", "constant_slip", "fauske", "moddy", "simpson", "thom", "baroczy", "lockhart_martenelli"] | None = None,
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
                discharge_coefficient=discharge_coefficient,
                dissolved_gor=dissolved_gor,
                fixed=fixed,
                free_gas_rate=free_gas_rate,
                gas_mw=gas_mw,
                gor=gor,
                oil_density=oil_density,
                oil_rate=oil_rate,
                perry_multiplier=perry_multiplier,
                pipe_diameter_ratio=pipe_diameter_ratio,
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
        composition: dict[str, Any] | None = None,
        gas_molecular_weight: float | None = None,
        h2s_fraction: float | None = None,
        max_pressure_ratio: float | None = None,
        mechanical_efficiency: float | None = None,
        method: str | None = None,
        min_pressure_ratio: float | None = None,
        n2_fraction: float | None = None,
        polytropic_efficiency: float | None = None,
    ) -> dict[str, Any]:
        """Reciprocating (positive-displacement) compressor. Mass flow is set by
        displacement x speed x volumetric efficiency, not supplied.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
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
                composition=composition,
                gas_molecular_weight=gas_molecular_weight,
                h2s_fraction=h2s_fraction,
                max_pressure_ratio=max_pressure_ratio,
                mechanical_efficiency=mechanical_efficiency,
                method=method,
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
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong"] | None = None,
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
        mole_fractions: list[float],
        algorithm: Literal["classical", "bell_jaeger", "multi_start", "envelope"] | None = None,
        binary_interactions: list[dict[str, Any]] | None = None,
        boundary: Literal["bubble", "dew"] | None = None,
        component_names: list[str] | None = None,
        components: list[dict[str, Any]] | None = None,
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong"] | None = None,
        pressure_mpa: float | None = None,
        temperature_k: float | None = None,
    ) -> dict[str, Any]:
        """Bubble- or dew-point pressure of a composition at a temperature, or
        bubble- or dew-point temperature at a pressure (cubic EOS).

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "calculate_saturation_pressure",
            _present(
                mole_fractions=mole_fractions,
                algorithm=algorithm,
                binary_interactions=binary_interactions,
                boundary=boundary,
                component_names=component_names,
                components=components,
                eos_model=eos_model,
                pressure_mpa=pressure_mpa,
                temperature_k=temperature_k,
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
        composition: dict[str, Any] | None = None,
        gas_molecular_weight: float | None = None,
        h2s_fraction: float | None = None,
        mechanical_efficiency: float | None = None,
        method: str | None = None,
        n2_fraction: float | None = None,
        polytropic_efficiency: float | None = None,
        subtype: str | None = None,
        volumetric_efficiency: float | None = None,
    ) -> dict[str, Any]:
        """Screw (positive-displacement) compressor closed by black-box efficiencies;
        mass flow = volumetric efficiency x suction density x displacement per
        revolution x shaft speed in rev/s.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
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
                composition=composition,
                gas_molecular_weight=gas_molecular_weight,
                h2s_fraction=h2s_fraction,
                mechanical_efficiency=mechanical_efficiency,
                method=method,
                n2_fraction=n2_fraction,
                polytropic_efficiency=polytropic_efficiency,
                subtype=subtype,
                volumetric_efficiency=volumetric_efficiency,
            ),
        )

    def calculate_turbine(
        self,
        *,
        inlet_pressure: float,
        inlet_temperature: float,
        pressure_ratio: float,
        composition: dict[str, Any] | None = None,
        fluid: dict[str, Any] | None = None,
        isentropic_efficiency: float | None = None,
        mechanical_efficiency: float | None = None,
        method: str | None = None,
        polytropic_efficiency: float | None = None,
    ) -> dict[str, Any]:
        """Single-stage centrifugal turbine/expander: outlet P/T and power generated
        from inlet P/T and an expansion pressure ratio (0 < PR < 1).

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "calculate_turbine",
            _present(
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                pressure_ratio=pressure_ratio,
                composition=composition,
                fluid=fluid,
                isentropic_efficiency=isentropic_efficiency,
                mechanical_efficiency=mechanical_efficiency,
                method=method,
                polytropic_efficiency=polytropic_efficiency,
            ),
        )

    def calculate_turbo_machine(
        self,
        *,
        inlet_pressure: float,
        inlet_temperature: float,
        turbo_machine: dict[str, Any],
        composition: dict[str, Any] | None = None,
        discharge_pressure: float | None = None,
        fluid: dict[str, Any] | None = None,
        mechanical_efficiency: float | None = None,
        mode: Literal["compressor", "expander"] | None = None,
    ) -> dict[str, Any]:
        """Any network turbo machine (centrifugal simple/mapped/map/multistage,
        axial, screw, reciprocating) at one operating point, on a GERG gas or an
        EOS composition, with a chosen path method.

        Costs 1 credit, plus 1 per 250 ms beyond the first 0.25 s of compute.
        """
        return self.call(
            "calculate_turbo_machine",
            _present(
                inlet_pressure=inlet_pressure,
                inlet_temperature=inlet_temperature,
                turbo_machine=turbo_machine,
                composition=composition,
                discharge_pressure=discharge_pressure,
                fluid=fluid,
                mechanical_efficiency=mechanical_efficiency,
                mode=mode,
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
        """Brine properties (density, viscosity, compressibility, heat capacity,
        enthalpy) with a salinity correction.

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
        method: Literal["kesler_lee", "twu", "sancet"] | None = None,
        molecular_weight: float | None = None,
        specific_gravity: float | None = None,
    ) -> dict[str, Any]:
        """Critical properties (Tc, Pc, Vc), acentric factor and Watson K of a
        pseudo-component from any two of MW / specific gravity / boiling point.

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
        correlations: list[Literal["Aziz", "Baxendell-Thomas", "Beggs-Brill", "CHAOS", "Dukler", "Duns-Ros", "Fancher-Brown", "GOAT", "Gray", "Griffith-Wallis", "Hagedorn-Slug", "KISS", "Mist", "ml_tuned", "Mukherjee-Brill", "Poettmann-Carpenter", "Default", "QC-high", "QC-low", "SinglePhaseGas", "SUPREME"]] | None = None,
        reference_measured_depth: float | None = None,
        reference_true_vertical_depth: float | None = None,
        reference_x: float | None = None,
        reference_y: float | None = None,
        reference_z: float | None = None,
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
                reference_measured_depth=reference_measured_depth,
                reference_true_vertical_depth=reference_true_vertical_depth,
                reference_x=reference_x,
                reference_y=reference_y,
                reference_z=reference_z,
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
        correlations: list[Literal["Aziz", "Baxendell-Thomas", "Beggs-Brill", "CHAOS", "Dukler", "Duns-Ros", "Fancher-Brown", "GOAT", "Gray", "Griffith-Wallis", "Hagedorn-Slug", "KISS", "Mist", "ml_tuned", "Mukherjee-Brill", "Poettmann-Carpenter", "Default", "QC-high", "QC-low", "SinglePhaseGas", "SUPREME"]] | None = None,
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
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong"] | None = None,
        separator_pressures_mpa: list[float] | None = None,
        separator_temperatures_k: list[float] | None = None,
    ) -> dict[str, Any]:
        """Flash a composition through a separator train to stock-tank: GOR, oil
        density, specific gravity and API gravity, gas gravity.

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
                separator_pressures_mpa=separator_pressures_mpa,
                separator_temperatures_k=separator_temperatures_k,
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
        point. Required inputs depend on ipr_model (19 models).

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
        mole_fractions: list[float],
        binary_interactions: list[dict[str, Any]] | None = None,
        component_names: list[str] | None = None,
        components: list[dict[str, Any]] | None = None,
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong"] | None = None,
    ) -> dict[str, Any]:
        """Two-phase P-T envelope (dew/bubble locus + critical point) of a
        composition (Peng-Robinson or SRK).

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "generate_phase_envelope",
            _present(
                mole_fractions=mole_fractions,
                binary_interactions=binary_interactions,
                component_names=component_names,
                components=components,
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
        a transient ipr_model, with the dimensionless groups derived from the
        geometry.

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
        """Critical properties (Tc, Pc in MPa, omega, MW) of a single EOS component
        by database name or short code.

        Free.
        """
        return self.call(
            "get_eos_component",
            _present(
                name=name,
            ),
        )

    def import_prp_fluid(
        self,
        *,
        prp_text: str,
    ) -> dict[str, Any]:
        """Import a PVTsim .prp fluid file (text) as a composition for the PVT,
        process-graph and MPFM tools.

        Costs 1 credit.
        """
        return self.call(
            "import_prp_fluid",
            _present(
                prp_text=prp_text,
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
        """List the network edge types by name (no_pressure_loss, pipe, choke,
        compressor, ...) and the fields of their data blocks.

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
        """List the component names in the built-in equation-of-state database and
        the short codes (C1, CO2, iC4, ...) the compositional tools also accept.

        Free.
        """
        return self.call(
            "list_eos_components",
            {},
        )

    def list_flow_assurance_models(
        self,
    ) -> dict[str, Any]:
        """The model vocabularies the flow-assurance tools accept — hydrate, wax,
        asphaltene, corrosion and scale models with their tier, plus how wax
        formers are identified, inhibitor names, vdWP guests, the scale risk
        bands, the NACE MR0175 regions and the API RP 14E service classes.

        Free.
        """
        return self.call(
            "list_flow_assurance_models",
            {},
        )

    def list_fluid_types(
        self,
    ) -> dict[str, Any]:
        """List the black-oil fluid types (oil, gas, water): configuration fields,
        correlation names and defaults, and the reported property keys with units.

        Free.
        """
        return self.call(
            "list_fluid_types",
            {},
        )

    def list_node_types(
        self,
    ) -> dict[str, Any]:
        """List the network node types by name (fixed_rate_source,
        fixed_pressure_source, pressure_dependent_source, network_node,
        fixed_pressure_sink, fixed_rate_sink) and their fields.

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
        """Fit A and B of the fetkovich_ab oil deliverability equation q = A·(pr −
        psat) + B·(psat² − pwf²) to test points.

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
        """Fit A and B of the forchheimer_ab gas deliverability equation pr² − pwf² =
        A·q + B·q² to test points.

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
        match_target: Literal["productivity_index", "skin", "forchheimer_ab", "fetkovich_ab"] | None = None,
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
        """Fit the pi model's productivity index to observed (oil rate, flowing-BHP)
        test points.

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
        """Fit the mechanical skin of a darcy or fetkovich model to observed (oil
        rate, flowing-BHP) test points.

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
        minimise an objective (phase rate, revenue, pressure drop, power,
        deviation from measurements) under rate / pressure / temperature /
        velocity / power / pump / resource constraints. The optimisation
        counterpart of solve_network.

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "optimise_network",
            _present(
                optimisation_json=optimisation_json,
            ),
        )

    def partition_acid_gas_in_water(
        self,
        *,
        pressure: float,
        temperature: float,
        water_analysis: dict[str, Any],
        co2_fugacity_bar: float | None = None,
        co2_mass_rate: float | None = None,
        co2_mole_fraction: float | None = None,
        h2s_fugacity_bar: float | None = None,
        h2s_mass_rate: float | None = None,
        h2s_mole_fraction: float | None = None,
        ph: float | None = None,
        water_mass_rate: float | None = None,
    ) -> dict[str, Any]:
        """How much CO2 and H2S is dissolved in the produced water, and the in-situ
        pH that leaves — the brine pH the corrosion models should consume instead
        of a condensed-water estimate.

        Costs 2 credits.
        """
        return self.call(
            "partition_acid_gas_in_water",
            _present(
                pressure=pressure,
                temperature=temperature,
                water_analysis=water_analysis,
                co2_fugacity_bar=co2_fugacity_bar,
                co2_mass_rate=co2_mass_rate,
                co2_mole_fraction=co2_mole_fraction,
                h2s_fugacity_bar=h2s_fugacity_bar,
                h2s_mass_rate=h2s_mass_rate,
                h2s_mole_fraction=h2s_mole_fraction,
                ph=ph,
                water_mass_rate=water_mass_rate,
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
        binary_interactions: list[dict[str, Any]] | None = None,
        component_names: list[str] | None = None,
        components: list[dict[str, Any]] | None = None,
        enthalpy_j_per_mol: float | None = None,
        entropy_j_per_mol_k: float | None = None,
        eos_model: Literal["PengRobinson", "SoaveRedlichKwong", "Gerg2008"] | None = None,
        flash_type: Literal["pt", "ph", "ps"] | None = None,
        gas_viscosity_model: Literal["auto", "lucas", "lbc", "wilke", "herning_zipperer", "chapman_enskog", "pedersen", "trapp", "expanded_fluid", "burgoyne_nielsen_stanko"] | None = None,
        include_properties: bool | None = None,
        liquid_viscosity_model: Literal["auto", "lucas", "lbc", "wilke", "herning_zipperer", "chapman_enskog", "pedersen", "trapp", "expanded_fluid", "burgoyne_nielsen_stanko"] | None = None,
        temperature_k: float | None = None,
    ) -> dict[str, Any]:
        """EOS flash of a composition (Peng-Robinson default, SRK, or GERG-2008 for
        single-phase gas): PT, PH or PS; phase split, K-values, phase densities,
        and optionally each phase's viscosity, enthalpy, entropy and heat
        capacities. Pressure in MPa.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "run_eos_flash",
            _present(
                mole_fractions=mole_fractions,
                pressure_mpa=pressure_mpa,
                binary_interactions=binary_interactions,
                component_names=component_names,
                components=components,
                enthalpy_j_per_mol=enthalpy_j_per_mol,
                entropy_j_per_mol_k=entropy_j_per_mol_k,
                eos_model=eos_model,
                flash_type=flash_type,
                gas_viscosity_model=gas_viscosity_model,
                include_properties=include_properties,
                liquid_viscosity_model=liquid_viscosity_model,
                temperature_k=temperature_k,
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

    def run_gas_depletion(
        self,
        *,
        experiment_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Gas-reservoir depletion study: p/z, Bg, recovery factor and retrograde
        liquid per pressure step.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "run_gas_depletion",
            _present(
                experiment_json=experiment_json,
            ),
        )

    def run_material_balance(
        self,
        *,
        reservoir_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Tank material balance over one or more reservoir zones (STOIIP/GIIP,
        cumulative production, pressure decline, recovery factor); no aquifer.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "run_material_balance",
            _present(
                reservoir_json=reservoir_json,
            ),
        )

    def run_mmp_probe(
        self,
        *,
        experiment_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Mixing-cell miscibility test at one pressure: forward and backward contact
        series and whether either reached miscibility.

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "run_mmp_probe",
            _present(
                experiment_json=experiment_json,
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

    def run_nodal_study(
        self,
        *,
        nodal_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Nodal analysis of one well inside a network: IPR sweep, VLP by full
        network solves, their operating point, and optional sensitivity overlays.

        Costs 100 credits, plus 1 per 100 ms beyond the first 10 s of compute.
        """
        return self.call(
            "run_nodal_study",
            _present(
                nodal_json=nodal_json,
            ),
        )

    def run_parametric_study(
        self,
        *,
        study_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Sensitivity / parametric study over a network (single-variable sweep,
        tornado, two-factor grid, Monte Carlo, envelope map); returns every run
        plus the study's summary.

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
        """Steady-state compositional flowsheet of Source, Separator, Mixer, Splitter
        and Sink nodes (no heaters or compressors), with recycle loops.

        Costs 25 credits, plus 1 per 250 ms beyond the first 5 s of compute.
        """
        return self.call(
            "run_process_graph",
            _present(
                process_graph_json=process_graph_json,
            ),
        )

    def run_pvt_regression_suite(
        self,
        *,
        experiment_json: dict[str, Any] | list[Any] | str,
    ) -> dict[str, Any]:
        """Run several PVT experiments on one EOS fluid against lab data: per-field
        residuals and a weighted objective (evaluation, not tuning).

        Costs 5 credits, plus 1 per 500 ms beyond the first 2 s of compute.
        """
        return self.call(
            "run_pvt_regression_suite",
            _present(
                experiment_json=experiment_json,
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
        Flux): the general-purpose scheme, carrying temperature, the oil/water
        split and salinity.

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

    def screen_scale_risk(
        self,
        *,
        pressure: float,
        temperature: float,
        water_analysis: dict[str, Any],
        co2_fugacity_bar: float | None = None,
        co2_mole_fraction: float | None = None,
        h2s_fugacity_bar: float | None = None,
        h2s_mole_fraction: float | None = None,
        minerals: list[Literal["calcite", "aragonite", "siderite", "barite", "celestite", "gypsum", "anhydrite", "halite"]] | None = None,
        ph: float | None = None,
        water_mass_rate: float | None = None,
    ) -> dict[str, Any]:
        """Mineral-scale saturation indices from a produced-water ion analysis —
        calcite, aragonite, siderite, barite, celestite, gypsum, anhydrite and
        halite — with the precipitable mass and the limiting ion.

        Costs 2 credits.
        """
        return self.call(
            "screen_scale_risk",
            _present(
                pressure=pressure,
                temperature=temperature,
                water_analysis=water_analysis,
                co2_fugacity_bar=co2_fugacity_bar,
                co2_mole_fraction=co2_mole_fraction,
                h2s_fugacity_bar=h2s_fugacity_bar,
                h2s_mole_fraction=h2s_mole_fraction,
                minerals=minerals,
                ph=ph,
                water_mass_rate=water_mass_rate,
            ),
        )

    def screen_wax_risk(
        self,
        *,
        temperature: float,
        activity_model: Literal["regular_solution", "ideal"] | None = None,
        component_names: list[str] | None = None,
        density_c7_plus: float | None = None,
        enthalpies_of_fusion: list[float] | None = None,
        enthalpies_of_transition: list[float] | None = None,
        heat_capacity_correction: bool | None = None,
        heat_capacity_of_fusion_j_per_mol_k: list[float] | None = None,
        liquid_molar_volume_cm3_per_mol: list[float] | None = None,
        liquid_solubility_parameter_mpa_half: list[float] | None = None,
        melting_points: list[float] | None = None,
        model: Literal["screening", "won", "sle"] | None = None,
        mole_fractions: list[float] | None = None,
        molecular_weights: list[float] | None = None,
        mw_c7_plus: float | None = None,
        paraffin_mass_fraction: float | None = None,
        solid_molar_volume_ratio: float | None = None,
        transition_temperatures: list[float] | None = None,
        watson_k: float | None = None,
        wax_former_mask: list[bool] | None = None,
    ) -> dict[str, Any]:
        """Wax appearance temperature and margin over three tiers: a C7+ screening
        correlation, the Won multi-solid SLE, or the non-ideal SLE flash with per-
        component solid fractions, measured melting data and the solubility
        gradient for the deposition rate.

        Costs 2 credits.
        """
        return self.call(
            "screen_wax_risk",
            _present(
                temperature=temperature,
                activity_model=activity_model,
                component_names=component_names,
                density_c7_plus=density_c7_plus,
                enthalpies_of_fusion=enthalpies_of_fusion,
                enthalpies_of_transition=enthalpies_of_transition,
                heat_capacity_correction=heat_capacity_correction,
                heat_capacity_of_fusion_j_per_mol_k=heat_capacity_of_fusion_j_per_mol_k,
                liquid_molar_volume_cm3_per_mol=liquid_molar_volume_cm3_per_mol,
                liquid_solubility_parameter_mpa_half=liquid_solubility_parameter_mpa_half,
                melting_points=melting_points,
                model=model,
                mole_fractions=mole_fractions,
                molecular_weights=molecular_weights,
                mw_c7_plus=mw_c7_plus,
                paraffin_mass_fraction=paraffin_mass_fraction,
                solid_molar_volume_ratio=solid_molar_volume_ratio,
                transition_temperatures=transition_temperatures,
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
        """Data-reconciliation solve (MAP): reconcile pressure gauges and rate
        meters, each with a variance, against the network physics and estimate
        uncertain source inputs, returning posterior variances and a per-
        measurement misfit.

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
