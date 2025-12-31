```typescript
it("enforces auth middleware", async () => {
  const app = new Spikard();

  app.pre("/protected/*", async (req) => {
    if (!req.headers.authorization?.startsWith("Bearer ")) {
      return { status: 401, body: { error: "Unauthorized" } };
    }
    return req;
  });

  app.get("/protected/data", async () => ({ data: "secret" }));

  const client = new TestClient(app);

  // Without auth
  let response = await client.get("/protected/data");
  expect(response.statusCode).toBe(401);

  // With auth
  response = await client.get("/protected/data", {
    authorization: "Bearer token123"
  });
  expect(response.statusCode).toBe(200);
});
```
