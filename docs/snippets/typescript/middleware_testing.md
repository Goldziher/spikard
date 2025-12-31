```typescript
import { describe, it, expect } from "vitest";
import { authGuard } from "./middleware";

describe("authGuard", () => {
  it("allows valid token", async () => {
    const request = {
      headers: { authorization: "Bearer valid-jwt-token" },
      method: "GET",
      path: "/api/users",
    };

    const result = await authGuard(request);

    expect(result.context?.userId).toBeDefined();
  });

  it("rejects missing token", async () => {
    const request = {
      headers: {},
      method: "GET",
      path: "/api/users",
    };

    await expect(authGuard(request)).rejects.toThrow("401");
  });
});
```
