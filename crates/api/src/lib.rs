#![forbid(unsafe_code)]
//! API layer (inbound adapter) for Workflow Platform.
//!
//! Axum router, HTTP handlers, DTOs, middleware, and error mapping.
//! Depends on `application` and `infrastructure` for wiring types.

pub mod dto;
pub mod error_mapping;
pub mod middleware;
pub mod openapi;
pub mod routes;
pub mod state;

pub use routes::build_router;
pub use state::AppState;
