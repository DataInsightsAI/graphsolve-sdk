# Long-running jobs

Most tools answer in milliseconds to seconds. Transient simulations can run for
minutes or hours, so they are submitted as jobs and polled.

## Price it first

`quote` asks what a run would cost. It runs nothing and spends nothing.

=== "Python"

    ```python
    quote = gs.quote("run_transient_wave", request)
    print(quote["quote"])
    ```

=== "TypeScript"

    ```ts
    const quote = await gs.quote("run_transient_wave", request);
    console.log(quote.quote);
    ```

=== "Rust"

    ```rust
    let quote = gs.quote("run_transient_wave", &request).await?;
    println!("{:?}", quote.quote);
    ```

## Run and wait

`run` submits the job and polls until it finishes, so the call reads like any
other. Give it a maximum wait; if that passes, the job is cancelled.

=== "Python"

    ```python
    response = gs.run("run_transient_wave", request, poll_interval=5.0, max_wait=3600)
    ```

=== "TypeScript"

    ```ts
    const response = await gs.run("run_transient_wave", request, {
      pollIntervalMs: 5_000,
      maxWaitMs: 3_600_000,
    });
    ```

=== "Rust"

    ```rust
    use std::time::Duration;

    let response = gs
        .run("run_transient_wave", &request, Duration::from_secs(5), Some(Duration::from_secs(3600)))
        .await?;
    ```

## Managing jobs yourself

`submit` returns at once with the job's id; `job(id)` reports its state and,
when it has finished, its result; `cancel(id)` stops it.
