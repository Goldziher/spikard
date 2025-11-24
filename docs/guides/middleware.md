# Middleware

Middleware is the right place for cross-cutting behavior like logging, auth, or request shaping. The API mirrors per-language conventions but calls into the same Rust pipeline.

## Add middleware

=== "Python"

    ```python
    def logging_middleware(ctx, next_fn):
        print(f"{ctx.method} {ctx.path}")
        return next_fn()

    app.use(logging_middleware)
    ```

=== "TypeScript"

    ```typescript
    app.use(async (ctx, next) => {
      console.log(`${ctx.method} ${ctx.path}`);
      return next();
    });
    ```

=== "Ruby"

    ```ruby
    App.use do |ctx, next_middleware|
      puts "#{ctx.method} #{ctx.path}"
      next_middleware.call
    end
    ```

=== "Rust"

    ```rust
    use tower_http::trace::TraceLayer;

    let mut app = App::new();
    app.layer(TraceLayer::new_for_http());
    ```

## Patterns
- **Auth guards**: check headers/cookies, enrich context with the authenticated principal, and short-circuit on failures.
- **Observability**: emit structured logs and traces; forward request IDs/correlation IDs.
- **Request shaping**: normalize headers, coerce params, or inject tenant/feature flags.

## Tips
- Keep middleware pure and side-effect free when possible; expensive IO should be async.
- Prefer per-route middleware for sensitive endpoints.
- Use shared context keys to pass data to handlers; keep namespaced to avoid collisions.
