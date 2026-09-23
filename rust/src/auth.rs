//! API key exchange and token caching.
//!
//! The client holds an API key and trades it at the platform's token endpoint
//! for a short-lived access token. The token is refreshed before it expires
//! rather than after a 401, which keeps authentication off the retry path
//! entirely.

use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::Deserialize;

use crate::error::{Error, Result};

/// The most of a token's life to give up to the refresh margin. Wide enough
/// that a request started just before the check cannot arrive after expiry.
pub const MAX_REFRESH_MARGIN: Duration = Duration::from_secs(300);

/// How long before expiry to exchange a token that lives `expires_in`.
///
/// Capped at half the token's life, because the platform caps a token's
/// lifetime at the API key's own remaining life: a key in its last five
/// minutes mints tokens shorter than the flat margin, which would make every
/// cached token stale on arrival and every call exchange again.
#[must_use]
pub fn refresh_margin(expires_in: Duration) -> Duration {
    MAX_REFRESH_MARGIN.min(expires_in / 2)
}

/// The exchange is a small POST, not a solve, so it gets its own timeout.
const TOKEN_TIMEOUT: Duration = Duration::from_secs(10);

/// The OAuth grant the platform expects when a key is exchanged for a token.
const GRANT_TYPE: &str = "client_credentials";

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Deserialize)]
struct TokenErrorBody {
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Clone)]
struct CachedToken {
    value: String,
    /// When to exchange again — already inside the token's life by
    /// [`refresh_margin`], so a cached token is never served past expiry.
    refresh_at: Instant,
}

impl CachedToken {
    fn is_fresh(&self) -> bool {
        Instant::now() < self.refresh_at
    }
}

/// Fetches, caches and refreshes the access token.
///
/// The cache is behind an `Arc<Mutex<…>>` because [`GraphSolve`](crate::GraphSolve)
/// is `Clone` with `&self` methods: without sharing, every clone would keep its
/// own token and exchange separately. The mutex is `tokio`'s rather than
/// `std`'s because the exchange awaits while held, which is what makes
/// concurrent callers share one exchange rather than each starting their own.
pub(crate) struct TokenProvider {
    api_key: String,
    token_url: String,
    http: reqwest::Client,
    max_retries: u32,
    cache: Arc<tokio::sync::Mutex<Option<CachedToken>>>,
}

/// Written out rather than derived, so a `{:?}` of the client cannot print the
/// API key. A derived `Debug` on the client would reach straight through to it.
impl std::fmt::Debug for TokenProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenProvider")
            .field("api_key", &"<redacted>")
            .field("token_url", &self.token_url)
            .finish_non_exhaustive()
    }
}

impl Clone for TokenProvider {
    fn clone(&self) -> Self {
        Self {
            api_key: self.api_key.clone(),
            token_url: self.token_url.clone(),
            http: self.http.clone(),
            max_retries: self.max_retries,
            // Shared, not copied: a clone must see the same token.
            cache: Arc::clone(&self.cache),
        }
    }
}

impl TokenProvider {
    pub(crate) fn new(
        api_key: String,
        token_url: String,
        http: reqwest::Client,
        max_retries: u32,
    ) -> Self {
        Self {
            api_key,
            token_url,
            http,
            max_retries,
            cache: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    /// The current access token, exchanging for a new one if needed.
    pub(crate) async fn token(&self) -> Result<String> {
        let mut cache = self.cache.lock().await;
        if let Some(cached) = cache.as_ref() {
            if cached.is_fresh() {
                return Ok(cached.value.clone());
            }
        }
        let fresh = self.exchange().await?;
        let value = fresh.value.clone();
        *cache = Some(fresh);
        Ok(value)
    }

    async fn exchange(&self) -> Result<CachedToken> {
        let mut attempt = 0;
        loop {
            let response = match self
                .http
                .post(&self.token_url)
                .bearer_auth(&self.api_key)
                .form(&[("grant_type", GRANT_TYPE)])
                .timeout(TOKEN_TIMEOUT)
                .send()
                .await
            {
                Ok(response) => response,
                Err(error) => {
                    if attempt >= self.max_retries {
                        return Err(Error::TokenExchange(format!(
                            "could not reach the authentication service: {error}"
                        )));
                    }
                    tokio::time::sleep(crate::client::backoff(attempt, None)).await;
                    attempt += 1;
                    continue;
                }
            };

            let status = response.status();
            // Wider than the client's own policy: the exchange runs no tool and
            // is charged nothing, so repeating it after a 502 or 504 cannot
            // cost anything.
            let may_retry = crate::client::is_retryable(status)
                || crate::client::is_retryable_if_repeatable(status);
            if may_retry && attempt < self.max_retries {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .map(str::to_owned);
                tokio::time::sleep(crate::client::backoff(attempt, retry_after.as_deref())).await;
                attempt += 1;
                continue;
            }

            return store(response, status).await;
        }
    }
}

async fn store(response: reqwest::Response, status: reqwest::StatusCode) -> Result<CachedToken> {
    let text = response.text().await.unwrap_or_default();

    if status.is_client_error() || status.is_server_error() {
        return Err(exchange_error(status, &text));
    }

    let parsed: TokenResponse = serde_json::from_str(&text).map_err(|_| {
        Error::TokenExchange(
            "the authentication service returned a token response we could not read".to_owned(),
        )
    })?;

    // Refusing here beats caching a token that is stale on arrival, which would
    // mean one exchange per call for as long as the key lasts.
    if parsed.expires_in == 0 {
        return Err(Error::CredentialsRejected(
            "the authentication service issued a token that has already expired; \
             this API key is at or past its expiry, so issue a new one"
                .to_owned(),
        ));
    }

    let expires_in = Duration::from_secs(parsed.expires_in);
    Ok(CachedToken {
        value: parsed.access_token,
        refresh_at: Instant::now() + expires_in - refresh_margin(expires_in),
    })
}

/// Separate "the service is down" from "your API key is wrong".
fn exchange_error(status: reqwest::StatusCode, body: &str) -> Error {
    if status.is_server_error() {
        return Error::TokenExchange(format!(
            "the authentication service is unavailable (HTTP {})",
            status.as_u16()
        ));
    }
    let detail = serde_json::from_str::<TokenErrorBody>(body)
        .ok()
        .and_then(|b| b.error_description.or(b.error));
    match detail {
        Some(detail) => Error::CredentialsRejected(detail),
        None => Error::CredentialsRejected(format!("HTTP {}", status.as_u16())),
    }
}
