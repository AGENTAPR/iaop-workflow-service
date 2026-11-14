-- Inbox / deduplication: processed integration events are recorded so
-- redeliveries are idempotent. Unique on (consumer_name, event_id).

CREATE TABLE IF NOT EXISTS inbox_events (
    event_id        UUID        NOT NULL,
    consumer_name   TEXT        NOT NULL,
    received_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    processed_at    TIMESTAMPTZ,
    status          TEXT        NOT NULL DEFAULT 'received'
                    CHECK (status IN ('received', 'processed', 'failed')),
    last_error      TEXT,
    PRIMARY KEY (consumer_name, event_id)
);

CREATE INDEX IF NOT EXISTS idx_inbox_status
    ON inbox_events (status, received_at);
