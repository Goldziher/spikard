---
id: typescript_response_basic
language: typescript
title: Response Basic
tags:
  - typescript
---

```typescript
import { Spikard } from "spikard";

const app = new Spikard();

app.addRoute(
  { method: "GET", path: "/health", handler_name: "health", is_async: true },
  async () => ({ status: "ok" }),
);
```
