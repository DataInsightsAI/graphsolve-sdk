# Installation

=== "Python"

    Python 3.10 or newer.

    ```sh
    pip install graphsolve
    ```

    The only dependency is `httpx`.

=== "TypeScript"

    Node 18 or newer, or any runtime with a global `fetch`.

    ```sh
    npm install @graphsolve/sdk
    ```

    No runtime dependencies. ESM and CommonJS builds are both published.

=== "Rust"

    Rust 1.88 or newer. The client is async and runs on Tokio.

    ```sh
    cargo add graphsolve
    cargo add tokio --features macros,rt-multi-thread
    cargo add serde_json
    ```

    The crate also builds a `graphsolve` command-line tool:

    ```sh
    cargo install graphsolve
    graphsolve me
    ```

Pin the version in production. A client's major and minor version match the
API version it was generated from; the patch number is the client's own.
