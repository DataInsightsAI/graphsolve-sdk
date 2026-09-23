//! The GraphSolve client.
//!
//! Hand-written rather than generated. The behaviour here — which status codes
//! are retried, which failures are charged, and the idempotency key — is the
//! same in every client in this repo and is documented in CONTRIBUTING.md.

use std::time::Duration;

use rand::Rng;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Method, StatusCode};
use serde::Serialize;

use crate::auth::TokenProvider;
use crate::error::{Error, Result};
use crate::types::{
    CancelResponse, Job, Me, QuoteResponse, Status, SubmitResponse, ToolResponse, ToolSummary,
};

pub const DEFAULT_BASE_URL: &str = "https://api-prod.graphsolve.ai";

/// The platform endpoint that issues access tokens. Tokens are issued centrally
/// for every deployment, so this is not derived from the base URL.
pub const DEFAULT_TOKEN_URL: &str = "https://prod-user-api.graphsolve.ai/oauth/token";

/// Solves can take minutes. The server's load balancer allows 900 s; a shorter
/// client timeout would abandon work that is still running and still charged.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(900);

const ENV_API_KEY: &str = "GRAPHSOLVE_API_KEY";
const ENV_BASE_URL: &str = "GRAPHSOLVE_BASE_URL";
const ENV_TOKEN_URL: &str = "GRAPHSOLVE_TOKEN_URL";

/// Enough to ride out a deploy or a throttle without a caller waiting minutes
/// to be told something is wrong.
const DEFAULT_MAX_RETRIES: u32 = 3;

/// Caps the doubling. Never reached at 3 retries; guards against a larger
/// max_retries producing an absurd delay.
const MAX_BACKOFF_SHIFT: u32 = 6;

/// Backoff multiplier, so clients throttled at the same time do not all retry
/// at the same time.
const JITTER: std::ops::Range<f64> = 0.5..1.5;

/// Said alongside [`Error::OutcomeUnknown`]. The variant is what a caller
/// matches on; this is what a human reads in a log.
const MAY_HAVE_RUN: &str = "This request was not retried: it may have reached \
     the engine and run, in which case it was charged.";

/// Whether a status may be retried whatever the method.
///
/// Neither reached the engine, so repeating the request cannot duplicate a
/// charge. Any other 4xx fails the same way every time. The token exchange
/// uses this plus [`is_retryable_if_repeatable`]: it runs no tool and is
/// charged nothing, so a 502 or 504 there is safe to repeat.
pub(crate) fn is_retryable(status: StatusCode) -> bool {
    matches!(status.as_u16(), 429 | 503)
}

/// Whether a status may be retried only when the request is safe to repeat.
///
/// A 502 or 504 comes from the load balancer, which cannot say whether the
/// engine already ran the tool — and a tool that ran is metered whether or not
/// its response arrived. The engine does not read `Idempotency-Key`, so
/// repeating a POST would be charged twice.
pub(crate) fn is_retryable_if_repeatable(status: StatusCode) -> bool {
    matches!(status.as_u16(), 502 | 504)
}

/// The error for a call abandoned without knowing whether it ran.
fn outcome_unknown(message: &str) -> Error {
    Error::OutcomeUnknown(format!(
        "{}. {MAY_HAVE_RUN}",
        message.trim_end().trim_end_matches('.')
    ))
}

/// A client for the GraphSolve engine API.
///
/// ```no_run
/// # async fn f() -> graphsolve::Result<()> {
/// use graphsolve::{ConvertUnitsParams, GraphSolve};
///
/// let gs = GraphSolve::new()?;         // reads GRAPHSOLVE_API_KEY
/// let response = gs
///     .convert_units(ConvertUnitsParams {
///         value: 1.0,
///         from_unit: "MPa".into(),
///         to_unit: "psi".into(),
///         ..Default::default()
///     })
///     .await?;
/// # Ok(()) }
/// ```
///
/// One typed method per tool, generated from the API spec, so a wrong
/// correlation name is a compile error rather than a 400 from the server. Use
/// [`call`](GraphSolve::call) for a tool newer than this version.
///
/// The API key is exchanged for an access token on the first call and the token
/// is refreshed before it expires. Cloning is cheap and clones share one token.
#[derive(Debug, Clone)]
pub struct GraphSolve {
    http: reqwest::Client,
    base_url: String,
    tokens: TokenProvider,
    max_retries: u32,
}

