# Dependency Injection Examples

This directory contains examples demonstrating Spikard's dependency injection system.

## Examples

### rust_basic.rs

Demonstrates basic dependency injection patterns:

- **Value Dependencies**: Register static configuration values
- **Factory Dependencies**: Create resources on-demand with async factories
- **Dependency Relationships**: Factories that depend on other dependencies
- **Custom Dependencies**: Using the advanced `provide()` API

**Note**: These are standalone code examples. To run them, you'll need to:

1. Add the `di` feature to your dependencies
2. See the tests in `crates/spikard-http/tests/server_config_builder.rs` for working integration examples

## Key Concepts

### Value Dependencies

```rust
let config = ServerConfig::builder()
    .provide_value("app_name", "MyApp".to_string())
    .provide_value("version", "1.0.0".to_string())
    .build();
```

### Factory Dependencies

```rust
let config = ServerConfig::builder()
    .provide_value("db_url", "postgresql://localhost/mydb".to_string())
    .provide_factory("db_pool", |resolved| async move {
        let url: Arc<String> = resolved.get("db_url").ok_or("Missing db_url")?;
        // Create and return database pool
        Ok(DatabasePool::connect(&url).await?)
    })
    .build();
```

### Custom Dependencies

```rust
use spikard_core::di::ValueDependency;

let dep = ValueDependency::new("config", MyConfig::default());
let config = ServerConfig::builder()
    .provide(Arc::new(dep))
    .build();
```

## Testing

See `crates/spikard-http/tests/server_config_builder.rs` for comprehensive unit tests demonstrating all builder methods.

## Documentation

For full documentation on the dependency injection system, see:
- `docs/adr/0008-dependency-injection.md` - Architecture decision record
- `IMPLEMENTATION_PLAN.md` - Full implementation plan
