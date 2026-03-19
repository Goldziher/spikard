<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# spikard-graphql

GraphQL support for Spikard with async-graphql integration, schema building, and HTTP handler integration.

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://spikard.dev">
    <img src="https://img.shields.io/badge/docs-spikard.dev-007ec6" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard.svg?color=007ec6" alt="Crates.io">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard.svg?color=007ec6" alt="PyPI">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node.svg?color=007ec6" alt="npm">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard.svg?color=007ec6" alt="RubyGems">
  </a>
  <a href="https://packagist.org/packages/spikard/spikard">
    <img src="https://img.shields.io/packagist/v/spikard/spikard.svg?color=007ec6" alt="Packagist">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard.svg?color=007ec6" alt="Hex.pm">
  </a>
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-007ec6" alt="License">
  </a>
</div>

## Features
- **GraphQL execution**: queries, mutations, and optional subscriptions
- **Fluent schema builder** API with Query, Mutation, and Subscription types
- **HTTP integration** via Spikard's Handler trait
- **Structured error responses** compatible with GraphQL and HTTP specs
- **Apollo Federation** support (optional feature flag)
- **Subscriptions** support (optional feature flag)

## Installation

```toml
[dependencies]
spikard-graphql = "0.13.0"
```

## Quick Start

## Modules

- `executor`: Core GraphQL execution engine
- `handler`: HTTP request handling for GraphQL queries
- `schema`: Schema builder with introspection and limit controls
- `error`: Error types and HTTP conversion

## Dependencies

- `async-graphql`: GraphQL library
- `spikard-http`: HTTP server integration
- `tokio`: Async runtime
- `serde_json`: JSON serialization

## Documentation

- [Main Project README](../../README.md)
- [Full API Documentation](https://docs.rs/spikard-graphql)
- [spikard.dev](https://spikard.dev)

## License

MIT - See [LICENSE](../../LICENSE) for details
