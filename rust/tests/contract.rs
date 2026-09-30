//! The behavioural contract from CONTRIBUTING.md, as executable tests.
//!
//! Each of these is a rule every client in this repo must follow.

use std::time::Duration;

use graphsolve::{
    CalculateCompressorParams, CalculatePressureDropParams, Error, FlowCorrelationName, GraphSolve,
    SolveNetworkParams, TOOL_NAMES,
};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const KEY: &str = "gs_test_0123456789abcdef0123456789abcdef";
const TOKEN_PATH: &str = "/oauth/token";

fn token_url(server: &MockServer) -> String {
    format!("{}{TOKEN_PATH}", server.uri())
}

/// A client against one mock server that serves both the engine and the token
/// endpoint.
fn with_retries(server: &MockServer, retries: u32) -> graphsolve::Builder {
    GraphSolve::builder()
        .api_key(KEY)
        .base_url(server.uri())
        .token_url(token_url(server))
        .max_retries(retries)
}

/// No retries: the backoff curve is asserted directly in `client::tests`.
fn client(server: &MockServer) -> GraphSolve {
    with_retries(server, 0).build().expect("client")
}

async fn mount_token(server: &MockServer, token: &str, expires_in: u64) {
    Mock::given(method("POST"))
        .and(path(TOKEN_PATH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": token,
            "token_type": "Bearer",
            "expires_in": expires_in,
        })))
        .mount(server)
        .await;
}

async fn requests_to(server: &MockServer, prefix: &str) -> usize {
    server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .filter(|r| r.url.path().starts_with(prefix))
        .count()
}

fn envelope(status: &str, charged: u64) -> serde_json::Value {
    json!({
        "status": status,
        "result": { "ok": true },
        "warnings": [],
        "errors": if status == "success" { vec![] } else { vec!["it did not converge"] },
        "metadata": {
            "calculation_time_ms": 1,
            "billing": { "tool": "solve_network", "credits_charged": charged }
        }
    })
}

async fn mount_tool(server: &MockServer, tool: &str, response: ResponseTemplate) {
    Mock::given(method("POST"))
        .and(path(format!("/v1/tools/{tool}")))
        .respond_with(response)
        .mount(server)
        .await;
}

// ---------------------------------------------------------------------------
// Credentials and tokens
// ---------------------------------------------------------------------------

#[tokio::test]
async fn the_api_key_is_exchanged_and_the_token_is_sent_as_a_bearer() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    Mock::given(method("POST"))
        .and(path("/v1/tools/convert_units"))
        .and(header("authorization", "Bearer tok-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope("success", 0)))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .call("convert_units", &json!({}))
        .await
        .unwrap();

    let requests = server.received_requests().await.unwrap();
    let exchange = requests
        .iter()
        .find(|r| r.url.path() == TOKEN_PATH)
        .expect("no token exchange");
    assert_eq!(
        exchange
            .headers
            .get("authorization")
            .unwrap()
            .to_str()
            .unwrap(),
        format!("Bearer {KEY}")
    );
    assert_eq!(
        std::str::from_utf8(&exchange.body).unwrap(),
        "grant_type=client_credentials"
    );
}

#[tokio::test]
async fn the_token_comes_from_the_platform_not_the_engine_host() {
    // Tokens are issued by the platform for every deployment, not by the engine.
    let engine = MockServer::start().await;
    let platform = MockServer::start().await;
    mount_token(&platform, "tok-1", 3600).await;
    mount_tool(
        &engine,
        "convert_units",
        ResponseTemplate::new(200).set_body_json(envelope("success", 0)),
    )
    .await;

    GraphSolve::builder()
        .api_key(KEY)
        .base_url(engine.uri())
        .token_url(token_url(&platform))
        .max_retries(0)
        .build()
        .expect("client")
        .call("convert_units", &json!({}))
        .await
        .unwrap();

    assert_eq!(requests_to(&platform, TOKEN_PATH).await, 1);
    assert_eq!(
        requests_to(&engine, TOKEN_PATH).await,
        0,
        "exchanged against the engine host"
    );
}

#[test]
fn a_client_without_an_api_key_refuses_to_build() {
    // Skipped if a key is exported: env vars are process-global, so there is
    // no way to hide one from a single test.
    if std::env::var("GRAPHSOLVE_API_KEY").is_ok() {
        return;
    }
    let result = GraphSolve::builder().base_url("https://api.test").build();
    assert!(matches!(result, Err(Error::NoCredentials)), "{result:?}");
}

async fn mount_ok(server: &MockServer) {
    mount_tool(
        server,
        "convert_units",
        ResponseTemplate::new(200).set_body_json(envelope("success", 0)),
    )
    .await;
}

#[tokio::test]
async fn the_token_is_exchanged_once_and_reused() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    mount_ok(&server).await;

    let gs = client(&server);
    gs.call("convert_units", &json!({})).await.unwrap();
    gs.call("convert_units", &json!({})).await.unwrap();

    assert_eq!(
        requests_to(&server, TOKEN_PATH).await,
        1,
        "exchanged more than once"
    );
}

