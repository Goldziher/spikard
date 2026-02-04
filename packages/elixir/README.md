# Spikard

High-performance HTTP framework for Elixir powered by Rust.

## Installation

Add `spikard` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:spikard, "~> 0.10"}
  ]
end
```

## Quick Start

```elixir
defmodule MyApp.Router do
  use Spikard.Router

  get "/", &hello/1

  defp hello(_request) do
    Spikard.Response.json(%{message: "Hello, World!"})
  end
end

# Start the server
{:ok, _server} = Spikard.start(port: 4000, routes: MyApp.Router.routes())
```

## License

MIT
