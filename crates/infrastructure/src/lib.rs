#![forbid(unsafe_code)]
//! Infrastructure layer for Workflow Platform.
//!
//! Adapters implementing application ports: configuration, PostgreSQL
//! persistence (pool, migrations, outbox/inbox), messaging helpers,
//! and security utilities. Depends on `application` and `domain`.

pub mod config;
pub mod messaging;
pub mod persistence;
pub mod security;
