```typescript
it("rejects invalid input", async () => {
  const app = new Spikard();
  app.post("/users", async (req) => {
    if (!req.body.name || typeof req.body.age !== "number") {
      return { status: 400, body: { error: "Validation failed" } };
    }
    return { name: req.body.name, age: req.body.age };
  });

  const client = new TestClient(app);
  const response = await client.post("/users", {
    json: { name: "Bob", age: "invalid" }
  });

  expect(response.statusCode).toBe(400);
  expect(response.json().error).toContain("Validation");
});
```
