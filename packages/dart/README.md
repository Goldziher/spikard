# spikard

Rust-centric multi-language HTTP framework with polyglot bindings

## Installation

Add to your `pubspec.yaml`:

```yaml
dependencies:
  spikard: ^0.15.6-rc.9
```

Then run:

```sh
dart pub get
```

## Building

From the repository root:

```sh
cargo build -p spikard-dart
flutter_rust_bridge_codegen generate
dart pub get
dart analyze
dart test
```

## License

MIT
