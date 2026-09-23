//! Client for the GraphSolve engine API — production-network solving, PVT/EOS,
//! flow assurance and transient multiphase flow, as a hosted service.
//!
//! ```no_run
//! # async fn f() -> graphsolve::Result<()> {
//! use graphsolve::{GraphSolve, SolveNetworkParams};
//!
//! let gs = GraphSolve::new()?;        // reads GRAPHSOLVE_API_KEY
//! let response = gs
//!     .solve_network(SolveNetworkParams {
//!         network_json: serde_json::json!({ "graph_data": { "nodes": [] } }),
//!         ..Default::default()
//!     })
//!     .await?;
//! println!("{:?}", response.credits_charged());
//! # Ok(()) }
//! ```
//!
//! Units are SI throughout: pressure MPa, temperature K, rates kSm3/day, lengths
//! m. Permeability in millidarcy is the single exception.
//!
//! This crate is a **client**. It contains no physics: every call is an
//! authenticated HTTPS request to the hosted engine, and every one is metered.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

mod auth;
mod client;
mod error;
mod generated;
mod types;

pub use auth::{refresh_margin, MAX_REFRESH_MARGIN};
pub use client::{Builder, GraphSolve, DEFAULT_BASE_URL, DEFAULT_TIMEOUT, DEFAULT_TOKEN_URL};
pub use error::{Error, Result};
pub use generated::*;
pub use types::*;
