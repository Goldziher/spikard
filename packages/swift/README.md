# Spikard

Rust-centric multi-language HTTP framework with polyglot bindings

## Installation

Add to your `Package.swift`:

```swift
.package(url = "https://github.com/example/Spikard.git", branch: "main"),
```

## Building

```sh
cargo build -p spikard-swift
# Copy generated sources (see BUILDING.md for details)
swift build --package-path packages/swift
```

For detailed build instructions, see [BUILDING.md](BUILDING.md).

## License

MIT
