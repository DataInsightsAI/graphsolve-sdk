# Your first call

Every tool is one method on the client, named after the tool. Arguments are
keyword arguments in Python and a single object in TypeScript; in Rust, pass a
typed `…Params` struct to the method, or any JSON value to `call`.

=== "Python"

    ```python
    from graphsolve import GraphSolve

    gs = GraphSolve()
    response = gs.calculate_pressure_drop(
        correlation="Beggs-Brill",
        pressure=10.0,                 # MPa
        diameter=0.1,                  # m
        roughness=4.5e-5,              # m
        angle=0.0,                     # degrees from vertical; 0 is vertical upflow
        density=[80.0, 750.0, 1030.0], # kg/m3, [gas, oil, water]
        viscosity=[1.5e-5, 2.0e-3, 5.0e-4],  # Pa.s
        velocity=[2.0, 1.0, 0.3],      # m/s superficial
        ift=0.02,                      # N/m
    )
    ```

=== "TypeScript"

    ```ts
    const response = await gs.calculate_pressure_drop({
      correlation: "Beggs-Brill",
      pressure: 10.0,
      diameter: 0.1,
      roughness: 4.5e-5,
      angle: 0.0,
      density: [80.0, 750.0, 1030.0],
      viscosity: [1.5e-5, 2.0e-3, 5.0e-4],
      velocity: [2.0, 1.0, 0.3],
      ift: 0.02,
    });
    ```

=== "Rust"

    ```rust
    use graphsolve::{CalculatePressureDropParams, FlowCorrelationName, GraphSolve};

    let gs = GraphSolve::new()?;
    let response = gs
        .calculate_pressure_drop(CalculatePressureDropParams {
            correlation: FlowCorrelationName::BeggsBrill,
            pressure: 10.0,
            diameter: 0.1,
            roughness: 4.5e-5,
            angle: 0.0,
            density: vec![80.0, 750.0, 1030.0],
            viscosity: vec![1.5e-5, 2.0e-3, 5.0e-4],
            velocity: vec![2.0, 1.0, 0.3],
            ift: 0.02,
        })
        .await?;
    ```

Fixed vocabularies such as correlation names are typed in TypeScript and Rust,
so a misspelt name fails before the call is made.

## The response

Every tool returns the same envelope:

```json
{
  "status": "success",
  "result": { "total_gradient_mpa_per_m": 0.0081, "holdup": 0.62, "...": "..." },
  "metadata": {
    "calculation_time_ms": 3,
    "billing": { "tool": "calculate_pressure_drop", "credits_charged": 1 }
  },
  "warnings": [],
  "errors": []
}
```

- `result` holds the tool's answer. Its fields are listed with each tool in the
  [tool reference](../reference/tools/index.md).
- `metadata.billing.credits_charged` is what this call cost.
- `warnings` carries notes the engine attached to an answer it did return.

A request the engine cannot run raises an error instead; see
[Errors and retries](../guides/errors.md).