impl GraphSolve {
    /// Read the API key from the environment and take every other default.
    ///
    /// # Errors
    ///
    /// [`Error::NoCredentials`] if `GRAPHSOLVE_API_KEY` is unset or empty.
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    /// Use an explicit API key.
    ///
    /// # Errors
    ///
    /// [`Error::Transport`] if the underlying HTTP client cannot be built.
    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// The current access token, for a caller wiring their own HTTP.
    ///
    /// # Errors
    ///
    /// [`Error::CredentialsRejected`] if the API key is wrong, or
    /// [`Error::TokenExchange`] if the authentication service could not be
    /// reached.
    pub async fn access_token(&self) -> Result<String> {
        self.tokens.token().await
    }

    #[must_use]
    pub fn builder() -> Builder {
        Builder::default()
    }

    // ----------------------------------------------------------------- tools

    /// Run a tool and return its response envelope.
    ///
    /// Every generated method calls this. Use it directly for a tool newer
    /// than this client.
    ///
    /// # Errors
    ///
    /// [`Error::SolverDidNotConverge`] when the calculation ran and produced no
    /// answer. That arrives as a 200 and is charged. Every other variant means
    /// nothing was computed and nothing was billed.
    pub async fn call<T>(&self, tool: &str, args: &T) -> Result<ToolResponse>
    where
        T: Serialize + ?Sized,
    {
        let body = serde_json::to_value(args)?;
        let envelope: ToolResponse = self
            .request(Method::POST, &format!("/v1/tools/{tool}"), Some(body), &[])
            .await?;
        check_envelope(envelope, tool)
    }

    /// Every tool, with its domain, summary and price.
    ///
    /// # Errors
    ///
    /// The API or transport error behind the failure.
    pub async fn tools(&self) -> Result<Vec<ToolSummary>> {
        #[derive(serde::Deserialize)]
        struct Wrapper {
            tools: Vec<ToolSummary>,
        }
        let body: Wrapper = self.request(Method::GET, "/v1/tools", None, &[]).await?;
        Ok(body.tools)
    }

    /// One tool's full definition: schema, examples, price.
    ///
    /// # Errors
    ///
    /// [`Error::Http`] with `404` if there is no such tool.
    pub async fn describe(&self, tool: &str) -> Result<serde_json::Value> {
        self.request(Method::GET, &format!("/v1/tools/{tool}"), None, &[])
            .await
    }

    /// The identity and scopes the API key's token carries.
    ///
    /// Balances are held by the platform, not the engine, so they are not
    /// reported here.
    ///
    /// # Errors
    ///
    /// The API or transport error behind the failure.
    pub async fn me(&self) -> Result<Me> {
        self.request(Method::GET, "/v1/me", None, &[]).await
    }

    // ------------------------------------------------------------------ jobs

    /// Price a long-running call without running it.
    ///
    /// Spends nothing: no compute, no job. Worth calling before a transient
    /// run, where one field can change the cost by orders of magnitude.
    ///
    /// # Errors
    ///
    /// The API or transport error behind the failure.
    pub async fn quote<T>(&self, tool: &str, args: &T) -> Result<QuoteResponse>
    where
        T: Serialize + ?Sized,
    {
        let body = serde_json::to_value(args)?;
        self.request(
            Method::POST,
            &format!("/v1/jobs/{tool}"),
            Some(body),
            &[("dry_run", "true")],
        )
        .await
    }

    /// Start a job. Returns immediately with the job and its quote.
    ///
    /// # Errors
    ///
    /// [`Error::ToolNotPermitted`] if the API key lacks the scope the tool
    /// needs.
    pub async fn submit<T>(&self, tool: &str, args: &T) -> Result<SubmitResponse>
    where
        T: Serialize + ?Sized,
    {
        let body = serde_json::to_value(args)?;
        self.request(Method::POST, &format!("/v1/jobs/{tool}"), Some(body), &[])
            .await
    }

