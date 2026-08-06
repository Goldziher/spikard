```elixir
alias Spikard.{App, RouteBuilder, Method}

app = Spikard.App.new()

builder = RouteBuilder.new(Method.get(), "/stats")

# Declare the dependency keys the router must resolve before this handler
# runs. Resolving providers (values/factories) is not yet wired into an
# app-level `provide/2` API for the Elixir binding, so until that surface
# lands, read configuration from application env or process state inside
# the handler itself.
builder = RouteBuilder.handler_dependencies(builder, ["config", "db_pool"])

stats_handler = fn _conn ->
  db_url = Application.get_env(:my_app, :db_url, "postgresql://localhost/app")
  %{"db" => db_url, "env" => db_url}
end

app = App.route(app, builder, stats_handler)

App.run(app)
```
