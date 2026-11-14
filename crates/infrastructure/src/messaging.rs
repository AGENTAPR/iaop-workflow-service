//! Messaging helpers and the outbox/inbox model.
//!
//! Integration events use the canonical envelope from `iaop-event-contracts`.
//! Delivery is at-least-once; consumers must be idempotent (inbox table).
//! Exactly-once delivery is NOT claimed.
//!
//! Publisher implementation is added together with the first event-producing
//! use case; the `EventPublisher` port is defined in the application layer.

use iaop_workflow_service_application::ports::IntegrationEvent;

/// Validates that an event can be represented as an outbox row.
pub fn validate_event(event: &IntegrationEvent) -> Result<(), String> {
    if event.event_type.is_empty() {
        return Err("event_type must not be empty".into());
    }
    if event.event_version == 0 {
        return Err("event_version must be >= 1".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn event() -> IntegrationEvent {
        IntegrationEvent {
            event_id: Uuid::now_v7(),
            event_type: "agent.activated".into(),
            event_version: 1,
            source: "test".into(),
            aggregate_id: None,
            aggregate_version: None,
            occurred_at: Utc::now(),
            correlation_id: None,
            causation_id: None,
            payload: serde_json::json!({}),
        }
    }

    #[test]
    fn valid_event_passes() {
        assert!(validate_event(&event()).is_ok());
    }

    #[test]
    fn empty_event_type_fails() {
        let mut e = event();
        e.event_type = String::new();
        assert!(validate_event(&e).is_err());
    }
}
