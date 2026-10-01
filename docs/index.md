# GraphSolve SDK

Client libraries for the GraphSolve engine API: production-network solving,
PVT and equation-of-state flashes, flow assurance and transient multiphase flow,
as a hosted service.

| Language | Package | Install |
|---|---|---|
| Python | [`graphsolve`](https://pypi.org/project/graphsolve/) | `pip install graphsolve` |
| TypeScript | [`@graphsolve/sdk`](https://www.npmjs.com/package/@graphsolve/sdk) | `npm install @graphsolve/sdk` |
| Rust | [`graphsolve`](https://docs.rs/graphsolve) | `cargo add graphsolve` |

All three are generated from the same API specification, so every tool has a
method of the same name, with the same arguments, in each language.

=== "Python"

    ```python
    from graphsolve import GraphSolve

    gs = GraphSolve()  # reads GRAPHSOLVE_API_KEY
    response = gs.convert_units(value=1.0, from_unit="MPa", to_unit="psi")
    print(response["result"]["converted_value"])
    ```

=== "TypeScript"

    ```ts
    import { GraphSolve } from "@graphsolve/sdk";

    const gs = new GraphSolve(); // reads GRAPHSOLVE_API_KEY
    const response = await gs.convert_units({ value: 1.0, from_unit: "MPa", to_unit: "psi" });
    console.log(response.result);
    ```

=== "Rust"

    ```rust
    use graphsolve::GraphSolve;
    use serde_json::json;

    let gs = GraphSolve::new()?; // reads GRAPHSOLVE_API_KEY
    let response = gs
        .call("convert_units", &json!({ "value": 1.0, "from_unit": "MPa", "to_unit": "psi" }))
        .await?;
    println!("{:?}", response.result);
    ```

## Where to go next

- [Installation](getting-started/installation.md) and
  [authentication](getting-started/authentication.md) get a key working.
- [Your first call](getting-started/first-call.md) explains what a response
  contains.
- [Units](guides/units.md): every value is SI. Read this before building a model.
- [A single oil well](examples/single-well.md) solves a complete network and
  sizes its choke.
- The [tool reference](reference/tools/index.md) lists every tool with its
  arguments, price and an example.
