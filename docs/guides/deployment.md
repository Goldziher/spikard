# Deployment

Spikard can run as a compiled Rust binary, via the CLI, or packaged into containers. Pick the surface that matches your stack; the runtime behavior stays the same.

## Options
- **Rust binary**: embed routes/middleware directly in Rust and ship the compiled binary.
- **CLI**: `spikard run app.py` (or TypeScript/Ruby equivalents) to boot the Rust server with binding code.
- **Containers**: build minimal images around the Rust binary or CLI runner; include your binding runtime.

## Production Tips
- Set explicit `host`/`port` and timeouts; avoid relying on defaults in container platforms.
- Enable structured logging + tracing (OTel recommended) and forward request IDs.
- Run health checks against a lightweight endpoint with minimal middleware.
- Use the Taskfile to build bindings before containerizing (`task build` or targeted language tasks).

## Versioned Docs & Config
- Publish docs with `task docs:publish` after syncing schemas/code.
- Keep configuration declarative and environment-driven (see [Configuration](configuration.md)).

## Example Dockerfile Sketch
```dockerfile
FROM rust:1.79-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release -p spikard-http

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/spikard-http /usr/local/bin/spikard
ENV SPIKARD_PORT=8080
CMD ["spikard", "--port", "8080"]
```
Adjust packages/runtime depending on whether you run the CLI or embed routes in Rust.
