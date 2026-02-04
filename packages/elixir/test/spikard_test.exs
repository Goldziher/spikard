defmodule SpikardTest do
  use ExUnit.Case
  doctest Spikard

  describe "start/1" do
    test "returns error for not implemented" do
      assert {:error, :not_implemented} = Spikard.start(port: 4000, routes: [])
    end
  end

  describe "stop/1" do
    test "returns error for not implemented" do
      assert {:error, :not_implemented} = Spikard.stop(:fake_server)
    end
  end
end
