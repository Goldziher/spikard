# Dependency Injection

Register shared services once and inject them into handlers by name. Value dependencies are singletons; factory dependencies can be request-scoped (cacheable) or singletons.

## Register dependencies

=== "Python"

    --8<-- "snippets/python/dependency_injection.md"

=== "TypeScript"

    --8<-- "snippets/typescript/dependency_injection.md"

=== "Ruby"

    --8<-- "snippets/ruby/dependency_injection.md"

=== "Rust"

    --8<-- "snippets/rust/dependency_injection.md"

## Notes
- Value dependencies are cached globally. Use factories for per-request values and set `cacheable`/`use_cache` when you need a fresh value each time.
- Factories can depend on other dependencies; unresolved or circular graphs fail fast with clear errors.
- Cleanup generators (Python) and singleton toggles (TypeScript/Ruby) mirror the core DI engine semantics.
