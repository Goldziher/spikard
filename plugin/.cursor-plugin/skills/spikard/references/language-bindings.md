<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:d94a1e52eca0b07cb5af132d5b7fba85a8bf8939cb327556266b59b29c0f1481
Source-Hash: blake3:84f6dbbe893d49479fb99571e00f7caaad8c748add1fe1e87aa99532bca9d9eb
Schema-Version: v1
-->

# Language Bindings

Spikard exposes a Rust core with thin bindings.

Current binding summary:

- Python: `pip install spikard`
- TypeScript: `pnpm add @spikard/node`
- Ruby: `gem install spikard`
- PHP: `composer require goldziher/spikard`
- Elixir: add `{:spikard, "~> 0.17.2"}` to `mix.exs`, then run `mix deps.get`

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
