# Workflow Platform - Architecture

Hexagonal / clean architecture layout:

```text
crates/
  domain/          pure domain model - no infra deps
  application/     use cases + ports
  infrastructure/  adapters: config, postgres, messaging, security
  api/             axum inbound adapter: routes, dto, middleware, errors
src/main.rs        composition root
```

## Dependency rule

```
api            -> application, infrastructure, domain
infrastructure -> application, domain
application    -> domain
domain         -> (nothing)
```

The domain crate forbids unsafe code and has zero infrastructure dependencies.

## Status

Scaffold only - no business use cases implemented yet.
