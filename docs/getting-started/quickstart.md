# Quick Start

Build the same minimal service in each binding. Choose a tab, copy the snippet, and run.

## Define routes

=== "Python"

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

=== "TypeScript"

    ```typescript
    import { App } from "spikard";
    import { z } from "zod";

    const User = z.object({ id: z.number(), name: z.string() });
    type User = z.infer<typeof User>;

    const app = new App();

    app.get("/users/:id", ({ params }): User => ({
      id: Number(params.id),
      name: "Alice",
    }));

    app.post("/users", ({ body }): User => User.parse(body));

    app.listen({ port: 8000 });
    ```

=== "Ruby"

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

    App.run(port: 8000)
    ```

=== "Rust"

    ```rust
    use serde::{Deserialize, Serialize};
    use spikard::prelude::*;

    #[derive(Serialize, Deserialize)]
    struct User {
        id: i64,
        name: String,
    }

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = App::new();

        app.route(get("/users/:id"), |ctx: Context| async move {
            let id = ctx.path_param("id").unwrap_or("0").parse::<i64>().unwrap_or_default();
            Ok(Json(User { id, name: "Alice".into() }))
        })?;

        app.route(
            post("/users").request_body::<User>().response_body::<User>(),
            |ctx: Context| async move {
                let user: User = ctx.json()?;
                Ok(Json(user))
            },
        )?;

        app.run().await?;
        Ok(())
    }
    ```

## Run it

- Python: `python app.py` or `spikard run app.py`
- TypeScript: `pnpm ts-node app.ts` (or your runtime of choice), then hit `http://localhost:8000/users/1`
- Ruby: `ruby app.rb`
- Rust: `cargo run` inside your crate/binary

## Next steps
- Add middleware (logging, auth, tracing) with the same signature in every binding.
- Wire JSON Schema validation so request/response contracts stay enforced.
- Deploy using the Rust binary, the CLI, or container images (see [Deployment](../guides/deployment.md)).
