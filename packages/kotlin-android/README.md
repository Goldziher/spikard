# spikard - Kotlin/Android Bindings

Codegen-first polyglot web toolkit with a Rust core and bindings for 14 languages

## Installation

Add the generated AAR to your Android module's `build.gradle.kts`:

```kotlin
dependencies {
    implementation("dev.spikard:spikard-android:VERSION")
}
```

## Quick Start

```kotlin
import dev.spikard.Spikard

// The bundled native library is loaded via System.loadLibrary().
```

## Documentation

For full documentation, see the [spikard repository](https://github.com/Goldziher/spikard).

## License

See the [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) file in the root repository.
