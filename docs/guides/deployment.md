# Deployment

Spikard can run as a compiled Rust binary, via the CLI, or packaged into containers. Pick the surface that matches your stack; the runtime behavior stays the same.

## Local run options

=== "CLI"

    --8<-- "snippets/cli/run_app.md"

=== "Rust"

    --8<-- "snippets/rust/run_app.md"

=== "TypeScript"

    --8<-- "snippets/typescript/run_app.md"

=== "Python"

    --8<-- "snippets/python/run_app.md"

=== "Ruby"

    --8<-- "snippets/ruby/run_app.md"

## Production tips

- Set explicit `host`/`port` and timeouts; avoid relying on defaults in container platforms.
- Enable structured logging + tracing (OTel recommended) and forward request IDs.
- Run health checks against a lightweight endpoint with minimal middleware.
- Use the Taskfile to build bindings before containerizing (`task build` or targeted language tasks).

## Versioned docs & config

- Publish docs with `task docs:deploy` after syncing schemas/code.
- Keep configuration declarative and environment-driven (see [Configuration](configuration.md)).

## Example Dockerfile sketch

```dockerfile
FROM rust:1.85-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/your_app /usr/local/bin/your_app
ENV HOST=0.0.0.0
ENV PORT=8080
CMD ["your_app"]
```

Replace `your_app` with your binary name. Configure host and port via environment variables in your ServerConfig.
