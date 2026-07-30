# spikard

Rust-centric multi-language HTTP framework with polyglot bindings

## Installation

Install Zig from [ziglang.org](https://ziglang.org/download/).

## Building

```sh
zig build
zig build test
```

## Usage

Add to your `build.zig.zon`:

```text
.dependencies = .{
    .spikard = .{
        .path = "path/to/spikard",
    },
},
```

## License

MIT
