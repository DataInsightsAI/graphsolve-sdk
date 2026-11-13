# Transient runs and flow stability

The engine has three transient multiphase schemes and a point tool that screens a
well or line for the behaviour a transient run would show. This page says which
to call and how to read the screen. Pressures are MPa, rates kSm³/day, pipe
angles degrees from vertical along the flow (0 up, 90 level).

## Which transient scheme

All three take the same request (`{"transient_json": …}`) and run as
[jobs](jobs.md). `list_transient_solvers` is free and lists what each carries.

| Tool | Scheme | Use it for |
|---|---|---|
| `run_transient_wave` | explicit drift-flux mixture; resolves pressure waves and reports the sound speed; isothermal | valve slam, surge on a closing choke, blowdown front timing: seconds to minutes |
| `run_transient_flux` | semi-implicit three-fluid (gas with drops, oil, water); energy balance; salinity tracer | a single line over hours: turndown and ramp-up, terrain and severe slugging, shut-in and restart, thermal and salinity fronts |
| `run_transient_field` | fully implicit six-field; energy balance; dissolved gas as an unknown | wells on an inflow table or model, gas lift through a valve and annulus, separators, an upstream gas volume, networks of branches, falling-film slugging, slug capturing, slug tracking on a long line |

As a rule:

1. If the pressure wave is the answer, use **Wave**.
2. Otherwise, if the case needs a well, gas lift, a separator, a gas volume
   upstream or several branches, use **Field**.
3. Otherwise, use **Flux** first. Confirm a slugging conclusion on Field where
   liquid falling back drives the cycle: the two bracket the slug period and
   amplitude.

Wave's step is set by the sound speed, so it is the most expensive per hour of
simulated time by a wide margin. Field costs about what Flux costs on a steady
line and several times more on a slugging one.

## Closure studies

A transient result is the closure's structure at each point, the scheme that
carries it, and the boundary that drives it. The `closure` block on every
scheme's request lets a study move one mechanism of the closure at a time, so a
simulated phenomenon can be traced to its cause or that cause ruled out. Every
field defaults to the closure as calibrated: a run without the block is the
released model, and moving a field is a study, not a calibration. The
`trickle_detail` screen takes the same fields under `options`, so a screen at a
run's `closure` reads the structure the run carries.

The protocol: run the baseline and record the measures the phenomenon is judged
by (period, rise per cycle, rise share, mean and standard deviation of the
pressure, trough and peak of the rates over their means, the share of time the
liquid runs above its mean); screen the points that matter at the baseline's
closure with `regime_screens: false` and read the regime weights, the slip and
the flags; move one field through its range in two or three steps and compare
the same measures with the grid, steps and record interval held fixed; report
the before and after. A field whose whole range leaves the measures where they
were has ruled its mechanism out; one that moves them has found it, and its
calibration is then a question for the data the closure was calibrated on.

The fields by mechanism, with defaults and ranges (a value out of range, or on
a scheme that does not take it, is refused by name):

