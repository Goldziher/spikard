# TypeScript gRPC Streaming Handlers

The TypeScript package does not yet expose public streaming gRPC helper types.

Current public gRPC APIs in `@spikard/node` are:

- `GrpcHandler`
- `GrpcResponse`
- `GrpcError`
- `GrpcStatusCode`
- `GrpcService`
- `createUnaryHandler(...)`
- `createServiceHandler(...)`

For the current TypeScript surface, use unary handlers and register them through `GrpcService`.
