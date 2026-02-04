# spikard-elixir

Elixir bindings for the Spikard HTTP framework via Rustler NIFs.

## Overview

This crate provides the Rust-side NIF (Native Implemented Functions) implementation for Spikard's Elixir bindings.
The Elixir package is located at `packages/elixir/`.

## Building

This crate is built automatically when compiling the Elixir package:

```bash
cd packages/elixir
SPIKARD_BUILD=1 mix compile
```

## License

MIT