    /// Poll a job.
    ///
    /// # Errors
    ///
    /// [`Error::Http`] with `404` for a job id belonging to another caller.
    pub async fn job(&self, job_id: &str) -> Result<Job> {
        #[derive(serde::Deserialize)]
        struct Wrapper {
            job: Job,
        }
        let body: Wrapper = self
            .request(Method::GET, &format!("/v1/jobs/id/{job_id}"), None, &[])
            .await?;
        Ok(body.job)
    }

    /// Cancel a job. Compute already under way may run to completion and is
    /// charged.
    ///
    /// # Errors
    ///
    /// [`Error::Http`] with `404` if the job is not this caller's, or `409` if
    /// it has already finished.
    pub async fn cancel(&self, job_id: &str) -> Result<CancelResponse> {
        self.request(
            Method::POST,
            &format!("/v1/jobs/id/{job_id}/cancel"),
            None,
            &[],
        )
        .await
    }

    /// Submit a job and wait for it to finish.
    ///
    /// Cancels the job if `max_wait` is exceeded, so giving up on the wait
    /// also stops the charge.
    ///
    /// # Errors
    ///
    /// [`Error::JobAbandoned`] if `max_wait` runs out.
    /// [`Error::SolverDidNotConverge`] if the job finishes without an answer,
    /// which consumed compute and is charged.
    pub async fn run<T>(
        &self,
        tool: &str,
        args: &T,
        poll_interval: Duration,
        max_wait: Option<Duration>,
    ) -> Result<ToolResponse>
    where
        T: Serialize + ?Sized,
    {
        let submitted = self.submit(tool, args).await?;
        let job_id = submitted.job.id;
        let started = std::time::Instant::now();

        let job = loop {
            let job = self.job(&job_id).await?;
            if job.state.is_terminal() {
                break job;
            }
            if max_wait.is_some_and(|limit| started.elapsed() > limit) {
                self.cancel(&job_id).await?;
                return Err(Error::JobAbandoned {
                    job_id,
                    waited: started.elapsed(),
                });
            }
            tokio::time::sleep(poll_interval).await;
        };

        match job.result {
            Some(result) if job.state == crate::types::JobState::Succeeded => Ok(result),
            result => Err(Error::SolverDidNotConverge {
                message: format!(
                    "job {job_id} finished as {:?}: {}",
                    job.state,
                    job.error.unwrap_or_else(|| "no result".into())
                ),
                credits_charged: job.charged_credits,
                envelope: Box::new(result.unwrap_or_default()),
            }),
        }
    }

    // -------------------------------------------------------------- internal

    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}{path}", self.base_url);
        // One key per logical call, reused across its retries. The engine does
        // not read it yet; send it so that a server that does needs no change
        // here. Until then, not retrying a POST is what stops a double charge.
        let idempotency_key = uuid::Uuid::new_v4().to_string();
        let repeatable = method == Method::GET;
        let mut attempt = 0;
        loop {
            // Read per attempt, so a long retry sequence cannot outlive the
            // token. Outside the send, so a credential failure is not mistaken
            // for a network one and retried.
            let authorization = format!("Bearer {}", self.tokens.token().await?);

            let mut request = self
                .http
                .request(method.clone(), &url)
                .query(query)
                .header(AUTHORIZATION, &authorization)
                .header("idempotency-key", &idempotency_key);
            if let Some(body) = &body {
                request = request.header(CONTENT_TYPE, "application/json").json(body);
            }

            let response = match request.send().await {
                Ok(response) => response,
                // A POST that got no answer may still have been received and
                // run, so it is reported rather than repeated.
                Err(error) => {
                    if !repeatable {
                        return Err(outcome_unknown(&format!(
                            "could not reach GraphSolve: {error}"
                        )));
                    }
                    if attempt >= self.max_retries {
                        return Err(Error::Transport(error));
                    }
                    tokio::time::sleep(backoff(attempt, None)).await;
                    attempt += 1;
                    continue;
                }
            };

            let status = response.status();
            let may_retry =
                is_retryable(status) || (repeatable && is_retryable_if_repeatable(status));
            if may_retry && attempt < self.max_retries {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .map(str::to_owned);
                tokio::time::sleep(backoff(attempt, retry_after.as_deref())).await;
                attempt += 1;
                continue;
            }

            return decode(response, repeatable).await;
        }
    }
}

