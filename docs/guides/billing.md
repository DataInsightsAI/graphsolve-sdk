# Billing and credits

Every tool call is metered in credits. Each tool's price is listed in the
[tool reference](../reference/tools/index.md) and has the form

> base credits, plus 1 credit per *n* ms of compute beyond the first *m* ms

so a tool that finishes inside its free allowance costs exactly its base price.
Catalogue lookups, unit conversion and validation are free.

## What a call cost

Every response reports its own charge:

=== "Python"

    ```python
    response = gs.solve_network(network_json=model)
    print(response["metadata"]["billing"]["credits_charged"])
    ```

=== "TypeScript"

    ```ts
    const response = await gs.solve_network({ network_json: model });
    console.log(response.metadata?.billing?.credits_charged);
    ```

=== "Rust"

    ```rust
    let response = gs.call("solve_network", &json!({ "network_json": model })).await?;
    println!("{:?}", response.credits_charged());
    ```

## What is and is not charged

| Outcome | Charged |
|---|---|
| The tool ran and returned an answer | Yes |
| The tool ran and the calculation did not converge (`SolverDidNotConverge`) | Yes: the compute was used |
| The request was malformed (`InvalidPayload`, HTTP 400) | No |
| Not authorised, no credits, rate limited | No |
| Server error (5xx) | No |

Balances are held by the GraphSolve platform, not the engine, so `me()` reports
identity and scopes but not a balance. Check your balance in the app.

## Long runs

Transient tools can run for minutes or hours. `quote` prices a run without
running it or spending anything; see [Long-running jobs](jobs.md).
