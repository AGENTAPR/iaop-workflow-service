-- Transactional outbox: events are written in the same transaction as the
-- state change, then relayed to the broker by the publisher.
-- At-least-once delivery; consumers must deduplicate via inbox_events.

CREATE TABLE IF NOT EXISTS outbox_events (
    event_id            UUID PRIMARY KEY,
    aggregate_id        UUID,
    aggregate_type      TEXT,
    aggregate_version   BIGINT,
    event_type          TEXT        NOT NULL,
    event_version       INTEGER     NOT NULL DEFAULT 1,
    payload             JSONB       NOT NULL,
    occurred_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    published_at        TIMESTAMPTZ,
    publish_attempts    INTEGER     NOT NULL DEFAULT 0,
    last_error          TEXT
);

CREATE INDEX IF NOT EXISTS idx_outbox_unpublished
    ON outbox_events (created_at)
    WHERE published_at IS NULL;
