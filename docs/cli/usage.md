# CLI Usage

`spikard-cli` provides code generation, schema validation, and project scaffolding.

## Install
```bash
cargo install spikard-cli
```

## Commands

### Initialize a New Project

Create an idiomatic project structure for your chosen language:

```bash
spikard init --name my_api --language python
```

Supported languages: `python`, `typescript`, `ruby`, `php`, `rust`

**With Schema Integration:**
```bash
spikard init --name my_api --language python --schema openapi.yaml
```

Generates project structure + handlers from your schema.

See `spikard init --help` for all options.

### Generate Code from Schemas

Generate type-safe handlers and DTOs from contracts:

**GraphQL:**
```bash
spikard generate graphql schema.graphql --lang python --target all
spikard generate graphql schema.graphql --lang typescript --target types
```

**OpenAPI:**
```bash
spikard generate openapi openapi.yaml --lang python --output ./generated
```

**AsyncAPI:**
```bash
spikard generate asyncapi asyncapi.yaml --lang typescript
```

**OpenRPC:**
```bash
spikard generate openrpc rpc-spec.json --lang ruby
```

All generated code passes strict quality tools (mypy --strict, tsc, steep, phpstan level max, clippy).

### Run an App

Runtime serving from the CLI is planned. Today, start apps with the binding APIs:
- Python: `uv run python app.py`
- TypeScript: `pnpm dev` or `node app.js`
- Ruby: `bundle exec ruby app.rb`
- PHP: `php app.php`
- Rust: `cargo run`

## Development Notes

- Build locally: `cargo build -p spikard-cli` or `task build:cli`
- Run tests: `cargo test -p spikard-cli`
- Lint/format: `cargo clippy && cargo fmt`
