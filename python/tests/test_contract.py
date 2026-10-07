"""The behavioural contract from CONTRIBUTING.md, as executable tests.

Each of these is a rule every client in this repo must follow.
"""

from __future__ import annotations

import json
import threading
from urllib.parse import parse_qs

import httpx
import pytest

from graphsolve import (
    DEFAULT_TOKEN_URL,
    AuthenticationError,
    GraphSolve,
    InsufficientCredits,
    InvalidPayload,
    OutcomeUnknown,
    RateLimited,
    SolverDidNotConverge,
)
from graphsolve._auth import refresh_margin

KEY = "gs_test_0123456789abcdef0123456789abcdef"
TOKEN_URL = "https://auth.test/oauth/token"


def token_response(token="tok-1", expires_in=3600):
    return {"access_token": token, "token_type": "Bearer", "expires_in": expires_in}


def http(handler) -> httpx.Client:
    return httpx.Client(transport=httpx.MockTransport(handler), base_url="https://api.test")


def client(handler, **kw) -> GraphSolve:
    kw.setdefault("token_url", TOKEN_URL)
    return GraphSolve(KEY, client=http(handler), **kw)


def with_token(handler, *, expires_in=3600, tokens=None):
    """Wrap a handler so the token endpoint is served alongside it.

    Returns (handler, exchanges) so a test can assert how many exchanges the
    client performed, where it sent them and what it sent.
    """
    exchanges = []
    issued = iter(tokens) if tokens else None

    def wrapped(request):
        if request.url.path == "/oauth/token":
            form = parse_qs(request.content.decode())
            exchanges.append({
                "url": str(request.url),
                "authorization": request.headers.get("authorization"),
                "grant_type": form.get("grant_type", [None])[0],
            })
            token = next(issued) if issued else "tok-1"
            return httpx.Response(200, json=token_response(token, expires_in))
        return handler(request)

    return wrapped, exchanges


def envelope(status="success", result=None, failure_kind=None, charged=0, **extra):
    body = {
        "status": status,
        "result": result,
        "warnings": [],
        "errors": [] if status == "success" else ["it did not converge"],
        "metadata": {
            "calculation_time_ms": 1,
            "billing": {"tool": "t", "credits_charged": charged},
        },
    }
    if failure_kind:
        body["failure_kind"] = failure_kind
    body.update(extra)
    return body


def ok(request):
    return httpx.Response(200, json=envelope(result={"ok": True}))


# --------------------------------------------------------------------------
# Credentials and tokens
# --------------------------------------------------------------------------


def test_the_api_key_is_read_from_the_environment_and_exchanged(monkeypatch):
    monkeypatch.setenv("GRAPHSOLVE_API_KEY", KEY)
    monkeypatch.setenv("GRAPHSOLVE_TOKEN_URL", TOKEN_URL)
    seen = {}

    def handler(request):
        seen["auth"] = request.headers.get("authorization")
        return ok(request)

    wrapped, exchanges = with_token(handler)
    GraphSolve(client=http(wrapped)).call("convert_units", {})

    assert exchanges == [{
        "url": TOKEN_URL,
        "authorization": f"Bearer {KEY}",
        "grant_type": "client_credentials",
    }]
    assert seen["auth"] == "Bearer tok-1"


def test_a_missing_api_key_fails_at_construction(monkeypatch):
    """A missing key is a programming error, not a runtime surprise."""
    monkeypatch.delenv("GRAPHSOLVE_API_KEY", raising=False)
    with pytest.raises(AuthenticationError, match="GRAPHSOLVE_API_KEY"):
        GraphSolve()


def test_the_token_url_is_not_derived_from_the_base_url(monkeypatch):
    """Tokens are issued by the platform for every deployment, not by the engine."""
    monkeypatch.delenv("GRAPHSOLVE_TOKEN_URL", raising=False)
    wrapped, exchanges = with_token(ok)
    GraphSolve(KEY, client=http(wrapped)).call("convert_units", {})
    assert exchanges[0]["url"] == DEFAULT_TOKEN_URL


