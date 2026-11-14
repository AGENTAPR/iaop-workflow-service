//! PostgreSQL persistence: connection pool, migrations, and the
//! transactional outbox/inbox tables defined in `migrations/`.
//!
//! This service owns its database exclusively. No cross-service queries,
//! joins, or foreign keys are permitted.

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Establish a connection pool. Returns `None`-safe callers should check
/// `database_url` presence before calling.
pub async fn connect_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
}

/// Apply pending SQLx migrations from the service-root `migrations/` dir.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("../../migrations").run(pool).await
}

/// Readiness probe: verifies the pool can reach PostgreSQL.
pub async fn ping(pool: &PgPool) -> bool {
    sqlx::query("SELECT 1").fetch_one(pool).await.is_ok()
}

/// Outbox record mirroring the `outbox_events` table.
/// The background publisher (added with the first event-producing use
/// case) claims unpublished rows, publishes them to the broker, and sets
/// `published_at`.
#[derive(Debug, Clone)]
pub struct OutboxRecord {
    pub event_id: uuid::Uuid,
    pub aggregate_id: Option<uuid::Uuid>,
    pub aggregate_type: Option<String>,
    pub aggregate_version: Option<i64>,
    pub event_type: String,
    pub event_version: i32,
    pub payload: serde_json::Value,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub publish_attempts: i32,
    pub last_error: Option<String>,
}
