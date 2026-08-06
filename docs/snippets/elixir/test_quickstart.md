```elixir
defmodule HelloRouteTest do
  use ExUnit.Case, async: true

  setup do
    app = Spikard.App.new()

    app =
      Spikard.App.get(app, "/hello", fn _conn ->
        %{"message" => "Hello, World!"}
      end)

    app = Spikard.App.config(app, %Spikard.ServerConfig{host: "127.0.0.1", port: 4123})

    {:ok, pid} = Task.start_link(fn -> Spikard.App.run(app) end)
    Process.sleep(100)
    on_exit(fn -> Process.exit(pid, :kill) end)

    :ok
  end

  test "returns a greeting" do
    {:ok, response} = Req.get(url: "http://127.0.0.1:4123/hello")

    assert response.status == 200
    assert response.body == %{"message" => "Hello, World!"}
  end
end
```
