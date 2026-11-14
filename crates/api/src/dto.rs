use serde::Serialize;

/// Wire format for `GET /health/live` and `/health/ready`.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

/// Wire format for `GET /api/v1/system/info`.
#[derive(Debug, Serialize)]
pub struct SystemInfoResponse {
    pub service: String,
    pub version: String,
    pub environment: String,
    pub status: &'static str,
}
