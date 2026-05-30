# Spikard

Rust-centric multi-language HTTP framework with polyglot bindings

## Installation

Add to your `Package.swift`:

```swift
.package(path: "packages/swift"),
```

## Building

```sh
cargo build -p spikard-swift
OUT=$(ls -dt target/debug/build/spikard-swift-*/out 2>/dev/null | head -1)
cat "$OUT/SwiftBridgeCore.h" "$OUT/spikard-swift/spikard-swift.h" \
    > packages/swift/Sources/RustBridgeC/RustBridgeC.h
{ echo "import RustBridgeC"; cat "$OUT/SwiftBridgeCore.swift"; } \
    > packages/swift/Sources/RustBridge/SwiftBridgeCore.swift
{ echo "import RustBridgeC"; cat "$OUT/spikard-swift/spikard-swift.swift"; } \
    > packages/swift/Sources/RustBridge/spikard-swift.swift
swift build --package-path packages/swift
swift test --package-path packages/swift
```

The generated `Sources/RustBridgeC` and `Sources/RustBridge` artifacts are
rewritten after each Cargo clean or rebuild.

## License

MIT