def test_the_token_is_exchanged_once_and_reused():
    wrapped, exchanges = with_token(ok)
    gs = client(wrapped)
    gs.call("convert_units", {})
    gs.call("convert_units", {})
    assert len(exchanges) == 1, f"{len(exchanges)} exchanges for two calls"


def test_the_token_is_refreshed_before_it_expires():
    """Refreshing early is what keeps a 401 off the retry path.

    The margin is asserted on the pure function: a token is never issued
    already stale, so there is nothing to observe from the outside.
    """
    assert refresh_margin(3600) == 300, "the flat margin is not applied"
    for expires_in in (1, 30, 200, 600, 3600, 86400):
        margin = refresh_margin(expires_in)
        assert 0 < margin < expires_in, f"{expires_in}s token refreshes at {margin}s"


def test_a_key_near_its_expiry_does_not_exchange_on_every_call():
    """The platform caps a token's life at the key's own remaining life.

    A key in its last minutes therefore mints tokens shorter than the flat
    300 s margin. Comparing against that flat value made every cached token
    stale on arrival, so every call exchanged again — and the token endpoint is
    rate limited.
    """
    wrapped, exchanges = with_token(ok, expires_in=200)
    gs = client(wrapped)
    for _ in range(20):
        gs.call("convert_units", {})
    assert len(exchanges) == 1, f"20 calls caused {len(exchanges)} token exchanges"


def test_an_already_expired_token_is_refused_rather_than_cached(monkeypatch):
    monkeypatch.setattr("time.sleep", lambda s: None)

    def handler(request):
        if request.url.path == "/oauth/token":
            return httpx.Response(200, json=token_response(expires_in=0))
        return ok(request)

    with pytest.raises(AuthenticationError, match="already expired"):
        client(handler).call("convert_units", {})


def test_concurrent_callers_share_one_exchange():
    """A client is shared across threads; a stampede must not mean N exchanges."""
    barrier = threading.Barrier(8)
    wrapped, exchanges = with_token(ok)
    gs = client(wrapped)

    def worker():
        barrier.wait()
        gs.call("convert_units", {})

    threads = [threading.Thread(target=worker) for _ in range(8)]
    for thread in threads:
        thread.start()
    for thread in threads:
        thread.join()

    assert len(exchanges) == 1, f"{len(exchanges)} exchanges from 8 threads"


def test_a_rejected_api_key_is_not_retried(monkeypatch):
    monkeypatch.setattr("time.sleep", lambda s: None)
    attempts = []

    def handler(request):
        attempts.append(1)
        return httpx.Response(401, json={"error_description": "unknown key"})

    with pytest.raises(AuthenticationError, match="rejected"):
        client(handler).call("convert_units", {})
    assert len(attempts) == 1, f"token exchange retried {len(attempts)} times"


def test_an_unavailable_auth_service_is_retried_then_reported(monkeypatch):
    slept = []
    monkeypatch.setattr("time.sleep", lambda s: slept.append(s))
    attempts = []

    def handler(request):
        attempts.append(1)
        return httpx.Response(503, headers={"retry-after": "2"}, json={})

    with pytest.raises(AuthenticationError, match="unavailable"):
        client(handler, max_retries=2).call("convert_units", {})
    assert len(attempts) == 3
    assert slept == [2.0, 2.0], f"Retry-After ignored: {slept}"


def test_the_api_key_never_appears_in_an_error_or_a_repr():
    def handler(request):
        return httpx.Response(403, json={"error": "forbidden"})

    gs = client(handler)
    with pytest.raises(AuthenticationError) as excinfo:
        gs.call("convert_units", {})

    assert KEY not in str(excinfo.value)
    assert KEY not in repr(excinfo.value)
    # A traceback or a debugger will render these; neither may leak it.
    assert KEY not in repr(gs)
    assert KEY not in repr(gs.__dict__)


