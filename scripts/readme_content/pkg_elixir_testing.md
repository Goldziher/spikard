Use the `TestClient` for integration tests without network overhead:

```elixir
defmodule MyAppTest do
  use ExUnit.Case

  test "GET / returns hello" do
    {:ok, client} = Spikard.TestClient.new(routes: MyApp.Router.routes())
    {:ok, response} = Spikard.TestClient.get(client, "/")

    assert response.status_code == 200
    assert Spikard.TestClient.Response.json(response) == %{"message" => "Hello, World!"}
  end
end
```
