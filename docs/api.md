# Workflow Platform - API

Base path: `/api/v1`. OpenAPI 3.1 document: `GET /api/docs/openapi.json`.

## Baseline endpoints

| Method | Path                  | Purpose            |
|--------|-----------------------|--------------------|
| GET    | /health/live          | Liveness probe     |
| GET    | /health/ready         | Readiness probe    |
| GET    | /api/v1/system/info   | Service metadata   |

## Standards

- RFC 3339 UTC timestamps, UUID identifiers, typed pagination, RFC 9457
  problem-details errors (`type`, `title`, `status`, `detail`, `code`,
  `correlationId`).
- Contracts governed by `iaop-api-contracts`.
