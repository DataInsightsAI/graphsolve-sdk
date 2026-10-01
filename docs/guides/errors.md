# Errors and retries

Each client raises a typed error, so you can tell "nothing happened" from "it
ran and was charged".

| Error | When | Charged | Retry? |
|---|---|---|---|
| `InvalidPayload` | HTTP 400: the request was wrong | No | No: it will fail the same way |
| `AuthenticationError` | HTTP 401: key or token rejected or expired | No | No |
| `InsufficientCredits` | HTTP 402: nothing left to spend | No | After topping up |
| `ToolNotPermitted` | HTTP 403: the key lacks the scope the tool needs | No | No |
| `RateLimited` | HTTP 429: too many requests | No | Yes, after `retry_after` |
| `ServerError` | HTTP 5xx | No | Yes |
| `SolverDidNotConverge` | The calculation ran and produced no answer | **Yes** | Only with different inputs |
| `OutcomeUnknown` | The call was abandoned and may have run | Possibly | Check before repeating |

In Rust these are variants of `graphsolve::Error`; in TypeScript and Python they
are classes deriving from `GraphSolveError`.

## What the client retries for you

- `429` and `503` on any request, with exponential backoff and jitter,
  honouring `Retry-After`. Neither reached the engine, so repeating cannot
  charge twice.
- `502`, `504` and transport failures **only on reads**. Each of those can
  arrive after the engine has already run the tool, and a tool that ran is
  charged whether or not its answer arrived. A tool call that fails this way
  raises `OutcomeUnknown` instead of being repeated.

Nothing else is retried. A `400` fails identically every time.

## The distinction that matters

A calculation that ran and did not converge arrives as a successful HTTP
response with `failure_kind: "computation_failed"`, and the client raises
`SolverDidNotConverge`. It is charged, because it used the compute. It is not a
fault in the request: try different inputs, a different correlation, or a
better initial guess.

=== "Python"

    ```python
    from graphsolve import InvalidPayload, SolverDidNotConverge

    try:
        response = gs.solve_network(network_json=model)
    except InvalidPayload as e:
        print("fix the model:", e)            # free
    except SolverDidNotConverge as e:
        print("ran but did not converge:", e)  # charged
    ```

=== "TypeScript"

    ```ts
    import { InvalidPayload, SolverDidNotConverge } from "@graphsolve/sdk";

    try {
      await gs.solve_network({ network_json: model });
    } catch (e) {
      if (e instanceof InvalidPayload) console.log("fix the model:", e.message);
      else if (e instanceof SolverDidNotConverge) console.log("did not converge:", e.message);
      else throw e;
    }
    ```

=== "Rust"

    ```rust
    use graphsolve::Error;

    match gs.call("solve_network", &json!({ "network_json": model })).await {
        Ok(response) => println!("{:?}", response.credits_charged()),
        Err(Error::InvalidPayload(msg)) => println!("fix the model: {msg}"),
        Err(Error::SolverDidNotConverge { .. }) => println!("ran but did not converge"),
        Err(other) => return Err(other.into()),
    }
    ```
