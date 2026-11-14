//! Ports (interfaces) that the application layer requires from the
//! outside world. Implementations live in `infrastructure`.
//!
//! Convention: a port is only introduced when a use case actually needs it.
//! Do not add ports speculatively.

use std::future::Future;
use std::pin::Pin;

use iaop_workflow_service_domain::EntityId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Canonical integration-event envelope published through the outbox.
///
/// Mirrors the `iaop-event-contracts` envelope schema. Domain events are
/// translated to integration events at the application boundary — internal
/// domain objects are never published directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub event_version: u32,
    pub source: String,
    pub aggregate_id: Option<EntityId>,
    pub aggregate_version: Option<u64>,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub payload: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("failed to publish event: {0}")]
pub struct PublishError(pub String);

/// Port: publish an integration event durably (implemented via the
/// transactional outbox in `infrastructure::persistence`).
pub trait EventPublisher: Send + Sync {
    fn publish<'a>(
        &'a self,
        event: &'a IntegrationEvent,
    ) -> Pin<Box<dyn Future<Output = Result<(), PublishError>> + Send + 'a>>;
}