def test_me_reports_identity_and_scopes():
    seen = {}

    def handler(request):
        seen["path"] = request.url.path
        return httpx.Response(200, json={
            "user_id": "u_1", "tenant_id": "acme", "org_id": None,
            "key_id": "key_1", "scopes": ["engine:read", "engine:solve"], "billable": True,
        })

    me = client(with_token(handler)[0]).me()
    assert seen["path"] == "/v1/me"
    assert "engine:solve" in me["scopes"]


# --------------------------------------------------------------------------
# Retries
# --------------------------------------------------------------------------


def test_every_call_carries_an_idempotency_key():
    """A retry after a network timeout must not be charged twice."""
    keys = []

    def handler(request):
        keys.append(request.headers.get("idempotency-key"))
        return ok(request)

    gs = client(with_token(handler)[0])
    gs.call("convert_units", {})
    gs.call("convert_units", {})
    assert all(keys), "no idempotency key sent"
    assert keys[0] != keys[1], "two logical calls must not share a key"


def test_out_of_credits_is_never_retried():
    """Unlike a rate limit, waiting does not help."""
    calls = []

    def handler(request):
        calls.append(1)
        return httpx.Response(402, json={"status": "error", "errors": ["Insufficient credits"]})

    with pytest.raises(InsufficientCredits):
        client(with_token(handler)[0], max_retries=3).call("solve_network", {})
    assert len(calls) == 1, f"402 was retried {len(calls)} times"


def test_bad_payload_is_never_retried():
    calls = []

    def handler(request):
        calls.append(1)
        return httpx.Response(400, json={"status": "error", "errors": ["bad field"]})

    with pytest.raises(InvalidPayload):
        client(with_token(handler)[0], max_retries=3).call("solve_network", {})
    assert len(calls) == 1


def test_rate_limit_is_retried_and_honours_retry_after(monkeypatch):
    slept = []
    monkeypatch.setattr("time.sleep", lambda s: slept.append(s))
    calls = []

    def handler(request):
        calls.append(1)
        if len(calls) == 1:
            return httpx.Response(429, headers={"retry-after": "7"},
                                  json={"status": "error", "errors": ["slow down"]})
        return ok(request)

    assert client(with_token(handler)[0]).call("solve_network", {})["status"] == "success"
    assert len(calls) == 2, "429 was not retried"
    assert slept == [7.0], f"Retry-After ignored: slept {slept}"


def test_rate_limit_gives_up_and_raises_with_retry_after(monkeypatch):
    monkeypatch.setattr("time.sleep", lambda s: None)

    def handler(request):
        return httpx.Response(429, headers={"retry-after": "30"},
                              json={"status": "error", "errors": ["slow down"]})

    with pytest.raises(RateLimited) as excinfo:
        client(with_token(handler)[0], max_retries=2).call("solve_network", {})
    assert excinfo.value.retry_after == 30.0


def test_a_gateway_failure_is_not_retried_on_a_billed_call(monkeypatch):
    """The load balancer cannot say whether the engine already ran the tool, and
    the engine does not read Idempotency-Key, so a second POST is a second charge.
    """
    monkeypatch.setattr("time.sleep", lambda s: None)
    calls = []

    def handler(request):
        calls.append(1)
        return httpx.Response(504, json={"errors": ["gateway timeout"]})

    with pytest.raises(OutcomeUnknown) as excinfo:
        client(with_token(handler)[0], max_retries=3).call("solve_network", {})
    assert len(calls) == 1, f"a 504 on a POST was retried {len(calls)} times"
    assert "was charged" in str(excinfo.value), (
        f"the caller is not told the call may have run: {excinfo.value}"
    )


def test_a_gateway_failure_is_retried_on_a_read(monkeypatch):
    monkeypatch.setattr("time.sleep", lambda s: None)
    calls = []

    def handler(request):
        calls.append(1)
        if len(calls) == 1:
            return httpx.Response(502)
        return httpx.Response(200, json={"user_id": "u", "scopes": ["engine:read"]})

    assert client(with_token(handler)[0], max_retries=3).me()["user_id"] == "u"
    assert len(calls) == 2, "a 502 on a GET was not retried"