#[test]
fn the_token_is_refreshed_before_it_expires() {
    // Asserted on the pure function: a token is never issued already stale, so
    // there is nothing to observe from the outside.
    assert_eq!(
        graphsolve::refresh_margin(Duration::from_secs(3600)),
        Duration::from_secs(300),
        "the flat margin is not applied"
    );
    for seconds in [1, 30, 200, 600, 3600, 86_400] {
        let life = Duration::from_secs(seconds);
        let margin = graphsolve::refresh_margin(life);
        assert!(
            margin > Duration::ZERO && margin < life,
            "a {seconds}s token refreshes at {margin:?}"
        );
    }
}

#[tokio::test]
async fn a_key_near_its_expiry_does_not_exchange_on_every_call() {
    // The platform caps a token's life at the key's own remaining life, so a
    // key in its last minutes mints tokens shorter than the flat 300 s margin.
    // Comparing against that flat value made every cached token stale on
    // arrival, and the token endpoint is rate limited.
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 200).await;
    mount_ok(&server).await;

    let gs = client(&server);
    for _ in 0..20 {
        gs.call("convert_units", &json!({})).await.unwrap();
    }

    assert_eq!(
        requests_to(&server, TOKEN_PATH).await,
        1,
        "20 calls caused more than one token exchange"
    );
}

#[tokio::test]
async fn an_already_expired_token_is_refused_rather_than_cached() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 0).await;
    mount_ok(&server).await;

    let error = client(&server)
        .call("convert_units", &json!({}))
        .await
        .unwrap_err();

    assert!(
        matches!(&error, Error::CredentialsRejected(m) if m.contains("already expired")),
        "{error}"
    );
}

#[tokio::test]
async fn clones_share_one_token() {
    // GraphSolve is Clone with &self methods, so without a shared cache every
    // clone would exchange separately.
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    mount_ok(&server).await;

    let gs = client(&server);
    let clone = gs.clone();
    gs.call("convert_units", &json!({})).await.unwrap();
    clone.call("convert_units", &json!({})).await.unwrap();

    assert_eq!(
        requests_to(&server, TOKEN_PATH).await,
        1,
        "a clone re-exchanged"
    );
}

#[tokio::test]
async fn concurrent_callers_share_one_exchange() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    mount_ok(&server).await;

    let gs = client(&server);
    let args = json!({});
    let (a, b, c, d) = tokio::join!(
        gs.call("convert_units", &args),
        gs.call("convert_units", &args),
        gs.call("convert_units", &args),
        gs.call("convert_units", &args),
    );
    for result in [a, b, c, d] {
        result.unwrap();
    }

    assert_eq!(
        requests_to(&server, TOKEN_PATH).await,
        1,
        "concurrent stampede"
    );
}

#[tokio::test]
async fn a_rejected_api_key_is_not_retried() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(TOKEN_PATH))
        .respond_with(
            ResponseTemplate::new(401).set_body_json(json!({ "error_description": "unknown key" })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let gs = with_retries(&server, 3).build().expect("client");
    let error = gs.call("convert_units", &json!({})).await.unwrap_err();

    assert!(matches!(error, Error::CredentialsRejected(_)), "{error:?}");
    assert!(!error.is_charged());
}

#[tokio::test]
async fn an_unavailable_auth_service_is_reported_as_such() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(TOKEN_PATH))
        .respond_with(ResponseTemplate::new(503).set_body_json(json!({})))
        .mount(&server)
        .await;

    let error = client(&server)
        .call("convert_units", &json!({}))
        .await
        .unwrap_err();

    // Not CredentialsRejected: the API key may well be fine.
    assert!(matches!(error, Error::TokenExchange(_)), "{error:?}");
}

