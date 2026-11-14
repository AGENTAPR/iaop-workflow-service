# syntax=docker/dockerfile:1
FROM rust:1.100.0-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --locked --bin iaop-workflow-service

FROM debian:bookworm-slim AS runtime
RUN useradd --system --uid 10001 --create-home appuser
WORKDIR /app
COPY --from=builder /app/target/release/iaop-workflow-service /app/iaop-workflow-service
USER appuser
EXPOSE 8080
ENV RUST_LOG=info
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s \
    CMD ["sh", "-c", "wget -qO- http://127.0.0.1:${PORT:-8080}/health/live >/dev/null 2>&1 || exit 1"]
ENTRYPOINT ["/app/iaop-workflow-service"]
