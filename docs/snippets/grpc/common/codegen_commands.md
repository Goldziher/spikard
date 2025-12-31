# Code Generation Commands

Generate protobuf code for each language:

**Python**:
```bash
protoc --python_out=. user_service.proto
```

**TypeScript**:
```bash
# Install protobufjs CLI
npm install -g protobufjs-cli

# Generate TypeScript definitions
pbjs -t static-module -w commonjs -o user_service.js user_service.proto
pbts -o user_service.d.ts user_service.js
```

**Ruby**:
```bash
grpc_tools_ruby_protoc --ruby_out=. user_service.proto
```

**PHP**:
```bash
protoc --php_out=. user_service.proto
```

**Rust** (add to `build.rs`):
```rust
fn main() {
    prost_build::compile_protos(&["user_service.proto"], &["."]).unwrap();
}
```
