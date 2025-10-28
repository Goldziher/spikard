# spikard-http

Axum-based HTTP runtime that hosts Spikard routes, enforces JSON-schema validation, and coordinates cross-language handler execution.

## Components
- `router` and `route` translate Python route metadata into strongly typed Rust handlers.
- `validation` checks headers, cookies, query params, and bodies against the fixtures in `testing_data/`.
- `server` wraps Axum/Tokio bootstrapping and exposes defaults through `ServerConfig`.

## Development
- Build with `cargo build -p spikard-http` or `task build:http`.
- Execute tests and fixture validations via `cargo test -p spikard-http`.
- When altering schemas, sync the Python fixtures and regenerate bindings before rerunning the CLI.