/// Exponential backoff with jitter, deferring to `Retry-After`.
///
/// Kept pure so the delay can be asserted directly in tests. Shared with the
/// token exchange, which follows the same policy.
pub(crate) fn backoff(attempt: u32, retry_after: Option<&str>) -> Duration {
    if let Some(seconds) = retry_after.and_then(|v| v.parse::<f64>().ok()) {
        if seconds.is_finite() && seconds >= 0.0 {
            return Duration::from_secs_f64(seconds);
        }
    }
    let jitter = rand::rng().random_range(JITTER);
    Duration::from_secs_f64(f64::from(1u32 << attempt.min(MAX_BACKOFF_SHIFT)) * jitter)
}

async fn decode<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
    repeatable: bool,
) -> Result<T> {
    let status = response.status();
    let text = response.text().await.unwrap_or_default();
    let body: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);

    if status.is_success() {
        return Ok(serde_json::from_value(body)?);
    }

    let message = body
        .get("errors")
        .and_then(|e| e.get(0))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|| {
            status
                .canonical_reason()
                .unwrap_or("unknown error")
                .to_owned()
        });

    Err(match status.as_u16() {
        400 => Error::InvalidPayload(message),
        401 => Error::Authentication(message),
        402 => Error::InsufficientCredits(message),
        403 => Error::ToolNotPermitted(message),
        429 => Error::RateLimited {
            message,
            retry_after: None,
        },
        // 502 and 504 are the load balancer's, not the engine's: the tool may
        // have run and been metered. Only a POST reaches here unretried.
        _ if is_retryable_if_repeatable(status) && !repeatable => outcome_unknown(&message),
        code if code >= 500 => Error::Server(message),
        code => Error::Http {
            status: code,
            message,
        },
    })
}

/// Return an error if a 200 response reports a failed calculation.
///
/// The call ran and is charged, but there is no `result` to unpack, so fail
/// here rather than let the caller fail further downstream.
fn check_envelope(envelope: ToolResponse, tool: &str) -> Result<ToolResponse> {
    if envelope.status == Status::Success {
        return Ok(envelope);
    }
    let credits_charged = envelope.credits_charged().unwrap_or(0);
    let message = format!(
        "{tool}: {}",
        envelope
            .errors
            .first()
            .map(String::as_str)
            .unwrap_or("no result")
    );
    Err(Error::SolverDidNotConverge {
        message,
        credits_charged,
        envelope: Box::new(envelope),
    })
}

/// Builds a [`GraphSolve`].
#[derive(Debug, Default)]
pub struct Builder {
    api_key: Option<String>,
    base_url: Option<String>,
    token_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    http: Option<reqwest::Client>,
}

