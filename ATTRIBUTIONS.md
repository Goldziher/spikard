# Third-Party Code Attributions

This document lists third-party code that has been vendored into this project.

## Litestar

Portions of this codebase are derived from the [Litestar](https://github.com/litestar-org/litestar) web framework, specifically:

- FieldDefinition - Universal IR for all type systems (adapted from `litestar/typing.py`)
- ParsedSignature - Function signature parsing (adapted from `litestar/utils/signature.py`)
- Type annotation utilities - unwrap_annotation, type helpers (adapted from `litestar/utils/typing.py`)
- Constraint extraction from annotated_types (adapted from `litestar/typing.py`)

The following files contain vendored Litestar code:
- `packages/python/spikard/_internal/field_definition.py`
- `packages/python/spikard/_internal/parsed_signature.py`
- `packages/python/spikard/_internal/utils.py`
- `packages/python/spikard/_internal/types.py`
- `packages/python/spikard/_internal/constraints.py`

These components have been vendored and adapted for use in Spikard's universal parameter validation system.

**Original Source**: https://github.com/litestar-org/litestar
**License**: MIT License
**Copyright**: Copyright (c) 2021, 2022, 2023, 2024, 2025 Litestar Org.

### Litestar License (MIT)

```
The MIT License (MIT)

Copyright (c) 2021, 2022, 2023, 2024, 2025 Litestar Org.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### Modifications

The vendored code has been modified for Spikard's specific use cases:

1. **Simplified for Python 3.10+ only** - Removed compatibility shims for older Python versions
2. **Adapted to generate JSON Schema** - Modified to output JSON Schema instead of OpenAPI Schema
3. **Streamlined for parameter validation** - Removed dependency injection and other framework-specific features
4. **Integrated with Rust validation** - Modified to work with Rust-based JSON Schema validation

The original Litestar code was excellent reference material for handling the complexity of Python's type annotation system, including support for `Annotated`, Pydantic Field constraints, and various type wrappers.

## Lucchetto

Portions of the Ruby GVL helper macros are adapted from the [Lucchetto](https://github.com/Maaarcocr/lucchetto) project, specifically the `without_gvl` proc macro and GVL call helper patterns.

The following files contain vendored or adapted Lucchetto code:
- `crates/spikard-rb-macros/src/lib.rs`
- `packages/ruby/vendor/crates/spikard-rb-macros/src/lib.rs`
- `crates/spikard-rb/src/gvl.rs`
- `packages/ruby/vendor/crates/spikard-rb/src/gvl.rs`

**Original Source**: https://github.com/Maaarcocr/lucchetto
**License**: MIT License

### Modifications

The vendored code has been modified for Spikard's use cases:

1. **Adjusted macro pathing** - Updated macro expansions to target Spikard's internal helper macro.
2. **Receiver support** - Extended the proc macro to handle methods with `self` receivers.
3. **Safety adjustments** - Reworked result handling to avoid unwraps and added explicit GVL helpers.
