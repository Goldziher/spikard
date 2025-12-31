```
+-------------------------------------------------------------+
|  Your Handler (Python, TypeScript, Ruby, PHP, Rust)         |
|  - Implements: handle_request(GrpcRequest) -> GrpcResponse  |
+-------------------------------------------------------------+
                            |
                            v
+-------------------------------------------------------------+
|  FFI Layer (spikard-py, spikard-node, spikard-rb, etc.)    |
|  - Binary protobuf payloads, metadata conversion            |
+-------------------------------------------------------------+
                            |
                            v
+-------------------------------------------------------------+
|  Rust Runtime (spikard-http + Tonic)                        |
|  - HTTP/2, gRPC protocol, routing, status codes             |
+-------------------------------------------------------------+
```
