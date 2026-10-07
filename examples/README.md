# Examples

## `smoke.py`: does my key work?

Runs the whole path once against a live deployment: exchanges the API key for
a token, reads the key's identity and scopes, lists the tools, and runs one
free tool. With `--solve` it also runs one billed tool, which costs 1 credit.

```sh
pip install graphsolve
export GRAPHSOLVE_API_KEY=...
python examples/smoke.py
python examples/smoke.py --solve
```

Each step prints what it saw, so the step that failed names the problem: a
rejected key, a key without the scope a tool needs, or the wrong host.

### Which hosts

Tokens are issued centrally, so `GRAPHSOLVE_TOKEN_URL` stays at its default
for every production deployment. Only the engine host changes:

| Deployment | `GRAPHSOLVE_BASE_URL` | `GRAPHSOLVE_TOKEN_URL` |
|---|---|---|
| Production | default | default |
| A dedicated customer deployment | `https://api-<customer>.graphsolve.ai` | default |
| Development | `https://api-dev.graphsolve.ai` | `https://dev-user-api.graphsolve.ai/oauth/token` |

A key issued on one platform is not valid on another. A token from the
production platform is refused by the development engine and the other way
round, and a dedicated deployment accepts only its own customer's tokens.

## `equipment/`: compressors, trains and compositional equipment

Each script runs one tool against a live deployment and prints the result.
They describe the gas by its composition on a cubic equation of state, the
form that lets the stream condense or flash inside the equipment.

| Script | Tool | What it shows |
|---|---|---|
| `01_mapped_compressor.py` | `calculate_turbo_machine` | A two-speed performance map run between its speed lines, with the Schultz path method |
| `02_compression_train.py` | `calculate_compression_train` | Three stages with intercoolers and scrubbers; the liquid each scrubber removes and the gas delivered |
| `03_compositional_jt_valve.py` | `calculate_jt_valve` | A Joule-Thomson valve whose outlet comes from an enthalpy flash, with the liquid that drops out |
| `04_condensing_cooler.py` | `calculate_heater_cooler` | A cooler across the dew point, with the latent heat in the duty |
| `05_calibrate_compressor.py` | `analyse_turbo_performance`, then `calculate_turbo_machine` | Head and efficiencies from test points by two path methods, a fitted map, and the calibrated machine at a new flow |

```sh
python examples/equipment/02_compression_train.py
```

Each call costs 1 credit.

## The same check from the CLI

```sh
cargo install graphsolve
export GRAPHSOLVE_API_KEY=...
graphsolve me
echo '{"value": 1, "from_unit": "MPa", "to_unit": "psi"}' | graphsolve convert_units
```
