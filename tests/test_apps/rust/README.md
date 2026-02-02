# Spikard Rust Test App

## Purpose

Test application that validates the published `spikard` crate (v0.10.1) works correctly in a standalone Rust project.

## Setup

```bash
cd tests/test_apps/rust
cargo build
```

## Run Tests

```bash
cargo test
```

Run the server:
```bash
cargo run
```

## Troubleshooting

### Crate not found
- Verify `spikard@0.7.0` is published to crates.io
- Check crate availability: `cargo search spikard`
- Try updating index: `cargo update`

### Compilation errors
- Ensure Rust 2024 edition support (Rust 1.85+)
- Check feature flags: `features = ["http"]` must be enabled
- Verify tokio runtime is configured with "full" features

### Test failures
- Confirm server spawns on random port (0)
- Check async runtime: tokio::test attribute required
- Verify reqwest client is configured correctly

### Linking errors
- Ensure all dependencies are compatible
- Check for conflicting tokio versions
- Try `cargo clean && cargo build`

### Runtime errors
- Verify Handler trait is implemented correctly
- Check Arc<dyn Fn> signature matches expected type
- Ensure all async handlers return proper Result types
