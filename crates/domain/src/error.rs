use thiserror::Error;

/// Domain-level error type.
///
/// Mapped to API problem details by the api crate. Never contains
/// infrastructure details (SQL, HTTP status, stack traces).
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("validation failed: {0}")]
    Validation(String),

    #[error("resource not found")]
    NotFound,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("operation not permitted in current state")]
    InvalidState,
}

impl DomainError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }
}