#[tokio::test]
async fn the_api_key_never_appears_in_an_error_or_a_debug_rendering() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(TOKEN_PATH))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({ "error": "forbidden" })))
        .mount(&server)
        .await;

    let gs = client(&server);

    // A derived Debug on the client would reach straight through the token
    // provider and print the key.
    assert!(!format!("{gs:?}").contains(KEY), "{gs:?}");
    assert!(format!("{gs:?}").contains("redacted"), "{gs:?}");

    let error = gs.call("convert_units", &json!({})).await.unwrap_err();
    assert!(matches!(error, Error::CredentialsRejected(_)), "{error:?}");
    assert!(!format!("{error}").contains(KEY));
    assert!(!format!("{error:?}").contains(KEY));
}

#[tokio::test]
async fn me_reports_identity_and_scopes() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    Mock::given(method("GET"))
        .and(path("/v1/me"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "user_id": "u_1", "tenant_id": "acme", "org_id": null,
            "key_id": "key_1", "scopes": ["engine:read", "engine:solve"], "billable": true
        })))
        .mount(&server)
        .await;

    let me = client(&server).me().await.unwrap();

    assert_eq!(me.tenant_id, "acme");
    assert!(me
        .scopes
        .unwrap_or_default()
        .iter()
        .any(|scope| scope == "engine:solve"));
}

// ---------------------------------------------------------------------------
// Retries
// ---------------------------------------------------------------------------

#[tokio::test]
async fn every_call_carries_an_idempotency_key() {
    // A retry after a network timeout must not be charged twice.
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    mount_ok(&server).await;

    let gs = client(&server);
    gs.call("convert_units", &json!({})).await.unwrap();
    gs.call("convert_units", &json!({})).await.unwrap();

    let requests = server.received_requests().await.unwrap();
    let keys: Vec<_> = requests
        .iter()
        .filter(|r| r.url.path().starts_with("/v1/tools/"))
        .map(|r| {
            r.headers
                .get("idempotency-key")
                .expect("no idempotency key")
        })
        .collect();
    assert_ne!(keys[0], keys[1], "two logical calls must not share a key");
}

#[tokio::test]
async fn out_of_credits_is_never_retried() {
    // Unlike a rate limit, waiting does not help.
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    Mock::given(method("POST"))
        .and(path("/v1/tools/solve_network"))
        .respond_with(
            ResponseTemplate::new(402)
                .set_body_json(json!({ "status": "error", "errors": ["Insufficient credits"] })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let gs = with_retries(&server, 3).build().unwrap();
    let error = gs.call("solve_network", &json!({})).await.unwrap_err();

    assert!(matches!(error, Error::InsufficientCredits(_)), "{error:?}");
    assert!(!error.is_charged());
    // `expect(1)` is asserted when the server drops.
}

#[tokio::test]
async fn a_bad_payload_is_never_retried() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    Mock::given(method("POST"))
        .and(path("/v1/tools/solve_network"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_json(json!({ "status": "error", "errors": ["bad field"] })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let gs = with_retries(&server, 3).build().unwrap();
    let error = gs.call("solve_network", &json!({})).await.unwrap_err();

    assert!(matches!(error, Error::InvalidPayload(_)), "{error:?}");
    assert!(!error.is_charged());
}

#[tokio::test]
async fn a_rate_limit_is_retried() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    // Retry-After of zero: the delay itself is asserted in `client::tests`.
    Mock::given(method("POST"))
        .and(path("/v1/tools/solve_network"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "0")
                .set_body_json(json!({ "status": "error", "errors": ["slow down"] })),
        )
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/tools/solve_network"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope("success", 0)))
        .expect(1)
        .mount(&server)
        .await;

    let gs = with_retries(&server, 3).build().unwrap();
    gs.call("solve_network", &json!({})).await.unwrap();

    assert_eq!(
        requests_to(&server, "/v1/tools/").await,
        2,
        "429 was not retried"
    );
}

#[tokio::test]
async fn a_gateway_failure_is_not_retried_on_a_billed_call() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    // The load balancer cannot say whether the engine already ran the tool, and
    // the engine does not read `Idempotency-Key`, so a second POST would be a
    // second charge.
    mount_tool(
        &server,
        "solve_network",
        ResponseTemplate::new(504).set_body_json(json!({ "errors": ["gateway timeout"] })),
    )
    .await;

    let gs = with_retries(&server, 3).build().unwrap();
    let error = gs.call("solve_network", &json!({})).await.unwrap_err();

    assert_eq!(
        requests_to(&server, "/v1/tools/").await,
        1,
        "a 504 on a POST was retried"
    );
    assert!(
        matches!(&error, Error::OutcomeUnknown(m) if m.contains("was charged")),
        "the caller is not told the call may have run: {error}"
    );
}

#[tokio::test]
async fn a_gateway_failure_is_retried_on_a_read() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    Mock::given(method("GET"))
        .and(path("/v1/me"))
        .respond_with(ResponseTemplate::new(502))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/me"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "user_id": "user_1",
            "tenant_id": "tenant_1",
            "org_id": null,
            "key_id": "key_1",
            "scopes": ["engine:read"],
            "billable": true,
        })))
        .mount(&server)
        .await;

    let gs = with_retries(&server, 3).build().unwrap();
    gs.me().await.expect("a GET is safe to repeat");

    assert_eq!(
        requests_to(&server, "/v1/me").await,
        2,
        "a 502 on a GET was not retried"
    );
}

