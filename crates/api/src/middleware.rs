//! HTTP middleware: request IDs, tracing, and request timeouts.

use std::time::Instant;

use axum::extract::{Request, State};
use axum::http::header::{self, HeaderValue};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use tracing::Instrument;
use uuid::Uuid;

use crate::error_mapping::ApiError;
use crate::state::AppState;

pub const REQUEST_ID_HEADER: &str = "x-request-id";
pub const CORRELATION_ID_HEADER: &str = "x-correlation-id";

/// Adds a request/correlation ID, emits a tracing span with method/path/
/// latency, and enforces the configured request timeout.
pub async fn request_context(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let request_id = request
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok())
        .unwrap_or_else(Uuid::now_v7);

    request.extensions_mut().insert(RequestContext {
        request_id,
        correlation_id: request_id,
    });

    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        correlation_id = %request_id,
        method = %method,
        path = %path,
    );

    async move {
        let start = Instant::now();
        let response =
            match tokio::time::timeout(state.config.request_timeout, next.run(request)).await {
                Ok(resp) => resp,
                Err(_) => ApiError::new(
                    axum::http::StatusCode::GATEWAY_TIMEOUT,
                    "REQUEST_TIMEOUT",
                    "Request timeout",
                    "The request exceeded the allowed processing time",
                )
                .with_correlation(Some(request_id))
                .into_response(),
            };
        let latency_ms = start.elapsed().as_millis() as u64;
        tracing::info!(
            latency_ms,
            status = response.status().as_u16(),
            "request completed"
        );
        let mut response = response;
        if let Ok(v) = HeaderValue::from_str(&request_id.to_string()) {
            response
                .headers_mut()
                .insert(header::HeaderName::from_static("x-request-id"), v);
        }
        response
    }
    .instrument(span)
    .await
}

/// Per-request identifiers propagated to logs and downstream calls.
#[derive(Debug, Clone, Copy)]
pub struct RequestContext {
    pub request_id: Uuid,
    pub correlation_id: Uuid,
}

impl RequestContext {
    /// Retrieves the context inserted by [`request_context`].
    pub fn from_request(req: &Request) -> Option<Self> {
        req.extensions().get::<RequestContext>().copied()
    }
}