def test_a_transport_failure_is_not_retried_on_a_billed_call(monkeypatch):
    """A POST that got no answer may still have been received and run."""
    monkeypatch.setattr("time.sleep", lambda s: None)
    calls = []

    def handler(request):
        calls.append(1)
        raise httpx.ReadTimeout("timed out", request=request)

    with pytest.raises(OutcomeUnknown) as excinfo:
        client(with_token(handler)[0], max_retries=3).call("solve_network", {})
    assert len(calls) == 1, f"a timed-out POST was retried {len(calls)} times"
    assert "was charged" in str(excinfo.value)


# --------------------------------------------------------------------------
# Charged versus free
# --------------------------------------------------------------------------


def test_a_failed_calculation_is_a_200_and_is_charged():
    """A non-converged solve consumed compute and IS charged; a bad payload is a
    400 and is free. The client must not collapse the two.
    """
    def handler(request):
        return httpx.Response(
            200,
            json=envelope(status="error", failure_kind="computation_failed", charged=25),
        )

    with pytest.raises(SolverDidNotConverge) as excinfo:
        client(with_token(handler)[0]).call("solve_network", {})
    assert excinfo.value.credits_charged == 25, "the cost must be visible where it is noticed"
    assert excinfo.value.envelope["failure_kind"] == "computation_failed"


# --------------------------------------------------------------------------
# Long-running work
# --------------------------------------------------------------------------


def test_a_quote_spends_nothing():
    seen = {}

    def handler(request):
        seen["url"] = str(request.url)
        return httpx.Response(200, json={"quote": {
            "tool": "run_transient_wave", "estimated_credits": 41200, "basis": "cells x steps",
        }})

    q = client(with_token(handler)[0]).quote("run_transient_wave", {})
    assert "dry_run=true" in seen["url"]
    assert q["quote"]["estimated_credits"] == 41200


def test_run_blocks_until_the_job_finishes(monkeypatch):
    monkeypatch.setattr("time.sleep", lambda s: None)
    states = iter(["queued", "running", "succeeded"])

    def handler(request):
        if request.method == "POST":
            return httpx.Response(202, json={"job": {"id": "job_1", "state": "queued"}})
        return httpx.Response(200, json={"job": {
            "id": "job_1", "state": next(states),
            "result": envelope(result={"pressure": 1.0}), "charged_credits": 3,
        }})

    result = client(with_token(handler)[0]).run("run_transient_wave", {})
    assert result["result"]["pressure"] == 1.0


def test_abandoning_the_wait_cancels_the_job(monkeypatch):
    monkeypatch.setattr("time.sleep", lambda s: None)
    cancelled = []

    def handler(request):
        if request.url.path.endswith("/cancel"):
            cancelled.append(1)
            return httpx.Response(200, json={"job_id": "job_1", "state": "cancelled"})
        if request.method == "POST":
            return httpx.Response(202, json={"job": {"id": "job_1", "state": "queued"}})
        return httpx.Response(200, json={"job": {"id": "job_1", "state": "running"}})

    with pytest.raises(Exception, match="cancelled"):
        client(with_token(handler)[0]).run("run_transient_wave", {}, max_wait=-1)
    assert len(cancelled) == 1


# --------------------------------------------------------------------------
# The generated surface
# --------------------------------------------------------------------------


def test_every_tool_in_the_spec_has_a_method():
    """Every tool in the spec must have a generated method."""
    import pathlib

    from graphsolve._generated import GeneratedMethods

    spec = json.loads(
        (pathlib.Path(__file__).parents[2] / "spec" / "graphsolve-v1.json").read_text()
    )
    tools = {
        item["post"]["operationId"]
        for path, item in spec["paths"].items()
        if path.startswith("/v1/tools/") and "post" in item
    }
    missing = {t for t in tools if not callable(getattr(GeneratedMethods, t, None))}
    assert not missing, f"{len(missing)} tools have no method: {sorted(missing)[:5]}"
    assert len(tools) == 101


