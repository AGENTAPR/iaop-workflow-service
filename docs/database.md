# Workflow Platform - Database

- Engine: PostgreSQL (database-per-service; this service owns its DB exclusively).
- Migrations: SQLx, timestamp-prefixed files in `migrations/`, applied at startup
  via `sqlx::migrate!` when `DATABASE_URL` is configured.
- Templates included: `outbox_events` (transactional outbox) and
  `inbox_events` (consumer deduplication).

## Conventions

- `UUID` primary keys, `TIMESTAMPTZ` instants, `NUMERIC` money, `JSONB` only when justified.
- Expand-and-contract schema evolution; no destructive auto-migrations in production.
- Production migration runs require explicit authorization - never automatic on deploy.
