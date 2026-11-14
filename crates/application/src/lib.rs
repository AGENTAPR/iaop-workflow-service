#![forbid(unsafe_code)]
//! Application layer for Workflow Platform.
//!
//! Contains use cases (commands and queries) and the ports they depend on.
//! Depends only on the `domain` crate — never on Axum, SQLx, or other
//! infrastructure. Adapters in `infrastructure` implement the ports.

pub mod ports;
pub mod service_info;
