```typescript
it("completes user workflow", async () => {
  const usersDb = new Map();
  const app = new Spikard();

  app.post("/users", async (req) => {
    const id = usersDb.size + 1;
    const user = { id, name: req.body.name };
    usersDb.set(id, user);
    return user;
  });

  app.get("/users/:id", async (req) => {
    const user = usersDb.get(Number(req.params.id));
    return user || { status: 404, body: { error: "Not found" } };
  });

  const client = new TestClient(app);

  // Create user
  const createRes = await client.post("/users", {
    json: { name: "Alice" }
  });
  const user = createRes.json();
  expect(user.name).toBe("Alice");

  // Retrieve user
  const getRes = await client.get(`/users/${user.id}`);
  expect(getRes.json()).toEqual(user);
});
```