#[tokio::test]
async fn a_rate_limit_that_never_clears_surfaces_as_one() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    Mock::given(method("POST"))
        .and(path("/v1/tools/solve_network"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "0")
                .set_body_json(json!({ "status": "error", "errors": ["slow down"] })),
        )
        .mount(&server)
        .await;

    let gs = with_retries(&server, 2).build().unwrap();
    let error = gs.call("solve_network", &json!({})).await.unwrap_err();

    assert!(matches!(error, Error::RateLimited { .. }), "{error:?}");
    assert_eq!(requests_to(&server, "/v1/tools/").await, 3);
}

// ---------------------------------------------------------------------------
// Charged versus free
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_failed_calculation_is_a_200_and_is_charged() {
    // A non-converged solve consumed compute and IS charged; a bad payload is
    // a 400 and is free. The client must not collapse the two.
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    mount_tool(
        &server,
        "solve_network",
        ResponseTemplate::new(200).set_body_json({
            let mut body = envelope("error", 25);
            body["failure_kind"] = json!("computation_failed");
            body
        }),
    )
    .await;

    let error = client(&server)
        .call("solve_network", &json!({}))
        .await
        .unwrap_err();

    let Error::SolverDidNotConverge {
        credits_charged, ..
    } = &error
    else {
        panic!("wrong variant: {error:?}");
    };
    assert_eq!(
        *credits_charged, 25,
        "the cost must be visible where it is noticed"
    );
    assert!(error.is_charged());
    assert_eq!(
        error.envelope().unwrap().failure_kind,
        Some(graphsolve::FailureKind::ComputationFailed)
    );
}

// ---------------------------------------------------------------------------
// Long-running work
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_quote_spends_nothing() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    Mock::given(method("POST"))
        .and(path("/v1/jobs/run_transient_wave"))
        .and(query_param("dry_run", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "quote": {
                "tool": "run_transient_wave",
                "estimated_credits": 41200,
                "basis": "cells x timesteps"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let quote = client(&server)
        .quote("run_transient_wave", &json!({}))
        .await
        .unwrap();

    assert_eq!(quote.quote.estimated_credits, 41200);
}

#[tokio::test]
async fn run_waits_until_the_job_finishes() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    Mock::given(method("POST"))
        .and(path("/v1/jobs/run_transient_wave"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "job": { "id": "job_1", "state": "queued" }
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/jobs/id/job_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "job": { "id": "job_1", "state": "running", "charged_credits": 0 }
        })))
        .up_to_n_times(2)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/jobs/id/job_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "job": {
                "id": "job_1",
                "state": "succeeded",
                "charged_credits": 3,
                "result": envelope("success", 3)
            }
        })))
        .mount(&server)
        .await;

    let response = client(&server)
        .run(
            "run_transient_wave",
            &json!({}),
            Duration::from_millis(1),
            None,
        )
        .await
        .unwrap();

    assert_eq!(response.credits_charged(), Some(3));
    // Two "running" polls plus the one that finished.
    assert_eq!(requests_to(&server, "/v1/jobs/id/").await, 3);
}

