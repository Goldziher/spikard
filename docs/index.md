# Spikard Documentation

Spikard is a codegen-first polyglot web toolkit with a Rust core and 14 language bindings: Python, TypeScript/Node, Ruby, PHP, Elixir, Go, Java, C#, Kotlin, Dart, Swift, Zig, C FFI, and WebAssembly. It keeps routing, middleware, validation, and streaming semantics identical across languages so teams can mix runtimes without relearning frameworks.

!!! warning "Experimental"

    Spikard is experimental and pre-1.0. APIs change between releases and not every binding is at the
    same level of maturity, so it is not yet recommended for production. Feedback is genuinely wanted
    at this stage — [open an issue](https://github.com/Goldziher/spikard/issues) if something is
    wrong, awkward, or missing.

## Hello Route (pick a binding)

=== "Python"

    --8<-- "snippets/python/hello_route.md"

=== "TypeScript"

    --8<-- "snippets/typescript/hello_route.md"

=== "Ruby"

    --8<-- "snippets/ruby/hello_route.md"

=== "PHP"

    --8<-- "snippets/php/hello_route.md"

=== "Rust"

    --8<-- "snippets/rust/hello_route.md"

## Documentation Map

- **[Getting Started](getting-started/quickstart.md)** – First route in each language plus how to run it.
- **[Installation](getting-started/installation.md)** – Binding install commands and repo setup.
- **[Guides](guides/routing.md)** – Routing, requests/responses, middleware, validation, dependency injection, deployment.
- **[Concepts](concepts/architecture.md)** – Architecture, runtime model, validation, middleware, streaming internals.
- **[Reference](reference/feature-parity.md)** – Cross-language feature parity, types, and configuration surface.
- **[CLI](cli/usage.md)** – Running the HTTP server and invoking generators from `spikard-cli`.
- **[ADRs](adr/README.md)** – Design history and rationale behind the runtime.

## Getting Help

- **Questions / bugs**: open an issue on [GitHub](https://github.com/Goldziher/spikard).
- **Chat**: join the community Discord (`https://discord.gg/pXxagNK2zN`).
- **Contributing**: see [Contributing](contributing.md) for coding standards, environment setup, and testing instructions.
