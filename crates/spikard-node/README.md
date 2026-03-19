<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# spikard-node

High-performance Node.js bindings for Spikard HTTP framework via napi-rs.

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

> **User-facing documentation:** See [Spikard for Node.js](../../packages/node/README.md) for the full API, examples, and usage guide.

## Architecture

This crate provides the Rust-side napi-rs bindings that power the TypeScript/Node.js package. All HTTP handling, middleware, and validation runs in Rust; this crate translates between the TypeScript/Node.js runtime and the Spikard core.

## Architecture Highlights
- **Zero-copy FFI** via napi-rs
- **ThreadsafeFunction** for async JavaScript callbacks
- **Dedicated Tokio runtime** (does not block Node event loop)
- **Direct type conversion** without JSON serialization overhead

## Building

```bash
cd crates/spikard-node && pnpm build
```

**Requirements:** Rust stable toolchain

## Documentation

- [TypeScript/Node.js Package README](../../packages/node/README.md)
- [Rust API Documentation](https://docs.rs/spikard-node)
- [Main Project README](../../README.md)
- [spikard.dev](https://spikard.dev)

## License

MIT - See [LICENSE](../../LICENSE) for details
