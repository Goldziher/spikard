# Language Bindings

Spikard exposes a Rust core with thin bindings.

Current binding summary:

- Python: `pip install spikard`
- TypeScript: `npm install spikard`
- Ruby: `gem install spikard`

Working rules:

- Keep framework logic in Rust and `spikard-http`.
- Treat generated language code as scaffolding or adapter code, not a second implementation of the runtime.
- Use language-appropriate DTO styles:
  - Python: `dataclass` or `msgspec`
  - TypeScript: `zod`
  - Ruby: `dry_schema`
  - Rust: `serde`
  - PHP: `readonly_class`

If a task is primarily about project scaffolding or schema-driven generation, use the CLI or MCP server rather than editing generated outputs manually first.
