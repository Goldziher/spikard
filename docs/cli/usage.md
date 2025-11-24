# CLI Usage

`spikard-cli` handles code generation and schema validation for DTOs and handlers.

## Install
```bash
cargo install spikard-cli
```

## Run an App
Runtime serving from the CLI is planned. Today, start apps with the binding APIs:
- Python: `python app.py`
- TypeScript: `node app.js` or `bun run app.ts`
- Ruby: `ruby app.rb`

## Generate Code
Use the CLI to generate DTOs/handlers from contracts (OpenAPI/AsyncAPI):
```bash
spikard generate openapi ./openapi.yaml --lang python --output ./generated
spikard generate openapi ./openapi.yaml --lang typescript --output ./generated
```
Generated code stays aligned with the runtime schemas and fixture-based tests.

## Development Notes
- Build locally with `cargo build -p spikard-cli` or `task build:cli`.
- End-to-end scenarios run via `cargo run --package spikard-cli -- run examples/app.py`.
- The CLI inherits workspace lint/format settings; run `cargo fmt`/`cargo clippy` before publishing.
