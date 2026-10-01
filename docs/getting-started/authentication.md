# Authentication

Create an API key in the GraphSolve app, then make it available to the client:

```sh
export GRAPHSOLVE_API_KEY=...
```

Every client reads `GRAPHSOLVE_API_KEY` when you do not pass a key explicitly,
and refuses to start without one. The key is never printed, logged or included
in an error message.

## How the key is used

The client exchanges the key for a short-lived access token at the platform's
token endpoint, caches it, and refreshes it before it expires. You do not need
to handle tokens yourself. If you call the HTTP API from your own code, every
client can hand you its current token.

A key carries scopes:

| Scope | Allows |
|---|---|
| `engine:read` | Free lookups: catalogues, unit conversion, validation |
| `engine:solve` | Anything that runs a calculation |

Check what a key can do:

=== "Python"

    ```python
    from graphsolve import GraphSolve

    me = GraphSolve().me()
    print(me["scopes"])
    ```

=== "TypeScript"

    ```ts
    const me = await new GraphSolve().me();
    console.log(me.scopes);
    ```

=== "Rust"

    ```rust
    let me = GraphSolve::new()?.me().await?;
    println!("{:?}", me.scopes);
    ```

## Hosts

| Setting | Environment variable | Default |
|---|---|---|
| Engine API | `GRAPHSOLVE_BASE_URL` | `https://api-prod.graphsolve.ai` |
| Token endpoint | `GRAPHSOLVE_TOKEN_URL` | `https://prod-user-api.graphsolve.ai/oauth/token` |

Tokens are issued centrally, so the token endpoint stays at its default for
every production deployment. If your organisation has a dedicated deployment,
set only `GRAPHSOLVE_BASE_URL` to its host. A key is valid only on the platform
that issued it.
