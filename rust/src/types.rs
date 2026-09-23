//! The wire types: what the API sends back.
//!
//! Field names match the API, so serde needs no renaming.

use serde::{Deserialize, Serialize};

/// Why a call failed.
///
/// `InvalidInput` is the caller's to fix and is not charged. `ComputationFailed`
/// means the calculation ran and produced no answer; it consumed compute and IS
/// charged. `Internal` is a server fault and is not charged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    InvalidInput,
    ComputationFailed,
    Internal,
}

/// What a call cost. Balances are held by the platform, not reported here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Billing {
    #[serde(default)]
    pub tool: String,
    pub credits_charged: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ToolMetadata {
    #[serde(default)]
    pub calculation_time_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub converged: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iterations: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing: Option<Billing>,
}

/// The standard response envelope.
///
/// `result` is the tool's own payload and differs per tool, so it stays a
/// [`serde_json::Value`] unless the caller deserialises it with
/// [`ToolResponse::result_as`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResponse {
    /// Not `#[serde(default)]`: a response missing its status would otherwise
    /// decode as a success.
    pub status: Status,
    #[serde(default)]
    pub result: serde_json::Value,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub errors: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_kind: Option<FailureKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ToolMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Success,
    Error,
}

/// Hand-written rather than derived. The only place an envelope is built from
/// nothing is a failure path, so the default status is Error.
impl Default for ToolResponse {
    fn default() -> Self {
        Self {
            status: Status::Error,
            result: serde_json::Value::Null,
            warnings: Vec::new(),
            errors: Vec::new(),
            failure_kind: None,
            metadata: None,
        }
    }
}

impl ToolResponse {
    /// What this call cost, when the server said.
    #[must_use]
    pub fn credits_charged(&self) -> Option<u64> {
        self.metadata
            .as_ref()?
            .billing
            .as_ref()
            .map(|billing| billing.credits_charged)
    }

    /// Deserialise `result` into a shape of the caller's own.
    ///
    /// # Errors
    ///
    /// [`Error::Decode`](crate::Error::Decode) if the payload is not that shape.
    pub fn result_as<'de, T: Deserialize<'de>>(&'de self) -> crate::Result<T> {
        // Borrows the `Value` in place. `serde_json::from_value` would need an
        // owned clone of a payload that can be a whole network.
        Ok(T::deserialize(&self.result)?)
    }
}

/// One entry from [`GraphSolve::tools`](crate::GraphSolve::tools).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolSummary {
    pub name: String,
    pub domain: String,
    pub summary: String,
    pub cost: Cost,
}

/// A tool's price: a flat charge, plus compute beyond a free window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cost {
    pub base_credits: u32,
    pub free_ms: u32,
    pub ms_per_credit: u32,
}

/// The identity and scopes an access token carries, from
/// [`GraphSolve::me`](crate::GraphSolve::me).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Me {
    pub user_id: String,
    pub tenant_id: String,
    #[serde(default)]
    pub org_id: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    /// `None` for the platform's own unscoped traffic.
    #[serde(default)]
    pub scopes: Option<Vec<String>>,
    #[serde(default)]
    pub billable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    Queued,
    Running,
    Succeeded,
    /// Ran and produced no answer, or the tool rejected the request.
    Failed,
    Cancelled,
}

impl JobState {
    #[must_use]
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            JobState::Succeeded | JobState::Failed | JobState::Cancelled
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub tenant_id: String,
    #[serde(default)]
    pub tool: String,
    pub state: JobState,
    /// The quote the job was submitted against.
    #[serde(default)]
    pub estimated_credits: u64,
    /// What the run actually cost. Zero until it finishes.
    #[serde(default)]
    pub charged_credits: u64,
    #[serde(default)]
    pub submitted_at: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<ToolResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// What a job would cost, and why.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    #[serde(default)]
    pub tool: String,
    pub estimated_credits: u64,
    /// How the number was arrived at, so a surprising quote can be argued with.
    #[serde(default)]
    pub basis: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_runtime_s: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub quote: Quote,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitResponse {
    pub job: Job,
    #[serde(default)]
    pub quote: Option<Quote>,
}

/// The answer to [`GraphSolve::cancel`](crate::GraphSolve::cancel).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelResponse {
    pub job_id: String,
    pub state: JobState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_envelope_without_a_status_does_not_decode_as_success() {
        // Failing open would let a non-converged solve read as an answer.
        let result = serde_json::from_str::<ToolResponse>(r#"{"warnings":[],"errors":[]}"#);
        assert!(result.is_err(), "accepted {result:?}");
        assert_eq!(ToolResponse::default().status, Status::Error);
    }

    #[test]
    fn a_result_deserialises_without_cloning_the_payload() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Gradient {
            total: f64,
        }
        let response = ToolResponse {
            status: Status::Success,
            result: serde_json::json!({ "total": 1.25 }),
            ..Default::default()
        };
        assert_eq!(
            response.result_as::<Gradient>().expect("decode"),
            Gradient { total: 1.25 }
        );
    }

    #[test]
    fn billing_is_readable_straight_off_the_envelope() {
        let response: ToolResponse = serde_json::from_value(serde_json::json!({
            "status": "success",
            "result": {},
            "warnings": [],
            "errors": [],
            "metadata": {
                "calculation_time_ms": 12,
                "billing": { "tool": "solve_network", "credits_charged": 27 }
            }
        }))
        .expect("decode");

        assert_eq!(response.credits_charged(), Some(27));
    }

    #[test]
    fn me_decodes_unscoped_internal_traffic() {
        let me: Me = serde_json::from_value(serde_json::json!({
            "user_id": "internal", "tenant_id": "internal", "org_id": null,
            "key_id": null, "scopes": null, "billable": false
        }))
        .expect("decode");
        assert_eq!(me.scopes, None);
        assert!(!me.billable);
    }
}
