# Spikard Documentation

Spikard is a polyglot API toolkit with a Rust core and first-class bindings for Python, TypeScript/Node, Ruby, and Rust. It keeps routing, middleware, validation, and streaming semantics identical across languages so teams can mix runtimes without relearning frameworks.

## Hello Route (pick a binding)

=== "Python"

    --8<-- "snippets/python/hello_route.md"

=== "TypeScript"

    --8<-- "snippets/typescript/hello_route.md"

=== "Ruby"

    --8<-- "snippets/ruby/hello_route.md"

=== "Rust"

    --8<-- "snippets/rust/hello_route.md"

## Documentation Map

- **[Getting Started](getting-started/quickstart.md)** – First route in each language plus how to run it.
- **[Installation](getting-started/installation.md)** – Binding install commands and repo setup.
- **[Guides](guides/routing.md)** – Routing, requests/responses, middleware, validation, deployment.
- **[Concepts](concepts/architecture.md)** – Architecture, runtime model, validation, middleware, streaming internals.
- **[Reference](reference/api-python.md)** – Language APIs, configuration surface, types, and error semantics.
- **[CLI](cli/usage.md)** – Running the HTTP server and invoking generators from `spikard-cli`.
- **[ADRs](adr/README.md)** – Design history and rationale behind the runtime.

## Getting Help

- **Questions / bugs**: open an issue at [github.com/Goldziher/spikard](https://github.com/Goldziher/spikard).
- **Chat**: join the community Discord (`https://discord.gg/pXxagNK2zN`).
- **Contributing**: see [Contributing](contributing.md) for coding standards, environment setup, and testing instructions.
