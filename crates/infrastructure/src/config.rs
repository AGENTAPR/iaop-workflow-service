//! Typed application configuration loaded from environment variables.

use std::time::Duration;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable: {0}")]
    MissingVar(&'static str),
    #[error("invalid value for {0}: {1}")]
    InvalidValue(&'static str, String),
}

/// Service configuration. Secrets (e.g. DATABASE_URL) are never logged;
/// use [`AppConfig::redacted_summary`] for safe diagnostics.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub service_name: String,
    pub service_version: String,
    pub environment: String,
    pub bind_addr: String,
    /// Optional: services without persistence leave this unset and
    /// `/health/ready` does not probe a database.
    pub database_url: Option<String>,
    pub database_max_connections: u32,
    pub request_timeout: Duration,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            service_name: env_or("SERVICE_NAME", "iaop-workflow-service"),
            service_version: env_or("SERVICE_VERSION", env!("CARGO_PKG_VERSION")),
            environment: env_or("ENVIRONMENT", "local"),
            bind_addr: env_or("BIND_ADDR", "0.0.0.0:8080"),
            database_url: std::env::var("DATABASE_URL").ok().filter(|s| !s.is_empty()),
            database_max_connections: env_parse("DATABASE_MAX_CONNECTIONS", 10)?,
            request_timeout: Duration::from_secs(env_parse("REQUEST_TIMEOUT_SECS", 30)?),
        })
    }

    /// Diagnostics-safe summary: never includes secrets.
    pub fn redacted_summary(&self) -> String {
        format!(
            "service={} version={} env={} bind={} db={} timeout={}s",
            self.service_name,
            self.service_version,
            self.environment,
            self.bind_addr,
            if self.database_url.is_some() {
                "configured"
            } else {
                "disabled"
            },
            self.request_timeout.as_secs(),
        )
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_parse<T: std::str::FromStr>(key: &'static str, default: T) -> Result<T, ConfigError> {
    match std::env::var(key) {
        Ok(v) => v
            .parse::<T>()
            .map_err(|_| ConfigError::InvalidValue(key, v)),
        Err(_) => Ok(default),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_loads_with_defaults() {
        let cfg = AppConfig::from_env().expect("defaults must be valid");
        assert!(!cfg.service_name.is_empty());
        assert!(cfg.redacted_summary().contains("db="));
    }
}
