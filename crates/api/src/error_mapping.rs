//! RFC 9457 Problem Details error mapping.
//!
//! Never expose stack traces, SQL errors, or internal details to clients.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub title: &'static str,
    pub detail: String,
    pub correlation_id: Option<Uuid>,
}

impl ApiError {
    pub fn new(
        status: StatusCode,
        code: &'static str,
        title: &'static str,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            status,
            code,
            title,
            detail: detail.into(),
            correlation_id: None,
        }
    }

    pub fn validation(detail: impl Into<String>) -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
            "Validation failed",
            detail,
        )
    }

    pub fn not_found() -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            "NOT_FOUND",
            "Resource not found",
            "The requested resource does not exist",
        )
    }

    pub fn internal(detail: impl Into<String>) -> Self {
        // Detail is logged server-side; clients get a generic message.
        tracing::error!(error = %detail.into(), "internal error");
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "Internal error",
            "An unexpected error occurred",
        )
    }

    pub fn with_correlation(mut self, id: Option<Uuid>) -> Self {
        self.correlation_id = id;
        self
    }
}

#[derive(Serialize)]
struct ProblemDetails {
    #[serde(rename = "type")]
    problem_type: String,
    title: String,
    status: u16,
    detail: String,
    code: String,
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    correlation_id: Option<Uuid>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ProblemDetails {
            problem_type: format!(
                "https://api.iaop.internal/problems/{}",
                self.code.to_lowercase()
            ),
            title: self.title.to_string(),
            status: self.status.as_u16(),
            detail: self.detail,
            code: self.code.to_string(),
            correlation_id: self.correlation_id,
        };
        (self.status, Json(body)).into_response()
    }
}
