defmodule E2EElixirApp.GraphqlTest do
  @moduledoc """
  Generated tests for graphql fixtures.

  Each test starts its own isolated server with only its route,
  avoiding conflicts when multiple fixtures share the same path.
  """
  use ExUnit.Case, async: false

  alias E2EElixirApp.AppFactories

  @base_url "http://127.0.0.1:59800"

  setup do
    :inets.start()
    :ssl.start()
    :ok
  end
end
