# Rust client

Install with `cargo add graphsolve`. The full API reference, with every tool's
typed `…Params` struct and every fixed vocabulary as an enum, is on docs.rs:

**[docs.rs/graphsolve](https://docs.rs/graphsolve)**

Each tool has a typed method taking its `…Params` struct. `call(tool, &args)`
takes any serialisable value instead, which is what the examples in the
[tool reference](tools/index.md) use, and reaches a tool newer than the crate.
