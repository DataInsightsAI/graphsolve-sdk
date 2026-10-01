# A single oil well

W-1 is a synthetic oil well, small enough to read in one go and complete enough
to show how a network model fits together. The files are in
[`examples/single_well/`](https://github.com/DataInsightsAI/graphsolve-sdk/tree/main/examples/single_well).

```
reservoir ──sandface──▶ bottomhole ──tubing──▶ wellhead ──choke──▶ choke outlet ──flowline──▶ separator
 25 MPa, PI 250                       2500 m vertical       25 mm                 3 km horizontal      2 MPa
```

- **Reservoir:** 25 MPa and 360 K, a straight-line productivity index of
  250 Sm³/day/MPa, 20 % water cut and a GOR of 120 Sm³/Sm³.
- **Tubing:** 2500 m of 0.1 m pipe, vertical (angle 0°, upflow), Hagedorn-Slug.
- **Production choke:** 25 mm.
- **Flowline:** 3 km of 0.15 m pipe, horizontal (angle 90°), Beggs-Brill.
- **Separator:** held at 2 MPa.

The well's rate is not an input. It is whatever the inflow delivers at the
bottomhole pressure the network settles at.

## The model

??? example "network.json"

    ```json
    --8<-- "examples/single_well/network.json"
    ```

Note the two references that tie the model together: the reservoir node's
`fluid_id` names an entry in `fluids`, and its `reservoir_props_id` names an
entry in `sources`.

## Solving it

```python
--8<-- "examples/single_well/solve.py"
```

```sh
python solve.py --sweep
```

```
node            P [MPa]  T [degC]
bottomhole       20.016      86.9
wellhead          4.888      77.9
choke-outlet      2.908      77.1
separator         2.000      59.7

oil 0.997, water 0.249, gas 119.6 kSm3/day
choke critical: False

choke [mm]  oil [kSm3/d]  WHP [MPa]  critical
        15         0.494      6.643      True
        20         0.760      5.762      True
        25         0.997      4.888     False
        30         1.139      4.268     False
```

## Reading the answer

The well flows about 1000 Sm³/day of oil and 250 Sm³/day of water. That is the
inflow's 1250 Sm³/day of liquid at a drawdown of 5 MPa (25 → 20 MPa) times the
productivity index of 250.

The choke sweep shows the two regimes a choke works in. At 15 and 20 mm the
flow through the choke is critical: the rate is set by the pressure upstream of
the choke alone, and the separator pressure has no effect on it. Opening to
25 mm takes the choke out of critical flow, and from there the downstream system
shares the control of the rate.

Each solve costs 25 credits, so the sweep costs 100.
