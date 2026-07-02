# Dependency Injection

Spikard's DI system (feature-gated, enabled via `di` feature flag) allows you to declare handler dependencies and have them resolved automatically before execution.

## Core Concepts

- **Dependencies are named**: Declare what your handler needs by key (e.g., `"db_pool"`, `"config"`)
- **Resolution is per-route**: Specify dependencies on each route builder
- **Scope**: Container owns all dependency instances; resolution happens at request-time

## How It Works

1. Define a `DIContainer` with dependency factories
2. Pass it to `ServerConfig.di_container`
3. On each route, declare dependencies via `handler_dependencies(["key1", "key2", ...])`
4. At request-time, the container resolves dependencies and passes them to the handler

## Examples

=== "Python"

    --8<-- "snippets/python/dependency_injection.md"

=== "TypeScript"

    --8<-- "snippets/typescript/dependency_injection.md"

=== "Ruby"

    --8<-- "snippets/ruby/dependency_injection.md"

=== "PHP"

    --8<-- "snippets/php/dependency_injection.md"

=== "Rust"

    --8<-- "snippets/rust/dependency_injection.md"

## Language Binding Support

DI is currently fully implemented in Rust. Language bindings (Python, TypeScript, Ruby, PHP, Elixir) will follow with native APIs aligned to their runtime conventions.

## Notes

- Dependencies are resolved per-request (not cached globally unless you implement caching within the factory)
- Circular or missing dependencies fail fast with clear errors
- The feature is behind a Rust `di` Cargo feature flag
