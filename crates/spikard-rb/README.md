<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# spikard-rb

High-performance Ruby bindings for Spikard HTTP framework via Magnus.

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

> **User-facing documentation:** See [Spikard Ruby](../../packages/ruby/README.md) for the full API, examples, and usage guide.

## Architecture

This crate provides the Rust-side Magnus bindings that power the Ruby package. All HTTP handling, middleware, and validation runs in Rust; this crate translates between the Ruby runtime and the Spikard core.

## Architecture Highlights

- **Zero-overhead FFI** via Magnus
- **rb-sys** for modern Ruby 3.2-4.x integration
- **Idiomatic Ruby** blocks and procs
- **GC-safe handler storage**

## Building

```bash
cd packages/ruby && bundle exec rake ext:build
```

**Requirements:** Rust stable toolchain

## Documentation

- [Ruby Package README](../../packages/ruby/README.md)
- [Rust API Documentation](https://docs.rs/spikard-rb)
- [Main Project README](../../README.md)
- [spikard.dev](https://spikard.dev)

## License

MIT - See [LICENSE](../../LICENSE) for details
