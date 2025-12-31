```typescript
import { describe, it, expect } from "vitest";
import request from "supertest";

describe("User creation validation", () => {
  it("accepts valid requests", async () => {
    const response = await request(app)
      .post("/users")
      .send({
        email: "test@example.com",
        age: 25,
        username: "testuser"
      });

    expect(response.status).toBe(200);
  });

  it("rejects invalid email", async () => {
    const response = await request(app)
      .post("/users")
      .send({
        email: "not-an-email",
        age: 25,
        username: "testuser"
      });

    expect(response.status).toBe(422);
    expect(response.body.details[0].field).toContain("email");
  });

  it("rejects age below minimum", async () => {
    const response = await request(app)
      .post("/users")
      .send({
        email: "test@example.com",
        age: 16,
        username: "testuser"
      });

    expect(response.status).toBe(422);
  });

  it("rejects missing required fields", async () => {
    const response = await request(app)
      .post("/users")
      .send({
        email: "test@example.com",
        age: 25
      });

    expect(response.status).toBe(422);
  });
});
```
