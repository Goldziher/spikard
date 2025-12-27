# Python GraphQL Bindings for Spikard

This document describes the Python GraphQL bindings for the Spikard framework, enabling GraphQL schema configuration and execution from Python.

## Overview

The Python GraphQL bindings provide PyO3 wrappers for spikard-graphql functionality, making it easy to configure GraphQL schemas with Pythonic APIs.

## Features

- **GraphQL Schema Configuration**: Create and configure GraphQL schemas with introspection, complexity limits, and depth limits
- **Builder Pattern**: Fluent API for schema configuration
- **Type Safety**: Full type hints and documentation
- **Zero-Copy Conversion**: Efficient conversion between Python and Rust types

## Building with GraphQL Support

To build the Python extension with GraphQL support:

```bash
# Using maturin (recommended for Python packages)
maturin build --release -F graphql

# Using cargo directly
cargo build -p spikard-py --release --features graphql
```

## API Reference

### GraphQLSchemaConfig

A configuration object for GraphQL schemas.

#### Constructor

```python
from spikard import GraphQLSchemaConfig

config = GraphQLSchemaConfig()
```

#### Properties

- **introspection_enabled** (bool): Enable/disable GraphQL introspection queries. Default: True
- **complexity_limit** (int | None): Maximum query complexity. None = unlimited. Default: None
- **depth_limit** (int | None): Maximum query nesting depth. None = unlimited. Default: None

#### Methods

```python
# Set complexity limit (0 = unlimited)
config.set_complexity(5000)

# Set depth limit (0 = unlimited)
config.set_depth(50)

# Check if introspection is enabled
is_enabled = config.is_introspection_enabled()

# Get limits
complexity = config.get_complexity_limit()
depth = config.get_depth_limit()

# Validate configuration
is_valid = config.validate()
```

#### Example

```python
from spikard import GraphQLSchemaConfig

# Create and configure
config = GraphQLSchemaConfig()
config.introspection_enabled = False
config.complexity_limit = 5000
config.depth_limit = 50

# Validate
if config.validate():
    print(f"Configuration valid: {config}")
```

### GraphQLSchemaBuilder

A builder for constructing GraphQL schemas with a fluent API.

#### Constructor

```python
from spikard import GraphQLSchemaBuilder

builder = GraphQLSchemaBuilder()
```

#### Methods

All methods modify the builder in-place and return None (for Python-style mutation):

```python
# Enable/disable introspection
builder.enable_introspection(True)

# Set complexity limit (0 = unlimited)
builder.complexity_limit(5000)

# Set depth limit (0 = unlimited)
builder.depth_limit(50)

# Query configuration
is_enabled = builder.is_introspection_enabled()
complexity = builder.get_complexity_limit()
depth = builder.get_depth_limit()

# Get underlying configuration object
config = builder.config()

# Build and finalize
final_config = builder.build()
```

#### Example

```python
from spikard import GraphQLSchemaBuilder

# Build schema configuration
builder = GraphQLSchemaBuilder()
builder.enable_introspection(True)
builder.complexity_limit(5000)
builder.depth_limit(50)

# Get the final configuration
config = builder.build()
print(f"Built: {config}")
```

## Usage Patterns

### Pattern 1: Direct Configuration

```python
from spikard import GraphQLSchemaConfig

config = GraphQLSchemaConfig()
config.introspection_enabled = False
config.complexity_limit = 5000
config.depth_limit = 50
```

### Pattern 2: Builder Pattern

```python
from spikard import GraphQLSchemaBuilder

builder = GraphQLSchemaBuilder()
builder.enable_introspection(False)
builder.complexity_limit(5000)
builder.depth_limit(50)

config = builder.build()
```

### Pattern 3: With Server Integration (Future)

```python
from spikard import Spikard, GraphQLSchemaBuilder

# Configure GraphQL
builder = GraphQLSchemaBuilder()
builder.complexity_limit(5000)
builder.depth_limit(50)

# In the future:
# app = Spikard()
#
# @app.graphql("/graphql", schema_config=builder.build())
# async def graphql_handler(query: str, variables: dict = None):
#     return execute_graphql(query, variables)
```

## Integration with Spikard Server

Full integration with the Spikard HTTP server is coming soon. The current bindings focus on schema configuration. Future versions will add:

- GraphQL route handlers
- Query execution via Python
- Subscription support
- Middleware integration

## Architecture

The bindings follow Spikard's thin binding pattern:

1. **Rust Core** (`spikard-graphql`): Contains all heavy computation and validation
2. **Python Wrappers** (`crates/spikard-py/src/graphql/`): PyO3 wrappers for Pythonic API
3. **Module Exports** (`_spikard` module): Classes exported to Python

This architecture ensures:
- Consistency across language bindings
- Minimal performance overhead
- Safe FFI boundaries
- Easy maintenance

## Type Hints

All Python APIs include full type hints:

```python
from spikard import GraphQLSchemaBuilder, GraphQLSchemaConfig

def configure_graphql() -> GraphQLSchemaConfig:
    builder: GraphQLSchemaBuilder = GraphQLSchemaBuilder()
    builder.complexity_limit(5000)
    builder.depth_limit(50)

    config: GraphQLSchemaConfig = builder.build()
    return config
```

## Error Handling

Configuration methods validate inputs automatically:

```python
from spikard import GraphQLSchemaConfig

config = GraphQLSchemaConfig()

# Validate configuration
try:
    if config.validate():
        print("Configuration is valid")
except Exception as e:
    print(f"Configuration error: {e}")
```

## Performance Considerations

1. **Zero-Copy**: Configuration objects are zero-copy across FFI boundaries
2. **Lazy Validation**: Validation only runs when explicitly requested
3. **Efficient Limits**: Limits are stored as Options, no allocation overhead

## Testing

Run the GraphQL binding tests:

```bash
cargo test -p spikard-py --features graphql --lib graphql
```

Expected output:
```
running 10 tests
test graphql::schema::tests::test_py_schema_builder_chaining ... ok
test graphql::schema::tests::test_py_schema_builder_complexity_limit ... ok
test graphql::schema::tests::test_py_schema_builder_depth_limit ... ok
test graphql::schema::tests::test_py_schema_builder_build ... ok
test graphql::schema::tests::test_py_schema_builder_enable_introspection ... ok
test graphql::schema::tests::test_py_schema_builder_new ... ok
test graphql::schema::tests::test_py_schema_config_display ... ok
test graphql::schema::tests::test_py_schema_builder_display ... ok
test graphql::schema::tests::test_py_schema_config_new ... ok
test graphql::schema::tests::test_py_schema_config_zero_limits ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured
```

## Examples

See `/examples/python_graphql_bindings.py` for complete usage examples.

## Future Work

Planned enhancements:

1. **GraphQL Execution**: Direct query execution from Python
2. **Route Integration**: GraphQL routes with automatic handling
3. **Subscription Support**: WebSocket subscriptions via GraphQL
4. **Middleware**: Custom GraphQL middleware
5. **Introspection Schema**: Programmatic schema introspection
6. **Error Details**: Extended error information with locations and paths

## Related Files

- **Implementation**: `crates/spikard-py/src/graphql/`
- **Tests**: `crates/spikard-py/src/graphql/schema.rs` (inline tests)
- **Module Export**: `crates/spikard-py/src/lib.rs` (module registration)
- **Examples**: `examples/python_graphql_bindings.py`
- **Core Crate**: `crates/spikard-graphql/`

## See Also

- [Spikard GraphQL Crate](../spikard-graphql/README.md)
- [Python Bindings](./README.md)
- [Architecture Decisions](../../docs/adr/0001-architecture-and-layering.md)