#[tokio::test]
async fn abandoning_the_wait_cancels_the_job_rather_than_the_charge_running_on() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    Mock::given(method("POST"))
        .and(path("/v1/jobs/run_transient_wave"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "job": { "id": "job_1", "state": "queued" }
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/jobs/id/job_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "job": { "id": "job_1", "state": "running" }
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/jobs/id/job_1/cancel"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "job_id": "job_1",
            "state": "cancelled",
            "note": "Compute already under way may run to completion and is charged."
        })))
        .expect(1)
        .mount(&server)
        .await;

    let error = client(&server)
        .run(
            "run_transient_wave",
            &json!({}),
            Duration::from_millis(1),
            Some(Duration::ZERO),
        )
        .await
        .unwrap_err();

    assert!(matches!(error, Error::JobAbandoned { .. }), "{error:?}");
}

// ---------------------------------------------------------------------------
// The generated surface
// ---------------------------------------------------------------------------

#[test]
fn every_tool_in_the_spec_is_named() {
    // Every tool in the spec must have a generated method.
    let spec: serde_json::Value =
        serde_json::from_str(include_str!("../../spec/graphsolve-v1.json")).expect("spec");
    let tools: Vec<&str> = spec["paths"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(path, item)| path.starts_with("/v1/tools/") && item.get("post").is_some())
        .map(|(_, item)| item["post"]["operationId"].as_str().unwrap())
        .collect();

    assert_eq!(tools.len(), 98);
    for tool in tools {
        assert!(TOOL_NAMES.contains(&tool), "{tool} has no method");
    }
}

#[tokio::test]
async fn a_generated_method_forwards_to_the_named_tool() {
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    Mock::given(method("POST"))
        .and(path("/v1/tools/calculate_pressure_drop"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope("success", 1)))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .calculate_pressure_drop(CalculatePressureDropParams {
            angle: 90.0,
            correlation: FlowCorrelationName::BeggsBrill,
            density: vec![800.0, 20.0],
            diameter: 0.15,
            ift: 0.02,
            pressure: 10.0,
            roughness: 4.5e-5,
            velocity: vec![1.2, 3.4],
            viscosity: vec![1.0e-3, 1.8e-5],
        })
        .await
        .unwrap();

    let requests = server.received_requests().await.unwrap();
    let body: serde_json::Value = requests
        .iter()
        .find(|r| r.url.path().starts_with("/v1/tools/"))
        .expect("no tool call")
        .body_json()
        .unwrap();
    // The enum must serialise to the string the engine accepts.
    assert_eq!(body["correlation"], "Beggs-Brill");
    assert_eq!(body["angle"], 90.0);
}

#[tokio::test]
async fn an_unset_optional_is_absent_from_the_payload_not_null() {
    // An unset optional must be absent from the payload, not sent as null.
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    mount_tool(
        &server,
        "calculate_compressor",
        ResponseTemplate::new(200).set_body_json(envelope("success", 1)),
    )
    .await;

    client(&server)
        .calculate_compressor(CalculateCompressorParams {
            inlet_pressure: 5.0,
            inlet_temperature: 350.0,
            pressure_ratio: 2.0,
            fluid: serde_json::json!({ "gas_rate": 500_000, "gas_mw": 18.5 }),
            ..Default::default()
        })
        .await
        .unwrap();

    let requests = server.received_requests().await.unwrap();
    let body: serde_json::Value = requests
        .iter()
        .find(|r| r.url.path().starts_with("/v1/tools/"))
        .expect("no tool call")
        .body_json()
        .unwrap();
    let object = body.as_object().unwrap();
    assert!(
        !object.contains_key("mechanical_efficiency"),
        "unset optional was sent: {body}"
    );
    assert!(!object.contains_key("polytropic_efficiency"));
}

#[tokio::test]
async fn a_blob_parameter_accepts_an_object() {
    // The *_json parameters are anyOf[object, array, string].
    let server = MockServer::start().await;
    mount_token(&server, "tok-1", 3600).await;
    mount_tool(
        &server,
        "solve_network",
        ResponseTemplate::new(200).set_body_json(envelope("success", 25)),
    )
    .await;

    client(&server)
        .solve_network(SolveNetworkParams {
            network_json: json!({ "graph_data": { "nodes": [] } }),
        })
        .await
        .unwrap();

    let requests = server.received_requests().await.unwrap();
    let body: serde_json::Value = requests
        .iter()
        .find(|r| r.url.path().starts_with("/v1/tools/"))
        .expect("no tool call")
        .body_json()
        .unwrap();
    assert!(body["network_json"].is_object());
}
