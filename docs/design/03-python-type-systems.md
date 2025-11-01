# Python Type System Support

## Overview

Spikard validates requests in Rust, then passes validated data to Python handlers. Handlers can use any Python type system for their parameters. This document describes our testing strategy and conversion paths.

## Supported Type Systems

### 1. **Plain dict (fastest, default)**
```python
def handler(body: dict[str, Any]) -> Any:
    return body  # No conversion needed!
```
- **Speed**: Instant (no conversion)
- **Use case**: Maximum performance, dynamic data
- **Conversion**: None - Rust passes dict directly

### 2. **TypedDict (stdlib, typed)**
```python
from typing import TypedDict

class RequestBody(TypedDict):
    name: str
    price: float

def handler(body: RequestBody) -> Any:
    return body  # Still a dict at runtime
```
- **Speed**: Instant (no runtime conversion)
- **Use case**: Type hints without overhead
- **Conversion**: None - TypedDict is just hints, runtime is still dict

### 3. **dataclass (stdlib, typed)**
```python
from dataclasses import dataclass

@dataclass
class RequestBody:
    name: str
    price: float

def handler(body: RequestBody) -> Any:
    return dataclasses.asdict(body)
```
- **Speed**: Fast (simple construction)
- **Use case**: Mutable typed objects
- **Conversion**: `RequestBody(**dict_data)`

### 4. **NamedTuple (stdlib, typed, immutable)**
```python
from typing import NamedTuple

class RequestBody(NamedTuple):
    name: str
    price: float

def handler(body: RequestBody) -> Any:
    return body._asdict()
```
- **Speed**: Fast (tuple construction)
- **Use case**: Immutable typed data
- **Conversion**: `RequestBody(**dict_data)`

### 5. **msgspec.Struct (fastest typed)**
```python
import msgspec

class RequestBody(msgspec.Struct):
    name: str
    price: float

def handler(body: RequestBody) -> Any:
    return msgspec.to_builtins(body)
```
- **Speed**: Fastest typed conversion (C implementation)
- **Use case**: High-performance typed APIs
- **Conversion**: `msgspec.convert(dict_data, RequestBody)`

### 6. **Pydantic BaseModel (popular)**
```python
from pydantic import BaseModel

class RequestBody(BaseModel):
    name: str
    price: float

def handler(body: RequestBody) -> Any:
    return body.model_dump()
```
- **Speed**: Slower (re-validates, serializes)
- **Use case**: Compatibility, ecosystem
- **Conversion**: `RequestBody.model_validate(dict_data)`

## Fixture Metadata

Fixtures can specify the type system via `handler.body_type`:

```json
{
  "handler": {
    "route": "/items",
    "method": "POST",
    "body_type": "msgspec",  // or "dict", "typed_dict", "dataclass", "namedtuple", "pydantic"
    "body_schema": {
      "type": "object",
      "properties": {
        "name": {"type": "string"},
        "price": {"type": "number"}
      }
    }
  }
}
```

**Default**: If `body_type` is omitted, use plain `dict[str, Any]` for maximum performance.

## convert_params Fast Paths

The `convert_params` function uses fast paths for each type:

```python
def convert_params(params: dict, handler_func: Callable) -> dict:
    type_hints = get_type_hints(handler_func)
    sig = inspect.signature(handler_func)
    handler_params = set(sig.parameters.keys())

    converted = {}
    for key, value in params.items():
        # Skip params handler doesn't accept
        if key not in handler_params:
            continue

        if key not in type_hints:
            converted[key] = value
            continue

        target_type = type_hints[key]
        origin = get_origin(target_type)

        # FAST PATH 1: dict types (no conversion)
        if target_type == dict or origin == dict:
            converted[key] = value
            continue

        # FAST PATH 2: TypedDict (no runtime conversion)
        if hasattr(target_type, '__annotations__') and hasattr(target_type, '__total__'):
            # TypedDict - just a dict at runtime
            converted[key] = value
            continue

        # FAST PATH 3: msgspec.Struct (fastest typed)
        if is_msgspec_struct(target_type):
            converted[key] = msgspec.convert(value, target_type)
            continue

        # FAST PATH 4: dataclass
        if is_dataclass(target_type):
            converted[key] = target_type(**value)
            continue

        # FAST PATH 5: NamedTuple
        if is_namedtuple(target_type):
            converted[key] = target_type(**value)
            continue

        # SLOW PATH: Pydantic
        if hasattr(target_type, 'model_validate'):
            converted[key] = target_type.model_validate(value)
            continue

        # Fallback: try msgspec.convert
        try:
            converted[key] = msgspec.convert(value, target_type)
        except:
            converted[key] = value

    return converted
```

## Test Strategy

Generate handlers using all 6 type systems to ensure comprehensive testing:

1. **Group fixtures by category** (json_bodies, query_params, etc.)
2. **Rotate type systems** within each category
3. **Ensure each type system** is tested with various schemas

Example distribution:
- json_bodies/01: dict
- json_bodies/02: TypedDict
- json_bodies/03: dataclass
- json_bodies/04: NamedTuple
- json_bodies/05: msgspec
- json_bodies/06: Pydantic
- json_bodies/07: dict (cycle back)
- ...

This ensures every type system is tested across different validation scenarios.

## Performance Implications

**Fastest to Slowest:**
1. dict / TypedDict: 0 conversion overhead
2. msgspec.Struct: ~2x slower than dict (but typed)
3. dataclass / NamedTuple: ~3-4x slower
4. Pydantic: ~10-20x slower (validation + serialization)

**Recommendation**: Use dict or TypedDict for maximum performance, msgspec for typed performance, Pydantic only when ecosystem integration is needed.
