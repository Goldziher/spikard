---
id: grpc_typescript_error_handling
language: typescript
title: Error Handling
tags:
  - grpc
  - typescript
---

```typescript
import { GrpcError, GrpcStatusCode } from "spikard";

// Explicit status codes
throw new GrpcError(GrpcStatusCode.INVALID_ARGUMENT, "Invalid ID");
throw new GrpcError(GrpcStatusCode.NOT_FOUND, "User not found");
throw new GrpcError(GrpcStatusCode.UNAUTHENTICATED, "Auth required");
throw new GrpcError(GrpcStatusCode.PERMISSION_DENIED, "Access denied");
throw new GrpcError(GrpcStatusCode.INTERNAL, "Internal error");
```

Explicit `GrpcError` for all status codes.
