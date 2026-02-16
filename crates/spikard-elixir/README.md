# spikard-elixir

Rustler NIF crate for Spikard's Elixir bindings.

[![Documentation](https://img.shields.io/badge/docs-spikard.dev-blue)](https://spikard.dev)
[![Crates.io](https://img.shields.io/crates/v/spikard-elixir.svg)](https://crates.io/crates/spikard-elixir)
[![Hex.pm](https://img.shields.io/hexpm/v/spikard.svg)](https://hex.pm/packages/spikard)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE)

## Overview

`spikard-elixir` provides the Rust-side implementation for Spikard's Elixir runtime integration.
The public Elixir API lives in `packages/elixir/`.

## Build

This crate is compiled as part of the Elixir package build:

```bash
cd packages/elixir
SPIKARD_BUILD=1 mix compile
```

## Related

- Elixir package README: `packages/elixir/README.md`
- Root project README: `README.md`

## License

MIT
