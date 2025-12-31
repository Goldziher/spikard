```typescript
import { describe, it, expect } from "vitest";
import { Spikard, TestClient } from "@spikard/app";

const app = new Spikard();
app.get("/hello", async () => ({ message: "Hello, World!" }));

describe("Hello endpoint", () => {
  it("returns greeting", async () => {
    const client = new TestClient(app);
    const response = await client.get("/hello");

    expect(response.statusCode).toBe(200);
    expect(response.json()).toEqual({ message: "Hello, World!" });
  });
});
```
