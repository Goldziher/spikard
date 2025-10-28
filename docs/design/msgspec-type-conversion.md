# Design Decision: msgspec-Based Type Conversion

## Context

Spikard is a high-performance HTTP framework with a Rust core and Python interface. When handling HTTP requests, we need to:

1. **Validate** incoming request data (path params, query params, request body)
2. **Convert** validated data from JSON primitives to Python types (dates, UUIDs, Pydantic models, etc.)
3. **Pass** typed objects to Python handler functions

This document describes our approach to type conversion in the Python layer.

## Decision

We use **msgspec** for type conversion with a custom decoder registration system, inspired by Litestar's architecture.

### Architecture

```
HTTP Request
    ↓
Rust Layer: JSON Schema Validation
    ↓ (validated JSON dict)
Python Layer: msgspec.convert() with type hints
    ↓ (typed Python objects)
Handler Function
```

### Key Principles

1. **Validation in Rust**: All validation happens in Rust using JSON Schema. This is universal, fast, and runs before the GIL.
2. **Conversion in Python**: Python only performs type conversion (no re-validation) using msgspec based on handler type hints.
3. **Trust Validated Data**: Python trusts that Rust has already validated the data, so we use non-validating conversion.
4. **Extensibility**: Users can register custom decoders for their own types.

## Implementation

### Core Conversion Function

Located in `packages/python/spikard/_internal/converters.py`:

```python
def convert_params(
    params: dict[str, Any],
    handler_func: Callable[..., Any],
    *,
    strict: bool = False,
) -> dict[str, Any]:
    """Convert validated parameter dict to typed Python objects.

    Uses msgspec.convert() with handler's type hints to create properly
    typed objects from validated JSON primitives.
    """
    type_hints = get_type_hints(handler_func)
    converted = {}

    for key, value in params.items():
        if key not in type_hints:
            converted[key] = value
            continue

        target_type = type_hints[key]
        converted[key] = msgspec.convert(
            value,
            type=target_type,
            strict=strict,
            builtin_types=(datetime, date, time, timedelta),
            dec_hook=_default_dec_hook,
        )

    return converted
```

### Custom Decoder Support

Users can register custom decoders for their types:

```python
from spikard import register_decoder

def my_custom_decoder(type_: type, obj: Any) -> Any:
    if isinstance(obj, dict) and type_ is MyCustomType:
        return MyCustomType.from_dict(obj)
    raise NotImplementedError

register_decoder(my_custom_decoder)
```

The decoder chain tries:
1. User-registered custom decoders
2. Pydantic's `model_validate()` for Pydantic models
3. msgspec's default handling

### Pydantic Integration

Pydantic models are handled via `model_validate()`:

```python
def _pydantic_decoder(type_: type, obj: Any) -> Any:
    """Decoder for Pydantic models using model_validate."""
    if hasattr(type_, "model_validate"):
        return type_.model_validate(obj)
    raise NotImplementedError
```

This trusts the pre-validated data from Rust, avoiding double validation.

## Alternatives Considered

### 1. Manual Type Conversion in Rust

**Approach**: Convert types in Rust before passing to Python.

**Rejected because**:
- Requires Rust to understand Python's type system
- Pydantic models can't be constructed in Rust
- Less extensible for custom types
- More complex PyO3 code

### 2. Re-validation in Python

**Approach**: Use Pydantic to validate data in Python.

**Rejected because**:
- Double validation (Rust + Python) is wasteful
- Rust validation is faster and doesn't hold the GIL
- Pydantic error messages would differ from JSON Schema errors
- Unnecessary complexity

### 3. JSON Schema Validation in Python

**Approach**: Skip Rust validation, validate in Python with JSON Schema.

**Rejected because**:
- Python JSON Schema validators are slower than Rust
- Validation would hold the GIL
- Rust validation is universal across all language bindings

## Benefits

1. **Performance**: msgspec is C-based and extremely fast
2. **Universal Validation**: Rust validation works for any language binding
3. **No Double Validation**: Each request is validated exactly once
4. **Extensibility**: Custom decoder registration like Litestar
5. **Pydantic Integration**: First-class support for Pydantic models
6. **Type Safety**: Uses Python's type hints for conversion

## Trade-offs

1. **Two-stage conversion**: JSON → validated JSON → Python types (but this is necessary given the architecture)
2. **Dependency on msgspec**: We commit to msgspec as our serialization library (acceptable given its performance and features)

## Related Files

- `packages/python/spikard/_internal/converters.py` - Type conversion implementation
- `crates/spikard-http/src/handler.rs` - Calls Python converter (validated_params_to_py_kwargs)
- `crates/spikard-http/src/parameters.rs` - JSON Schema validation in Rust
- `packages/python/spikard/__init__.py` - Exports register_decoder

## Future Enhancements

1. Support for more standard library types (Decimal, IPv4Address, etc.)
2. Performance benchmarks vs Pydantic validation
3. Caching of type hints for handlers
4. Optional strict mode for development
