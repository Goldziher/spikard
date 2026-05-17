---
id: grpc_common_architecture_diagram
title: Architecture Diagram
tags:
  - grpc
  - common
---

<pre><code>
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
</code></pre>