| Mechanism | Fields |
| --- | --- |
| The slug unit: body, bubble, no-slip limit | `malnes_coefficient` (83, 20 to 300: lower is a more aerated body), `bubble_c0` (1.2, 1 to 1.5: the bubble nose's speed), `bubble_drift_coefficient` (0.35, 0.1 to 1: the bubble's rise in inclined flow), `body_gas_drift_m_s` (0.2, 0 to 1: the bubbles' drift through the body), `noslip_froude` ([4, 18], 0 < lo < hi: at the default a fast riser is a no-slip mixture; raised, say [40, 180], the unit keeps its slip and its film falls back), `regime_screens` (false) |
| The film: reversal, loading, entrainment | `annular_onset` ([1, 1.5], 0.5 to 3: the film-reversal band over which the gas comes to carry the film), `taylor_film_friction` (0.005, 0.001 to 0.05), `dense_gas_film_friction` (30, 0 to 100: what moves the loading onset down with pressure), `entrainment_cap` (0.35, 0 to 0.9: how much liquid rides as drops at gas speed), `distributed_limit` ([1.5, 3], 1 to 10), `film_lift_angle_floor` (0.3, 0.05 to 1); Flux `deposition_coefficient` (2.3e-4, > 0) and Field `drop_relaxation_transits` (1, > 0) |
| The stratified layer: waves and growth | `kh_band` ([0.8, 1.25], 0 < lo < hi: where a layer goes to slugs), `wave_friction` ({coefficient 15, onset_velocity_m_s 5, density_cap 4}, each > 0: the large-wave interfacial friction that sets the layer's holdup) |
| Oil and water | `emulsion` (true), `oil_water_slip` (true), `inversion_water_cut` (0 < v < 1), `oil_water_interface` (`faster_layer` or `wavy`), `dispersed_cap` (1, 0 to 1) |
| Solver-side blend | `zone_band` ([0.001, 0.05], Field and Flux: the holdup band below which a thin liquid zone rides its carrier) |

A sweep, one field at a time, comparing one measure:

```python
import numpy as np

def swing(run):
    rec = run["recording"]
    p = np.array(rec["series"][0]["values"]).reshape(-1, rec["n_cells"])
    top = p[len(p) // 2:, 0] * 10.0  # outlet cell, second half of the run, bar
    return np.percentile(top, 95) - np.percentile(top, 5)

print("baseline", swing(client.run_transient_field(request)))
for field, values in {
    "noslip_froude": [[10, 30], [40, 180]],
    "entrainment_cap": [0.1, 0.6],
    "annular_onset": [[0.7, 1.0], [1.3, 2.0]],
}.items():
    for v in values:
        run = client.run_transient_field({**request, "closure": {field: v}})
        print(field, v, swing(run))
```

With `results.convergence` set, the response's `convergence` holds each step's
largest holdup, pressure and temperature change, so a study that moved the step
can be told from one that moved the physics. The grid, the steps, the boundary
and the geometry are not closure items: hold them through a closure study and
vary them in their own. The full page, with a worked riser case, is the docs
site's *Closure studies*.

## Slug tracking on a long line

Slug capturing needs cells of about one pipe diameter, so it stops at a few
kilometres. Field's `slug_tracking` block keeps cells of tens of diameters and
carries the slugs as objects: where the flow structure holds a slug unit the
grid runs the separated flow (the film) and each body is a liquid-full stretch
at the unit's body holdup, its tail at Bendiksen's bubble velocity, laid out
again on the grid whenever its nose crosses a cell. Bodies are born where the
unit begins, the line starts with a train of them (`train_unit_length_m`,
default 300 m), and a body entering a stretch without a unit, a riser say, is
released to the grid's own closure. The response adds `tracked_slugs` with the
events (born, merged, died, left, released) and every body's front, tail,
velocities and mean holdup at each record. It runs at about 1 s steps on a 20 km
line at 10 m cells, several times the cost of a plain run, and only on a single
line.

Where a choke closes the line, `outlet.downstream_resistance: {"gas", "liquid"}`
(Pa per Sm³/s) makes the choke discharge against `outlet_pressure` plus the
resistances times the rates through it, which is what a pressure measured after
a choke that rises with the throughput fits.

```python
run = client.run_transient_field({
    **request,
    "slug_tracking": {"train_unit_length_m": 300},
    "boundary": {**request["boundary"], "outlet": {
        "type": "choke", "area": 0.00513, "cd": 0.78,
        "downstream_resistance": {"gas": 1.95e4, "liquid": 4.59e6},
    }},
})
for event in run["tracked_slugs"]["events"]:
    if event["kind"] == "released":
        print(event["t_s"], event["x_m"], event["length_m"])
```

## Screening with `trickle_detail`

`trickle_detail` solves the TRICKLE flow structure at one point (the structure
every transient scheme reads at each cell). It scans the gas rate at the
point's liquid rates and names, in `stability.flags`, the conditions it finds.
It costs 1 credit.

=== "Python"

    ```python
    from graphsolve import GraphSolve

    gs = GraphSolve()
    detail = gs.trickle_detail(
        pressure=10.0,                       # MPa
        diameter=0.1,                        # m
        roughness=4.5e-5,                    # m, absolute
        angle=0.0,                           # degrees from vertical; 0 is vertical upflow
        density=[80.0, 750.0, 1030.0],       # kg/m3, [gas, oil, water]
        viscosity=[1.5e-5, 2.0e-3, 5.0e-4],  # Pa.s
        velocity=[2.0, 1.0, 0.3],            # m/s superficial
        ift=0.02,                            # N/m, gas-oil
        ift_gas_water=0.06,                  # N/m
        ift_oil_water=0.03,                  # N/m
    )["result"]
    print(detail["stability"]["flags"], detail["scan"]["gradient_minimum"])
    ```

=== "TypeScript"

    ```ts
    const { result } = await gs.trickle_detail({
      pressure: 10.0,
      diameter: 0.1,
      roughness: 4.5e-5,
      angle: 0.0,
      density: [80.0, 750.0, 1030.0],
      viscosity: [1.5e-5, 2.0e-3, 5.0e-4],
      velocity: [2.0, 1.0, 0.3],
      ift: 0.02,
      ift_gas_water: 0.06,
      ift_oil_water: 0.03,
    });
    ```

=== "Rust"

    ```rust
    use graphsolve::TrickleDetailParams;

    let response = gs
        .trickle_detail(TrickleDetailParams {
            pressure: 10.0,
            diameter: 0.1,
            roughness: 4.5e-5,
            angle: 0.0,
            density: vec![80.0, 750.0, 1030.0],
            viscosity: vec![1.5e-5, 2.0e-3, 5.0e-4],
            velocity: vec![2.0, 1.0, 0.3],
            ift: 0.02,
            ift_gas_water: Some(0.06),
            ift_oil_water: Some(0.03),
            ..Default::default()
        })
        .await?;
    let flags = &response.result["stability"]["flags"];
    ```

For an oil, `calculate_fluid_properties` gives the three tensions as `iftog`,
`iftgw` and `iftow`. Leave out `ift_gas_water` on a watery stream and the tool
takes the oil's tension for the whole liquid, which understates it.

### Reading the flags

A flag says the flow at the point is of a kind that can feed an instability.
Whether it grows depends on the system around the point, which a transient run
decides. Read the result in five steps.

1. **Is there anything to screen?** A single-phase point, or one with no free
   gas, raises no flag.
2. **What carries the flow?** Add `slug_body`, `slug_film_layer` and
   `slug_film_wall` from `regime_weights` into a slug share S. A falling slug film
   counts where S ≥ 0.25 and is a trace below 0.1. If `stability.margins.long_wave`
   is present the point is near level; if it is null the point is steep.
3. **Sort the flags.**

    | Kind | Flags |
    |---|---|
    | Mechanism | `slug_film_falls_back` (with S as above), `film_cannot_be_carried_liquid_loading`, `below_film_reversal`, near level `long_wave_unstable` (unless `unstable_layer_too_thin_to_bridge` is also raised) |
    | Boundary | `near_long_wave_instability`, `near_film_lift`, `near_film_reversal`, `near_bubble_transition`, `slug_unit_fading`, `pseudo_slug_to_film`, `near_inviscid_kh_limit` |
    | Multiple states | `bistable`, `near_film_fold_multiple_solutions` |
    | Context | `left_of_gradient_minimum`, `near_gradient_minimum`, `beyond_inviscid_kh_limit`, `counter_current_film`, `unstable_layer_too_thin_to_bridge` |

4. **Weigh the combination for the geometry.**
    - **Steep (wells, risers).** A loading flag on its own is liquid loading. A
      falling film with S ≥ 0.25 together with `left_of_gradient_minimum` is the
      heading mechanism of gas-lifted tubing. Weigh it by the scan's
      `gradient_minimum.ratio` at the top of the tubing. At 0.8 or more the
      tubing usually settles; below 0.8 it can head. `left_of_gradient_minimum`
      alone is not a warning.
    - **Near level (flowlines).** `long_wave_unstable` on a layer deep enough to
      bridge is hydrodynamic slug flow, a regime to size for.
      `left_of_gradient_minimum` or `below_film_reversal` in an uphill section
      means liquid accumulates there at low gas rate. A riser downstream makes
      that the severe-slugging layout.
    - **Any geometry.** A mechanism together with a multiple-state flag is
      path-dependent.
5. **Verdict.**
    - **Steady**: no mechanism counts.
    - **Watch**: boundary or multiple-state flags only.
    - **Mechanism**: one mechanism flag counts.
    - **Act**: a reinforcing combination from step 4. Confirm it with a
      transient run that includes the system: Field for wells, gas lift and
      risers; Flux for long lines over hours.

The 0.8 threshold comes from Hu's (2005) density-wave gas-lift wells, run on
Graphsolve-Field. Of the 10 operating points with a wellhead ratio of 0.8 or
more, 8 settled. Of the 19 below it, 16 headed or had no steady flowing state.
Along a well or line, screen every point of a steady solve's profile or a
transient run's state, and judge by where the verdicts hold and over what
length.

The script
[`examples/basics/06_flow_stability.py`](https://github.com/DataInsightsAI/graphsolve-sdk/tree/main/examples/basics/06_flow_stability.py)
runs the screen at one point and prints the verdict.
