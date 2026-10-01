# Network models

`solve_network` takes a whole production system: wells, pipes, chokes,
equipment and the boundaries that drive it. `validate_solver_payload` and
`solve_network_map` take the same model, and `optimise_network` takes it as the
network inside its request.

## The shape of a model

```json
{
  "format": "model",
  "elements": [ { "group": "nodes", "data": { "id": "...", "node_type": "..." } },
                { "group": "edges", "data": { "id": "...", "source": "...", "target": "...", "edge_type": "..." } } ],
  "fluids": [ ... ],
  "sources": [ ... ],
  "process_equipment": [ ... ],
  "solve_parameters": { "max_n_iter": 200, "tolerance": 1e-4 }
}
```

- **Nodes** are points in the network. A node's `node_type` is one of
  `fixed_rate_source`, `fixed_pressure_source`, `pressure_dependent_source`,
  `network_node`, `fixed_pressure_sink` or `fixed_rate_sink`. Sources and sinks
  carry their boundary condition in `source_sink_data`.
- **Edges** join two nodes by id, from `source` to `target` in the direction of
  flow. An edge's `edge_type` is one of `no_pressure_loss`, `pipe`, `choke`,
  `heat_exchanger`, `compressor`, `pump`, `turbine`, `jt_valve`,
  `heater_cooler` or `reactor`, with its data in the matching block
  (`pipe_data`, `choke_data`, ...).
- **`fluids`** are the fluid models sources refer to by `fluid_id`.
- **`sources`** are the inflow models a `pressure_dependent_source` refers to by
  `reservoir_props_id`.

Ids are strings of your choosing. The reply returns your model with a `results`
block on every node and edge, so you read answers back by the same ids. An
optional `label` on any element is echoed unchanged.

## Boundary conditions

A network needs enough fixed pressures and rates to be determined. A typical
production system has its wells at the upstream end and a separator at a fixed
pressure downstream:

| Node type | You give | The solve finds |
|---|---|---|
| `pressure_dependent_source` | Reservoir pressure and an inflow model | The rate the well delivers |
| `fixed_rate_source` | A rate | The pressure needed to deliver it |
| `fixed_pressure_source` | A pressure | The rate that pressure drives |
| `fixed_pressure_sink` | A pressure | The rate arriving |

## Check before you solve

`validate_solver_payload` runs the same build step as `solve_network` and is
free. Call it first when assembling a model.

## Reconciling measurements

`solve_network_map` takes the same model plus measurements, each with a
variance:

- a pressure gauge is `fixed_pressure: {value, variance}` on a `network_node`;
- a rate meter is `measured_phase_rates: {oil|water|gas: {value, variance}}` on
  an edge.

Source inputs you are unsure of, such as a well's GOR or water cut, can be given
a variance in `source_sink_data.beliefs` and are then estimated from the
measurements. Reconciliation needs `"flow_model": "multiphase"` in
`solve_parameters`. The reply adds a `posterior_std_dev` to each result and
`belief_posteriors` with each estimate and its uncertainty.

See [A single oil well](../examples/single-well.md) for a complete model.
