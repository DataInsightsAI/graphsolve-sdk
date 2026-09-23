//! Errors returned by the client.
//!
//! One variant per failure mode the API distinguishes, because the right
//! response differs in each case, and so does whether the call was charged.

use std::time::Duration;

use crate::types::ToolResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// 401 — the access token was missing, invalid or expired.
    #[error("authentication failed: {0}")]
    Authentication(String),

    /// The API key was rejected when exchanging it for a token.
    ///
    /// Distinct from [`Error::TokenExchange`]: this one will not fix itself.
    #[error("this API key was rejected: {0}")]
    CredentialsRejected(String),

    /// The token could not be obtained, but the API key may well be fine — the
    /// authentication service was unreachable or answered with a 5xx.
    #[error("token exchange failed: {0}")]
    TokenExchange(String),

    /// 403 — the token lacks the scope this tool needs: `engine:read` for free
    /// lookups, `engine:solve` for anything that runs a calculation.
    #[error("tool not permitted: {0}")]
    ToolNotPermitted(String),

    /// 402 — nothing left to spend.
    ///
    /// Never retried: unlike a rate limit, waiting does not help.
    #[error("insufficient credits: {0}")]
    InsufficientCredits(String),

    /// 429 — too many requests, or the compute pool is saturated.
    #[error("rate limited: {message}")]
    RateLimited {
        message: String,
        /// Seconds to wait, from `Retry-After`, when the server said.
        retry_after: Option<f64>,
    },

    /// 400 — the request was wrong. Nothing was computed and nothing was charged.
    #[error("invalid payload: {0}")]
    InvalidPayload(String),

    /// The calculation ran and produced no answer.
    ///
    /// Not a rejected request. It arrives as a 200 and IS charged, because it
    /// consumed compute. An error rather than an `Ok` because the envelope has
    /// no `result` to unpack.
    #[error("{message}")]
    SolverDidNotConverge {
        message: String,
        /// On the error so the cost is visible here.
        credits_charged: u64,
        envelope: Box<ToolResponse>,
    },

    /// The call was abandoned without knowing whether it ran.
    ///
    /// A 502, a 504 or a dropped connection on a request that is not safe to
    /// repeat. The engine may already have run the tool, and a tool that ran
    /// is charged, so the client does not send it again. Reconcile before
    /// repeating it by hand.
    #[error("{0}")]
    OutcomeUnknown(String),

    /// 5xx — our fault. Not charged.
    ///
    /// A 502 or 504 on a request that cannot be repeated is
    /// [`Error::OutcomeUnknown`] instead: there, whether it was charged is
    /// exactly what is not known.
    #[error("server error: {0}")]
    Server(String),

    /// Anything else the API answered with.
    #[error("HTTP {status}: {message}")]
    Http { status: u16, message: String },

    /// A job exceeded `max_wait` and was cancelled.
    ///
    /// Cancelled rather than left running, so it stops spending credits.
    #[error("job {job_id} exceeded max_wait of {waited:?} and was cancelled")]
    JobAbandoned { job_id: String, waited: Duration },

    /// The request never got an answer.
    #[error("could not reach GraphSolve: {0}")]
    Transport(#[from] reqwest::Error),

    /// A response that did not parse as the shape it claims to be.
    #[error("could not decode the response: {0}")]
    Decode(#[from] serde_json::Error),

    /// No API key was passed and the environment does not carry one.
    #[error("no API key: pass one to the builder or set GRAPHSOLVE_API_KEY")]
    NoCredentials,
}

impl Error {
    /// Whether this failure consumed compute and was therefore charged.
    ///
    /// Only `SolverDidNotConverge` is.
    #[must_use]
    pub fn is_charged(&self) -> bool {
        matches!(self, Error::SolverDidNotConverge { .. })
    }

    /// The response envelope, when the failure carried one.
    #[must_use]
    pub fn envelope(&self) -> Option<&ToolResponse> {
        match self {
            Error::SolverDidNotConverge { envelope, .. } => Some(envelope),
            _ => None,
        }
    }
}