def test_a_generated_method_forwards_to_the_named_tool():
    seen = {}

    def handler(request):
        seen["path"] = request.url.path
        seen["body"] = json.loads(request.content)
        return httpx.Response(200, json=envelope(result={"outlet_pressure": 10.0}))

    client(with_token(handler)[0]).calculate_compressor(
        inlet_pressure=5.0,
        inlet_temperature=350.0,
        pressure_ratio=2.0,
        fluid={"gas_rate": 500000, "gas_mw": 18.5},
    )
    assert seen["path"] == "/v1/tools/calculate_compressor"
    assert seen["body"]["inlet_pressure"] == 5.0


def test_nested_machine_and_composition_objects_pass_through_unchanged():
    """A tagged-union machine block and a composition block are sent as the
    objects given, and an omitted black-oil fluid is left out."""
    seen = {}

    def handler(request):
        seen["path"] = request.url.path
        seen["body"] = json.loads(request.content)
        return httpx.Response(200, json=envelope(result={}))

    machine = {
        "kind": "centrifugal_map",
        "shaft_speed": 1100.0,
        "speed_lines": [
            {
                "shaft_speed": 1000.0,
                "head_curve": {"x": [0.2, 0.35], "y": [85000.0, 75000.0]},
                "efficiency_curve": {"x": [0.2, 0.35], "y": [0.74, 0.80]},
            }
        ],
    }
    composition = {
        "component_names": ["methane", "ethane"],
        "mole_fractions": [0.9, 0.1],
        "mass_rate": 10.0,
    }
    client(with_token(handler)[0]).calculate_turbo_machine(
        inlet_pressure=4.0,
        inlet_temperature=300.0,
        turbo_machine=machine,
        composition=composition,
    )
    assert seen["path"] == "/v1/tools/calculate_turbo_machine"
    assert seen["body"]["turbo_machine"] == machine
    assert seen["body"]["composition"] == composition
    assert "fluid" not in seen["body"]


def test_the_compression_train_and_calibration_tools_post_to_their_paths():
    paths = []

    def handler(request):
        paths.append(request.url.path)
        return httpx.Response(200, json=envelope(result={}))

    gs = client(with_token(handler)[0])
    gs.calculate_compression_train(
        inlet_pressure=2.0,
        inlet_temperature=360.0,
        stages=[{"turbo_machine": {"kind": "centrifugal_simple",
                                   "compressor_pressure_ratio": 2.0,
                                   "polytropic_efficiency": 0.78}}],
        composition={"component_names": ["methane"], "mole_fractions": [1.0],
                     "mass_rate": 5.0},
    )
    gs.analyse_turbo_performance(
        points=[{"inlet_pressure": 4.0, "inlet_temperature": 300.0,
                 "outlet_pressure": 8.0, "outlet_temperature": 364.0}],
    )
    assert paths == [
        "/v1/tools/calculate_compression_train",
        "/v1/tools/analyse_turbo_performance",
    ]


def test_omitted_optional_arguments_are_absent_not_null():
    """An unset optional must be absent from the payload, not sent as null.
    Several tools treat "not given" and null differently."""

    def handler(request):
        body = json.loads(request.content)
        assert "mechanical_efficiency" not in body, "unset optional was sent as null"
        assert "polytropic_efficiency" not in body
        return httpx.Response(200, json=envelope(result={}))

    client(with_token(handler)[0]).calculate_compressor(
        inlet_pressure=5.0, inlet_temperature=350.0, pressure_ratio=2.0, fluid={}
    )


def test_a_blob_parameter_accepts_an_object():
    """The *_json parameters are anyOf[object, array, string]."""

    def handler(request):
        assert isinstance(json.loads(request.content)["network_json"], dict)
        return httpx.Response(200, json=envelope(result={}))

    client(with_token(handler)[0]).solve_network(network_json={"graph_data": {"nodes": []}})
