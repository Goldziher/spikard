# Quick Start

Build the same minimal service in each binding. Choose a tab, copy the snippet, and run.

## Define routes

=== "Python"

    --8<-- "snippets/python/quickstart_routes.md"

=== "TypeScript"

    --8<-- "snippets/typescript/quickstart_routes.md"

=== "Ruby"

    --8<-- "snippets/ruby/quickstart_routes.md"

=== "PHP"

    --8<-- "snippets/php/quickstart_routes.md"

=== "Rust"

    --8<-- "snippets/rust/quickstart_routes.md"

## Run it

=== "Python"

    ```bash
    python app.py
    ```

    Then hit `http://localhost:8000/users/1`

=== "TypeScript"

    ```bash
    pnpm ts-node app.ts
    ```

    Then hit `http://localhost:8000/users/1`

=== "Ruby"

    ```bash
    ruby app.rb
    ```

    Then hit `http://localhost:8000/users/1`

=== "PHP"

    --8<-- "snippets/php/run_app.md"

=== "Rust"

    ```bash
    cargo run
    ```

    inside your crate/binary, then hit `http://localhost:8000/users/1`

## Next steps
- Add middleware (logging, auth, tracing) with the same signature in every binding.
- Wire JSON Schema validation so request/response contracts stay enforced.
- Deploy using the Rust binary, the CLI, or container images (see [Deployment](../guides/deployment.md)).
