### Required Tools

1. **protoc** (Protocol Buffers compiler):
   ```bash
   # macOS
   brew install protobuf

   # Ubuntu/Debian
   apt-get install protobuf-compiler

   # Verify installation
   protoc --version  # Should be 3.0+
   ```

2. **Spikard CLI**:
   ```bash
   cargo install spikard-cli
   ```

3. **Language-specific protobuf runtime**:

=== "Python"
    ```bash
    pip install protobuf  # or: uv add protobuf
    ```

=== "TypeScript"
    ```bash
    npm install protobufjs  # or: pnpm add protobufjs
    ```

=== "Ruby"
    ```bash
    gem install google-protobuf
    ```

=== "PHP"
    ```bash
    composer require google/protobuf
    ```

=== "Rust"
    ```bash
    cargo add prost prost-types
    ```
