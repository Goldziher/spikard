# Spikard Documentation

Spikard is a polyglot API toolkit with a Rust core and first-class bindings for Python, TypeScript/Node, Ruby, and Rust. It keeps routing, middleware, validation, and streaming semantics identical across languages so teams can mix runtimes without relearning frameworks.

## Hello Route (pick a binding)

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

    app.listen({ port: 8000 });
    ```

=== "Ruby"

    ```ruby
    require "spikard"

    App = Spikard::App.new

    App.get("/users/:id") do |ctx|
      { id: ctx.params[:id].to_i, name: "Alice" }
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

        app.run().await?;
        Ok(())
    }
    ```

## Documentation Map

- **[Getting Started](getting-started/quickstart.md)** – First route in each language plus how to run it.
- **[Installation](getting-started/installation.md)** – Binding install commands and repo setup.
- **[Guides](guides/routing.md)** – Routing, requests/responses, middleware, validation, deployment.
- **[Concepts](concepts/architecture.md)** – Architecture, runtime model, validation, middleware, streaming internals.
- **[Reference](reference/api-python.md)** – Language APIs, configuration surface, types, and error semantics.
- **[CLI](cli/usage.md)** – Running the HTTP server and invoking generators from `spikard-cli`.
- **[ADRs](adr/README.md)** – Design history and rationale behind the runtime.

## Getting Help

- **Questions / bugs**: open an issue at [github.com/Goldziher/spikard](https://github.com/Goldziher/spikard).
- **Chat**: join the community Discord (`https://discord.gg/pXxagNK2zN`).
- **Contributing**: see [Contributing](contributing.md) for coding standards, environment setup, and testing instructions.
