#![forbid(unsafe_code)]
//! Domain layer for Workflow Platform.
//!
//! This crate contains the pure domain model: entities, value objects,
//! aggregates, domain events, and domain errors. It MUST NOT depend on
//! Axum, SQLx, HTTP clients, Keycloak, message brokers, or any other
//! infrastructure concern.
//!
//! Dependency rule: `domain` has no dependencies on sibling crates.

pub mod entity_id;
pub mod error;

pub use entity_id::EntityId;
pub use error::DomainError;
