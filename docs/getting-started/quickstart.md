# Quick Start

A minimal Spikard service looks familiar in every language. Pick your binding and run the same app structure with consistent routing, validation, and middleware semantics.

## Python
```python
from spikard import App
from msgspec import Struct

class User(Struct):
    id: int
    name: str

app = App()

@app.get("/users/{id:int}")
async def get_user(id: int) -> User:
    return User(id=id, name="Alice")

@app.post("/users")
async def create_user(user: User) -> User:
    return user

if __name__ == "__main__":
    app.run(port=8000)
```
Run with `python app.py` or through the CLI: `spikard run app.py`.

## TypeScript / Node.js
```typescript
import { App } from "spikard";
import { z } from "zod";

const UserSchema = z.object({
  id: z.number(),
  name: z.string(),
});

type User = z.infer<typeof UserSchema>;

const app = new App();

app.get("/users/:id", ({ params }): User => ({
  id: Number(params.id),
  name: "Alice",
}));

app.post("/users", async ({ body }): Promise<User> => {
  const user = UserSchema.parse(body);
  return user;
});

app.listen({ port: 8000 });
```

## Ruby
```ruby
require "spikard"

App = Spikard::App.new

App.get("/users/:id") do |ctx|
  { id: ctx.params[:id].to_i, name: "Alice" }
end

App.post("/users") do |ctx|
  user = ctx.json
  { id: user["id"], name: user["name"] }
end

App.listen(port: 8000)
```

## Rust
```rust
use spikard::prelude::*;

fn main() {
    let app = App::new()
        .get("/users/:id", |ctx: Context| async move {
            let id = ctx.path_param("id").unwrap_or_default();
            Ok(Json(json!({ "id": id.parse::<i64>().unwrap_or(0), "name": "Alice" })))
        })
        .post("/users", |ctx: Context| async move {
            let user: serde_json::Value = ctx.json()?;
            Ok(Json(user))
        });

    app.listen(8000).unwrap();
}
```

## Next Steps
- Add middleware (logging, auth, tracing) with the same signature in every binding.
- Wire JSON Schema validation so request/response contracts stay enforced.
- Deploy using the Rust binary, the CLI, or container images (see [Deployment](../guides/deployment.md)).
