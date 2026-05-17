---
id: typescript_middleware_basic
language: typescript
title: Middleware Basic
tags:
  - typescript
---

```typescript
import { Spikard, type Request } from "spikard";

const app = new Spikard();

app.onRequest(async (request: Request): Promise<Request> => {
  console.log(`${request.method} ${request.path}`);
  return request;
});
```
