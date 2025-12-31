```typescript
it("creates user", async () => {
  const app = new Spikard();
  app.post("/users", async (req) => ({
    id: 1,
    name: req.body.name,
    email: req.body.email
  }));

  const client = new TestClient(app);
  const response = await client.post("/users", {
    json: { name: "Alice", email: "alice@example.com" }
  });

  expect(response.statusCode).toBe(200);
  const data = response.json();
  expect(data.name).toBe("Alice");
});
```
