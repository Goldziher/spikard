---
id: grpc_typescript_registration
language: typescript
title: Registration
tags:
  - grpc
  - typescript
---

```typescript
import { GrpcService, Spikard } from "spikard";

const grpcService = new GrpcService();

grpcService.registerUnary("userservice.UserService", "GetUser", userServiceHandler);

const app = new Spikard();
app.useGrpc(grpcService);
```
