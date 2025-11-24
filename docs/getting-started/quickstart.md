# Quick Start

Build the same minimal service in each binding. Choose a tab, copy the snippet, and run.

## Define routes

=== "Python"

    --8<-- "snippets/python/quickstart_routes.md"

=== "TypeScript"

    --8<-- "snippets/typescript/quickstart_routes.md"

=== "Ruby"

    --8<-- "snippets/ruby/quickstart_routes.md"

=== "Rust"

    --8<-- "snippets/rust/quickstart_routes.md"

## Run it

- Python: `python app.py`
- TypeScript: `pnpm ts-node app.ts` (or your runtime of choice), then hit `http://localhost:8000/users/1`
- Ruby: `ruby app.rb`
- Rust: `cargo run` inside your crate/binary

## Next steps
- Add middleware (logging, auth, tracing) with the same signature in every binding.
- Wire JSON Schema validation so request/response contracts stay enforced.
- Deploy using the Rust binary, the CLI, or container images (see [Deployment](../guides/deployment.md)).