impl Builder {
    #[must_use]
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Override the engine API, for a non-production deployment.
    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Override the token endpoint, for a non-production platform. Set it
    /// alongside [`base_url`](Builder::base_url): it is never derived from it.
    #[must_use]
    pub fn token_url(mut self, url: impl Into<String>) -> Self {
        self.token_url = Some(url.into());
        self
    }

    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    #[must_use]
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// Supply your own HTTP client: a proxy, an existing connection pool, or a
    /// shorter timeout for a test.
    #[must_use]
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }

    /// # Errors
    ///
    /// [`Error::NoCredentials`] when no API key is given to the builder or set
    /// in the environment, or [`Error::Transport`] if the HTTP client cannot be
    /// built.
    ///
    /// Only the network exchange is lazy; a missing key is a programming error
    /// and surfaces as one, here.
    pub fn build(self) -> Result<GraphSolve> {
        let api_key = self
            .api_key
            .or_else(|| std::env::var(ENV_API_KEY).ok())
            .filter(|value| !value.is_empty())
            .ok_or(Error::NoCredentials)?;

        let http = match self.http {
            Some(client) => client,
            None => reqwest::Client::builder()
                .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
                .user_agent(concat!("graphsolve-rust/", env!("CARGO_PKG_VERSION")))
                .build()?,
        };

        let base_url = setting(self.base_url, ENV_BASE_URL, DEFAULT_BASE_URL)
            .trim_end_matches('/')
            .to_owned();
        let token_url = setting(self.token_url, ENV_TOKEN_URL, DEFAULT_TOKEN_URL);
        let max_retries = self.max_retries.unwrap_or(DEFAULT_MAX_RETRIES);

        Ok(GraphSolve {
            tokens: TokenProvider::new(api_key, token_url, http.clone(), max_retries),
            http,
            base_url,
            max_retries,
        })
    }
}

/// The builder's value, else the environment's, else the default. An empty
/// environment variable counts as unset.
fn setting(given: Option<String>, env: &str, default: &str) -> String {
    given
        .or_else(|| std::env::var(env).ok())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn retry_after_wins_outright() {
        assert_eq!(backoff(0, Some("7")), Duration::from_secs(7));
        assert_eq!(backoff(9, Some("0.5")), Duration::from_millis(500));
    }

    #[test]
    fn a_retry_after_we_cannot_read_falls_back_to_the_curve() {
        // Some gateways send an HTTP-date rather than seconds.
        for header in ["Wed, 21 Oct 2026 07:28:00 GMT", "", "-1"] {
            let delay = backoff(0, Some(header));
            assert!(
                (Duration::from_millis(500)..Duration::from_millis(1500)).contains(&delay),
                "{header:?} gave {delay:?}"
            );
        }
    }

    #[test]
    fn backoff_grows_and_is_jittered() {
        // Without jitter, clients throttled at the same time all retry at the
        // same time.
        let samples: Vec<Duration> = (0..32).map(|_| backoff(3, None)).collect();
        assert!(
            samples
                .iter()
                .all(|d| (Duration::from_secs(4)..Duration::from_secs(12)).contains(d)),
            "{samples:?}"
        );
        assert!(
            samples.iter().collect::<HashSet<_>>().len() > 1,
            "identical delays: the jitter is not doing anything"
        );
        assert!(backoff(0, None) < backoff(6, None), "backoff does not grow");
    }

    #[test]
    fn only_a_throttle_or_an_unserved_request_is_retried_on_any_method() {
        for code in [429, 503] {
            assert!(is_retryable(StatusCode::from_u16(code).unwrap()), "{code}");
        }
        // A 402 means out of credits and a 400 means the payload is wrong.
        // Repeating either changes nothing.
        for code in [200, 400, 401, 402, 403, 404, 500] {
            assert!(!is_retryable(StatusCode::from_u16(code).unwrap()), "{code}");
        }
    }

    #[test]
    fn a_gateway_failure_is_retried_only_where_repeating_is_free() {
        // The engine may already have run the tool and metered it, and it does
        // not read `Idempotency-Key`, so a POST is never repeated on these.
        for code in [502, 504] {
            let status = StatusCode::from_u16(code).unwrap();
            assert!(is_retryable_if_repeatable(status), "{code}");
            assert!(!is_retryable(status), "{code} retried whatever the method");
        }
        for code in [429, 500, 503, 400, 200] {
            let status = StatusCode::from_u16(code).unwrap();
            assert!(!is_retryable_if_repeatable(status), "{code}");
        }
    }

    #[test]
    fn a_call_whose_outcome_is_unknown_says_so() {
        let rendered = outcome_unknown("server error").to_string();
        assert!(rendered.contains("was charged"), "{rendered}");
        // One full stop, not two, whether or not the message brought its own.
        assert_eq!(outcome_unknown("server error.").to_string(), rendered);
    }
}
